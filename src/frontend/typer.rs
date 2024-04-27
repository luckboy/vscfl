//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
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
        Some(tmp_local_type) => *tmp_local_type,
        None => {
            let tmp_local_type = LocalType::new(*local_type_counter);
            type_param_env.add_var(ident.clone(), tmp_local_type);
            *local_type_counter += 1;
            tmp_local_type
        },
    }
}

fn local_type_for_type_param_ident(ident: &String, type_param_env: &Environment<LocalType>) -> FrontendResultWithErrors<LocalType>
{
    match type_param_env.var(ident) {
        Some(tmp_local_type) => Ok(*tmp_local_type),
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

fn shared_flag_for_type_var_ident(ident: &String, tree: &Tree) -> FrontendResultWithErrors<Option<SharedFlag>>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let mut type_var_r = type_var.borrow_mut();
            match &mut *type_var_r {
                TypeVar::Builtin(_, _, shared_flag) => Ok(*shared_flag),
                TypeVar::Data(_, _, shared_flag) => Ok(*shared_flag),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_var_ident: type variable is type synonym"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_var_ident: no type variable"))])),
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

fn shared_flag_for_type_var_ident2(ident: &String, tree: &Tree) -> FrontendResultWithErrors<SharedFlag>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let mut type_var_r = type_var.borrow_mut();
            match &mut *type_var_r {
                TypeVar::Builtin(_, _, Some(shared_flag)) => Ok(*shared_flag),
                TypeVar::Data(_, _, Some(shared_flag)) => Ok(*shared_flag),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_var_ident2: type variable is type synonym or no shared flag"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_flag_for_type_var_ident2: no type variable"))])),
    }
}

fn add_local_type(local_type: LocalType, pos: Pos, typ: &Type, local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
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
                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_local_type: no identifier"))]))
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_local_type: no type parameter entry"))])),
        }
    }
}

fn new_type_from_type_value_and_type_param_env(type_value: Rc<TypeValue>, type_param_env: &Environment<LocalType>, local_type_counter: &Option<usize>) -> FrontendResultWithErrors<Type>
{
    match local_type_counter {
        Some(local_type_counter) => {
            let mut idents: Vec<Option<String>> = vec![None; *local_type_counter];
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

pub struct Typer
{
    type_matcher: TypeMatcher,
    builtins: Builtins,
}

impl Typer
{
    pub fn new() -> Self
    { Typer { type_matcher: TypeMatcher::new(), builtins: Builtins::new(), } }

    pub fn evalaute_types_for_type_vars(&self, tree: &Tree) -> FrontendResultWithErrors<()>
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

    fn set_shared(&self, ident: &str, local_type: LocalType, pos: Pos, tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match self.type_matcher.set_shared(local_type, tree, local_types) {
            Ok(true) => Ok(()),
            Ok(false) => {
                errs.push(FrontendError::Message(pos, format!("variable {} mustn't shared with type {}", ident, LocalTypeWithLocalTypes(local_type, local_types))));
                Ok(())
            },
            Err(err) => Err(FrontendErrors::new(vec![err])),
        }
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
                                                        return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("shared_type_idents_for_type_ident: too few argument type values"))]))
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
                                                        return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_shared_flag_for_type_ident: too few argument type values"))]))
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
                    TypeValueName::Name(ident) => shared_flag_for_type_var_ident(ident, tree)?,
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
                                                        return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_recursions_for_data_ident: too few argument type values"))]))
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
    
    fn check_type_args_for_impl_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Impl(impl1, pos) => {
                    let impl_r = impl1.borrow();
                    let (trait_ident, type_name) = match &*impl_r {
                        Impl::Builtin(tmp_trait_ident, tmp_type_name, _) => (tmp_trait_ident.clone(), tmp_type_name.clone()),
                        Impl::Impl(tmp_trait_ident, tmp_type_name, _, _) => (tmp_trait_ident.clone(), tmp_type_name.clone()),
                    };
                    let trait_type_arg_count = type_arg_count_for_trait_ident(&trait_ident, tree)?;
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
                }
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
                                match parse_where_with_path(format!("({} where).vscfl", ident).as_str(), builtin_var.type_source.as_str()) {
                                    Ok(where_tuples) => {
                                        match check_idents_for_type_with_where(&type_expr, &where_tuples, tree) {
                                            Ok(()) => {
                                                let mut type_param_env: Environment<LocalType> = Environment::new();
                                                let mut local_type_counter = Some(0usize);
                                                type_param_env.push_new_vars();
                                                match self.evaluate_type_for_type_expr(&type_expr, tree, &mut type_param_env, &mut local_type_counter, errs)? {
                                                    Some(type_value) => {
                                                        let mut new_type = new_type_from_type_value_and_type_param_env(type_value, &type_param_env, &local_type_counter)?;
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
                        let mut new_type = new_type_from_type_value_and_type_param_env(type_value, &type_param_env, &local_type_counter)?;
                        if self.evaluate_types_for_where_tuples(ident.as_str(), where_tuples.as_slice(), trait_ident, pos, tree, &mut type_param_env, &mut new_type, errs)? {
                            if self.shared_flag_for_type(&new_type, tree)? == SharedFlag::Shared {
                                match expr {
                                    Some(expr) => {
                                        let mut new_local_types = LocalTypes::new();
                                        let new_local_type = new_local_types.set_defined_type(&new_type);
                                        let mut var_env: Environment<LocalType> = Environment::new();
                                        self.evalute_types_for_expr(&mut **expr, tree, &mut var_env, &mut type_param_env, &mut new_local_types, errs)?;
                                        let mut var_env2: Environment<(LocalType, usize, Pos)> = Environment::new();
                                        self.set_shareds_for_expr(&**expr, tree, &mut var_env2, &new_local_types, errs)?;
                                        *local_type = Some(new_local_type);
                                        *local_types = Some(Box::new(new_local_types));
                                    },
                                    None => *local_types = None,
                                }
                                *typ = Some(Box::new(new_type));
                            } else {
                                errs.push(FrontendError::Message(type_expr_pos(&type_expr).clone(), format!("variable {} mustn't non-shared with type {}", ident, new_type)))
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
                            let mut new_type = new_type_from_type_value_and_type_param_env(fun_type_value, &type_param_env, &local_type_counter)?;
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
                                                    self.evalute_types_for_expr(&mut **body, tree, &mut var_env, &mut type_param_env, &mut new_local_types, errs)?;
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
                                                    return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_var: too few argument type values"))]))
                                                }
                                            },
                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_types_for_var: no local types"))])),
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
                    TypeValueName::Name(ident) => shared_flag_for_type_var_ident2(ident, tree)?,
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
                            self.add_local_types_for_type_value(&**type_value, pos, typ, &mut local_types, processed_local_types, errs)?;
                        }
                        Ok(local_types)
                    },
                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_param_recursions_for_local_type: no position"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_type_param_recursions_for_local_type: no type parameter entry"))])),
        }
    }
    
    fn add_local_types_for_type_value(&self, type_value: &TypeValue, pos: &Pos, typ: &Type, local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_value {
            TypeValue::Param(_, local_type) => add_local_type(*local_type, pos.clone(), typ, local_types, processed_local_types, errs)?,
            TypeValue::Type(_, _, type_values) => {
                for type_value2 in type_values {
                    self.add_local_types_for_type_value(&**type_value2, pos, typ, local_types, processed_local_types, errs)?;
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
                                (Some(type_param_ident), Some(type_param_pos)) =>{
                                    errs.push(FrontendError::Message(type_param_pos.clone(), format!("type parameter {} mustn't be shared", type_param_ident)));
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
    
    fn check_type_value_for_as(&self, type_value: &TypeValue, tree: &Tree) -> FrontendResultWithErrors<bool>
    {
        match type_value {
            TypeValue::Type(_, TypeValueName::Tuple | TypeValueName::Array(_), type_values) => {
                let mut is_for_as = true;
                for type_value2 in type_values {
                    if !self.check_type_value_for_as(type_value2, tree)? {
                        is_for_as = false;
                    }
                }
                Ok(is_for_as)
            },
            TypeValue::Type(_, TypeValueName::Name(ident), type_values) => {
                let mut is_for_as = self.has_primitive_for_type_ident(ident, tree)?;
                if is_for_as {
                    for type_value2 in type_values {
                        if !self.check_type_value_for_as(type_value2, tree)? {
                            is_for_as = false;
                        }
                    }
                }
                Ok(is_for_as)
            },
            _ => Ok(false),
        }
    }
    
    fn evalute_types_for_expr(&self, expr: &mut Expr, tree: &Tree, var_env: &mut Environment<LocalType>, type_param_env: &mut Environment<LocalType>, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, local_type, _) => {
                self.evalute_types_for_literal(&mut **literal, tree, var_env, type_param_env, local_types, errs, Self::evalute_types_for_expr)?;
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
                self.evalute_types_for_expr(&mut **body, tree, var_env, type_param_env, local_types, errs)?;
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
                            self.evalute_types_for_expr(&mut **field_expr, tree, var_env, type_param_env, local_types, errs)?;
                        }
                    }
                }
                *con_local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::PrintfApp(exprs, local_type, _) => {
                for expr2 in exprs {
                    self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::App(expr2, exprs, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                for expr3 in exprs {
                    self.evalute_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::GetField(expr2, _, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Get2Field(expr2, _, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::SetField(expr2, _, expr3, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evalute_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::UpdateField(expr2, _, expr3, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evalute_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::UpdateGet2Field(expr2, _, expr3, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evalute_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Uniq(expr2, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Shared(expr2, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Typed(expr2, type_expr, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                match self.evaluate_type_for_type_expr(&**type_expr, tree, type_param_env, &mut None, errs)? {
                    Some(type_value) => *local_type = Some(local_types.add_type_value(type_value)),
                    None => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
            },
            Expr::As(expr2, type_expr, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                match self.evaluate_type_for_type_expr(&**type_expr, tree, type_param_env, &mut None, errs)? {
                    Some(type_value) => {
                        if self.check_type_value_for_as(&*type_value, tree)? {
                            *local_type = Some(local_types.add_type_value(type_value.clone()));
                        } else {
                            *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                            errs.push(FrontendError::Message(type_expr_pos(type_expr).clone(), format!("can't cast to type {}", TypeValueWithLocalTypes(type_value, local_types))));
                        }
                    },
                    None => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
            },
            Expr::If(expr2, expr3, expr4, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                self.evalute_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                self.evalute_types_for_expr(&mut **expr4, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Expr::Let(binds, expr2, local_type, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.evalute_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                            self.evalute_types_for_pattern(&mut **pattern, tree, var_env, type_param_env, local_types, errs)?;
                        },
                    }
                }
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                var_env.pop_vars();
            },
            Expr::Match(expr2, cases, local_type, _) => {
                self.evalute_types_for_expr(&mut **expr2, tree, var_env, type_param_env, local_types, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_new_vars();
                            self.evalute_types_for_pattern(&mut **pattern, tree, var_env, type_param_env, local_types, errs)?;
                            self.evalute_types_for_expr(&mut **expr3, tree, var_env, type_param_env, local_types, errs)?;
                            var_env.pop_vars();
                        },
                    }
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            }
        }
        Ok(())
    }

    fn evalute_types_for_pattern(&self, pattern: &mut Pattern, tree: &Tree, var_env: &mut Environment<LocalType>, type_param_env: &mut Environment<LocalType>, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, local_type, _) => {
                self.evalute_types_for_literal(&mut **literal, tree, var_env, type_param_env, local_types, errs, Self::evalute_types_for_pattern)?;
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Pattern::As(literal, type_expr, local_type, _) => {
                self.evalute_types_for_literal(&mut **literal, tree, var_env, type_param_env, local_types, errs, Self::evalute_types_for_pattern)?;
                match self.evaluate_type_for_type_expr(&**type_expr, tree, type_param_env, &mut None, errs)? {
                    Some(type_value) => {
                        if self.check_type_value_for_as(&*type_value, tree)? {
                            *local_type = Some(local_types.add_type_value(type_value.clone()));
                        } else {
                            *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                            errs.push(FrontendError::Message(type_expr_pos(type_expr).clone(), format!("can't cast to type {}", TypeValueWithLocalTypes(type_value, local_types))));
                        }
                    },
                    None => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
                }
            },
            Pattern::Const(_, local_type, _) => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
            Pattern::UnnamedFieldCon(_, patterns, con_local_type, local_type, _) => {
                for pattern2 in patterns {
                    self.evalute_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?;
                }
                *con_local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
            Pattern::NamedFieldCon(_, pattern_named_field_pairs, con_local_type, local_type, _) => {
                for pattern_named_field_pair in pattern_named_field_pairs {
                    match pattern_named_field_pair {
                        NamedFieldPair(_, pattern2, _) => self.evalute_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?,
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
                self.evalute_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?;
            },
            Pattern::Wildcard(local_type, _) => *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new())))),
            Pattern::Alt(patterns, local_type, _) => {
                for pattern2 in patterns {
                    self.evalute_types_for_pattern(&mut **pattern2, tree, var_env, type_param_env, local_types, errs)?;
                }
                *local_type = Some(local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
            },
        }
        Ok(())
    }


    fn evalute_types_for_literal<T, F>(&self, literal: &mut Literal<T>, tree: &Tree, var_env: &mut Environment<LocalType>, type_param_env: &mut Environment<LocalType>, local_types: &mut LocalTypes, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
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
            self.set_shared(ident.as_str(), tuple.0, tuple.2.clone(), tree, local_types, errs)?;
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
            Pattern::As(literal, _, _, _) =>  self.set_shareds_for_literal(literal, tree, var_env, local_types, errs, Self::set_shareds_for_pattern)?,
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
}
