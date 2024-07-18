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
use crate::frontend::error::*;
use crate::frontend::evals::*;
use crate::frontend::tree::*;
use crate::frontend::type_stack::*;

fn type_name_for_var_ident_and_local_type(ident: &String, local_type: LocalType, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes) -> FrontendResultWithErrors<Option<TypeName>>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            let (trait_ident, typ) = match &*var_r {
                Var::Builtin(tmp_trait_ident, Some(tmp_type)) => (tmp_trait_ident, tmp_type), 
                Var::Var(_, _, _, _, tmp_trait_ident, _, _, Some(tmp_type), _) => (tmp_trait_ident, tmp_type),
                Var::Fun(_, tmp_trait_ident, Some(tmp_type)) => (tmp_trait_ident, tmp_type),
                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_name_for_var_ident_and_local_type: no type"))])),
            };
            match trait_ident {
                Some(trait_ident) => {
                    match type_stack.push_type_entries_for_local_type(local_type, local_types) {
                        Ok(new_local_type) => {
                            match type_stack.type_name_for_local_type_and_type(new_local_type, typ, trait_ident.as_str()) {
                                Ok(type_name) => {
                                    type_stack.pop_type_entries();
                                    Ok(type_name)
                                },
                                Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("{}", err))])),
                            }
                        },
                        Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("{}", err))])),
                    }
                },
                None => Ok(None),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_name_for_var_ident_and_local_type: no variable"))])),
    }
}

fn add_var_key(ident: &String, type_name: &Option<TypeName>, pos: Pos, tree: &Tree, keys: &mut Vec<(String, Option<TypeName>)>, processed_keys: &BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, trait_ident, _, _, _, _) => {
                    let key_type_name = match type_name {
                        Some(type_name) => {
                            match trait_ident {
                                Some(trait_ident) => {
                                    match tree.trait1(trait_ident) {
                                        Some(trait1) => {
                                            let trait_r = trait1.borrow();
                                            match &*trait_r {
                                                Trait(_, _, Some(trait_vars)) => {
                                                    match trait_vars.impl1(&type_name) {
                                                        Some(impl1) => {
                                                            let impl_r = impl1.borrow();
                                                            let impl_vars = match &*impl_r {
                                                                Impl::Builtin(_, _, Some(tmp_impl_vars)) => tmp_impl_vars,
                                                                Impl::Impl(_, _, _, Some(tmp_impl_vars)) => tmp_impl_vars,
                                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_key: no implementation variables"))])),
                                                            };
                                                            if impl_vars.var(ident).is_some() {
                                                                Some(type_name.clone())
                                                            } else {
                                                                None
                                                            }
                                                        },
                                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_key: no implementation"))])),
                                                    }
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_key: no trait variables"))])),
                                            }
                                        },
                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_key: no trait"))])),
                                    }
                                },
                                None => None,
                            }
                        },
                        None => None,
                    };
                    let key = (ident.clone(), key_type_name);
                    if processed_keys.contains(&key) {
                        keys.push(key);
                    } else {
                        errs.push(FrontendError::Message(pos, format!("definition of variable {} is recursive", ident)));
                    }
                    Ok(())
                },
                _ => Ok(()),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_key: no variable"))])),
    }
}

pub struct Evaluator
{
    evals: Evals,
}

impl Evaluator
{
    pub fn new() -> Self
    { Evaluator { evals: Evals::new(), } }
}
