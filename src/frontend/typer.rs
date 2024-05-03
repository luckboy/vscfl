//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::rc::*;
use crate::frontend::builtins::*;
use crate::frontend::error::*;
use crate::frontend::namer::*;
use crate::frontend::parser::*;
use crate::frontend::tree::*;
use crate::frontend::type_matcher::*;
use crate::utils::dfs::*;
use crate::utils::env::*;

fn type_expr_pos(type_expr: &TypeExpr) -> &Pos
{
    match type_expr {
        TypeExpr::Tuple(_, pos) => pos,
        TypeExpr::Fun(_, _, pos) => pos,
        TypeExpr::Array(_, _, pos) => pos,
        TypeExpr::Param(_, pos) => pos,
        TypeExpr::Var(_, pos) => pos,
        TypeExpr::App(_, _, pos) => pos,
        TypeExpr::Uniq(_, pos) => pos,
    }
}

fn expr_pos(expr: &Expr) -> &Pos
{
    match expr {
        Expr::Literal(_, _, pos) => pos,
        Expr::Lambda(_, _, _, _, _, _, _, pos) => pos,
        Expr::Var(_, _, pos) => pos,
        Expr::NamedFieldConApp(_, _, _, _, pos) => pos,
        Expr::PrintfApp(_, _, pos) => pos,
        Expr::App(_, _, _, pos) => pos,
        Expr::GetField(_, _, _, pos) => pos,
        Expr::Get2Field(_, _, _, pos) => pos,
        Expr::SetField(_, _, _, _, pos) => pos,
        Expr::UpdateField(_, _, _, _, pos) => pos,
        Expr::UpdateGet2Field(_, _, _, _, pos) => pos,
        Expr::Uniq(_, _, pos) => pos,
        Expr::Shared(_, _, pos) => pos,
        Expr::Typed(_, _, _, pos) => pos,
        Expr::As(_, _, _, pos) => pos,
        Expr::Let(_, _, _, pos) => pos,
        Expr::If(_, _, _, _, pos) => pos,
        Expr::Match(_, _, _, pos) => pos,
    }
}

fn pattern_pos(pattern: &Pattern) -> &Pos
{
    match pattern {
        Pattern::Literal(_, _, pos) => pos,
        Pattern::As(_, _, _, _, pos) => pos,
        Pattern::Const(_, _, pos) => pos,
        Pattern::UnnamedFieldCon(_, _, _, _, pos) => pos,
        Pattern::NamedFieldCon(_, _, _, _, pos) => pos,
        Pattern::Var(_, _, _, pos) => pos,
        Pattern::At(_, _, _, _, pos) => pos,
        Pattern::Wildcard(_, pos) => pos,
        Pattern::Alt(_, _, pos) => pos,
    }
}

fn add_error(err: FrontendError, errs2: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    match err {
        FrontendError::Internal(msg) => return Err(FrontendErrors::new(vec![FrontendError::Internal(msg.clone())])),
        _ => {
            errs2.push(err);
            Ok(())
        },
    }
}

fn add_errors(errs: &mut FrontendErrors, errs2: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    for err in errs.errors() {
        match err {
            FrontendError::Internal(msg) => return Err(FrontendErrors::new(vec![FrontendError::Internal(msg.clone())])),
            _ => (),
        }
    }
    errs.append_to(errs2);
    Ok(())
}

//
// Evaluation of types for type variables.
//

fn add_type_synonym_ident(ident: &String, pos: Pos, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(_)) => Ok(()),
                TypeVar::Synonym(_, _, None) => {
                    if !processed_idents.contains(ident) {
                        idents.push(ident.clone());
                    } else {
                        errs.push(FrontendError::Message(pos, format!("definition of type synonym {} is recursive", ident)));
                    }
                    Ok(())
                },
                _ => Ok(()),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no type variable"))])),
    }
}

fn add_type_ident(ident: &String, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>) -> FrontendResultWithErrors<()>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let mut type_var_r = type_var.borrow_mut();
            match &mut *type_var_r {
                TypeVar::Builtin(_, _, Some(_)) => Ok(()),
                TypeVar::Builtin(_, _, shared_flag @ None) => {
                    if !processed_idents.contains(ident) {
                        idents.push(ident.clone());
                    } else {
                        *shared_flag = Some(SharedFlag::Shared);
                    }
                    Ok(())
                },
                TypeVar::Data(_, _, Some(_)) => Ok(()),
                TypeVar::Data(_, _, shared_flag @ None) => {
                    if !processed_idents.contains(ident) {
                        idents.push(ident.clone());
                    } else {
                        *shared_flag = Some(SharedFlag::Shared);
                    }
                    Ok(())
                },
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_type_ident: type variable is type synonym"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_type_ident: no type variable"))])),
    }
}

fn add_data_ident(ident: &String, pos: Pos, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let mut type_var_r = type_var.borrow_mut();
            match &mut *type_var_r {
                TypeVar::Builtin(_, _, _) => Ok(()),
                TypeVar::Data(_, _, _) => {
                    if !processed_idents.contains(ident) {
                        idents.push(ident.clone());
                    } else {
                        errs.push(FrontendError::Message(pos, format!("recursive type {} must be in reference type", ident)));
                    }
                    Ok(())
                },
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_data_ident: type variable is type synonym"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_data_ident: no type variable"))])),
    }
}

fn add_type_param_local_type(ident: &String, type_param_env: &mut Environment<LocalType>, local_type_counter: &mut usize) -> LocalType
{
    match type_param_env.var(ident) {
        Some(local_type) => *local_type,
        None => {
            let local_type = LocalType::new(*local_type_counter);
            type_param_env.add_var(ident.clone(), local_type);
            *local_type_counter += 1;
            local_type
        },
    }
}

fn local_type_for_type_param_ident(ident: &String, type_param_env: &Environment<LocalType>) -> FrontendResultWithErrors<LocalType>
{
    match type_param_env.var(ident) {
        Some(local_type) => Ok(*local_type),
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_type_param_ident: no type parameter"))])),
    }
}

fn type_value_and_type_arg_count_for_type_var_ident(ident: &String, pos: Pos, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<(Rc<TypeValue>, usize)>>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), _, _) => {
                    let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                    for i in 0..type_args.type_arg_idents().len() {
                        type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(i))));
                    }
                    Ok(Some((Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(ident.clone()), type_values)), type_args.type_arg_idents().len())))
                },
                TypeVar::Builtin(None, _, _) => {
                    errs.push(FrontendError::Message(pos, format!("built-in type {} hasn't evalauted type arguments", ident)));
                    Ok(None)
                },
                TypeVar::Data(type_args, _, _) => {
                    let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                    for i in 0..type_args.len() {
                        type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(i))));
                    }
                    Ok(Some((Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(ident.clone()), type_values)), type_args.len())))
                },
                TypeVar::Synonym(type_args, _, Some(type_value)) => {
                    Ok(Some((type_value.clone(), type_args.len())))
                },
                TypeVar::Synonym(_, _, None) => {
                    errs.push(FrontendError::Message(pos, format!("unevaluated type synonym {}", ident)));
                    Ok(None)
                },
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_value_and_type_arg_count_for_type_var_ident: no type variable"))])),
    }
}

fn shared_flag_for_type_ident_and_evaluation(ident: &String, tree: &Tree) -> FrontendResultWithErrors<Option<SharedFlag>>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let mut type_var_r = type_var.borrow_mut();
            match &mut *type_var_r {
                TypeVar::Builtin(_, _, shared_flag) => Ok(*shared_flag),
                TypeVar::Data(_, _, shared_flag) => Ok(*shared_flag),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_ident_and_evaluation: type variable is type synonym"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_ident_and_evaluation: no type variable"))])),
    }
}

//
// Evaluation of types for variables.
//

fn type_arg_count_for_type_ident(ident: &String, tree: &Tree) -> FrontendResultWithErrors<usize>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), _, _) => Ok(type_args.type_arg_idents().len()),
                TypeVar::Builtin(None, _, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_arg_count_for_type_ident: no type arguments"))])),
                TypeVar::Data(type_args, _, _) => Ok(type_args.len()),
                TypeVar::Synonym(_, _, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_arg_count_for_type_ident: type variable is type synonym"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_arg_count_for_type_ident: no type variable"))])),
    }
}

fn type_arg_count_for_trait_ident(ident: &String, tree: &Tree) -> FrontendResultWithErrors<usize>
{
    match tree.trait1(ident) {
        Some(trait1) => {
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(type_args, _, _) => Ok(type_args.len()),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_arg_count_for_trait_ident: no type variable"))])),
    }
}

fn shared_flag_for_type_ident(ident: &String, tree: &Tree) -> FrontendResultWithErrors<SharedFlag>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let mut type_var_r = type_var.borrow_mut();
            match &mut *type_var_r {
                TypeVar::Builtin(_, _, Some(shared_flag)) => Ok(*shared_flag),
                TypeVar::Data(_, _, Some(shared_flag)) => Ok(*shared_flag),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_ident: type variable is type synonym or no shared flag"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_ident: no type variable"))])),
    }
}

fn shared_flag_for_type_name(type_name: &TypeName, tree: &Tree) -> FrontendResultWithErrors<SharedFlag>
{
    match type_name {
        TypeName::Tuple(_) => Ok(SharedFlag::Shared),
        TypeName::Array(_) => Ok(SharedFlag::Shared),
        TypeName::Fun(_) => Ok(SharedFlag::Shared),
        TypeName::Name(ident) => shared_flag_for_type_ident(ident, tree),
    }
}

fn add_local_type_for_recursion(local_type: LocalType, pos: Pos, typ: &Type, local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    if !processed_local_types.contains(&local_type) {
        local_types.push(local_type);
        Ok(())
    } else {
        match typ.type_param_entry(local_type) {
            Some(type_param_entry) => {
                let type_param_entry_r = type_param_entry.borrow();
                match &type_param_entry_r.ident {
                    Some(ident) => {
                        errs.push(FrontendError::Message(pos, format!("trait definition of type parameter {} is recursive", ident)));
                        Ok(())
                    },
                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_local_type_for_recursion: no identifier"))]))
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_local_type_for_recursion: no type parameter entry"))])),
        }
    }
}

fn new_type_from_type_value_and_type_param_env(type_value: Rc<TypeValue>, type_param_env: &Environment<LocalType>, local_type_counter: Option<usize>) -> FrontendResultWithErrors<Type>
{
    match local_type_counter {
        Some(local_type_counter) => {
            let mut idents: Vec<Option<String>> = vec![None; local_type_counter];
            type_param_env.foreach(|ident, local_type| idents[local_type.index()] = Some(ident.clone()));
            let mut idents2: Vec<String> = Vec::new();
            for ident in idents {
                match ident {
                    Some(ident) => idents2.push(ident),
                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("new_type_from_type_value_and_type_param_env: no identifier"))])),
                }
            }
            Ok(Type::new(type_value, idents2.as_slice()))
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("new_type_from_type_value_and_type_param_env: no local type counter"))])),
    }
}

fn merge_tuples(tuple1: &(LocalType, usize, Pos), tuple2: &(LocalType, usize, Pos)) -> (LocalType, usize, Pos)
{
    if tuple1.1 > tuple2.1 {
        tuple1.clone()
    } else {
        tuple2.clone()
    }
}

fn add_local_type_for_substitution(local_type: LocalType, type_values: &[Rc<TypeValue>], local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>) -> FrontendResultWithErrors<()>
{
    if !processed_local_types.contains(&local_type) {
        if local_type.index() < type_values.len() {
            match &*type_values[local_type.index()] {
                TypeValue::Type(_, _, _) => local_types.push(local_type),
                _ => (),
            }
            Ok(())
        } else {
            Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_local_type_for_substitution: no type value"))]))
        }
    } else {
        Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_local_type_for_substitution: trait definition of type parameter is recursive"))]))
    }
}

//
// Inference of types.
//

fn type_for_var_ident_in<T, F>(ident: &String, tree: &Tree, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&Type) -> FrontendResultWithErrors<T>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Builtin(_, Some(typ)) => f(typ),
                Var::Var(_, _, _, _, _, _, _, Some(typ), _) => f(typ),
                Var::Fun(_, _, Some(typ)) => f(typ),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_var_ident_in: no type"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_var_ident_in: no variable"))])),
    }
}

fn type_for_fun_ident_in<T, F>(ident: &String, tree: &Tree, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&Type) -> FrontendResultWithErrors<T>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(_, _, Some(typ)) => f(typ),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_fun_ident_in: variable isn't function or no type"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_fun_ident_in: no variable"))])),
    }
}

fn type_and_named_fields_for_con_ident_in<T, F>(ident: &String, tree: &Tree, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&Type, &NamedFields) -> FrontendResultWithErrors<T>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, Some(typ)) => {
                    match &**fun {
                        Fun::Con(con) => {
                            let con_r = con.borrow();
                            match &*con_r {
                                Con::NamedField(_, _, _, Some(named_fields), _) => f(&**typ, &**named_fields),
                                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_and_named_fields_for_con_ident_in: constructor isn't named field contructor or no named fields"))])),
                            }
                        },
                        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_and_named_fields_for_con_ident_in: function isn't contructor"))])),
                    }
                },
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_and_named_fields_for_con_ident_in: variable isn't function or no type"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_and_named_fields_for_con_ident_in: no variable"))])),
    }
}

fn set_type_for_local_types(local_type: LocalType, typ: &Type, local_types: &mut LocalTypes) -> FrontendResultWithErrors<()>
{
    match local_types.set_type(local_type, typ) {
        Ok(true) => Ok(()),
        Ok(false) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("set_type_for_local_types: no local type"))])),
        Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("set_type_for_local_types: {}", err))])),
    }
}

#[derive(Clone)]
struct ClosureStack
{
    stack: Vec<(BTreeMap<(String, usize), LocalType>, usize)>,
}

impl ClosureStack
{
    fn new() -> Self
    { ClosureStack { stack: Vec::new(), } }
    
    fn push_new_closure(&mut self, stack_idx: usize)
    { self.stack.push((BTreeMap::new(), stack_idx)) }
    
    fn merge_and_pop_closure(&mut self)
    {
        if self.stack.len() >= 2 {
            let local_types = &self.stack[self.stack.len() - 1].0;
            let mut new_local_types = self.stack[self.stack.len() - 2].0.clone();
            let new_stack_idx = self.stack[self.stack.len() - 2].1;
            for (key @ (_, stack_idx), local_type) in local_types {
                if *stack_idx < new_stack_idx {
                    if !new_local_types.contains_key(key) {
                        new_local_types.insert(key.clone(), *local_type);
                    }
                }
            }
            let len = self.stack.len();
            self.stack[len - 2].0 = new_local_types;
        }
        self.stack.pop();
    }
    
    fn add_local_type(&mut self, key: (String, usize), local_type: LocalType) -> bool
    {
        match self.stack.last_mut() {
            Some((local_types, stack_idx)) => {
                if key.1 < *stack_idx {
                    local_types.insert(key, local_type);
                    true
                } else {
                    false
                }
            },
            None => false,
        }
    }
    
    pub fn foreach_with_result<E, F>(&self, mut f: F) -> Result<(), E>
        where F: FnMut(&(String, usize), LocalType) -> Result<(), E>
    {
        match self.stack.last() {
            Some((local_types, _)) => {
                for (key, local_type) in local_types {
                    f(key, *local_type)?;
                }
            },
            None => (),
        }
        Ok(())
    }
}

pub struct Typer
{
    type_matcher: TypeMatcher,
    builtins: Builtins,
}

impl Typer
{
    pub fn new() -> Self
    { Typer { type_matcher: TypeMatcher::new(), builtins: Builtins::new(), } }

    pub fn new_with_builtins(builtins: Builtins) -> Self
    { Typer { type_matcher: TypeMatcher::new(), builtins, } }
    
    pub fn builtins(&self) -> &Builtins
    { &self.builtins }

    pub fn builtins_mut(&mut self) -> &mut Builtins
    { &mut self.builtins }
    
    pub fn set_builtins(&mut self, builtins: Builtins)
    { self.builtins = builtins; }

    pub fn evaluate_types_for_type_vars(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.evaluate_type_args_for_builtin_type_defs(tree, &mut errs)?;
        self.evaluate_types_for_type_synonym_defs(tree, &mut errs)?;
        self.evaluate_types_for_type_defs(tree, &mut errs)?;
        self.evaluate_shared_flags_for_type_defs(tree)?;
        self.check_type_recursions_for_data_defs(tree, &mut errs)?;
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }

    pub fn evaluate_types_for_vars(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.check_type_arg_counts_for_impl_defs(tree, &mut errs)?;
        self.evaluate_types_for_var_and_trait_defs(tree, &mut errs)?;
        self.check_impls_for_impl_defs(tree, &mut errs)?;
        self.evaluate_types_for_impl_defs(tree, &mut errs)?;
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }

    pub fn evaluate_types(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        self.evaluate_types_for_type_vars(tree)?;
        self.evaluate_types_for_vars(tree)?;
        Ok(())
    }

    pub fn infer_types(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.infer_types_for_defs(tree, &mut errs)?;
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }

    pub fn check_types(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        self.evaluate_types(tree)?;
        self.infer_types(tree)?;
        Ok(())
    }
    
    pub fn evalute_type_with_where(&self, ident: &str, type_expr: &TypeExpr, where_tuples: &[WhereTuple], trait_ident: &Option<String>, pos: &Pos, tree: &Tree) -> FrontendResultWithErrors<Type>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        let mut type_param_env: Environment<LocalType> = Environment::new();
        let mut local_type_counter = Some(0usize);
        type_param_env.push_new_vars();
        match self.evaluate_type_for_type_expr(type_expr, tree, &mut type_param_env, &mut local_type_counter, &mut errs)? {
            Some(type_value) => {
                let mut typ = new_type_from_type_value_and_type_param_env(type_value, &type_param_env, local_type_counter)?;
                if self.evaluate_types_for_where_tuples(ident, where_tuples, trait_ident, pos.clone(), tree, &mut type_param_env, &mut typ, &mut errs)? {
                    Ok(typ)
                } else  {
                    Err(FrontendErrors::new(errs))
                }
            },
            None => Err(FrontendErrors::new(errs)),
        }
    }
    
    fn uniq_flag_and_shared_flag_for_local_type(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<(UniqFlag, SharedFlag)>
    {
        match self.type_matcher.uniq_flag_and_shared_flag(local_type, tree, local_types) {
            Ok(pair) => Ok(pair),
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }

    fn real_uniq_flag_for_type_value(&self, type_value: &Rc<TypeValue>, local_types: &LocalTypes) -> FrontendResultWithErrors<UniqFlag>
    {
        match self.type_matcher.real_uniq_flag_for_type_value(type_value, local_types) {
            Ok(uniq_flag) => Ok(uniq_flag),
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }
    
    fn shared_flag_for_local_type(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<SharedFlag>
    {
        match self.type_matcher.shared_flag(local_type, tree, local_types) {
            Ok(shared_flag) => Ok(shared_flag),
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }
    
    fn set_shared_for_local_type_and_var(&self, ident: &str, local_type: LocalType, pos: &Pos, tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match self.type_matcher.set_shared(local_type, tree, local_types) {
            Ok(true) => Ok(()),
            Ok(false) => {
                errs.push(FrontendError::Message(pos.clone(), format!("variable {} mustn't be shared with type {}", ident, LocalTypeWithLocalTypes(local_type, local_types))));
                Ok(())
            },
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }

    fn set_shared_for_local_type_and_value(&self, local_type: LocalType, pos: &Pos, tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match self.type_matcher.set_shared(local_type, tree, local_types) {
            Ok(true) => Ok(()),
            Ok(false) => {
                errs.push(FrontendError::Message(pos.clone(), format!("value mustn't be shared with type {}", LocalTypeWithLocalTypes(local_type, local_types))));
                Ok(())
            },
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }
    
    fn match_type_values(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, pos: &Pos, tree: &Tree, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match self.type_matcher.match_type_values(type_value1, type_value2, tree, local_types) {
            Ok(TypeMatcherResult::Matched) => Ok(()),
            Ok(TypeMatcherResult::Mismatched(infos)) => {
                errs.push(FrontendError::Message(pos.clone(), format!("can't match type {} with type {}", TypeValueWithLocalTypes(type_value1.clone(), local_types), TypeValueWithLocalTypes(type_value2.clone(), local_types))));
                for info in &infos {
                    errs.push(FrontendError::Message(pos.clone(), format!("{}", MismatchedTypeInfoWidthLocalTypes(info, local_types))));
                }
                Ok(())
            },
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }
 
    fn match_local_types(&self, local_type1: LocalType, local_type2: LocalType, pos: &Pos, tree: &Tree, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match self.type_matcher.matches(local_type1, local_type2, tree, local_types) {
            Ok(TypeMatcherResult::Matched) => Ok(()),
            Ok(TypeMatcherResult::Mismatched(infos)) => {
                errs.push(FrontendError::Message(pos.clone(), format!("can't match type {} with type {}", LocalTypeWithLocalTypes(local_type1, local_types), LocalTypeWithLocalTypes(local_type2, local_types))));
                for info in &infos {
                    errs.push(FrontendError::Message(pos.clone(), format!("{}", MismatchedTypeInfoWidthLocalTypes(info, local_types))));
                }
                Ok(())
            },
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }

    fn match_local_types_for_first_pattern_type(&self, local_type1: LocalType, local_type2: LocalType, pos: &Pos, tree: &Tree, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match self.type_matcher.match_for_first_pattern_type(local_type1, local_type2, tree, local_types) {
            Ok(TypeMatcherResult::Matched) => Ok(()),
            Ok(TypeMatcherResult::Mismatched(infos)) => {
                errs.push(FrontendError::Message(pos.clone(), format!("can't match type {} with type {}", LocalTypeWithLocalTypes(local_type1, local_types), LocalTypeWithLocalTypes(local_type2, local_types))));
                for info in &infos {
                    errs.push(FrontendError::Message(pos.clone(), format!("{}", MismatchedTypeInfoWidthLocalTypes(info, local_types))));
                }
                Ok(())
            },
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }    

    fn match_local_types_for_second_pattern_type(&self, local_type1: LocalType, local_type2: LocalType, pos: &Pos, tree: &Tree, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match self.type_matcher.match_for_second_pattern_type(local_type1, local_type2, tree, local_types) {
            Ok(TypeMatcherResult::Matched) => Ok(()),
            Ok(TypeMatcherResult::Mismatched(infos)) => {
                errs.push(FrontendError::Message(pos.clone(), format!("can't match type {} with type {}", LocalTypeWithLocalTypes(local_type1, local_types), LocalTypeWithLocalTypes(local_type2, local_types))));
                for info in &infos {
                    errs.push(FrontendError::Message(pos.clone(), format!("{}", MismatchedTypeInfoWidthLocalTypes(info, local_types))));
                }
                Ok(())
            },
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
    }    
    
    fn match_local_type_entries_for_casting(&self, local_type_entry1: &LocalTypeEntry, local_type_entry2: &LocalTypeEntry, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<bool>
    {
        match (local_type_entry1, local_type_entry2) {
            (LocalTypeEntry::Type(type_value1), LocalTypeEntry::Type(type_value2)) => {
                match (&**type_value1, &**type_value2) {
                    (TypeValue::Type(uniq_flag1, type_value_name1, type_values1), TypeValue::Type(uniq_flag2, type_value_name2, type_values2)) => {
                        if uniq_flag1 != uniq_flag2 {
                            return Ok(false);
                        }
                        let mut is_success = true;
                        match (type_value_name1, type_value_name2) {
                            (TypeValueName::Name(ident1), TypeValueName::Name(ident2)) => {
                                return Ok(self.has_primitive_for_type_ident(ident1, tree)? && self.has_primitive_for_type_ident(ident2, tree)?);
                            },
                            (TypeValueName::Tuple, TypeValueName::Tuple) => (),
                            (TypeValueName::Array(Some(len1)), TypeValueName::Array(Some(len2))) if len1 == len2 => (),
                            _ => is_success = false,
                        }
                        if !is_success {
                            return Ok(false);
                        }
                        if type_values1.len() != type_values2.len() {
                            return Ok(false);
                        }
                        for (type_value1, type_value2) in type_values1.iter().zip(type_values2.iter()) {
                            if !self.match_type_values_for_casting(type_value1, type_value2, tree, local_types)? {
                                is_success = false;
                            }
                        }
                        if !is_success {
                            return Ok(false);
                        }
                        Ok(true)
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("match_local_type_entries_for_casting: no variable"))])),
                }
            },
            _ => Ok(false),
        }
    }

    fn match_type_values_for_casting(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<bool>
    {
        let local_type_entry1 = local_types.type_entry_for_type_value(type_value1);
        let local_type_entry2 = local_types.type_entry_for_type_value(type_value2);
        match (local_type_entry1, local_type_entry2) {
            (Some(local_type_entry1), Some(local_type_entry2)) => self.match_local_type_entries_for_casting(&local_type_entry1, &local_type_entry2, tree, local_types),
            (_, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("match_type_values_for_casting: no local type entry"))])),
        }
    }

    fn match_local_types_for_casting(&self, local_type1: LocalType, local_type2: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<bool>
    {
        let type_value1 = Rc::new(TypeValue::Param(UniqFlag::None, local_type1));
        let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, local_type2));
        self.match_type_values_for_casting(&type_value1, &type_value2, tree, local_types)
    }
    
    fn cast_local_type(&self, local_type1: LocalType, local_type2: LocalType, pos: &Pos, tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        if !self.match_local_types_for_casting(local_type1, local_type2, tree, local_types)? {
            errs.push(FrontendError::Message(pos.clone(), format!("can't cast type {} to type {}", LocalTypeWithLocalTypes(local_type1, local_types), LocalTypeWithLocalTypes(local_type2, local_types))));
        }
        Ok(())
    }
    
    //
    // Evaluation of types for type variables.
    //    
    
    fn evaluate_type_args_for_builtin_type_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Type(ident, type_var, pos) => {
                    let mut type_var_r = type_var.borrow_mut();
                    self.evaluate_type_args_for_builtin_type(ident, &mut *type_var_r, pos.clone(), errs)?;
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn evaluate_type_args_for_builtin_type(&self, ident: &String, type_var: &mut TypeVar, pos: Pos, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_var {
            TypeVar::Builtin(type_args, _, _) => {
                match self.builtins.type_var(ident) {
                    Some(builtin_type_var) => {
                        match parse_type_args_with_path(format!("({} type args).vscfl", ident).as_str(), builtin_type_var.type_arg_source.as_str()) {
                            Ok(type_args2) => {
                                match check_idents_for_type_args(type_args2.as_slice()) {
                                    Ok(()) => {
                                        let mut new_type_args = TypeArgs::new();
                                        for type_arg2 in type_args2 {
                                            match type_arg2 {
                                                TypeArg(type_arg_ident, _) => new_type_args.add_type_arg_ident(type_arg_ident),
                                            }
                                        }
                                        *type_args = Some(Box::new(new_type_args));
                                    },
                                    Err(mut errs2) => add_errors(&mut errs2, errs)?,
                                }
                            },
                            Err(err) => add_error(err, errs)?,
                        }
                    },
                    None => errs.push(FrontendError::Message(pos, format!("type variable {} mustn't be built-in type variable", ident))),
                }
            },
            _ => (),
        }
        Ok(())
    }
    
    fn evaluate_types_for_type_synonym_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let mut visited_idents: BTreeSet<String> = BTreeSet::new();
        for def in tree.defs() {
            match &**def {
                Def::Type(ident, type_var, _) => {
                    let type_var_r = type_var.borrow();
                    self.evaluate_types_for_type_synonym(ident, &*type_var_r, &mut visited_idents, tree, errs)?;
                },
                _ => (),
            }
        }
        Ok(())
    }
    
    fn evaluate_types_for_type_synonym(&self, ident: &String, type_var: &TypeVar, visited_idents: &mut BTreeSet<String>, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_var {
            TypeVar::Synonym(_, _, _) => {
                dfs_with_result(ident, visited_idents, errs, |ident, processed_idents, errs| {
                        self.type_synonym_idents_for_type_synonym_ident(ident, tree, processed_idents, errs)
                }, |ident, errs| {
                        self.evaluate_type_for_type_synonym_ident(ident, tree, errs)
                })?;
            },
            _ => (),
        }
        Ok(())
    }
    
    fn evaluate_types_for_type_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Type(ident, type_var, pos) => {
                    let mut type_var_r = type_var.borrow_mut();
                    self.evaluate_types_for_type(ident, &mut *type_var_r, pos.clone(), tree, errs)?;
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn evaluate_shared_flags_for_type_defs(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        let mut visited_idents: BTreeSet<String> = BTreeSet::new();
        for def in tree.defs() {
            match &**def {
                Def::Type(ident, type_var, _) => {
                    let type_var_r = type_var.borrow();
                    self.evaluate_shared_flags_for_type(ident, &*type_var_r, &mut visited_idents, tree)?;
                },
                _ => (),
            }
        }
        Ok(())
    }    

    fn check_type_recursions_for_data_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let mut visited_idents: BTreeSet<String> = BTreeSet::new();
        for def in tree.defs() {
            match &**def {
                Def::Type(ident, type_var, _) => {
                    let type_var_r = type_var.borrow();
                    self.check_type_recursions_for_data(ident, &*type_var_r, &mut visited_idents, tree, errs)?;
                },
                _ => (),
            }
        }
        Ok(())
    }    
    
    fn evaluate_types_for_type(&self, ident: &String, type_var: &mut TypeVar, pos: Pos, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_var {
            TypeVar::Builtin(Some(type_args), fields, _) => {
                match self.builtins.type_var(ident) {
                    Some(builtin_type_var) => {
                        let mut type_param_env: Environment<LocalType> = Environment::new();
                        type_param_env.push_new_vars();
                        let mut type_arg_idents: Vec<String> = Vec::new();
                        for (i, type_arg_ident) in type_args.type_arg_idents().iter().enumerate() {
                            type_param_env.add_var(type_arg_ident.clone(), LocalType::new(i));
                            type_arg_idents.push(type_arg_ident.clone());
                        }
                        let mut new_fields = Fields::new();
                        let mut is_success = true;
                        for (i, field_src) in builtin_type_var.field_type_sources.iter().enumerate() {
                            match parse_type_with_path(format!("({} field {}).vscfl", ident, i).as_str(), field_src) {
                                Ok(type_expr) => {
                                    match check_idents_for_type_with_type_args(&type_expr, type_arg_idents.as_slice(), tree) {
                                        Ok(()) => {
                                            match self.evaluate_type_for_type_expr(&type_expr, tree, &mut type_param_env, &mut None, errs)? {
                                                Some(type_value) => new_fields.add_field_type_value(type_value),
                                                None => is_success = false, 
                                            }
                                        },
                                        Err(mut errs2) => add_errors(&mut errs2, errs)?,
                                    }
                                },
                                Err(err) => add_error(err, errs)?,
                            }
                        }
                        if is_success {
                            for (field_ident, i) in &builtin_type_var.field_indices {
                                new_fields.add_field_index(field_ident.clone(), *i);
                            }
                            *fields = Some(Box::new(new_fields));
                        }
                    },
                    None => (),
                }
            },
            TypeVar::Builtin(None, _, _) => errs.push(FrontendError::Message(pos, format!("built-in type {} hasn't evalauted type arguments", ident))),
            TypeVar::Data(type_args, cons, _) => {
                for (i, type_arg) in type_args.iter().enumerate() {
                    let mut type_param_env: Environment<LocalType> = Environment::new();
                    type_param_env.push_new_vars();
                    let mut tmp_type_values: Vec<Rc<TypeValue>> = Vec::new();
                    let mut type_arg_idents: Vec<String> = Vec::new();
                    match type_arg {
                        TypeArg(type_arg_ident, _) => {
                            type_param_env.add_var(type_arg_ident.clone(), LocalType::new(i));
                            tmp_type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(i))));
                            type_arg_idents.push(type_arg_ident.clone());
                        },
                    }
                    let ret_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(ident.clone()), tmp_type_values));
                    for con in &*cons {
                        let con_r = con.borrow();
                        let pair = match &*con_r {
                            Con::UnnamedField(con_ident, field_type_exprs, _, _) => {
                                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                                let mut is_success = true;
                                for field_type_expr in field_type_exprs {
                                    match self.evaluate_type_for_type_expr(&**field_type_expr, tree, &mut type_param_env, &mut None, errs)? {
                                        Some(type_value) => type_values.push(type_value),
                                        None => is_success = false, 
                                    }
                                }
                                if is_success {
                                    Some((con_ident.clone(), type_values))
                                } else {
                                    None
                                }
                            },
                            Con::NamedField(con_ident, type_expr_named_field_pairs, _, _, _) => {
                                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                                let mut is_success = true;
                                for type_expr_named_field_pair in type_expr_named_field_pairs {
                                    match type_expr_named_field_pair {
                                        NamedFieldPair(_, field_type_expr, _) => {
                                            match self.evaluate_type_for_type_expr(&**field_type_expr, tree, &mut type_param_env, &mut None, errs)? {
                                                Some(type_value) => type_values.push(type_value),
                                                None => is_success = false, 
                                            }
                                        },
                                    }
                                }
                                if is_success {
                                    Some((con_ident.clone(), type_values))
                                } else {
                                    None
                                }
                            }
                        };
                        match pair {
                            Some((con_ident, mut type_values)) => {
                                match tree.var(&con_ident) {
                                    Some(var) => {
                                        let mut var_r = var.borrow_mut();
                                        match &mut *var_r {
                                            Var::Fun(_, _, typ) => {
                                                type_values.push(ret_type_value.clone());
                                                let type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values));
                                                *typ = Some(Box::new(Type::new(type_value, type_arg_idents.as_slice())));
                                            },
                                            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_type: variable isn't function"))])),
                                        }
                                    },
                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_type: no variable"))])),
                                }
                            },
                            None => (),
                        }
                    }
                }
            },
            _ => (),
        }
        Ok(())
    }
    
    fn type_synonym_idents_for_type_synonym_ident(&self, ident: &String, tree: &Tree, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Vec<String>>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Synonym(_, type_expr, _) => {
                        let mut idents: Vec<String> = Vec::new();
                        self.add_type_synonym_idents_for_type_expr(&**type_expr, tree, &mut idents, processed_idents, errs)?;
                        Ok(idents)
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_synonym_idents_for_type_synonym_ident: type variable isn't type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_synonym_idents_for_type_synonym_ident: no type variable"))])),
        }
    }
        
    fn add_type_synonym_idents_for_type_expr(&self, type_expr: &TypeExpr, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_expr {
            TypeExpr::Tuple(field_type_exprs, _) => {
                for field_type_expr in field_type_exprs {
                    self.add_type_synonym_idents_for_type_expr(&**field_type_expr, tree, idents, processed_idents, errs)?;
                }
                
            },
            TypeExpr::Fun(arg_type_exprs, ret_type_expr, _) => {
                for arg_type_expr in arg_type_exprs {
                    self.add_type_synonym_idents_for_type_expr(&**arg_type_expr, tree, idents, processed_idents, errs)?;
                }
                self.add_type_synonym_idents_for_type_expr(&**ret_type_expr, tree, idents, processed_idents, errs)?
            },
            TypeExpr::Array(elem_type_expr, _, _) => self.add_type_synonym_idents_for_type_expr(&**elem_type_expr, tree, idents, processed_idents, errs)?,
            TypeExpr::Param(_, _) => (),
            TypeExpr::Var(ident, pos) => add_type_synonym_ident(ident, pos.clone(), tree, idents, processed_idents, errs)?,
            TypeExpr::App(ident, _, pos) => add_type_synonym_ident(ident, pos.clone(), tree, idents, processed_idents, errs)?,
            TypeExpr::Uniq(type_expr2, _) => self.add_type_synonym_idents_for_type_expr(&**type_expr2, tree, idents, processed_idents, errs)?,
        }
        Ok(())
    }
    
    fn evaluate_type_for_type_synonym_ident(&self, ident: &String, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Synonym(type_args, type_expr, type_value) => {
                        let mut type_param_env: Environment<LocalType> = Environment::new();
                        type_param_env.push_new_vars();
                        for (i, type_arg) in type_args.iter().enumerate() {
                            match type_arg {
                                TypeArg(type_arg_ident, _) => {
                                    type_param_env.add_var(type_arg_ident.clone(), LocalType::new(i));
                                },
                            }
                        }
                        *type_value = self.evaluate_type_for_type_expr(&**type_expr, tree, &mut type_param_env, &mut None, errs)?;
                        Ok(())
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_type_for_type_synonym_ident: type variable isn't type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_type_for_type_synonym_ident: no type variable"))])),
        }
    }

    fn evaluate_type_for_type_expr(&self, type_expr: &TypeExpr, tree: &Tree, type_param_env: &mut Environment<LocalType>, local_type_counter: &mut Option<usize>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<Rc<TypeValue>>>
    {
        match type_expr {
            TypeExpr::Tuple(field_type_exprs, _) => {
                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                let mut is_success = true;
                for field_type_expr in field_type_exprs {
                    match self.evaluate_type_for_type_expr(&**field_type_expr, tree, type_param_env, local_type_counter, errs)? {
                        Some(type_value) => type_values.push(type_value),
                        None => is_success = false,
                    }
                }
                if is_success {
                    Ok(Some(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, type_values))))
                } else {
                    Ok(None)
                }
            },
            TypeExpr::Fun(arg_type_exprs, ret_type_expr, _) => {
                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                let mut is_success = false;
                for arg_type_expr in arg_type_exprs {
                    match self.evaluate_type_for_type_expr(&**arg_type_expr, tree, type_param_env, local_type_counter, errs)? {
                        Some(type_value) => type_values.push(type_value),
                        None => is_success = false,
                    }
                }
                match self.evaluate_type_for_type_expr(&**ret_type_expr, tree, type_param_env, local_type_counter, errs)? {
                    Some(type_value) => type_values.push(type_value),
                    None => is_success = false,
                }
                if is_success {
                    Ok(Some(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values))))
                } else {
                    Ok(None)
                }
            },
            TypeExpr::Array(elem_type_expr, len, _) => {
                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                let mut is_success = true;
                match self.evaluate_type_for_type_expr(&**elem_type_expr, tree, type_param_env, local_type_counter, errs)? {
                    Some(type_value) => type_values.push(type_value),
                    None => is_success = false,
                }
                if is_success {
                    Ok(Some(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Array(*len), type_values))))
                } else {
                    Ok(None)
                }
            },
            TypeExpr::Param(ident, _) => {
                let local_type = match local_type_counter {
                    Some(local_type_counter) => add_type_param_local_type(ident, type_param_env, local_type_counter),
                    None => local_type_for_type_param_ident(ident, type_param_env)?,
                };
                Ok(Some(Rc::new(TypeValue::Param(UniqFlag::None, local_type))))
            },
            TypeExpr::Var(ident, pos) => {
                match type_value_and_type_arg_count_for_type_var_ident(ident, pos.clone(), tree, errs)? {
                    Some((type_value, type_arg_count)) => {
                        if type_arg_count == 0 {
                            Ok(Some(type_value))
                        } else {
                            errs.push(FrontendError::Message(pos.clone(), format!("type variable {} has type arguments", ident)));
                            Ok(None)
                        }
                    },
                    None => Ok(None),
                }
            },
            TypeExpr::App(ident, type_exprs, pos) => {
                match type_value_and_type_arg_count_for_type_var_ident(ident, pos.clone(), tree, errs)? {
                    Some((type_value, type_arg_count)) => {
                        if type_exprs.len() < type_arg_count {
                            errs.push(FrontendError::Message(pos.clone(), format!("too few type arguments")));
                            return Ok(None);
                        } else if type_exprs.len() > type_arg_count {
                            errs.push(FrontendError::Message(pos.clone(), format!("too many type arguments")));
                            return Ok(None);
                        }
                        let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                        let mut is_success = true;
                        for type_expr2 in type_exprs {
                            match self.evaluate_type_for_type_expr(&**type_expr2, tree, type_param_env, local_type_counter, errs)? {
                                Some(type_value) => type_values.push(type_value),
                                None => is_success = false,
                            }
                        }
                        if is_success {
                            match type_value.substitute(type_values.as_slice()) {
                                Ok(Some(new_type_value)) => Ok(Some(new_type_value)),
                                Ok(None) => Ok(Some(type_value)),
                                Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("evaluate_type_for_type_expr: {}", err))]))
                            }
                        } else {
                            Ok(None)
                        }
                    },
                    None => Ok(None),
                }
            },
            TypeExpr::Uniq(type_expr2, _) => {
                match self.evaluate_type_for_type_expr(&**type_expr2, tree, type_param_env, local_type_counter, errs)? {
                    Some(type_value) => {
                        let mut type_value3 = (*type_value).clone();
                        type_value3.set_uniq_flag(UniqFlag::Uniq);
                        Ok(Some(Rc::new(type_value3)))
                    },
                    None => Ok(None),
                }
            },
        }
    }
    
    fn evaluate_shared_flags_for_type(&self, ident: &String, type_var: &TypeVar, visited_idents: &mut BTreeSet<String>, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        match type_var {
            TypeVar::Builtin(_, _, _) | TypeVar::Data(_, _, _) => {
                dfs_with_result(ident, visited_idents, &mut (), |ident, processed_idents, _| {
                        self.shared_type_idents_for_type_ident(ident, tree, processed_idents)
                }, |ident, _| {
                        self.evaluate_shared_flag_for_type_ident(ident, tree)
                })?;
            },
            _ => (),
        }
        Ok(())
    }

    fn shared_type_idents_for_type_ident(&self, ident: &String, tree: &Tree, processed_idents: &BTreeSet<String>) -> FrontendResultWithErrors<Vec<String>>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Builtin(_, _, _) => Ok(Vec::new()),
                    TypeVar::Data(_, cons, _) => {
                        let mut idents: Vec<String> = Vec::new();
                        for con in &*cons {
                            let con_r = con.borrow();
                            let con_ident = match &*con_r {
                                Con::UnnamedField(tmp_con_ident, _, _, _) => tmp_con_ident.clone(),
                                Con::NamedField(tmp_con_ident, _, _, _, _) => tmp_con_ident.clone(),
                            };
                            match tree.var(&con_ident) {
                                Some(var) => {
                                    let var_r = var.borrow();
                                    match &*var_r {
                                        Var::Fun(_, _, Some(typ)) => {
                                            match &**typ.type_value() {
                                                TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values) => {
                                                    if type_values.len() >= 1 {
                                                        for type_value2 in &type_values[0..(type_values.len() - 1)]  {
                                                            self.add_shared_type_idents_for_type_value(&**type_value2, tree, &mut idents, processed_idents)?
                                                        }
                                                    } else {
                                                        return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_type_idents_for_type_ident: too few type values"))]))
                                                    }
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_type_idents_for_type_ident: variable isn't function or no type"))])),
                                            }
                                        },
                                        Var::Fun(_, _, None) => (),
                                        _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_type_idents_for_type_ident: variable isn't function"))])),
                                    }
                                },
                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_type_idents_for_type_ident: no variable"))])),
                            }
                        }
                        Ok(idents)
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_type_idents_for_type_ident: type variable is type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_type_idents_for_type_ident: no type variable"))])),
        }
    }
    
    fn add_shared_type_idents_for_type_value(&self, type_value: &TypeValue, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>) -> FrontendResultWithErrors<()>
    {
        match type_value {
            TypeValue::Type(UniqFlag::None, type_value_name, type_values) => {
                match type_value_name {
                    TypeValueName::Name(ident) => add_type_ident(ident, tree, idents, processed_idents)?,
                    _ => (),
                }
                for type_value2 in type_values {
                    self.add_shared_type_idents_for_type_value(&**type_value2, tree, idents, processed_idents)?;
                }
            },
            _ => (),
        }
        Ok(())
    }

    fn evaluate_shared_flag_for_type_ident(&self, ident: &String, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Builtin(_, _, shared_flag) => {
                        match self.builtins.type_var(ident) {
                            Some(builtin_type_var) => {
                                *shared_flag = Some(builtin_type_var.shared_flag);
                                Ok(())
                            },
                            None => Ok(()),
                        }
                    },
                    TypeVar::Data(_, cons, shared_flag) => {
                        let mut new_shared_flag = SharedFlag::Shared;
                        let mut is_success = true;
                        for con in &*cons {
                            let con_r = con.borrow();
                            let con_ident = match &*con_r {
                                Con::UnnamedField(tmp_con_ident, _, _, _) => tmp_con_ident.clone(),
                                Con::NamedField(tmp_con_ident, _, _, _, _) => tmp_con_ident.clone(),
                            };
                            match tree.var(&con_ident) {
                                Some(var) => {
                                    let var_r = var.borrow();
                                    match &*var_r {
                                        Var::Fun(_, _, Some(typ)) => {
                                            match &**typ.type_value() {
                                                TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values) => {
                                                    if type_values.len() >= 1 {
                                                        for type_value2 in &type_values[0..(type_values.len() - 1)]  {
                                                            match self.evaluate_shared_flag_for_type_value(&**type_value2, tree)? {
                                                                Some(shared_flag2) => {
                                                                    if shared_flag2 == SharedFlag::None {
                                                                        new_shared_flag = SharedFlag::None;
                                                                    }
                                                                },
                                                                None => is_success = false,
                                                            }
                                                        }
                                                    } else {
                                                        return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_shared_flag_for_type_ident: too few type values"))]))
                                                    }
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_shared_flag_for_type_ident: variable isn't function or no type"))])),
                                            }
                                        },
                                        Var::Fun(_, _, None) => is_success = false,
                                        _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_shared_flag_for_type_ident: variable isn't function"))])),
                                    }
                                },
                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_shared_flag_for_type_ident: no variable"))])),
                            }
                        }
                        if is_success {
                            *shared_flag = Some(new_shared_flag);
                        } else {
                            *shared_flag = None;
                        }
                        Ok(())
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_shared_flag_for_type_ident: type variable is type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_shared_flag_for_type_ident: no type variable"))])),
        }
    }
    
    fn evaluate_shared_flag_for_type_value(&self, type_value: &TypeValue, tree: &Tree) -> FrontendResultWithErrors<Option<SharedFlag>>
    {
        match type_value {
            TypeValue::Param(UniqFlag::None, _) => Ok(Some(SharedFlag::Shared)),
            TypeValue::Type(UniqFlag::None, TypeValueName::Fun, _) => Ok(Some(SharedFlag::Shared)),
            TypeValue::Type(UniqFlag::None, type_value_name, type_values) => {
                let shared_flag = match type_value_name {
                    TypeValueName::Name(ident) => shared_flag_for_type_ident_and_evaluation(ident, tree)?,
                    _ => Some(SharedFlag::Shared),
                };
                match shared_flag {
                    Some(mut shared_flag) => {
                        let mut is_success = true;
                        if shared_flag == SharedFlag::Shared {
                            for type_value2 in type_values {
                                match self.evaluate_shared_flag_for_type_value(&**type_value2, tree)? {
                                    Some(shared_flag2) => {
                                        if shared_flag2 == SharedFlag::None {
                                            shared_flag = SharedFlag::None;
                                        }
                                    },
                                    None => is_success = false,
                                }
                            }
                        }
                        if is_success {
                            Ok(Some(shared_flag))
                        } else {
                            Ok(None)
                        }
                    },
                    None => Ok(None),
                }
            },
            _ => Ok(Some(SharedFlag::None)),
        }
    }

    fn check_type_recursions_for_data(&self, ident: &String, type_var: &TypeVar, visited_idents: &mut BTreeSet<String>, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_var {
            TypeVar::Data(_, _, _) => {
                dfs_with_result(ident, visited_idents, errs, |ident, processed_idents, errs| {
                        self.check_type_recursions_for_data_ident(ident, tree, processed_idents, errs)
                }, |_, _| Ok(()))?;
            },
            _ => (),
        }
        Ok(())
    }
    
    fn check_type_recursions_for_data_ident(&self, ident: &String, tree: &Tree, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Vec<String>>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Data(_, cons, _) => {
                        let mut idents: Vec<String> = Vec::new();
                        for con in &*cons {
                            let con_r = con.borrow();
                            let (con_ident, poses) = match &*con_r {
                                Con::UnnamedField(tmp_con_ident, field_type_exprs, _, _) => {
                                    let mut tmp_poses: Vec<Pos> = Vec::new();
                                    for field_type_expr in field_type_exprs {
                                        tmp_poses.push(type_expr_pos(&**field_type_expr).clone());
                                    }
                                    (tmp_con_ident.clone(), tmp_poses)
                                },
                                Con::NamedField(tmp_con_ident, type_expr_named_field_pairs, _, _, _) => {
                                    let mut tmp_poses: Vec<Pos> = Vec::new();
                                    for type_expr_named_field_pair in type_expr_named_field_pairs {
                                        match type_expr_named_field_pair {
                                            NamedFieldPair(_, field_type_expr, _) => tmp_poses.push(type_expr_pos(&**field_type_expr).clone()),
                                        }
                                    }
                                    (tmp_con_ident.clone(), tmp_poses)
                                },
                            };
                            match tree.var(&con_ident) {
                                Some(var) => {
                                    let var_r = var.borrow();
                                    match &*var_r {
                                        Var::Fun(_, _, Some(typ)) => {
                                            match &**typ.type_value() {
                                                TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values) => {
                                                    if type_values.len() >= 1 {
                                                        for (type_value2, pos) in (&type_values[0..(type_values.len() - 1)]).iter().zip(poses.iter())  {
                                                            self.add_data_type_idents_for_type_value(&**type_value2, &pos, tree, &mut idents, processed_idents, errs)?
                                                        }
                                                    } else {
                                                        return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_recursions_for_data_ident: too few type values"))]))
                                                    }
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_recursions_for_data_ident: variable isn't function"))])),
                                            }
                                        },
                                        Var::Fun(_, _, None) => (),
                                        _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_recursions_for_data_ident: variable isn't function"))])),
                                    }
                                },
                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_recursions_for_data_ident: no variable"))])),
                            }
                        }
                        Ok(idents)
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_recursions_for_data_ident: type variable is built-in type or type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_recursions_for_data_ident: no type variable"))])),
        }
    }
    
    fn has_ref_type_for_type_ident(&self, ident: &String, tree: &Tree) -> FrontendResultWithErrors<bool>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Builtin(_, _, _) => {
                        match self.builtins.type_var(ident) {
                            Some(builtin_type_var) => Ok(builtin_type_var.is_ref_type),
                            None => Ok(false),
                        }
                    },
                    TypeVar::Data(_, _, _) => Ok(false),
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_ref_type_for_type_ident: type variable is type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_ref_type_for_type_ident: no type variable"))])),
        }
    }
    
    fn add_data_type_idents_for_type_value(&self, type_value: &TypeValue, pos: &Pos, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_value {
            TypeValue::Type(_, type_value_name, type_values) => {
                let is_ref_type = match type_value_name {
                    TypeValueName::Name(ident) => {
                        add_data_ident(ident, pos.clone(), tree, idents, processed_idents, errs)?;
                        self.has_ref_type_for_type_ident(ident, tree)?
                    },
                    _ => false,
                };
                if !is_ref_type {
                    for type_value2 in type_values {
                        self.add_data_type_idents_for_type_value(&**type_value2, pos, tree, idents, processed_idents, errs)?;
                    }
                }
            },
            _ => (),
        }
        Ok(())
    }

    //
    // Evaluation of types for variables.
    //
    
    fn check_type_arg_counts_for_impl_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Impl(impl1, pos) => {
                    let impl_r = impl1.borrow();
                    match &*impl_r {
                        Impl::Builtin(trait_ident, type_name, _) => {
                            if !self.builtins.has_impl_pair(&(trait_ident.clone(), type_name.clone())) {
                                errs.push(FrontendError::Message(pos.clone(), format!("implementation of trait {} for type {} isn't built-in implementation", trait_ident, type_name)));
                            }
                        },
                        _ => (),
                    }
                    let (trait_ident, type_name) = match &*impl_r {
                        Impl::Builtin(tmp_trait_ident, tmp_type_name, _) => (tmp_trait_ident, tmp_type_name),
                        Impl::Impl(tmp_trait_ident, tmp_type_name, _, _) => (tmp_trait_ident, tmp_type_name),
                    };
                    let trait_type_arg_count = type_arg_count_for_trait_ident(trait_ident, tree)?;
                    let type_arg_count = match &type_name {
                        TypeName::Tuple(count) => *count,
                        TypeName::Array(_) => 1,
                        TypeName::Fun(count) => *count + 1,
                        TypeName::Name(ident) => type_arg_count_for_type_ident(ident, tree)?,
                    };
                    if type_arg_count < trait_type_arg_count {
                        errs.push(FrontendError::Message(pos.clone(), format!("too few type arguments of type {}", type_name)));
                    } else if type_arg_count > trait_type_arg_count {
                        errs.push(FrontendError::Message(pos.clone(), format!("too many type arguments of type {}", type_name)));
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn evaluate_types_for_var_and_trait_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Var(ident, var, pos) => {
                    let mut var_r = var.borrow_mut();
                    self.evaluate_types_for_var(ident, &mut *var_r, pos.clone(), tree, errs)?;
                },
                Def::Trait(_, trait1, _) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, trait_defs, _) => {
                            for trait_def in trait_defs {
                                match &**trait_def {
                                    TraitDef(ident, var, pos) => {
                                        let mut var_r = var.borrow_mut();
                                        self.evaluate_types_for_var(ident, &mut *var_r, pos.clone(), tree, errs)?;
                                    },
                                }
                            }
                        },
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn check_impls_for_impl_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Impl(impl1, pos) => {
                    let impl_r = impl1.borrow();
                    let (trait_ident, type_name) = match &*impl_r {
                        Impl::Builtin(tmp_trait_ident, tmp_type_name, _) => (tmp_trait_ident, tmp_type_name), 
                        Impl::Impl(tmp_trait_ident, tmp_type_name, _, _) => (tmp_trait_ident, tmp_type_name),
                    };
                    if shared_flag_for_type_name(&type_name, tree)? == SharedFlag::None {
                        match tree.trait1(&trait_ident) {
                            Some(trait1) => {
                                let trait_r = trait1.borrow();
                                let mut is_success = true;
                                match &*trait_r {
                                    Trait(_, _, Some(trait_vars)) => {
                                        for trait_var in trait_vars.vars().values() {
                                            let trait_var_r = trait_var.borrow();
                                            let typ = match &*trait_var_r {
                                                Var::Builtin(_, Some(tmp_type)) => tmp_type,
                                                Var::Var(_, _, _, _, _, _, _, Some(tmp_type), _) => tmp_type,
                                                Var::Fun(_, _, Some(tmp_type)) => tmp_type,
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_impls_for_impl_defs: no type"))])),
                                            };
                                            for type_param_entry in typ.type_param_entries() {
                                                let type_param_entry_r = type_param_entry.borrow();
                                                if type_param_entry_r.trait_names.contains(&TraitName::Name(trait_ident.clone())) {
                                                    if type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                                                        is_success = false;
                                                    }
                                                }
                                            }
                                        }
                                    },
                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_impls_for_impl_defs: no trait variables"))])),
                                }
                                if !is_success {
                                    errs.push(FrontendError::Message(pos.clone(), format!("defined implementation of trait {} for type {} that is unique type", trait_ident, type_name)));
                                }
                            }
                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_impls_for_impl_defs: no trait"))])),
                        }
                    }
                    match tree.trait1(&trait_ident) {
                        Some(trait1) => {
                            let trait_r = trait1.borrow();
                            match &*trait_r {
                                Trait(_, _, Some(trait_vars)) => {
                                    match &*impl_r {
                                        Impl::Impl(_, _, impl_defs, _) => {
                                            for impl_def in impl_defs {
                                                match &**impl_def {
                                                    ImplDef(impl_var_ident, impl_var, impl_var_pos) => {
                                                       match trait_vars.var(impl_var_ident) {
                                                            Some(trait_var) => {
                                                                let trait_var_r = trait_var.borrow();
                                                                let impl_var_r = impl_var.borrow();
                                                                match (&*trait_var_r, &*impl_var_r) {
                                                                    (Var::Builtin(_, Some(trait_var_type)), ImplVar::Fun(impl_fun, _)) => {
                                                                        match &**trait_var_type.type_value() {
                                                                            TypeValue::Type(_, TypeValueName::Fun, type_values) => {
                                                                                if type_values.len() >= 1 {
                                                                                    match &**impl_fun {
                                                                                        ImplFun(impl_args, _, _, _) => {
                                                                                            if impl_args.len() < type_values.len() - 1  {
                                                                                                errs.push(FrontendError::Message(impl_var_pos.clone(), String::from("too few arguments")));
                                                                                            } else if impl_args.len() > type_values.len() - 1 {
                                                                                                errs.push(FrontendError::Message(impl_var_pos.clone(), String::from("too many arguments")));
                                                                                            }
                                                                                        },
                                                                                    }
                                                                                } else {
                                                                                    return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_impls_for_impl_defs: too few type values"))]))
                                                                                }
                                                                            },
                                                                            _ => errs.push(FrontendError::Message(impl_var_pos.clone(), format!("type of built-in variable {} isn't function type", impl_var_ident))),
                                                                        }
                                                                    },
                                                                    (Var::Builtin(_, None), ImplVar::Fun(_, _)) => {
                                                                        errs.push(FrontendError::Message(impl_var_pos.clone(), format!("unevaluated type of built-in variable {}", impl_var_ident)));
                                                                    },
                                                                    _ => (),
                                                                }
                                                            },
                                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_impls_for_impl_defs: no trait variable"))])),
                                                       }
                                                    },
                                                }
                                            }
                                        },
                                        _ => (),
                                    }
                                },
                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_impls_for_impl_defs: no trait variables"))])),
                            }
                        },
                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_impls_for_impl_defs: no trait"))])),
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }

    fn evaluate_types_for_impl_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Impl(impl1, pos) => {
                    let impl_r = impl1.borrow();
                    match &*impl_r {
                        Impl::Builtin(trait_ident, type_name, Some(impl_vars)) => {
                            for (ident, impl_var) in impl_vars.vars() {
                                let mut impl_var_r = impl_var.borrow_mut();
                                self.evaluate_types_for_impl_var(ident, &mut *impl_var_r, pos.clone(), trait_ident, type_name, tree, true, errs)?;
                            }
                        },
                        Impl::Builtin(_, _, None) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_impl_defs: no implementation variables"))])),
                        Impl::Impl(trait_ident, type_name, impl_defs, _) => {
                            for impl_def in impl_defs {
                                match &**impl_def {
                                    ImplDef(ident, impl_var, impl_var_pos) => {
                                        let mut impl_var_r = impl_var.borrow_mut();
                                        self.evaluate_types_for_impl_var(ident, &mut *impl_var_r, impl_var_pos.clone(), trait_ident, type_name, tree, false, errs)?;
                                    },
                                }
                            }
                        },
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }
    
    fn evaluate_types_for_var(&self, ident: &String, var: &mut Var, pos: Pos, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match var {
            Var::Builtin(trait_ident, typ) => {
                match self.builtins.var(ident) {
                    Some(builtin_var) => {
                        match parse_type_with_path(format!("({} type).vscfl", ident).as_str(), builtin_var.type_source.as_str()) {
                            Ok(type_expr) => {
                                match parse_where_with_path(format!("({} where).vscfl", ident).as_str(), builtin_var.where_source.as_str()) {
                                    Ok(where_tuples) => {
                                        match check_idents_for_type_with_where(&type_expr, &where_tuples, tree) {
                                            Ok(()) => {
                                                let mut type_param_env: Environment<LocalType> = Environment::new();
                                                let mut local_type_counter = Some(0usize);
                                                type_param_env.push_new_vars();
                                                match self.evaluate_type_for_type_expr(&type_expr, tree, &mut type_param_env, &mut local_type_counter, errs)? {
                                                    Some(type_value) => {
                                                        let mut new_type = new_type_from_type_value_and_type_param_env(type_value, &type_param_env, local_type_counter)?;
                                                        if self.evaluate_types_for_where_tuples(ident.as_str(), where_tuples.as_slice(), trait_ident, pos, tree, &mut type_param_env, &mut new_type, errs)? {
                                                            if self.shared_flag_for_type(&new_type, tree)? == SharedFlag::Shared {
                                                                *typ = Some(Box::new(new_type));
                                                            } else {
                                                                errs.push(FrontendError::Message(type_expr_pos(&type_expr).clone(), format!("built-in variable {} mustn't non-shared with type {}", ident, new_type)))
                                                            }
                                                        }
                                                    },
                                                    None => (),
                                                }
                                            },
                                            Err(mut errs2) => add_errors(&mut errs2, errs)?,
                                        }
                                    },
                                    Err(err) => add_error(err, errs)?,
                                }
                            },
                            Err(err) => add_error(err, errs)?,
                        }
                    },
                    None => errs.push(FrontendError::Message(pos, format!("variable {} mustn't be built-in variable", ident))),
                }
            },
            Var::Var(_, type_expr, where_tuples, expr, trait_ident, local_type, local_types, typ, _) => {
                let mut type_param_env: Environment<LocalType> = Environment::new();
                let mut local_type_counter = Some(0usize);
                type_param_env.push_new_vars();
                match self.evaluate_type_for_type_expr(&**type_expr, tree, &mut type_param_env, &mut local_type_counter, errs)? {
                    Some(type_value) => {
                        let mut new_type = new_type_from_type_value_and_type_param_env(type_value, &type_param_env, local_type_counter)?;
                        if self.evaluate_types_for_where_tuples(ident.as_str(), where_tuples.as_slice(), trait_ident, pos, tree, &mut type_param_env, &mut new_type, errs)? {
                            if self.shared_flag_for_type(&new_type, tree)? == SharedFlag::Shared {
                                match expr {
                                    Some(expr) => {
                                        let mut new_local_types = LocalTypes::new();
                                        let new_local_type = new_local_types.set_defined_type(&new_type);
                                        let mut var_env: Environment<LocalType> = Environment::new();
                                        self.evaluate_types_for_expr(&mut **expr, tree, &mut var_env, &mut type_param_env, &mut new_local_types, errs)?;
                                        let mut var_env2: Environment<(LocalType, usize, Pos)> = Environment::new();
                                        self.set_shareds_for_expr(&**expr, tree, &mut var_env2, &new_local_types, errs)?;
                                        *local_type = Some(new_local_type);
                                        *local_types = Some(Box::new(new_local_types));
                                    },
                                    None => *local_types = None,
                                }
                                *typ = Some(Box::new(new_type));
                            } else {
                                errs.push(FrontendError::Message(type_expr_pos(&type_expr).clone(), format!("variable {} mustn't be non-shared with type {}", ident, new_type)))
                            }
                        }
                    },
                    None => (),
                }
            },
            Var::Fun(fun, trait_ident, typ) => {
                match &mut **fun {
                    Fun::Fun(_, args, ret_type_expr, where_tuples, body, ret_local_type, local_types) => {
                        let mut type_param_env: Environment<LocalType> = Environment::new();
                        let mut local_type_counter = Some(0usize);
                        let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                        let mut is_success = true;
                        type_param_env.push_new_vars();
                        for arg in &*args {
                            match arg {
                                Arg(_, type_expr, _, _) => {
                                    match self.evaluate_type_for_type_expr(&**type_expr, tree, &mut type_param_env, &mut local_type_counter, errs)? {
                                        Some(type_value) => type_values.push(type_value),
                                        None => is_success = false,
                                    }
                                },
                            }
                        }
                        match self.evaluate_type_for_type_expr(&**ret_type_expr, tree, &mut type_param_env, &mut local_type_counter, errs)? {
                            Some(type_value) => type_values.push(type_value),
                            None => is_success = false,
                        }
                        if is_success {
                            let fun_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values.clone()));
                            let mut new_type = new_type_from_type_value_and_type_param_env(fun_type_value, &type_param_env, local_type_counter)?;
                            if self.evaluate_types_for_where_tuples(ident.as_str(), where_tuples.as_slice(), trait_ident, pos, tree, &mut type_param_env, &mut new_type, errs)? {
                                match body {
                                    Some(body) => {
                                        let mut new_local_types = LocalTypes::new();
                                        match new_local_types.set_defined_fun_types(&new_type) {
                                            Some(new_local_types2) => {
                                                let mut var_env: Environment<LocalType> = Environment::new();
                                                var_env.push_new_vars();
                                                if new_local_types2.len() >= 1 {
                                                    for (arg, new_local_type) in args.iter_mut().zip((&new_local_types2[0..(new_local_types2.len() - 1)]).iter()) {
                                                        match arg {
                                                            Arg(arg_ident, _, arg_local_type, _) => {
                                                                var_env.add_var(arg_ident.clone(), *new_local_type);
                                                                *arg_local_type = Some(*new_local_type);
                                                            },
                                                        }
                                                    }
                                                    self.evaluate_types_for_expr(&mut **body, tree, &mut var_env, &mut type_param_env, &mut new_local_types, errs)?;
                                                    let mut var_env2: Environment<(LocalType, usize, Pos)> = Environment::new();
                                                    var_env2.push_new_vars();
                                                    for (arg, new_local_type) in args.iter().zip((&new_local_types2[0..(new_local_types2.len() - 1)]).iter()) {
                                                        match arg {
                                                            Arg(arg_ident, _, _, pos) => {
                                                                var_env2.add_var(arg_ident.clone(), (*new_local_type, 0, pos.clone()));
                                                            },
                                                        }
                                                    }
                                                    self.set_shareds_for_expr(&**body, tree, &mut var_env2, &new_local_types, errs)?;
                                                    var_env2.foreach_with_result(|ident, tuple| self.set_shared_for_tuple(ident, tuple, tree, &new_local_types, errs))?;
                                                    *ret_local_type = Some(new_local_types2[new_local_types2.len() - 1]);
                                                    *local_types = Some(Box::new(new_local_types));
                                                } else {
                                                    return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_var: too few type values"))]))
                                                }
                                            },
                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_var: type value isn't function type"))])),
                                        }
                                    },
                                    None => *local_types = None,
                                }
                            }
                            *typ = Some(Box::new(new_type));
                        }
                    },
                    Fun::Con(_) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_var: variable is contructor"))])),
                }
            },
        }
        Ok(())
    }
    
    fn shared_flag_for_type(&self, typ: &Type, tree: &Tree) -> FrontendResultWithErrors<SharedFlag>
    { self.shared_flag_for_type_value(&**typ.type_value(), tree, typ) }

    fn shared_flag_for_type_value(&self, type_value: &TypeValue, tree: &Tree, typ: &Type) -> FrontendResultWithErrors<SharedFlag>
    {
        match type_value {
            TypeValue::Param(UniqFlag::None, local_type) => {
                match typ.type_param_entry(*local_type) {
                    Some(type_param_entry) => {
                        let type_param_entry_r = type_param_entry.borrow();
                        if type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                            Ok(SharedFlag::Shared)
                        } else {
                            Ok(SharedFlag::None)
                        }
                    },
                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_value: no type parameter"))])),
                }
            },
            TypeValue::Type(UniqFlag::None, TypeValueName::Fun, _) => Ok(SharedFlag::Shared),
            TypeValue::Type(UniqFlag::None, type_value_name, type_values) => {
                let mut shared_flag = match type_value_name {
                    TypeValueName::Name(ident) => shared_flag_for_type_ident(ident, tree)?,
                    _ => SharedFlag::Shared,
                };
                if shared_flag == SharedFlag::Shared {
                    for type_value2 in type_values {
                        if self.shared_flag_for_type_value(&**type_value2, tree, typ)? == SharedFlag::None {
                            shared_flag = SharedFlag::None;
                        }
                    }
                }
                Ok(shared_flag)
            },
            _ => Ok(SharedFlag::None),
        }
    }

    fn check_type_param_recursions_for_local_type(&self, local_type: LocalType, typ: &Type, processed_local_types: &BTreeSet<LocalType>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Vec<LocalType>>
    {
        match typ.type_param_entry(local_type) {
            Some(type_param_entry) => {
                let mut local_types: Vec<LocalType> = Vec::new();
                let type_param_entry_r = type_param_entry.borrow();
                match &type_param_entry_r.pos {
                    Some(pos) => {
                        for type_value in &type_param_entry_r.type_values {
                            self.add_local_types_for_type_value_and_recursion(&**type_value, pos, typ, &mut local_types, processed_local_types, errs)?;
                        }
                        Ok(local_types)
                    },
                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_param_recursions_for_local_type: no position"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_param_recursions_for_local_type: no type parameter entry"))])),
        }
    }
    
    fn add_local_types_for_type_value_and_recursion(&self, type_value: &TypeValue, pos: &Pos, typ: &Type, local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_value {
            TypeValue::Param(_, local_type) => add_local_type_for_recursion(*local_type, pos.clone(), typ, local_types, processed_local_types, errs)?,
            TypeValue::Type(_, _, type_values) => {
                for type_value2 in type_values {
                    self.add_local_types_for_type_value_and_recursion(&**type_value2, pos, typ, local_types, processed_local_types, errs)?;
                }
            },
        }
        Ok(())
    }    
    
    fn evaluate_types_for_where_tuples(&self, ident: &str, where_tuples: &[WhereTuple], trait_ident: &Option<String>, pos: Pos, tree: &Tree, type_param_env: &mut Environment<LocalType>, typ: &mut Type, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<bool>
    {
        if !where_tuples.is_empty() {
            let mut is_success = true;
            for where_tuple in where_tuples {
                match where_tuple {
                    WhereTuple::Traits(type_param_ident, trait_names, type_exprs, where_tuple_pos) => {
                        for trait_name in trait_names {
                            match trait_name {
                                TraitName::Shared => (),
                                TraitName::Fun => {
                                    if type_exprs.len() < 1 {
                                        errs.push(FrontendError::Message(where_tuple_pos.clone(), format!("no type expressions of type parameter {} for trait ->", type_param_ident)));
                                        is_success = false;
                                    }
                                },
                                TraitName::Name(trait_ident) => {
                                    let type_arg_count = type_arg_count_for_type_ident(trait_ident, tree)?;
                                    if type_arg_count != type_exprs.len() {
                                        errs.push(FrontendError::Message(where_tuple_pos.clone(), format!("number of type arguments of trait {} isn't equal to number of type expressions of type parameter {}", trait_ident, type_param_ident)));
                                        is_success = false;
                                    }
                                },
                            }
                        }
                    },
                    _ => (),
                }
            }
            if !is_success {
                return Ok(false);
            }
            for where_tuple in where_tuples {
                match where_tuple {
                    WhereTuple::Traits(type_param_ident, trait_names, type_exprs, where_tuple_pos) => {
                        match type_param_env.var(type_param_ident) {
                            Some(local_type) => {
                                match typ.type_param_entry(*local_type) {
                                    Some(type_param_entry) => {
                                        let mut type_param_entry_r = type_param_entry.borrow_mut();
                                        type_param_entry_r.trait_names.clear();
                                        for trait_name in trait_names {
                                            type_param_entry_r.trait_names.insert(trait_name.clone());
                                        }
                                        let mut tmp_is_success = true;
                                        type_param_entry_r.type_values.clear();
                                        for type_expr in type_exprs {
                                            match self.evaluate_type_for_type_expr(&**type_expr, tree, type_param_env, &mut None, errs)? {
                                                Some(type_value) => type_param_entry_r.type_values.push(type_value),
                                                None => tmp_is_success = false,
                                            }
                                        }
                                        type_param_entry_r.pos = Some(where_tuple_pos.clone());
                                        if !tmp_is_success {
                                            type_param_entry_r.trait_names.clear();
                                            type_param_entry_r.type_values.clear();
                                            type_param_entry_r.pos = None;
                                            is_success = false;
                                        }
                                    },
                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no type parameter entry"))])),
                                }
                            },
                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no type parameter"))])),
                        }
                    },
                    _ => (),
                }
            }
            if !is_success {
                return Ok(false);
            }
            for type_param_entry in typ.type_param_entries() {
                let type_param_entry_r = type_param_entry.borrow();
                if type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                    for type_value in &type_param_entry_r.type_values {
                        if self.shared_flag_for_type_value(&**type_value, tree, typ)? == SharedFlag::None {
                            match (&type_param_entry_r.ident, &type_param_entry_r.pos) {
                                (Some(type_param_ident), Some(type_param_pos)) => {
                                    errs.push(FrontendError::Message(type_param_pos.clone(), format!("type parameter {} must be shared", type_param_ident)));
                                    is_success = false;
                                },
                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no identifier or no position"))]))
                            }
                        }
                    }
                }
            }
            if is_success {
                let mut visited_local_types: BTreeSet<LocalType> = BTreeSet::new();
                let mut errs2: Vec<FrontendError> = Vec::new();
                for i in 0..typ.type_param_entries().len() {
                    dfs_with_result(&LocalType::new(i), &mut visited_local_types, &mut errs2, |local_type, processed_local_types, errs| {
                            self.check_type_param_recursions_for_local_type(*local_type, typ, processed_local_types, errs)
                    }, |_, _| Ok(()))?;
                }
                if !errs2.is_empty() {
                    errs.append(&mut errs2);
                    is_success = false;
                }
            }
            for where_tuple in where_tuples {
                match where_tuple {
                    WhereTuple::Eq(type_params) => {
                        match type_params.first() {
                            Some(TypeParam(type_param_ident, _)) => {
                                match type_param_env.var(type_param_ident) {
                                    Some(local_type) => {
                                        match typ.type_param_entry(*local_type) {
                                            Some(type_param_entry) => {
                                                let cloned_type_param_entry = type_param_entry.clone();
                                                for type_param in &type_params[1..] {
                                                    match type_param {
                                                        TypeParam(type_param_ident2, type_param_pos2) => {
                                                            match type_param_env.var(type_param_ident2) {
                                                                Some(local_type2) => {
                                                                    match typ.type_param_entry(*local_type2) {
                                                                        Some(type_param_entry2) => {
                                                                            let cloned_type_param_entry2 = type_param_entry2.clone();
                                                                            let type_param_entry_r = cloned_type_param_entry.borrow();
                                                                            let type_param_entry2_r = cloned_type_param_entry2.borrow();
                                                                            if type_param_entry_r.trait_names == type_param_entry2_r.trait_names {
                                                                                typ.set_eq_type_params(*local_type, *local_type2);
                                                                            } else {
                                                                                errs.push(FrontendError::Message(type_param_pos2.clone(), format!("type parameter {} hasn't same traits as type parameter {}", type_param_ident2, type_param_ident)));
                                                                                is_success = false;
                                                                            }
                                                                        },
                                                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no type parameter entry"))]))
                                                                    }
                                                                },
                                                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no type parameter"))]))
                                                            }
                                                        },
                                                    }
                                                }
                                            },
                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no type parameter entry"))]))
                                        }
                                    },
                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no type parameter"))]))
                                }
                            },
                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_where_tuples: no type parameters"))])),
                        }
                    },
                    _ => (),
                }
            }
            if !is_success {
                return Ok(false);
            }
            match trait_ident {
                Some(trait_ident) => {
                    let mut local_types: Vec<LocalType> = Vec::new();
                    for (i, type_param_entry) in typ.type_param_entries().iter().enumerate() {
                        let type_param_entry_r = type_param_entry.borrow();
                        if !type_param_entry_r.trait_names.contains(&TraitName::Name(trait_ident.clone())) {
                            local_types.push(LocalType::new(i));
                        }
                    }
                    match local_types.first() {
                        Some(local_type) => {
                            let mut tmp_is_success = true;
                            for local_type2 in &local_types[1..] {
                                if !typ.has_eq_type_params(*local_type, *local_type2) {
                                    tmp_is_success = false;
                                }
                            }
                            if !tmp_is_success {
                                errs.push(FrontendError::Message(pos, format!("type of variable {} has type parameters with trait {} which aren't equal", ident, trait_ident)));
                                is_success = false;
                            }
                        },
                        None => {
                            errs.push(FrontendError::Message(pos, format!("type of variable {} hasn't type parameters with trait {}", ident, trait_ident)));
                            is_success = false;
                        },
                    }
                },
                None => (),
            }
            Ok(is_success)
        } else {
            match trait_ident {
                Some(trait_ident) => {
                    errs.push(FrontendError::Message(pos, format!("type of variable {} must have type parameter with trait {}", ident, trait_ident)));
                    Ok(false)
                },
                None => Ok(true),
            }
        }
    }

    fn has_primitive_for_type_ident(&self, ident: &String, tree: &Tree) -> FrontendResultWithErrors<bool>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Builtin(_, _, _) => {
                        match self.builtins.type_var(ident) {
                            Some(builtin_type_var) => Ok(builtin_type_var.is_primitive),
                            None => Ok(false),
                        }
                    },
                    TypeVar::Data(_, _, _) => Ok(false),
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_primitive_for_type_ident: type variable is type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_primitive_for_type_ident: no type variable"))])),
        }
    }
    
    fn check_type_value_for_cast(&self, type_value: &TypeValue, tree: &Tree) -> FrontendResultWithErrors<bool>
    {
        match type_value {
            TypeValue::Type(_, TypeValueName::Tuple | TypeValueName::Array(Some(_)), type_values) => {
                let mut is_for_as = true;
                for type_value2 in type_values {
                    if !self.check_type_value_for_cast(type_value2, tree)? {
                        is_for_as = false;
                    }
                }
                Ok(is_for_as)
            },
            TypeValue::Type(_, TypeValueName::Name(ident), type_values) => {
                let mut is_for_as = self.has_primitive_for_type_ident(ident, tree)?;
                if is_for_as {
                    for type_value2 in type_values {
                        if !self.check_type_value_for_cast(type_value2, tree)? {
                            is_for_as = false;
                        }
                    }
                }
                Ok(is_for_as)
            },
            _ => Ok(false),
        }
    }
    
    fn evaluate_types_for_expr(&self, expr: &mut Expr, tree: &Tree, var_env: &mut Environment<LocalType>, type_param_env: &mut Environment<LocalType>, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, local_type, _) => {
                self.evaluate_types_for_literal(&mut **literal, tree, var_env, type_param_env, local_types, errs, Self::evaluate_types_for_expr)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Lambda(args, ret_type_expr, body, ret_local_type, local_type, _, _, _) => {
                var_env.push_new_vars();
                for arg in args {
                    match arg {
                        LambdaArg(ident, arg_type_expr, arg_local_type, _) => {
                            let tmp_local_type = match arg_type_expr {
                                Some(arg_type_expr) => {
                                    match self.evaluate_type_for_type_expr(&**arg_type_expr, tree, type_param_env, &mut None, errs)? {
                                        Some(type_value) => local_types.add_type_value(type_value),
                                        None => local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))),
                                    }
                                },
                                None => local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))),
                            };
                            *arg_local_type = Some(tmp_local_type);
                            var_env.add_var(ident.clone(), tmp_local_type);
                        },
                    }
                }
                match ret_type_expr {
                    Some(ret_type_expr) => {
                        match self.evaluate_type_for_type_expr(&**ret_type_expr, tree, type_param_env, &mut None, errs)? {
                            Some(type_value) => *ret_local_type = Some(local_types.add_type_value(type_value)),
                            None => *ret_local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                        }
                    },
                    None => *ret_local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
                self.evaluate_types_for_expr(&mut **body, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                var_env.pop_vars();
            },
            Expr::Var(ident, local_type, _) => {
                match var_env.var(ident) {
                    Some(var_local_type) => *local_type = Some(*var_local_type),
                    None => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
            },
            Expr::NamedFieldConApp(_, expr_named_field_pairs, con_local_type, local_type, _) => {
                for expr_named_field_pair in expr_named_field_pairs {
                    match expr_named_field_pair {
                        NamedFieldPair(_, field_expr, _) => {
                            self.evaluate_types_for_expr(&mut **field_expr, tree, var_env, type_param_env, local_types, errs)?;
                        }
                    }
                }
                *con_local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::PrintfApp(exprs, local_type, _) => {
                for expr2 in exprs {
                    self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::App(expr2, exprs, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                for expr3 in exprs {
                    self.evaluate_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::GetField(expr2, _, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Get2Field(expr2, _, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::SetField(expr2, _, expr3, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evaluate_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::UpdateField(expr2, _, expr3, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evaluate_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::UpdateGet2Field(expr2, _, expr3, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evaluate_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Uniq(expr2, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Shared(expr2, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Typed(expr2, type_expr, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                match self.evaluate_type_for_type_expr(&**type_expr, tree, type_param_env, &mut None, errs)? {
                    Some(type_value) => *local_type = Some(local_types.add_type_value(type_value)),
                    None => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
            },
            Expr::As(expr2, type_expr, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                match self.evaluate_type_for_type_expr(&**type_expr, tree, type_param_env, &mut None, errs)? {
                    Some(type_value) => {
                        if self.check_type_value_for_cast(&*type_value, tree)? {
                            *local_type = Some(local_types.add_type_value(type_value.clone()));
                        } else {
                            *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                            errs.push(FrontendError::Message(type_expr_pos(type_expr).clone(), format!("can't cast to type {} that isn't primive type", TypeValueWithLocalTypes(type_value, local_types))));
                        }
                    },
                    None => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
            },
            Expr::If(expr2, expr3, expr4, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evaluate_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                self.evaluate_types_for_expr(&mut **expr4, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Let(binds, expr2, local_type, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.evaluate_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                            self.evaluate_types_for_pattern(&mut **pattern, tree, var_env, type_param_env, local_types, errs)?;
                        },
                    }
                }
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                var_env.pop_vars();
            },
            Expr::Match(expr2, cases, local_type, _) => {
                self.evaluate_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_new_vars();
                            self.evaluate_types_for_pattern(&mut **pattern, tree, var_env, type_param_env, local_types, errs)?;
                            self.evaluate_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                            var_env.pop_vars();
                        },
                    }
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            }
        }
        Ok(())
    }

    fn evaluate_types_for_pattern(&self, pattern: &mut Pattern, tree: &Tree, var_env: &mut Environment<LocalType>, type_param_env: &mut Environment<LocalType>, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, local_type, _) => {
                self.evaluate_types_for_literal(&mut **literal, tree, var_env, type_param_env, local_types, errs, Self::evaluate_types_for_pattern)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Pattern::As(literal, type_expr, local_type1, local_type2, _) => {
                self.evaluate_types_for_literal(&mut **literal, tree, var_env, type_param_env, local_types, errs, Self::evaluate_types_for_pattern)?;
                *local_type1 = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                match self.evaluate_type_for_type_expr(&**type_expr, tree, type_param_env, &mut None, errs)? {
                    Some(type_value) => {
                        if self.check_type_value_for_cast(&*type_value, tree)? {
                            *local_type2 = Some(local_types.add_type_value(type_value.clone()));
                        } else {
                            *local_type2 = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                            errs.push(FrontendError::Message(type_expr_pos(type_expr).clone(), format!("can't cast to type {}", TypeValueWithLocalTypes(type_value, local_types))));
                        }
                    },
                    None => *local_type2 = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
            },
            Pattern::Const(_, local_type, _) => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
            Pattern::UnnamedFieldCon(_, patterns, con_local_type, local_type, _) => {
                for pattern2 in patterns {
                    self.evaluate_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?;
                }
                *con_local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Pattern::NamedFieldCon(_, pattern_named_field_pairs, con_local_type, local_type, _) => {
                for pattern_named_field_pair in pattern_named_field_pairs {
                    match pattern_named_field_pair {
                        NamedFieldPair(_, pattern2, _) => self.evaluate_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?,
                    }
                }
                *con_local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Pattern::Var(_, ident, local_type, _) => {
                let new_local_type = local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())));
                var_env.add_var(ident.clone(), new_local_type);
                *local_type = Some(new_local_type);
            },
            Pattern::At(_, ident, pattern2, local_type, _) => {
                let new_local_type = local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())));
                var_env.add_var(ident.clone(), new_local_type);
                *local_type = Some(new_local_type);
                self.evaluate_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?;
            },
            Pattern::Wildcard(local_type, _) => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
            Pattern::Alt(patterns, local_type, _) => {
                for pattern2 in patterns {
                    self.evaluate_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?;
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
        }
        Ok(())
    }


    fn evaluate_types_for_literal<T, F>(&self, literal: &mut Literal<T>, tree: &Tree, var_env: &mut Environment<LocalType>, type_param_env: &mut Environment<LocalType>, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &mut T, &Tree, &mut Environment<LocalType>, &mut Environment<LocalType>, &mut LocalTypes, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match literal {
            Literal::Tuple(field_others) => {
                for field_other in field_others {
                    f(self, field_other, tree, var_env, type_param_env, local_types, errs)?;
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, elem_other, tree, var_env, type_param_env, local_types, errs)?;
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, elem_other, tree, var_env, type_param_env, local_types, errs)?,
            _ => (),
        }
        Ok(())
    }

    fn set_shared_for_tuple(&self, ident: &String, tuple: &(LocalType, usize, Pos), tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        if tuple.1 > 1 {
            self.set_shared_for_local_type_and_var(ident.as_str(), tuple.0, &tuple.2, tree, local_types, errs)?;
        }
        Ok(())
    }
    
    fn set_shareds_for_expr(&self, expr: &Expr, tree: &Tree, var_env: &mut Environment<(LocalType, usize, Pos)>, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.set_shareds_for_literal(literal, tree, var_env, local_types, errs, Self::set_shareds_for_expr)?,
            Expr::Lambda(args, _, body, _, _, _, _, _) => {
                var_env.push_new_vars();
                for arg in args {
                    match arg {
                        LambdaArg(ident, _, Some(arg_local_type), pos) => {
                            var_env.add_var(ident.clone(), (*arg_local_type, 0, pos.clone()));
                        },
                        LambdaArg(_, _, None, _) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("set_shareds_for_expr: no local type"))])),
                    }
                }
                self.set_shareds_for_expr(&**body, tree, var_env, local_types, errs)?;
                var_env.foreach_with_result(|ident, tuple| self.set_shared_for_tuple(ident, tuple, tree, local_types, errs))?;
                var_env.pop_vars();
            },
            Expr::Var(ident, _, pos) => {
                match var_env.var_mut(ident) {
                    Some((_, counter, pos2)) => {
                        *counter += 1;
                        *pos2 = pos.clone();
                    },
                    None => (),
                }
            },
            Expr::NamedFieldConApp(_, expr_named_field_pairs, _, _, _) => {
                for expr_named_field_pair in expr_named_field_pairs {
                    match expr_named_field_pair {
                        NamedFieldPair(_, field_expr, _) => self.set_shareds_for_expr(&**field_expr, tree, var_env, local_types, errs)?,
                    }
                }
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                for expr3 in exprs {
                    self.set_shareds_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::Get2Field(expr2, _, _, _) => self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                self.set_shareds_for_expr(&**expr3, tree, var_env, local_types, errs)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                self.set_shareds_for_expr(&**expr3, tree, var_env, local_types, errs)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                self.set_shareds_for_expr(&**expr3, tree, var_env, local_types, errs)?;
            },
            Expr::Uniq(expr2, _, _) => self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::Shared(expr2, _, _) => self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::Typed(expr2, _, _, _) => self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::As(expr2, _, _, _) => self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                let saved_var_stack_idx = var_env.saved_var_stack_len();
                var_env.push_saved_vars();
                self.set_shareds_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                var_env.swap_saved_vars();
                var_env.push_saved_vars();
                self.set_shareds_for_expr(&**expr4, tree, var_env, local_types, errs)?;
                var_env.swap_saved_vars();
                var_env.merge_and_pop_saved_var_vars(saved_var_stack_idx, merge_tuples);
            },
            Expr::Let(binds, expr2, _, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.set_shareds_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                            self.set_shareds_for_pattern(&**pattern, tree, var_env, local_types, errs)?;
                        },
                    }
                }
                self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                var_env.foreach_with_result(|ident, tuple| self.set_shared_for_tuple(ident, tuple, tree, local_types, errs))?;
                var_env.pop_vars();
            },
            Expr::Match(expr2, cases, _, _) => {
                self.set_shareds_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                let saved_var_stack_idx = var_env.saved_var_stack_len();
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_saved_vars();
                            var_env.push_new_vars();
                            self.set_shareds_for_pattern(&**pattern, tree, var_env, local_types, errs)?;
                            self.set_shareds_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                            var_env.foreach_with_result(|ident, tuple| self.set_shared_for_tuple(ident, tuple, tree, local_types, errs))?;
                            var_env.pop_vars();
                            var_env.swap_saved_vars();
                        },
                    }
                }
                var_env.merge_and_pop_saved_var_vars(saved_var_stack_idx, merge_tuples);
            },
        }
        Ok(())
    }

    fn set_shareds_for_pattern(&self, pattern: &Pattern, tree: &Tree, var_env: &mut Environment<(LocalType, usize, Pos)>, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, _, _) =>  self.set_shareds_for_literal(literal, tree, var_env, local_types, errs, Self::set_shareds_for_pattern)?,
            Pattern::As(literal, _, _, _, _) =>  self.set_shareds_for_literal(literal, tree, var_env, local_types, errs, Self::set_shareds_for_pattern)?,
            Pattern::Const(_, _, _) => (),
            Pattern::UnnamedFieldCon(_, patterns, _, _, _) => {
                for pattern2 in patterns {
                    self.set_shareds_for_pattern(&**pattern2, tree, var_env, local_types, errs)?;
                }
            },
            Pattern::NamedFieldCon(_, pattern_named_field_pairs, _, _, _) => {
                for pattern_named_field_pair in pattern_named_field_pairs {
                    match pattern_named_field_pair {
                        NamedFieldPair(_, pattern2, _) => self.set_shareds_for_pattern(&**pattern2, tree, var_env, local_types, errs)?,
                    }
                }
            },
            Pattern::Var(_, ident, Some(local_type), pos) => {
                var_env.add_var(ident.clone(), (*local_type, 0, pos.clone()));
            },
            Pattern::Var(_, _, None, _) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("set_shareds_for_pattern: no local type"))])),
            Pattern::At(_, ident, pattern2, Some(local_type), pos) => {
                var_env.add_var(ident.clone(), (*local_type, 0, pos.clone()));
                self.set_shareds_for_pattern(&**pattern2, tree, var_env, local_types, errs)?;
            },
            Pattern::At(_, _, _, None, _) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("set_shareds_for_pattern: no local type"))])),
            Pattern::Wildcard(_, _) => (),
            Pattern::Alt(_, _, _) => (),
        }
        Ok(())
    }

    fn set_shareds_for_literal<T, F>(&self, literal: &Literal<T>, tree: &Tree, var_env: &mut Environment<(LocalType, usize, Pos)>, local_types: &LocalTypes, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, &Tree, &mut Environment<(LocalType, usize, Pos)>, &LocalTypes, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match literal {
            Literal::Tuple(field_others) => {
                for field_other in field_others {
                    f(self, &**field_other, tree, var_env, local_types, errs)?;
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, &**elem_other, tree, var_env, local_types, errs)?;
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, &**elem_other, tree, var_env, local_types, errs)?,
            _ => (),
        }
        Ok(())
    }

    fn local_types_for_local_type(&self, local_type: LocalType, type_values: &[Rc<TypeValue>], typ: &Type, processed_local_types: &BTreeSet<LocalType>) -> FrontendResultWithErrors<Vec<LocalType>>
    {
        match typ.type_param_entry(local_type) {
            Some(type_param_entry) => {
                let mut local_types: Vec<LocalType> = Vec::new();
                let type_param_entry_r = type_param_entry.borrow();
                for type_value in &type_param_entry_r.type_values {
                    self.add_local_types_for_type_value_and_substitution(&**type_value, type_values, &mut local_types, processed_local_types)?;
                }
                Ok(local_types)
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_types_for_local_type: no type parameter entry"))])),
        }
    }
    
    fn add_local_types_for_type_value_and_substitution(&self, type_value: &TypeValue, type_values: &[Rc<TypeValue>], local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>) -> FrontendResultWithErrors<()>
    {
        match type_value {
            TypeValue::Param(_, local_type) => add_local_type_for_substitution(*local_type, type_values, local_types, processed_local_types)?,
            TypeValue::Type(_, _, type_values2) => {
                for type_value2 in type_values2 {
                    self.add_local_types_for_type_value_and_substitution(&**type_value2, type_values, local_types, processed_local_types)?;
                }
            },
        }
        Ok(())
    }
    
    fn substitute_for_local_type(&self, local_type: LocalType, type_name: &TypeName, type_values: &mut [Rc<TypeValue>], typ: &Type) -> FrontendResultWithErrors<()>
    {
        match typ.type_param_entry(local_type) {
            Some(type_param_entry) => {
                let type_param_entry_r = type_param_entry.borrow();
                let mut new_type_values: Vec<Rc<TypeValue>> = Vec::new();
                for type_value in &type_param_entry_r.type_values {
                    match type_value.substitute(type_values) {
                        Ok(Some(new_type_value)) => new_type_values.push(new_type_value),
                        Ok(None) => new_type_values.push(type_value.clone()),
                        Err(err) => return Err(FrontendErrors::new(vec![FrontendError::Internal(format!("substitute_for_local_type: {}", err))])),
                    }
                }
                type_values[local_type.index()] = Rc::new(TypeValue::Type(UniqFlag::None, type_name.to_type_value_name(), new_type_values));
                Ok(())
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("substitute_for_local_type: no type parameter entry"))])),
        }
    }
    
    fn new_type_by_substitution(&self, typ: &Type, trait_ident: &String, type_name: &TypeName) -> FrontendResultWithErrors<Type>
    {
        let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
        let mut local_types: Vec<Option<LocalType>> = Vec::new();
        let trait_name = TraitName::Name(trait_ident.clone());
        let mut i = 0;
        for type_param_entry in typ.type_param_entries() {
            let type_param_entry_r = type_param_entry.borrow();
            if type_param_entry_r.trait_names.contains(&trait_name) {
                type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, Vec::new())));
                local_types.push(None);
            } else {
                type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(i))));
                local_types.push(Some(LocalType::new(i)));
                i += 1;
            }
        }
        let mut visited_local_types: BTreeSet<LocalType> = BTreeSet::new();
        for local_type in &local_types {
            match local_type {
                Some(local_type) => {
                    dfs_with_result(local_type, &mut visited_local_types, &mut type_values, |local_type, processed_local_types, type_values| {
                            self.local_types_for_local_type(*local_type, type_values, typ, processed_local_types)
                    }, |local_type, type_values| {
                            self.substitute_for_local_type(*local_type, type_name, type_values, typ)
                    })?;
                },
                None => (),
            }
        }
        let new_type_value = match typ.type_value().substitute(&type_values) {
            Ok(Some(type_value)) => type_value,
            Ok(None) => typ.type_value().clone(),
            Err(err) => return Err(FrontendErrors::new(vec![FrontendError::Internal(format!("new_type_by_substitution: {}", err))])),
        };
        let mut new_type = Type::new_with_type_param_entry_count(new_type_value, typ.type_param_entries().len());
        for (local_type, type_param_entry) in local_types.iter().zip(typ.type_param_entries().iter()) {
            match local_type {
                Some(local_type) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    match new_type.type_param_entry(*local_type) {
                        Some(new_type_param_entry) => {
                            let mut new_type_param_entry_r = new_type_param_entry.borrow_mut();
                            *new_type_param_entry_r = (*type_param_entry_r).clone();
                            for type_value in &mut new_type_param_entry_r.type_values {
                                match typ.type_value().substitute(&type_values) {
                                    Ok(Some(type_value2)) => *type_value = type_value2,
                                    Ok(None) => (),
                                    Err(err) => return Err(FrontendErrors::new(vec![FrontendError::Internal(format!("new_type_by_substitution: {}", err))])),
                                }
                            }
                        },
                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("new_type_by_substitution: no new type parameter entry"))])),
                    }
                },
                None => (),
            }
        }
        for j in 0..typ.type_param_entries().len() {
            for k in (j + 1)..typ.type_param_entries().len() {
                match (local_types[j], local_types[k]) {
                    (Some(local_type1), Some(local_type2)) => {
                        if typ.has_eq_type_params(LocalType::new(j), LocalType::new(k)) {
                            new_type.set_eq_type_params(local_type1, local_type2);
                        }
                    },
                    _ => (),
                }
            }
        }
        Ok(new_type)
    }
    
    fn evaluate_types_for_impl_var(&self, ident: &String, impl_var: &mut ImplVar, pos: Pos, trait_ident: &String, type_name: &TypeName, tree: &Tree, is_builtin_impl: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let new_type = match tree.trait1(trait_ident) {
            Some(trait1) => {
                let trait_r = trait1.borrow();
                match &*trait_r {
                    Trait(_, _, Some(trait_vars)) => {
                        match trait_vars.var(ident) {
                            Some(var) => {
                                let var_r = var.borrow();
                                match &*var_r {
                                    Var::Builtin(_, Some(typ)) => self.new_type_by_substitution(&**typ, trait_ident, type_name)?,
                                    Var::Var(_, _, _, _, _, _, _, Some(typ), _) => self.new_type_by_substitution(&**typ, trait_ident, type_name)?,
                                    Var::Fun(_, _, Some(typ)) => self.new_type_by_substitution(&**typ, trait_ident, type_name)?,
                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_impl_var: no type"))])),
                                }
                            },
                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_impl_var: no trait variable"))])),
                        }
                    },
                    Trait(_, _, None) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_impl_var: no trait variables"))])),
                }
            },
            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_impl_var: no trait"))])),
        };
        match impl_var {
            ImplVar::Builtin(typ) => {
                if !is_builtin_impl {
                    if !self.builtins.has_impl_var_tuple(&(trait_ident.clone(), type_name.clone(), ident.clone())) {
                        errs.push(FrontendError::Message(pos, format!("implementation variable {} mustn't be built-in variable", ident)))
                    }
                }
                *typ = Some(Box::new(new_type));
            },
            ImplVar::Var(expr, local_type, local_types, typ, _) => {
                let mut type_param_env: Environment<LocalType> = Environment::new();
                type_param_env.push_new_vars();
                for (i, type_param_entry) in new_type.type_param_entries().iter().enumerate() {
                    let type_param_entry_r = type_param_entry.borrow();
                    match &type_param_entry_r.ident {
                        Some(type_param_ident) => {
                            type_param_env.add_var(type_param_ident.clone(), LocalType::new(i));
                        },
                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_impl_var: no identifier"))])),
                    }
                }
                let mut new_local_types = LocalTypes::new();
                let new_local_type = new_local_types.set_defined_type(&new_type);
                let mut var_env: Environment<LocalType> = Environment::new();
                self.evaluate_types_for_expr(&mut **expr, tree, &mut var_env, &mut type_param_env, &mut new_local_types, errs)?;
                let mut var_env2: Environment<(LocalType, usize, Pos)> = Environment::new();
                self.set_shareds_for_expr(&**expr, tree, &mut var_env2, &new_local_types, errs)?;
                *local_type = Some(new_local_type);
                *local_types = Some(Box::new(new_local_types));
                *typ = Some(Box::new(new_type));
            },
            ImplVar::Fun(impl_fun, typ) => {
                match &mut **impl_fun {
                    ImplFun(args, body, ret_local_type, local_types) => {
                        let mut type_param_env: Environment<LocalType> = Environment::new();
                        type_param_env.push_new_vars();
                        for (i, type_param_entry) in new_type.type_param_entries().iter().enumerate() {
                            let type_param_entry_r = type_param_entry.borrow();
                            match &type_param_entry_r.ident {
                                Some(type_param_ident) => {
                                    type_param_env.add_var(type_param_ident.clone(), LocalType::new(i));
                                },
                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_impl_var: no identifier"))])),
                            }
                        }
                        let mut new_local_types = LocalTypes::new();
                        match new_local_types.set_defined_fun_types(&new_type) {
                            Some(new_local_types2) => {
                                let mut var_env: Environment<LocalType> = Environment::new();
                                var_env.push_new_vars();
                                if new_local_types2.len() >= 1 {
                                    let mut new_local_types3: Vec<LocalType> = Vec::new();
                                    for i in 0..args.len() {
                                        match &mut args[i] {
                                            ImplArg(arg_ident, arg_local_type, _) => {
                                                if i < new_local_types2.len() - 1 {
                                                    var_env.add_var(arg_ident.clone(), new_local_types2[i]);
                                                    *arg_local_type = Some(new_local_types2[i]);
                                                } else {
                                                    let new_local_type = new_local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())));
                                                    var_env.add_var(arg_ident.clone(), new_local_type);
                                                    *arg_local_type = Some(new_local_type);
                                                    new_local_types3.push(new_local_type);
                                                }
                                            },
                                        }
                                    }
                                    self.evaluate_types_for_expr(&mut **body, tree, &mut var_env, &mut type_param_env, &mut new_local_types, errs)?;
                                    let mut var_env2: Environment<(LocalType, usize, Pos)> = Environment::new();
                                    var_env2.push_new_vars();
                                    for i in 0..args.len() {
                                        match &args[i] {
                                            ImplArg(arg_ident, _, pos) => {
                                                if i < new_local_types2.len() - 1 {
                                                    var_env2.add_var(arg_ident.clone(), (new_local_types2[i], 0, pos.clone()));
                                                } else {
                                                    var_env2.add_var(arg_ident.clone(), (new_local_types3[i - (new_local_types2.len() - 1)], 0, pos.clone()));
                                                }
                                            },
                                        }
                                    }
                                    self.set_shareds_for_expr(&**body, tree, &mut var_env2, &new_local_types, errs)?;
                                    var_env2.foreach_with_result(|ident, tuple| self.set_shared_for_tuple(ident, tuple, tree, &new_local_types, errs))?;
                                    *ret_local_type = Some(new_local_types2[new_local_types2.len() - 1]);
                                    *local_types = Some(Box::new(new_local_types));
                                    *typ = Some(Box::new(new_type));
                                } else {
                                    return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_var: too few type values"))]))
                                }
                            },
                            None => (),
                        }
                    },
                }
            },
        }
        Ok(())
    }

    //
    // Inference of types.
    //

    fn infer_types_for_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Var(_, var, _) => {
                    let mut var_r = var.borrow_mut();
                    self.infer_types_for_var(&mut *var_r, tree, errs)?;
                },
                Def::Trait(_, trait1, _) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, trait_defs, _) => {
                            for trait_def in trait_defs {
                                match &**trait_def {
                                    TraitDef(_, var, _) => {
                                        let mut var_r = var.borrow_mut();
                                        self.infer_types_for_var(&mut *var_r, tree, errs)?;
                                    },
                                }
                            }
                        },
                    }
                },
                Def::Impl(impl1, _) => {
                    let impl_r = impl1.borrow();
                    match &*impl_r {
                        Impl::Builtin(_, _, _) => (),
                        Impl::Impl(_, _, impl_defs, _) => {
                            for impl_def in impl_defs {
                                match &**impl_def {
                                    ImplDef(_, impl_var, pos) => {
                                        let mut impl_var_r = impl_var.borrow_mut();
                                        self.infer_types_for_impl_var(&mut *impl_var_r, pos, tree, errs)?;
                                    },
                                }
                            }
                        },
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }
    
    fn infer_types_for_named_field_pairs<T, F>(&self, named_field_pairs: &mut [NamedFieldPair<T>], con_local_type: LocalType, local_type: LocalType, named_fields: &NamedFields, tree: &Tree, var_env: &mut Environment<()>, local_types: &mut LocalTypes, is_pattern: bool, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &mut T, &Tree, &mut Environment<()>, &mut LocalTypes, &mut Vec<FrontendError>) -> FrontendResultWithErrors<LocalType>
    {
        match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, con_local_type))) {
            Some(LocalTypeEntry::Type(type_value)) => {
                match &*type_value {
                    TypeValue::Type(_, TypeValueName::Fun, type_values) => {
                        for named_field_pair in named_field_pairs {
                            match named_field_pair {
                                NamedFieldPair(field_ident, other, field_pos) => {
                                    match named_fields.field_index(field_ident) {
                                        Some(field_idx) => {
                                            let field_local_type = f(self, other, tree, var_env, local_types, errs)?;
                                            if !is_pattern {
                                                self.match_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, field_local_type)), &type_values[field_idx], field_pos, tree, local_types, errs)?;
                                            } else {
                                                let uniq_flag = self.real_uniq_flag_for_type_value(&type_values[field_idx], local_types)?;
                                                if uniq_flag == UniqFlag::Uniq {
                                                    local_types.set_uniq(field_local_type);
                                                }
                                                self.match_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, field_local_type)), &type_values[field_idx], field_pos, tree, local_types, errs)?;
                                            }
                                        },
                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_named_field_pairs: no field index"))])),
                                    }
                                },
                            }
                        }
                        match type_values.last() {
                            Some(type_value) => {
                                local_types.set_type_value(local_type, type_value.clone());
                            },
                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_named_field_pairs: no return type value"))])),
                        }
                    },
                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_named_field_pairs: type value isn't type"))])),
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_named_field_pairs: local type entry isn't type or no local type entry"))])),
        }
        Ok(())
    }    

    fn infer_types_for_var(&self, var: &mut Var, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match var {
            Var::Builtin(_, _) => (),
            Var::Var(_, type_expr, _, Some(expr), _, Some(local_type), Some(local_types), _, _) => {
                let mut var_env: Environment<()> = Environment::new();
                let mut closure_stack = ClosureStack::new();
                let local_type2 = self.infer_types_for_expr(&mut **expr, tree, &mut var_env, &mut closure_stack, &mut **local_types, errs)?;
                self.match_local_types(*local_type, local_type2, type_expr_pos(&**type_expr), tree, local_types, errs)?;
            },
            Var::Var(_, _, _, None, _, _, _, _, _) => (),
            Var::Fun(fun, _, _) => {
                match &mut **fun {
                    Fun::Fun(_, args, ret_type_expr, _, Some(expr), Some(ret_local_type), Some(local_types)) => {
                        let mut var_env: Environment<()> = Environment::new();
                        let mut closure_stack = ClosureStack::new();
                        var_env.push_new_vars();
                        for arg in args {
                            match arg {
                                Arg(ident, _, _, _) => {
                                    var_env.add_var(ident.clone(), ());
                                },
                            }
                        }
                        let ret_local_type2 = self.infer_types_for_expr(&mut **expr, tree, &mut var_env, &mut closure_stack, &mut **local_types, errs)?;
                        self.match_local_types(*ret_local_type, ret_local_type2, type_expr_pos(&**ret_type_expr), tree, local_types, errs)?;
                    },
                    Fun::Fun(_, _, _, _, None, _, _) => (),
                    Fun::Con(_) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_var: variable is contructor"))])),
                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_var: no local type or no local types"))])),
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_var: no local type or no local types"))])),
        }
        Ok(())
    }

    fn check_builtin_type_ident(&self, ident: &String, count: usize, pos: Pos, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<bool>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let type_var_r = type_var.borrow();
                match &*type_var_r {
                    TypeVar::Builtin(Some(type_args), _, _) => {
                        if count == type_args.type_arg_idents().len() {
                            Ok(true)
                        } else {
                            errs.push(FrontendError::Message(pos.clone(), format!("number of type arguments of built-in type variable {} isn't equal to number of inferred type arguments", ident)));
                            Ok(false)
                        }
                    },
                    TypeVar::Builtin(None, _, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_builtin_type_ident: no type arguments"))])),
                    _ => {
                        errs.push(FrontendError::Message(pos, format!("type variable {} isn't built-in type variable", ident)));
                        Ok(false)
                    },
                }
            },
            None => {
                errs.push(FrontendError::Message(pos, format!("undefined built-in type variable {}", ident)));
                Ok(false)
            }
        }
    }    
    
    fn local_type_for_fields(&self, local_type: LocalType, fields: &mut [Field], pos: &Pos, tree: &Tree, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<LocalType>>
    {
        let mut current_local_type = local_type;
        for field in fields {
            let field_idx = match field {
                Field::Unnamed(tmp_field_idx, _) => Some(*tmp_field_idx),
                Field::Named(field_ident, _) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, current_local_type))) {
                        Some(LocalTypeEntry::Type(type_value)) => {
                            match &*type_value {
                                TypeValue::Type(_, TypeValueName::Name(type_ident), _) => {
                                    match tree.type_var(type_ident) {
                                        Some(type_var) => {
                                            let type_var_r = type_var.borrow();
                                            match &*type_var_r {
                                                TypeVar::Builtin(_, Some(fields2), _) => {
                                                    match fields2.field_index(field_ident) {
                                                        Some(tmp_field_idx) => Some(tmp_field_idx),
                                                        None => {
                                                            errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field_ident)));
                                                            None
                                                        },
                                                    }
                                                },
                                                TypeVar::Data(_, cons, _) => {
                                                    if cons.len() == 1 {
                                                        let con_r = cons[0].borrow();
                                                        match &*con_r {
                                                            Con::NamedField(_, _, _, Some(named_fields), _) => {
                                                                match named_fields.field_index(field_ident) {
                                                                    Some(tmp_field_idx) => Some(tmp_field_idx),
                                                                    None => {
                                                                        errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field_ident)));
                                                                        None
                                                                    },
                                                                }
                                                            },
                                                            Con::NamedField(_, _, _, None, _) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: type variable isn't type or no fields"))])),
                                                            _ => {
                                                                errs.push(FrontendError::Message(pos.clone(), format!("type {} has constructor without named fields", LocalTypeWithLocalTypes(current_local_type, local_types))));
                                                                None
                                                            },
                                                        }
                                                    } else {
                                                        errs.push(FrontendError::Message(pos.clone(), format!("type {} has too many constructors for fields", LocalTypeWithLocalTypes(current_local_type, local_types))));
                                                        None
                                                    }
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: type variable isn't type or no fields"))])),
                                            }
                                        },
                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: no type variable"))])),
                                    }
                                },
                                TypeValue::Type(_, _, _) => {
                                    errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field_ident)));
                                    None
                                },
                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: type value isn't type"))]))
                            }
                        },
                        Some(_) => {
                            errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field_ident)));
                            None
                        },
                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: no local type entry"))])),
                    }
                },
            };
            match field_idx {
                Some(field_idx) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, current_local_type))) {
                        Some(LocalTypeEntry::Type(type_value)) => {
                            match &*type_value {
                                TypeValue::Type(_, TypeValueName::Tuple, type_values) => {
                                    if field_idx < type_values.len() {
                                        current_local_type = local_types.add_type_value(type_values[field_idx].clone());
                                    } else {
                                        errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field)));
                                        return Ok(None);
                                    }
                                },
                                TypeValue::Type(_, TypeValueName::Name(type_ident), type_values) => {
                                    match tree.type_var(type_ident) {
                                        Some(type_var) => {
                                            let type_var_r = type_var.borrow();
                                            match &*type_var_r {
                                                TypeVar::Builtin(_, Some(fields2), _) => {
                                                    if field_idx < fields2.field_type_values().len() {
                                                        let new_type_value = match fields2.field_type_values()[field_idx].substitute(type_values) {
                                                            Ok(Some(tmp_type_value)) => tmp_type_value,
                                                            Ok(None) => fields2.field_type_values()[field_idx].clone(),
                                                            Err(err) => return Err(FrontendErrors::new(vec![FrontendError::Internal(format!("local_type_for_fields: {}", err))])),
                                                        };
                                                        current_local_type = local_types.add_type_value(new_type_value);
                                                    } else {
                                                        errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field)));
                                                        return Ok(None);
                                                    }
                                                },
                                                TypeVar::Data(_, cons, _) => {
                                                    if cons.len() == 1 {
                                                        let con_r = cons[0].borrow();
                                                        let con_ident = match &*con_r {
                                                            Con::UnnamedField(tmp_con_ident, _, _, _) => tmp_con_ident,
                                                            Con::NamedField(tmp_con_ident, _, _, _, _) => tmp_con_ident,
                                                        };
                                                        let is_success = type_for_fun_ident_in(con_ident, tree, |typ| {
                                                                match &**typ.type_value() {
                                                                    TypeValue::Type(_, TypeValueName::Fun, type_values2) => {
                                                                        if type_values.len() >= 1 {
                                                                            if field_idx < type_values2.len() - 1 {
                                                                                let new_type_value = match type_values2[field_idx].substitute(type_values) {
                                                                                    Ok(Some(tmp_type_value)) => tmp_type_value,
                                                                                    Ok(None) => type_values2[field_idx].clone(),
                                                                                    Err(err) => return Err(FrontendErrors::new(vec![FrontendError::Internal(format!("local_type_for_fields: {}", err))])),
                                                                                };
                                                                                current_local_type = local_types.add_type_value(new_type_value);
                                                                                Ok(true)
                                                                            } else {
                                                                                errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field)));
                                                                                Ok(false)
                                                                            }
                                                                        } else {
                                                                            Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: too few argument type values"))]))
                                                                        }
                                                                    },
                                                                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: type value isn't function type"))]))
                                                                }
                                                        })?;
                                                        if !is_success {
                                                            return Ok(None);
                                                        }
                                                    } else {
                                                        errs.push(FrontendError::Message(pos.clone(), format!("type {} has too many constructors for fields", LocalTypeWithLocalTypes(current_local_type, local_types))));
                                                        return Ok(None);
                                                    }
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: type variable isn't type or no fields"))])),
                                            }
                                        },
                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: no type variable"))])),
                                    }
                                },
                                TypeValue::Type(_, _, _) => {
                                    errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field)));
                                    return Ok(None);
                                },
                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: type value isn't type"))]))
                            }
                        },
                        Some(_) => {
                            errs.push(FrontendError::Message(pos.clone(), format!("type {} hasn't field {}", LocalTypeWithLocalTypes(current_local_type, local_types), field)));
                            return Ok(None);
                        },
                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("local_type_for_fields: no local type entry"))])),
                    }
                },
                _ => return Ok(None),
            }
            match field {
                Field::Unnamed(_, field_local_type) => *field_local_type = Some(current_local_type),
                Field::Named(_, field_local_type) => *field_local_type = Some(current_local_type),
            }
        }
        Ok(Some(current_local_type))
    }
    
    fn infer_types_for_expr(&self, expr: &mut Expr, tree: &Tree, var_env: &mut Environment<()>, closure_stack: &mut ClosureStack, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<LocalType>
    {
        match expr {
            Expr::Literal(literal, Some(local_type), pos) => {
                self.infer_types_for_literal(&mut **literal, *local_type, pos, tree, var_env, local_types, true, errs, |typer, expr, tree, var_env, local_types, errs| {
                        typer.infer_types_for_expr(expr, tree, var_env, closure_stack, local_types, errs)
                }, expr_pos)?;
                Ok(*local_type)
            },
            Expr::Lambda(args, _, body, Some(ret_local_type), Some(local_type), _, _, pos) => {
                let stack_idx = var_env.stack_len();
                var_env.push_new_vars();
                closure_stack.push_new_closure(stack_idx);
                for arg in &*args {
                    match arg {
                        LambdaArg(ident, _, _, _) => {
                            var_env.add_var(ident.clone(), ());
                        },
                    }
                }
                let body_local_type = self.infer_types_for_expr(body, tree, var_env, closure_stack, local_types, errs)?;
                self.match_local_types(body_local_type, *ret_local_type, pos, tree, local_types, errs)?;
                let mut uniq_flag = UniqFlag::None;
                let mut shared_flag = SharedFlag::Shared;
                let mut closure_local_types: BTreeSet<LocalType> = BTreeSet::new(); 
                closure_stack.foreach_with_result(|_, closure_local_type| {
                        let (uniq_flag2, shared_flag2) = self.uniq_flag_and_shared_flag_for_local_type(closure_local_type, tree, local_types)?;
                        if uniq_flag2 == UniqFlag::Uniq {
                            uniq_flag = UniqFlag::Uniq;
                        }
                        if shared_flag2 == SharedFlag::None {
                            shared_flag = SharedFlag::None;
                        }
                        closure_local_types.insert(closure_local_type);
                        Ok(())
                })?;
                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                for arg in &*args {
                    match arg {
                        LambdaArg(_, _, Some(arg_local_type), _) => type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, *arg_local_type))),
                        LambdaArg(_, _, None, _) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_expr: no local type"))])),
                    }
                }
                type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, *ret_local_type)));
                let mut is_in_non_uniq_lambda = false;
                match (uniq_flag, shared_flag) {
                    (UniqFlag::None, SharedFlag::None) => {
                        let mut type_param_entry = TypeParamEntry::new();
                        type_param_entry.trait_names.insert(TraitName::Fun);
                        type_param_entry.type_values = type_values;
                        type_param_entry.closure_local_types = closure_local_types;
                        local_types.set_type_param(*local_type, Rc::new(RefCell::new(type_param_entry)));
                        is_in_non_uniq_lambda = true;
                    },
                    (UniqFlag::None, SharedFlag::Shared) => {
                        local_types.set_type_value(*local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values)));
                        is_in_non_uniq_lambda = true;
                    },
                    (UniqFlag::Uniq, _) => {
                        local_types.set_type_value(*local_type, Rc::new(TypeValue::Type(UniqFlag::Uniq, TypeValueName::Fun, type_values)));
                    },
                };
                if is_in_non_uniq_lambda {
                    closure_stack.foreach_with_result(|_, closure_local_type| {
                            local_types.set_in_non_uniq_lambda(closure_local_type, true);
                            Ok(())
                    })?;
                }
                closure_stack.merge_and_pop_closure(); 
                var_env.pop_vars();
                Ok(*local_type)
            },
            Expr::Var(ident, Some(local_type), _) => {
                match var_env.stack_index(ident) {
                    Some(stack_idx) => {
                        closure_stack.add_local_type((ident.clone(), stack_idx), *local_type);
                    },
                    None => type_for_var_ident_in(ident, tree, |typ| set_type_for_local_types(*local_type, typ, local_types))?,
                }
                Ok(*local_type)
            },
            Expr::NamedFieldConApp(ident, expr_named_field_pairs, Some(con_local_type), Some(local_type), _) => {
                type_and_named_fields_for_con_ident_in(ident, tree, |typ, named_fields| {
                        set_type_for_local_types(*con_local_type, typ, local_types)?;
                        self.infer_types_for_named_field_pairs(expr_named_field_pairs.as_mut_slice(), *con_local_type, *local_type, named_fields, tree, var_env, local_types, false, errs, |typer, expr, tree, var_env, local_types, errs| {
                                typer.infer_types_for_expr(expr, tree, var_env, closure_stack, local_types, errs)
                        })?;
                        Ok(())
                })?;
                Ok(*local_type)
            },
            Expr::PrintfApp(exprs, Some(local_type), pos) => {
                match exprs.first_mut() {
                    Some(expr2) => {
                        if self.check_builtin_type_ident(&String::from("Char"), 0, pos.clone(), tree, errs)? && self.check_builtin_type_ident(&String::from("ConstantSlice"), 1, pos.clone(), tree, errs)? {
                            let expr2_local_type = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                            let str_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("ConstantSlice")), vec![Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()))]));
                            self.match_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, expr2_local_type)), &str_type_value, expr_pos(&**expr2), tree, local_types, errs)?;
                        }
                        for expr3 in &mut exprs[1..] {
                            let expr3_local_type = self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?;
                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, expr3_local_type))) {
                                Some(LocalTypeEntry::Type(type_value)) => {
                                    match &*type_value {
                                        TypeValue::Type(_, TypeValueName::Name(expr3_type_ident), _) => {
                                            if !self.has_primitive_for_type_ident(expr3_type_ident, tree)? {
                                                errs.push(FrontendError::Message(expr_pos(&**expr3).clone(), format!("printf must't take values with type {}", LocalTypeWithLocalTypes(expr3_local_type, local_types))));
                                            }
                                        },
                                        _ => errs.push(FrontendError::Message(expr_pos(&**expr3).clone(), format!("printf mustn't take values with type {}", LocalTypeWithLocalTypes(expr3_local_type, local_types)))),
                                    }
                                },
                                Some(LocalTypeEntry::Param(_, _, _, _)) => errs.push(FrontendError::Message(expr_pos(expr3).clone(), format!("printf must't take values with type {}", LocalTypeWithLocalTypes(expr3_local_type, local_types)))),
                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_expr: no local type entry"))])), 
                            }
                        }
                    },
                    None => errs.push(FrontendError::Message(pos.clone(), String::from("too few arguments for printf"))),
                }
                if self.check_builtin_type_ident(&String::from("Int"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(*local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
                }
                Ok(*local_type)
            },
            Expr::App(expr2, exprs, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                let mut local_types2: Vec<LocalType> = Vec::new();
                for expr3 in exprs {
                    local_types2.push(self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?);
                }
                let mut type_param_entry = TypeParamEntry::new();
                type_param_entry.trait_names.insert(TraitName::Fun);
                type_param_entry.type_values = local_types2.iter().map(|lt| Rc::new(TypeValue::Param(UniqFlag::None, *lt))).collect();
                type_param_entry.type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, *local_type)));
                let local_type3 = local_types.add_type_param(Rc::new(RefCell::new(type_param_entry)));
                self.match_local_types(local_type2, local_type3, pos, tree, local_types, errs)?;
                Ok(*local_type)
            },
            Expr::GetField(expr2, fields, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                match self.local_type_for_fields(local_type2, fields, pos, tree, local_types, errs)? {
                    Some(local_type3) => self.match_local_types(local_type3, *local_type, pos, tree, local_types, errs)?, 
                    None => (),
                }
                Ok(*local_type)
            },
            Expr::Get2Field(expr2, fields, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                match self.local_type_for_fields(local_type2, fields, pos, tree, local_types, errs)? {
                    Some(local_type3) => {
                        // (t3, t2)
                        let type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, vec![Rc::new(TypeValue::Param(UniqFlag::None, local_type3)), Rc::new(TypeValue::Param(UniqFlag::None, local_type2))]));
                        self.match_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), &type_value, pos, tree, local_types, errs)?;
                        self.set_shared_for_local_type_and_value(local_type3, pos, tree, local_types, errs)?;
                    }
                    None => (),
                }
                Ok(*local_type)
            },
            Expr::SetField(expr2, fields, expr3, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                let local_type3 = self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?;
                match self.local_type_for_fields(local_type2, fields, pos, tree, local_types, errs)? {
                    Some(local_type4) => {
                        // t4
                        self.match_local_types(local_type3, local_type4, expr_pos(&**expr3), tree, local_types, errs)?;
                        // t2
                        self.match_local_types(local_type2, *local_type, pos, tree, local_types, errs)?;
                    },
                    None => (),
                }
                Ok(*local_type)
            },
            Expr::UpdateField(expr2, fields, expr3, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                let local_type3 = self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?;
                match self.local_type_for_fields(local_type2, fields, pos, tree, local_types, errs)? {
                    Some(local_type4) => {
                        // (t4) -> t4
                        let mut type_param_entry = TypeParamEntry::new();
                        type_param_entry.trait_names.insert(TraitName::Fun);
                        type_param_entry.type_values = vec![Rc::new(TypeValue::Param(UniqFlag::None, local_type4)); 2];
                        let fun_local_type = local_types.add_type_param(Rc::new(RefCell::new(type_param_entry)));
                        self.match_local_types(local_type3, fun_local_type, expr_pos(&**expr3), tree, local_types, errs)?;
                        // t2
                        self.match_local_types(local_type2, *local_type, pos, tree, local_types, errs)?;
                    },
                    None => (),
                }
                Ok(*local_type)
            },
            Expr::UpdateGet2Field(expr2, fields, expr3, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                let local_type3 = self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?;
                match self.local_type_for_fields(local_type2, fields, pos, tree, local_types, errs)? {
                    Some(local_type4) => {
                        // (t4) -> (t5, t4)
                        let mut type_param_entry = TypeParamEntry::new();
                        type_param_entry.trait_names.insert(TraitName::Fun);
                        let local_type5 = local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())));
                        let ret_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, vec![Rc::new(TypeValue::Param(UniqFlag::None, local_type5)), Rc::new(TypeValue::Param(UniqFlag::None, local_type4))]));
                        type_param_entry.type_values = vec![Rc::new(TypeValue::Param(UniqFlag::None, local_type4)), ret_type_value];
                        let fun_local_type = local_types.add_type_param(Rc::new(RefCell::new(type_param_entry)));
                        self.match_local_types(local_type3, fun_local_type, expr_pos(&**expr3), tree, local_types, errs)?;
                        // (t5, t2)
                        let ret_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, vec![Rc::new(TypeValue::Param(UniqFlag::None, local_type5)), Rc::new(TypeValue::Param(UniqFlag::None, local_type2))]));
                        self.match_type_values(&ret_type_value2, &Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), pos, tree, local_types, errs)?;
                    },
                    None => (),
                }
                Ok(*local_type)
            },
            Expr::Uniq(expr2, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                    Some(LocalTypeEntry::Type(type_value)) => {
                        match &*type_value {
                            TypeValue::Type(_, type_value_name, type_values) => {
                                let new_type_value = Rc::new(TypeValue::Type(UniqFlag::Uniq, type_value_name.clone(), type_values.clone()));
                                self.match_type_values(&new_type_value, &Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), pos, tree, local_types, errs)?;
                            },
                            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_expr: type value isn't type"))])),
                        }
                    },
                    Some(_) => errs.push(FrontendError::Message(pos.clone(), format!("type {} is type parameter", LocalTypeWithLocalTypes(local_type2, local_types)))),
                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_expr: no local type entry"))])),
                }
                Ok(*local_type)
            },
            Expr::Shared(expr2, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                    Some(LocalTypeEntry::Type(type_value)) => {
                        match &*type_value {
                            TypeValue::Type(_, type_value_name, type_values) => {
                                let new_type_value = Rc::new(TypeValue::Type(UniqFlag::None, type_value_name.clone(), type_values.clone()));
                                self.match_type_values(&new_type_value, &Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), pos, tree, local_types, errs)?;
                                if self.shared_flag_for_local_type(*local_type, tree, local_types)? == SharedFlag::None {
                                    errs.push(FrontendError::Message(pos.clone(), format!("type {} is unique type", LocalTypeWithLocalTypes(local_type2, local_types))));
                                }
                            },
                            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_expr: type value isn't type"))])),
                        }
                    },
                    Some(_) => errs.push(FrontendError::Message(pos.clone(), format!("type {} is type parameter", LocalTypeWithLocalTypes(local_type2, local_types)))),
                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_expr: no local type entry"))])),
                }
                Ok(*local_type)
            },
            Expr::Typed(expr2, _, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                self.match_local_types(local_type2, *local_type, pos, tree, local_types, errs)?;
                Ok(*local_type)
            },
            Expr::As(expr2, _, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                self.cast_local_type(local_type2, *local_type, pos, tree, local_types, errs)?;
                Ok(*local_type)
            },
            Expr::If(expr2, expr3, expr4, Some(local_type), pos) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                if self.check_builtin_type_ident(&String::from("Bool"), 0, pos.clone(), tree, errs)? {
                    self.match_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2)), &Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Bool")), Vec::new())), pos, tree, local_types, errs)?;
                }
                let local_type3 = self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?;
                let local_type4 = self.infer_types_for_expr(&mut **expr4, tree, var_env, closure_stack, local_types, errs)?;
                self.match_local_types(local_type3, local_type4, pos, tree, local_types, errs)?;
                self.match_local_types(local_type3, *local_type, pos, tree, local_types, errs)?;
                Ok(*local_type)
            },
            Expr::Let(binds, expr2, Some(local_type), pos) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            let expr3_local_type = self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?;
                            let mut var_local_types: Vec<(String, LocalType, Pos)> = Vec::new();
                            let pattern_local_type = self.infer_types_for_pattern(&mut **pattern, tree, var_env, &mut var_local_types, local_types, false, errs)?;
                            self.match_local_types_for_first_pattern_type(pattern_local_type, expr3_local_type, pattern_pos(pattern), tree, local_types, errs)?;
                        },
                    }
                }
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                self.match_local_types(local_type2, *local_type, pos, tree, local_types, errs)?;
                var_env.pop_vars();
                Ok(*local_type)
            },
            Expr::Match(expr2, cases, Some(local_type), _) => {
                let local_type2 = self.infer_types_for_expr(&mut **expr2, tree, var_env, closure_stack, local_types, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_new_vars();
                            let mut var_local_types: Vec<(String, LocalType, Pos)> = Vec::new();
                            let pattern_local_type = self.infer_types_for_pattern(&mut **pattern, tree, var_env, &mut var_local_types, local_types, false, errs)?;
                            self.match_local_types_for_second_pattern_type(local_type2, pattern_local_type, pattern_pos(pattern), tree, local_types, errs)?;
                            let expr3_local_type = self.infer_types_for_expr(&mut **expr3, tree, var_env, closure_stack, local_types, errs)?;
                            self.match_local_types(expr3_local_type, *local_type, expr_pos(&**expr3), tree, local_types, errs)?;
                            var_env.pop_vars();
                        },
                    }
                }
                Ok(*local_type)
            }
            _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_expr: no local type"))])),
        }
    }

    fn infer_types_for_pattern(&self, pattern: &mut Pattern, tree: &Tree, var_env: &mut Environment<()>, var_local_types: &mut Vec<(String, LocalType, Pos)>, local_types: &mut LocalTypes, can_add_var_local_type: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<LocalType>
    {
        match pattern {
            Pattern::Literal(literal, Some(local_type), pos) => {
                self.infer_types_for_literal(&mut **literal, *local_type, pos, tree, var_env, local_types, false, errs, |typer, pattern, tree, var_env, local_types, errs| {
                        typer.infer_types_for_pattern(pattern, tree, var_env, var_local_types, local_types, can_add_var_local_type, errs)
                }, pattern_pos)?;
                Ok(*local_type)
            },
            Pattern::As(literal, _, Some(literal_local_type), Some(local_type), pos) => {
                self.infer_types_for_literal(&mut **literal, *literal_local_type, pos, tree, var_env, local_types, false, errs, |typer, pattern, tree, var_env, local_types, errs| {
                        typer.infer_types_for_pattern(pattern, tree, var_env, var_local_types, local_types, can_add_var_local_type, errs)
                }, pattern_pos)?;
                self.cast_local_type(*literal_local_type, *local_type, pos, tree, local_types, errs)?;
                Ok(*local_type)
            },
            Pattern::Const(ident, Some(local_type), _) => {
                type_for_var_ident_in(ident, tree, |typ| set_type_for_local_types(*local_type, typ, local_types))?;
                Ok(*local_type)
            },
            Pattern::UnnamedFieldCon(ident, patterns, Some(con_local_type), Some(local_type), _) => {
                type_for_var_ident_in(ident, tree, |typ| {
                        set_type_for_local_types(*con_local_type, typ, local_types)?;
                        match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *con_local_type))) {
                            Some(LocalTypeEntry::Type(type_value)) => {
                                match &*type_value {
                                    TypeValue::Type(_, TypeValueName::Fun, type_values) => {
                                        if type_values.len() >= 1 {
                                            for (pattern2, type_value) in patterns.iter_mut().zip(type_values.iter()) {
                                                let field_local_type = self.infer_types_for_pattern(&mut **pattern2, tree, var_env, var_local_types, local_types, can_add_var_local_type, errs)?;
                                                let uniq_flag = self.real_uniq_flag_for_type_value(type_value, local_types)?;
                                                if uniq_flag == UniqFlag::Uniq {
                                                    local_types.set_uniq(field_local_type);
                                                }
                                                self.match_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, field_local_type)), type_value, pattern_pos(&**pattern2), tree, local_types, errs)?;
                                            }
                                            local_types.set_type_value(*local_type, type_value.clone());
                                        } else {
                                            return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_patterns: no return type value"))]))
                                        }
                                    },
                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_patterns: type value isn't type"))])),
                                }
                            },
                            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_patterns: local type entry isn't type or no local type entry"))])),
                        }
                        Ok(())
                })?;
                Ok(*local_type)
            },
            Pattern::NamedFieldCon(ident, pattern_named_field_pairs, Some(con_local_type), Some(local_type), _) => {
                type_and_named_fields_for_con_ident_in(ident, tree, |typ, named_fields| {
                        set_type_for_local_types(*con_local_type, typ, local_types)?;
                        self.infer_types_for_named_field_pairs(pattern_named_field_pairs.as_mut_slice(), *con_local_type, *local_type, named_fields, tree, var_env, local_types, true, errs, |typer, pattern, tree, var_env, local_types, errs| {
                                typer.infer_types_for_pattern(pattern, tree, var_env, var_local_types, local_types, can_add_var_local_type, errs)
                        })?;
                        Ok(())
                })?;
                Ok(*local_type)
            },
            Pattern::Var(_, ident, Some(local_type), pos) => {
                var_env.add_var(ident.clone(), ());
                if can_add_var_local_type {
                    var_local_types.push((ident.clone(), *local_type, pos.clone()));
                }
                Ok(*local_type)
            },
            Pattern::At(_, ident, pattern2, Some(local_type), pos) => {
                var_env.add_var(ident.clone(), ());
                if can_add_var_local_type {
                    var_local_types.push((ident.clone(), *local_type, pos.clone()));
                }
                let mut var_local_types2: Vec<(String, LocalType, Pos)> = Vec::new();
                let local_type2 = self.infer_types_for_pattern(&mut **pattern2, tree, var_env, &mut var_local_types2, local_types, true, errs)?;
                self.match_local_types(*local_type, local_type2, pos, tree, local_types, errs)?;
                for (ident2, var_local_type2, pos2) in &var_local_types2 {
                    self.set_shared_for_local_type_and_var(ident2.as_str(), *var_local_type2, pos2, tree, local_types, errs)?;
                }
                Ok(*local_type)
            },
            Pattern::Wildcard(Some(local_type), _) => Ok(*local_type),
            Pattern::Alt(patterns, Some(local_type), _) => {
                for pattern2 in patterns {
                    let local_type2 = self.infer_types_for_pattern(&mut **pattern2, tree, var_env, var_local_types, local_types, can_add_var_local_type, errs)?;
                    self.match_local_types(local_type2, *local_type, pattern_pos(&**pattern2), tree, local_types, errs)?;
                }
                Ok(*local_type)
            },
            _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_pattern: no local type"))])),
        }
    }

    fn infer_types_for_literal<T, F, G>(&self, literal: &mut Literal<T>, local_type: LocalType, pos: &Pos, tree: &Tree, var_env: &mut Environment<()>, local_types: &mut LocalTypes, is_expr: bool, errs: &mut Vec<FrontendError>, mut f: F, mut g: G) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &mut T, &Tree, &mut Environment<()>, &mut LocalTypes, &mut Vec<FrontendError>) -> FrontendResultWithErrors<LocalType>,
            G: FnMut(&T) -> &Pos
    {
        match literal {
            Literal::Bool(_) => {
                if self.check_builtin_type_ident(&String::from("Bool"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Bool")), Vec::new())));
                }
            },
            Literal::Char(_) => {
                if self.check_builtin_type_ident(&String::from("Char"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new())));
                }
            },
            Literal::Int(_) => {
                if self.check_builtin_type_ident(&String::from("Int"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
                }
            },
            Literal::Long(_) => {
                if self.check_builtin_type_ident(&String::from("Long"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Long")), Vec::new())));
                }
            },
            Literal::Uint(_) => {
                if self.check_builtin_type_ident(&String::from("Uint"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Uint")), Vec::new())));
                }
            },
            Literal::Ulong(_) => {
                if self.check_builtin_type_ident(&String::from("Ulong"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Ulong")), Vec::new())));
                }
            },
            Literal::Float(_) => {
                if self.check_builtin_type_ident(&String::from("Float"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new())));
                }
            },
            Literal::Double(_) => {
                if self.check_builtin_type_ident(&String::from("Double"), 0, pos.clone(), tree, errs)? {
                    local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Double")), Vec::new())));
                }
            },
            Literal::String(_) => {
                if self.check_builtin_type_ident(&String::from("Char"), 0, pos.clone(), tree, errs)? && self.check_builtin_type_ident(&String::from("ConstantSlice"), 1, pos.clone(), tree, errs)? {
                    let str_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("ConstantSlice")), vec![Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()))]));
                    local_types.set_type_value(local_type, str_type_value);
                }
            },
            Literal::Tuple(field_others) => {
                let mut field_type_values: Vec<Rc<TypeValue>> = Vec::new();
                for field_other in field_others {
                    let field_local_type = f(self, &mut **field_other, tree, var_env, local_types, errs)?;
                    field_type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, field_local_type)));
                }
                local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, field_type_values)));
            },
            Literal::Array(elem_others) => {
                let elem_local_type = local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())));
                for elem_other in &mut *elem_others {
                    let elem_local_type2 = f(self, &mut **elem_other, tree, var_env, local_types, errs)?;
                    self.match_local_types(elem_local_type2, elem_local_type, g(&**elem_other), tree, local_types, errs)?;
                }
                local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Array(Some(elem_others.len())), vec![Rc::new(TypeValue::Param(UniqFlag::Uniq, elem_local_type))])));
            },
            Literal::FilledArray(elem_other, len) => {
                let elem_local_type = f(self, &mut **elem_other, tree, var_env, local_types, errs)?;
                if is_expr && *len > 1 {
                    self.set_shared_for_local_type_and_value(elem_local_type, g(&**elem_other), tree, local_types, errs)?;
                }
                local_types.set_type_value(local_type, Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Array(Some(*len)), vec![Rc::new(TypeValue::Param(UniqFlag::Uniq, elem_local_type))])));
            },
        }
        Ok(())
    }

    fn infer_types_for_impl_var(&self, impl_var: &mut ImplVar, pos: &Pos, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match impl_var {
            ImplVar::Builtin(_) => (),
            ImplVar::Var(expr, Some(local_type), Some(local_types), _, _) => {
                let mut var_env: Environment<()> = Environment::new();
                let mut closure_stack = ClosureStack::new();
                let local_type2 = self.infer_types_for_expr(&mut **expr, tree, &mut var_env, &mut closure_stack, &mut **local_types, errs)?;
                self.match_local_types(*local_type, local_type2, pos, tree, local_types, errs)?;
            },
            ImplVar::Fun(impl_fun, _) => {
                match &mut **impl_fun {
                    ImplFun(args, expr, Some(ret_local_type), Some(local_types)) => {
                        let mut var_env: Environment<()> = Environment::new();
                        let mut closure_stack = ClosureStack::new();
                        var_env.push_new_vars();
                        for arg in args {
                            match arg {
                                ImplArg(ident, _, _) => {
                                    var_env.add_var(ident.clone(), ());
                                },
                            }
                        }
                        let ret_local_type2 = self.infer_types_for_expr(&mut **expr, tree, &mut var_env, &mut closure_stack, &mut **local_types, errs)?;
                        self.match_local_types(*ret_local_type, ret_local_type2, pos, tree, local_types, errs)?;
                    },
                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_impl_var: no local type or no local types"))])),
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("infer_types_for_impl_var: no local type or no local types"))])),
        }
        Ok(())
    }
}
