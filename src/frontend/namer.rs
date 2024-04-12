//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;

fn add_error_for_type_var(ident: &str, defined_type_var: &TypeVar, pos: Pos, errs: &mut Vec<FrontendError>)
{
    match defined_type_var {
        TypeVar::Builtin(_, _) => errs.push(FrontendError::Message(pos, format!("already defined built-in type {}", ident))),
        TypeVar::Data(_, _, _) => errs.push(FrontendError::Message(pos, format!("already defined type {}", ident))),
        TypeVar::Synonym(_, _, _) => errs.push(FrontendError::Message(pos, format!("already defined type synonym {}", ident))),
    }
}

fn add_error_for_var(ident: &str, defined_var: &Var, pos: Pos, errs: &mut Vec<FrontendError>)
{
    match defined_var {
        Var::Builtin(_, _) => errs.push(FrontendError::Message(pos, format!("already defined built-in variable {}", ident))),
        Var::Var(_, _, _, _, _, _, _, _) => errs.push(FrontendError::Message(pos, format!("already defined variable {}", ident))),
        Var::Fun(fun, _, _) => {
            match &**fun {
                Fun::Fun(_, _, _, _, _, _, _) => errs.push(FrontendError::Message(pos, format!("already defined function {}", ident))),
                Fun::Con(_) => errs.push(FrontendError::Message(pos, format!("already defined constructor {}", ident))),
            }
        },
    }
}

fn add_error_for_trait(ident: &str, pos: Pos, errs: &mut Vec<FrontendError>)
{ errs.push(FrontendError::Message(pos, format!("already defined trait {}", ident))); }

pub struct Namer
{}

impl Namer
{
    pub fn new() -> Self
    { Namer {} }

    pub fn check_idents(&self, tree: &mut Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.add_defs(tree, &mut errs);
        self.add_impls_for_defs(tree, &mut errs);
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }
    
    fn add_defs(&self, tree: &mut Tree, errs: &mut Vec<FrontendError>)
    {
        for def in &tree.defs {
            match &**def {
                Def::Type(ident, type_var, pos) => {
                    match tree.type_vars.get(ident) {
                        Some(defined_type_var) => {
                            let defined_type_var_r = defined_type_var.borrow();
                            add_error_for_type_var(ident.as_str(), &*defined_type_var_r, pos.clone(), errs);
                        },
                        None => {
                            tree.type_vars.insert(ident.clone(), type_var.clone());
                            let type_var_r = type_var.borrow();
                            match &*type_var_r {
                                TypeVar::Data(_, cons, _) => {
                                    for con in cons {
                                        let con_r = con.borrow();
                                        let (con_ident, con_pos) = match &*con_r {
                                            Con::UnnamedField(tmp_ident, _, _, tmp_pos) => (tmp_ident, tmp_pos),
                                            Con::NamedField(tmp_ident, _, _, tmp_pos) => (tmp_ident, tmp_pos),
                                        };
                                        match tree.vars.get(con_ident) {
                                            Some(defined_var) => {
                                                let defined_var_r = defined_var.borrow();
                                                add_error_for_var(con_ident.as_str(), &*defined_var_r, con_pos.clone(), errs);
                                            },
                                            None => {
                                                tree.vars.insert(ident.clone(), Rc::new(RefCell::new(Var::Fun(Box::new(Fun::Con(con.clone())), None, None))));
                                            },
                                        }
                                    }
                                },
                                _ => (),
                            }
                        },
                    }
                },
                Def::Var(ident, var, pos) => {
                    match tree.vars.get(ident) {
                        Some(defined_var) => {
                            let defined_var_r = defined_var.borrow();
                            add_error_for_var(ident.as_str(), &*defined_var_r, pos.clone(), errs);
                        },
                        None => {
                            tree.vars.insert(ident.clone(), var.clone());
                        },
                    }
                },
                Def::Trait(ident, trait1, pos) => {
                    match tree.traits.get(ident) {
                        Some(_) => add_error_for_trait(ident.as_str(), pos.clone(), errs),
                        None => {
                            tree.traits.insert(ident.clone(), trait1.clone());
                            let mut trait_r = trait1.borrow_mut();
                            match &mut *trait_r {
                                Trait(_, trait_defs, trait_vars) => {
                                    for trait_def in trait_defs {
                                        match &**trait_def {
                                            TraitDef(var_ident, var, var_pos) => {
                                                match tree.vars.get(ident) {
                                                    Some(defined_var) => {
                                                        let defined_var_r = defined_var.borrow();
                                                        add_error_for_var(var_ident.as_str(), &*defined_var_r, var_pos.clone(), errs);
                                                    },
                                                    None => {
                                                        tree.vars.insert(var_ident.clone(), var.clone());
                                                        if trait_vars.is_none() {
                                                            *trait_vars = Some(Box::new(TraitVars::new()));
                                                        }
                                                        match trait_vars {
                                                            Some(trait_vars) => {
                                                                trait_vars.vars.insert(var_ident.clone(), var.clone());
                                                            },
                                                            None => errs.push(FrontendError::Internal(String::from("no trait variables"))),
                                                        }
                                                    },
                                                }
                                            },
                                        }
                                    }
                                },
                            }
                        },
                    }
                }
                _ => ()
            }
        }
    }

    fn add_impls_for_defs(&self, tree: &mut Tree, errs: &mut Vec<FrontendError>)
    {
        for def in &tree.defs {
            match &**def {
                Def::Impl(impl1, pos) => {
                    let mut impl_r = impl1.borrow_mut();
                    let (trait_ident, type_name) = match &*impl_r {
                        Impl::Builtin(tmp_trait_ident, tmp_type_name, _) => (tmp_trait_ident, tmp_type_name),
                        Impl::Impl(tmp_trait_ident, tmp_type_name, _, _) => (tmp_trait_ident, tmp_type_name),
                    };
                    match tree.traits.get(trait_ident) {
                        Some(trait1) => {
                            let mut trait_r = trait1.borrow_mut();
                            match &mut *trait_r {
                                Trait(_, _, Some(trait_vars)) => {
                                    match trait_vars.impls.get(type_name) {
                                        Some(_) => errs.push(FrontendError::Message(pos.clone(), format!("already defined implementation {} for type {}", trait_ident, type_name))),
                                        None => {
                                            trait_vars.impls.insert(type_name.clone(), impl1.clone());
                                        },
                                    }
                                    match &mut *impl_r {
                                        Impl::Builtin(_, _, impl_vars) => {
                                            if impl_vars.is_none() {
                                                *impl_vars = Some(Box::new(ImplVars::new()));
                                            }
                                            for trait_var_ident in trait_vars.vars.keys() {
                                                match impl_vars {
                                                    Some(impl_vars) => {
                                                        impl_vars.vars.insert(trait_var_ident.clone(), Rc::new(RefCell::new(ImplVar::Builtin(None))));
                                                    },
                                                    None => errs.push(FrontendError::Internal(String::from("no implementation variables"))),
                                                }
                                            }
                                        },
                                        Impl::Impl(trait_ident, _, impl_defs, impl_vars) => {
                                            if impl_vars.is_none() {
                                                *impl_vars = Some(Box::new(ImplVars::new()));
                                            }
                                            for impl_def in impl_defs {
                                                match &**impl_def {
                                                    ImplDef(impl_var_ident, impl_var, impl_var_pos) => {
                                                        match trait_vars.vars.get(impl_var_ident) {
                                                            Some(trait_var) => {
                                                                let trait_var_r = trait_var.borrow();
                                                                let impl_var_r = impl_var.borrow();
                                                                let mut is_impl_var = false;
                                                                match (&*trait_var_r, &*impl_var_r) {
                                                                    (Var::Builtin(_, _), _) => is_impl_var = true,
                                                                    (_, ImplVar::Builtin(_)) => is_impl_var = true,
                                                                    (Var::Var(_, _, _, _, _, _, _, _), ImplVar::Var(_, _, _, _)) => is_impl_var = true,
                                                                    (Var::Fun(_, _, _), ImplVar::Fun(_, _)) => is_impl_var = true,
                                                                    (Var::Var(_, _, _, _, _, _, _, _), ImplVar::Fun(_, _)) => errs.push(FrontendError::Message(impl_var_pos.clone(), format!("function {} must be variable at implementation {}", impl_var_ident, trait_ident))),
                                                                    (Var::Fun(_, _, _), ImplVar::Var(_, _, _, _)) =>  errs.push(FrontendError::Message(impl_var_pos.clone(), format!("variable {} must be function at implementation {}", impl_var_ident, trait_ident))),
                                                                }
                                                                if is_impl_var {
                                                                    match impl_vars {
                                                                        Some(impl_vars) => {
                                                                            impl_vars.vars.insert(impl_var_ident.clone(), impl_var.clone());
                                                                        },
                                                                        None => errs.push(FrontendError::Internal(String::from("no implementation variables"))),
                                                                    }
                                                                }
                                                            },
                                                            None => errs.push(FrontendError::Message(pos.clone(), format!("undefined variable {} at trait {}", impl_var_ident, trait_ident))),
                                                        }
                                                    },
                                                }
                                            }
                                            for (trait_var_ident, trait_var) in &trait_vars.vars {
                                                let trait_var_r = trait_var.borrow();
                                                match &*trait_var_r {
                                                    Var::Var(_, _, _, None, _, _, _, _) => {
                                                        match impl_vars {
                                                            Some(impl_vars) => {
                                                                if !impl_vars.vars.contains_key(trait_var_ident) {
                                                                    errs.push(FrontendError::Message(pos.clone(), format!("undefined required variable {} at implementation {}", trait_var_ident, trait_ident)));
                                                                }
                                                            },
                                                            None => errs.push(FrontendError::Internal(String::from("no implementation variables"))),
                                                        }
                                                    },
                                                    Var::Fun(fun, _, _) => {
                                                        match &**fun {
                                                            Fun::Fun(_, _, _, _, None, _, _) => {
                                                                match impl_vars {
                                                                    Some(impl_vars) => {
                                                                        if !impl_vars.vars.contains_key(trait_var_ident) {
                                                                            errs.push(FrontendError::Message(pos.clone(), format!("undefined required function {} at implementation {}", trait_var_ident, trait_ident)));
                                                                        }
                                                                    },
                                                                    None => errs.push(FrontendError::Internal(String::from("no implementation variables"))),
                                                                }
                                                            },
                                                            _ => (),
                                                        }
                                                    }
                                                    _ => (),
                                                }
                                            }
                                        },
                                    }
                                },
                                _ => errs.push(FrontendError::Internal(String::from("no trait variables"))),
                            }
                        },
                        None => errs.push(FrontendError::Message(pos.clone(), format!("undefined trait {}", trait_ident))),
                    }
                },
                _ => (),
            }
        }
    }
}
