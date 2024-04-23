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
use crate::frontend::tree::*;
use crate::frontend::type_matcher::*;
use crate::utils::dfs::*;
use crate::utils::env::*;

fn add_type_synonym_ident_for_type_var(ident: &String, pos: Pos, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => Ok(()),
                TypeVar::Synonym(_, _, None) => {
                    if !processed_idents.contains(ident) {
                        idents.push(ident.clone());
                        Ok(())
                    } else {
                        errs.push(FrontendError::Message(pos, format!("recursive definition of type synonym {}", ident)));
                        Ok(())
                    }
                },
                _ => Ok(()),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no type variable"))])),
    }
}

fn local_type_for_type_param(ident: &String, type_param_env: &Environment<LocalType>) -> FrontendResultWithErrors<LocalType>
{
    match type_param_env.var(ident) {
        Some(tmp_local_type) => Ok(*tmp_local_type),
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no type parameter"))])),
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
                TypeVar::Builtin(None, _, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no type arguments"))])),
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
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no type variable"))])),
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

    fn evaluate_types_for_type_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let mut visited_idents: BTreeSet<String> = BTreeSet::new();
        for def in tree.defs() {
            match &**def {
                Def::Type(ident, type_var, _) => {
                    let type_var_r = type_var.borrow();
                    match &*type_var_r {
                        TypeVar::Synonym(_, _, _) => {
                            dfs_with_result(ident, &mut visited_idents, errs, |ident, processed_idents, errs| {
                                    self.type_synonym_idents_for_type_synonym_ident(ident, tree, processed_idents, errs)
                            }, |ident, errs| {
                                    self.evaluate_type_for_type_synonym_ident(ident, tree, errs)
                            })?;
                        },
                        _ => (),
                    }
                },
                _ => (),
            }
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
                        self.type_synonym_idents_for_type_expr(&**type_expr, tree, &mut idents, processed_idents, errs)?;
                        Ok(idents)
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type variable isn't type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no type variable"))])),
        }
    }
        
    fn type_synonym_idents_for_type_expr(&self, type_expr: &TypeExpr, tree: &Tree, idents: &mut Vec<String>, processed_idents: &BTreeSet<String>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_expr {
            TypeExpr::Tuple(field_type_exprs, _) => {
                for field_type_expr in field_type_exprs {
                    self.type_synonym_idents_for_type_expr(&**field_type_expr, tree, idents, processed_idents, errs)?;
                }
                
            },
            TypeExpr::Fun(arg_type_exprs, ret_type_expr, _) => {
                for arg_type_expr in arg_type_exprs {
                    self.type_synonym_idents_for_type_expr(&**arg_type_expr, tree, idents, processed_idents, errs)?;
                }
                self.type_synonym_idents_for_type_expr(&**ret_type_expr, tree, idents, processed_idents, errs)?
            },
            TypeExpr::Array(elem_type_expr, _, _) => self.type_synonym_idents_for_type_expr(&**elem_type_expr, tree, idents, processed_idents, errs)?,
            TypeExpr::Param(_, _) => (),
            TypeExpr::Var(ident, pos) => add_type_synonym_ident_for_type_var(ident, pos.clone(), tree, idents, processed_idents, errs)?,
            TypeExpr::App(ident, _, pos) => add_type_synonym_ident_for_type_var(ident, pos.clone(), tree, idents, processed_idents, errs)?,
            TypeExpr::Uniq(type_expr2, _) => self.type_synonym_idents_for_type_expr(&**type_expr2, tree, idents, processed_idents, errs)?,
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
                            type_param_env.add_var(ident.clone(), LocalType::new(i));
                        }
                        *type_value = self.evaluate_type_for_type_expr(&**type_expr, tree, &mut type_param_env, &mut None, errs)?;
                        Ok(())
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type variable isn't type synonym"))])),
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no type variable"))])),
        }
    }

    fn evaluate_type_for_type_expr(&self, type_expr: &TypeExpr, tree: &Tree, type_param_env: &mut Environment<LocalType>, local_type_counter: &mut Option<usize>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<Rc<TypeValue>>>
    {
        match type_expr {
            TypeExpr::Tuple(field_type_exprs, _) => {
                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                let mut is_err = false;
                for field_type_expr in field_type_exprs {
                    match self.evaluate_type_for_type_expr(&**field_type_expr, tree, type_param_env, local_type_counter, errs)? {
                        Some(type_value) => type_values.push(type_value),
                        None => is_err = true,
                    }
                }
                if !is_err {
                    Ok(Some(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, type_values))))
                } else {
                    Ok(None)
                }
            },
            TypeExpr::Fun(arg_type_exprs, ret_type_expr, _) => {
                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                let mut is_err = false;
                for arg_type_expr in arg_type_exprs {
                    match self.evaluate_type_for_type_expr(&**arg_type_expr, tree, type_param_env, local_type_counter, errs)? {
                        Some(type_value) => type_values.push(type_value),
                        None => is_err = true,
                    }
                }
                match self.evaluate_type_for_type_expr(&**ret_type_expr, tree, type_param_env, local_type_counter, errs)? {
                    Some(type_value) => type_values.push(type_value),
                    None => is_err = true,
                }
                if !is_err {
                    Ok(Some(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values))))
                } else {
                    Ok(None)
                }
            },
            TypeExpr::Array(elem_type_expr, len, _) => {
                let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
                let mut is_err = false;
                match self.evaluate_type_for_type_expr(&**elem_type_expr, tree, type_param_env, local_type_counter, errs)? {
                    Some(type_value) => type_values.push(type_value),
                    None => is_err = true,
                }
                if !is_err {
                    Ok(Some(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Array(*len), type_values))))
                } else {
                    Ok(None)
                }
            },
            TypeExpr::Param(ident, _) => {
                let local_type = match local_type_counter {
                    Some(local_type_counter) => {
                        match type_param_env.var(ident) {
                            Some(tmp_local_type) => *tmp_local_type,
                            None => {
                                let tmp_local_type = LocalType::new(*local_type_counter);
                                type_param_env.add_var(ident.clone(), tmp_local_type);
                                *local_type_counter += 1;
                                tmp_local_type
                            },
                        }
                    },
                    None => local_type_for_type_param(ident, type_param_env)?,
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
                        let mut is_err = false;
                        for type_expr2 in type_exprs {
                            match self.evaluate_type_for_type_expr(&**type_expr2, tree, type_param_env, local_type_counter, errs)? {
                                Some(type_value) => type_values.push(type_value),
                                None => is_err = true,
                            }
                        }
                        if !is_err {
                            match type_value.substitute(type_values.as_slice()) {
                                Ok(Some(new_type_value)) => Ok(Some(new_type_value)),
                                Ok(None) => Ok(Some(type_value)),
                                Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("{}", err))]))
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
}
