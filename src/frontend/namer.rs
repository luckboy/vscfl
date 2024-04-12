//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeSet;
use std::cell::*;
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;
use crate::utils::env::*;

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

fn check_type_param_ident(ident: &String, type_param_env: &Environment<()>, pos: Pos, are_errs: bool, errs: &mut Vec<FrontendError>)
{
    if type_param_env.var(ident).is_none() {
        if are_errs {
            errs.push(FrontendError::Message(pos, format!("undefined type parameter {}", ident)));
        }
    }
}

fn check_type_var_ident(ident: &String, tree: &Tree, pos: Pos, are_errs: bool, errs: &mut Vec<FrontendError>)
{
    if !tree.type_vars.contains_key(ident) {
        if are_errs {
            errs.push(FrontendError::Message(pos, format!("undefined type variable {}", ident)));
        }
    }
}

fn check_var_ident(ident: &String, tree: &Tree, var_env: &Environment<()>, pos: Pos, errs: &mut Vec<FrontendError>)
{
    if var_env.var(ident).is_none() {
        if !tree.vars.contains_key(ident) {
            errs.push(FrontendError::Message(pos, format!("undefined variable {}", ident)));
        }
    }
}

fn check_const_ident(ident: &String, tree: &Tree, pos: Pos, errs: &mut Vec<FrontendError>)
{
    match tree.vars.get(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Builtin(_, _) => (),
                Var::Var(_, _, _, _, _, _, _, _) => (),
                Var::Fun(_, _, _) => errs.push(FrontendError::Message(pos, format!("variable {} is function", ident))),
            }
        },
        None => errs.push(FrontendError::Message(pos, format!("undefined variable {}", ident))),
    }
}

fn check_con_ident(ident: &String, tree: &Tree, pos: Pos, are_named_fields: bool, errs: &mut Vec<FrontendError>) -> Option<Rc<RefCell<Con>>>
{
    match tree.vars.get(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, _) => {
                    match &**fun {
                        Fun::Con(con) => {
                            if are_named_fields {
                                let con_r = con.borrow();
                                match &*con_r {
                                    Con::UnnamedField(_, _, _, _) => {
                                        errs.push(FrontendError::Message(pos, format!("constructor {} hasn't named fields", ident)));
                                        None
                                    },
                                    Con::NamedField(_, _, _, _, _) => Some(con.clone()),
                                }
                            } else {
                                Some(con.clone())
                            }
                        },
                        _ => {
                            errs.push(FrontendError::Message(pos, format!("variable {} isn't constructor", ident)));
                            None
                        },
                    }
                },
                _ => {
                    errs.push(FrontendError::Message(pos, format!("variable {} isn't constructor", ident)));
                    None
                },
            }
        },
        None => {
            errs.push(FrontendError::Message(pos, format!("undefined constructor {}", ident)));
            None
        },
    }
}

pub struct Namer
{}

impl Namer
{
    pub fn new() -> Self
    { Namer {} }

    pub fn check_idents(&self, tree: &mut Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.add_defs(tree, &mut errs)?;
        self.add_impls_for_defs(tree, &mut errs)?;
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }
    
    fn add_defs(&self, tree: &mut Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
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
                                        let mut con_r = con.borrow_mut();
                                        let (con_ident, con_pos) = match &*con_r {
                                            Con::UnnamedField(tmp_ident, _, _, tmp_pos) => (tmp_ident, tmp_pos),
                                            Con::NamedField(tmp_ident, _, _, _, tmp_pos) => (tmp_ident, tmp_pos),
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
                                        match &mut *con_r {
                                            Con::UnnamedField(_, _, _, _) => (),
                                            Con::NamedField(_, type_expr_named_field_pairs, _, named_fields, _) => {
                                                *named_fields = Some(Box::new(NamedFields::new()));
                                                let mut field_idents: BTreeSet<String> = BTreeSet::new();
                                                let mut field_idx = 0usize;
                                                for type_expr_named_field_pair in type_expr_named_field_pairs {
                                                    match type_expr_named_field_pair {
                                                        NamedFieldPair(field_ident, _, field_pos) => {
                                                            if !field_idents.contains(field_ident) {
                                                                match named_fields {
                                                                    Some(named_fields) => {
                                                                        named_fields.field_indices.insert(field_ident.clone(), field_idx);
                                                                    },
                                                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no named fields"))])),
                                                                }
                                                                field_idents.insert(field_ident.clone());
                                                                field_idx += 1;
                                                            } else {
                                                                errs.push(FrontendError::Message(field_pos.clone(), format!("undefined field {}", field_ident)));
                                                            }
                                                        },
                                                    }
                                                }
                                            },
                                        };
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
                                                        *trait_vars = Some(Box::new(TraitVars::new()));
                                                        match trait_vars {
                                                            Some(trait_vars) => {
                                                                trait_vars.vars.insert(var_ident.clone(), var.clone());
                                                            },
                                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no trait variables"))])),
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
        Ok(())
    }

    fn add_impls_for_defs(&self, tree: &mut Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
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
                                            *impl_vars = Some(Box::new(ImplVars::new()));
                                            for trait_var_ident in trait_vars.vars.keys() {
                                                match impl_vars {
                                                    Some(impl_vars) => {
                                                        impl_vars.vars.insert(trait_var_ident.clone(), Rc::new(RefCell::new(ImplVar::Builtin(None))));
                                                    },
                                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no implementation variables"))])),
                                                }
                                            }
                                        },
                                        Impl::Impl(trait_ident, _, impl_defs, impl_vars) => {
                                            *impl_vars = Some(Box::new(ImplVars::new()));
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
                                                                    (Var::Var(_, _, _, _, _, _, _, _), ImplVar::Fun(_, _)) => errs.push(FrontendError::Message(impl_var_pos.clone(), format!("function {} must be variable in implementation {}", impl_var_ident, trait_ident))),
                                                                    (Var::Fun(_, _, _), ImplVar::Var(_, _, _, _)) =>  errs.push(FrontendError::Message(impl_var_pos.clone(), format!("variable {} must be function in implementation {}", impl_var_ident, trait_ident))),
                                                                }
                                                                if is_impl_var {
                                                                    match impl_vars {
                                                                        Some(impl_vars) => {
                                                                            impl_vars.vars.insert(impl_var_ident.clone(), impl_var.clone());
                                                                        },
                                                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no implementation variables"))])),
                                                                    }
                                                                }
                                                            },
                                                            None => errs.push(FrontendError::Message(pos.clone(), format!("undefined variable {} in trait {}", impl_var_ident, trait_ident))),
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
                                                                    errs.push(FrontendError::Message(pos.clone(), format!("undefined required variable {} in implementation {}", trait_var_ident, trait_ident)));
                                                                }
                                                            },
                                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no implementation variables"))])),
                                                        }
                                                    },
                                                    Var::Fun(fun, _, _) => {
                                                        match &**fun {
                                                            Fun::Fun(_, _, _, _, None, _, _) => {
                                                                match impl_vars {
                                                                    Some(impl_vars) => {
                                                                        if !impl_vars.vars.contains_key(trait_var_ident) {
                                                                            errs.push(FrontendError::Message(pos.clone(), format!("undefined required function {} in implementation {}", trait_var_ident, trait_ident)));
                                                                        }
                                                                    },
                                                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no implementation variables"))])),
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
                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("no trait variables"))])),
                            }
                        },
                        None => errs.push(FrontendError::Message(pos.clone(), format!("undefined trait {}", trait_ident))),
                    }
                },
                _ => (),
            }
        }
        Ok(())
    }
    
    fn check_idents_for_type_args(&self, type_args: &[TypeArg], type_param_env: &mut Environment<()>, errs: &mut Vec<FrontendError>)
    {
        let mut type_arg_idents: BTreeSet<String> = BTreeSet::new();
        for type_arg in type_args {
            match type_arg {
                TypeArg(ident, pos) => {
                    if !type_arg_idents.contains(ident) {
                        type_param_env.add_var(ident.clone(), ());
                        type_arg_idents.insert(ident.clone());
                    } else {
                        errs.push(FrontendError::Message(pos.clone(), format!("already defined type argument {}", ident)));
                    }
                },
            }
        }
    }
    
    fn check_idents_for_named_field_pairs<T, F>(&self, named_field_pairs: &[NamedFieldPair<T>], con: Rc<RefCell<Con>>, tree: &Tree, type_param_env: &mut Environment<()>, pos: Pos, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, &Tree, &mut Environment<()>, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let con_r = con.borrow();
        match &*con_r {
            Con::NamedField(_, _, _, Some(named_fields), _) => {
                let mut field_idents: BTreeSet<String> = BTreeSet::new();
                let mut count = 0usize;
                for named_field_pair in named_field_pairs {
                    match named_field_pair {
                        NamedFieldPair(field_ident, other, field_pos) => {
                            if !field_idents.contains(field_ident) {
                                if named_fields.field_indices.contains_key(field_ident) {
                                    f(self, &**other, tree, type_param_env, errs)?;
                                    field_idents.insert(field_ident.clone());
                                    count += 1;
                                } else {
                                    errs.push(FrontendError::Message(field_pos.clone(), format!("undefined field {}", field_ident)))
                                }
                            } else {
                                errs.push(FrontendError::Message(field_pos.clone(), format!("already used field {}", field_ident)))
                            }
                        },
                    }
                }
                if count < named_fields.field_indices.len() {
                    errs.push(FrontendError::Message(pos.clone(), String::from("too few used fields")))
                } else if count > named_fields.field_indices.len() {
                    errs.push(FrontendError::Message(pos.clone(), String::from("too many used fields")))
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("unnamed field contructor or no named fields"))])),
        }
        Ok(())
    }
    
    fn check_idents_for_type_expr(&self, type_expr: &TypeExpr, tree: &Tree, type_param_env: &mut Environment<()>, can_add_type_params: bool, are_errs: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match type_expr {
            TypeExpr::Tuple(field_type_exprs, _) => {
                for field_type_expr in field_type_exprs {
                    self.check_idents_for_type_expr(&**field_type_expr, tree, type_param_env, can_add_type_params, are_errs, errs)?;
                }
            },
            TypeExpr::Fun(arg_type_exprs, ret_type_expr, _) => {
                for arg_type_expr in arg_type_exprs {
                    self.check_idents_for_type_expr(&**arg_type_expr, tree, type_param_env, can_add_type_params, are_errs, errs)?;
                }
                self.check_idents_for_type_expr(&**ret_type_expr, tree, type_param_env, can_add_type_params, are_errs, errs)?;
            },
            TypeExpr::Array(elem_type_expr, _, _) => self.check_idents_for_type_expr(&**elem_type_expr, tree, type_param_env, can_add_type_params, are_errs, errs)?,
            TypeExpr::Param(ident, pos) => {
                if can_add_type_params {
                    type_param_env.add_var(ident.clone(), ());
                } else {
                    check_type_param_ident(ident, type_param_env, pos.clone(), are_errs, errs);
                }
            },
            TypeExpr::Var(ident, pos) => check_type_var_ident(ident, tree, pos.clone(), are_errs, errs),
            TypeExpr::App(ident, type_exprs, pos) => {
                check_type_var_ident(ident, tree, pos.clone(), are_errs, errs);
                for type_expr2 in type_exprs {
                    self.check_idents_for_type_expr(&**type_expr2, tree, type_param_env, can_add_type_params, are_errs, errs)?;
                }
            },
            TypeExpr::Uniq(type_expr2, _) => self.check_idents_for_type_expr(&**type_expr2, tree, type_param_env, can_add_type_params, are_errs, errs)?,
        }
        Ok(())
    }

    fn check_idents_for_args(&self, args: &[Arg], tree: &Tree, var_env: &mut Environment<()>, type_param_env: &mut Environment<()>, are_errs: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let mut arg_idents: BTreeSet<String> = BTreeSet::new();
        for arg in args {
            match arg {
                Arg(ident, type_expr, _, pos) => {
                    if !arg_idents.contains(ident) {
                        var_env.add_var(ident.clone(), ());
                        arg_idents.insert(ident.clone());
                        self.check_idents_for_type_expr(&**type_expr, tree, type_param_env, true, are_errs, errs)?;
                    } else {
                        if are_errs {
                            errs.push(FrontendError::Message(pos.clone(), format!("already defined argument {}", ident)));
                        }
                    }
                },
            }
        }
        Ok(())
    }
    
    fn check_idents_for_expr(&self, expr: &Expr, tree: &Tree, var_env: &mut Environment<()>, type_param_env: &mut Environment<()>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.check_idents_for_literal(&**literal, tree, var_env, type_param_env, errs, Self::check_idents_for_expr)?,
            Expr::Lambda(args, ret_type_expr, body, _, _) => {
                var_env.push_new_vars();
                self.check_idents_for_lambda_args(args.as_slice(), tree, var_env, type_param_env, errs)?;
                match ret_type_expr {
                    Some(ret_type_expr) => self.check_idents_for_type_expr(&**ret_type_expr, tree, type_param_env, false, true, errs)?,
                    None => (),
                }
                self.check_idents_for_expr(&**body, tree, var_env, type_param_env, errs)?;
                var_env.pop_vars();
            },
            Expr::Var(ident, _, pos) => check_var_ident(ident, tree, var_env, pos.clone(), errs),
            Expr::NamedFieldConApp(ident, expr_named_field_pairs, _, pos) => {
                match check_con_ident(ident, tree, pos.clone(), true, errs) {
                    Some(con) => {
                        self.check_idents_for_named_field_pairs(expr_named_field_pairs.as_slice(), con, tree, type_param_env, pos.clone(), errs, |namer, expr, tree, type_param_env, errs| {
                                namer.check_idents_for_expr(expr, tree, var_env, type_param_env, errs)
                        })?;
                    },
                    None => (),
                }
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                for expr3 in exprs {
                    self.check_idents_for_expr(&**expr3, tree, var_env, type_param_env, errs)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?,
            Expr::Get2Field(expr2, _, _, _) => self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                self.check_idents_for_expr(&**expr3, tree, var_env, type_param_env, errs)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                self.check_idents_for_expr(&**expr3, tree, var_env, type_param_env, errs)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                self.check_idents_for_expr(&**expr3, tree, var_env, type_param_env, errs)?;
            },
            Expr::Uniq(expr2, _, _) => self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?,
            Expr::Shared(expr2, _, _) => self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?,
            Expr::Typed(expr2, type_expr, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                self.check_idents_for_type_expr(&**type_expr, tree, type_param_env, false, true, errs)?;
            },
            Expr::As(expr2, type_expr, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                self.check_idents_for_type_expr(&**type_expr, tree, type_param_env, false, true, errs)?;
            },
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                self.check_idents_for_expr(&**expr3, tree, var_env, type_param_env, errs)?;
                self.check_idents_for_expr(&**expr4, tree, var_env, type_param_env, errs)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.check_idents_for_expr(&**expr3, tree, var_env, type_param_env, errs)?;
                            let mut var_idents: BTreeSet<String> = BTreeSet::new();
                            self.check_idents_for_pattern(&**pattern, tree, var_env, type_param_env, &mut var_idents, false, errs)?;
                        },
                    }
                }
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                var_env.pop_vars();
            },
            Expr::Match(expr2, cases, _, _) => {
                self.check_idents_for_expr(&**expr2, tree, var_env, type_param_env, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_new_vars();
                            let mut var_idents: BTreeSet<String> = BTreeSet::new();
                            self.check_idents_for_pattern(&**pattern, tree, var_env, type_param_env, &mut var_idents, false, errs)?;
                            self.check_idents_for_expr(&**expr3, tree, var_env, type_param_env, errs)?;
                            var_env.pop_vars();
                        },
                    }
                }
            },
        }
        Ok(())
    }
    
    fn check_idents_for_pattern(&self, pattern: &Pattern, tree: &Tree, var_env: &mut Environment<()>, type_param_env: &mut Environment<()>, var_idents: &mut BTreeSet<String>, is_in_alt_pattern: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, _, _) => {
                self.check_idents_for_literal(&**literal, tree, var_env, type_param_env, errs, |namer, pattern, tree, var_env, type_param_env, errs| {
                    namer.check_idents_for_pattern(pattern, tree, var_env, type_param_env, var_idents, is_in_alt_pattern, errs)
                })?;
            },
            Pattern::As(literal, type_expr, _, _) => {
                self.check_idents_for_literal(&**literal, tree, var_env, type_param_env, errs, |namer, pattern, tree, var_env, type_param_env, errs| {
                    namer.check_idents_for_pattern(pattern, tree, var_env, type_param_env, var_idents, is_in_alt_pattern, errs)
                })?;
                self.check_idents_for_type_expr(&**type_expr, tree, type_param_env, false, true, errs)?;
            },
            Pattern::Const(ident, _, pos) => check_const_ident(ident, tree, pos.clone(), errs),
            Pattern::UnnamedFieldCon(ident, patterns, _, pos) => {
                match check_con_ident(ident, tree, pos.clone(), false, errs) {
                    Some(con) => {
                        let con_r = con.borrow();
                        match &*con_r {
                            Con::UnnamedField(_, field_type_exprs, _, _) => {
                                if patterns.len() < field_type_exprs.len() {
                                    errs.push(FrontendError::Message(pos.clone(), String::from("too few fields")));
                                } else if patterns.len() > field_type_exprs.len() {
                                    errs.push(FrontendError::Message(pos.clone(), String::from("too many fields")));
                                }
                            },
                            Con::NamedField(_, type_expr_named_field_pairs, _, _, _) => {
                                if patterns.len() < type_expr_named_field_pairs.len() {
                                    errs.push(FrontendError::Message(pos.clone(), String::from("too few fields")));
                                } else if patterns.len() > type_expr_named_field_pairs.len() {
                                    errs.push(FrontendError::Message(pos.clone(), String::from("too many fields")));
                                }
                            },
                        }
                        for pattern2 in patterns {
                            self.check_idents_for_pattern(&**pattern2, tree, var_env, type_param_env, var_idents, is_in_alt_pattern, errs)?;
                        }
                    },
                    None => (),
                }
            },
            Pattern::NamedFieldCon(ident, pattern_named_field_pairs, _, pos) => {
                match check_con_ident(ident, tree, pos.clone(), true, errs) {
                    Some(con) => {
                        self.check_idents_for_named_field_pairs(pattern_named_field_pairs.as_slice(), con, tree, type_param_env, pos.clone(), errs, |namer, expr, tree, type_param_env, errs| {
                                namer.check_idents_for_pattern(expr, tree, var_env, type_param_env, var_idents, is_in_alt_pattern, errs)
                        })?;
                    },
                    None => (),
                }
            }
            Pattern::Var(_, ident, _, pos) => {
                if !is_in_alt_pattern {
                    if !var_idents.contains(ident) {
                        var_env.add_var(ident.clone(), ());
                        var_idents.insert(ident.clone());
                    } else {
                        errs.push(FrontendError::Message(pos.clone(), format!("already variable {} in pattern", ident)));
                    }
                } else {
                    errs.push(FrontendError::Message(pos.clone(), String::from("variable pattern mustn't be in alternative pattern")));
                }
            },
            Pattern::At(_, ident, pattern2, _, pos) => {
                if !is_in_alt_pattern {
                    if !var_idents.contains(ident) {
                        var_env.add_var(ident.clone(), ());
                        var_idents.insert(ident.clone());
                    } else {
                        errs.push(FrontendError::Message(pos.clone(), format!("already defined {} in pattern", ident)));
                    }
                } else {
                    errs.push(FrontendError::Message(pos.clone(), String::from("variable pattern mustn't be in alternative pattern")));
                }
                self.check_idents_for_pattern(&**pattern2, tree, var_env, type_param_env, var_idents, is_in_alt_pattern, errs)?;
            },
            Pattern::Wildcard(_, _) => (),
            Pattern::Alt(patterns, _, _) => {
                for pattern2 in patterns {
                    self.check_idents_for_pattern(&**pattern2, tree, var_env, type_param_env, var_idents, true, errs)?;
                }
            },
        }
        Ok(())
    }
    
    fn check_idents_for_literal<T, F>(&self, literal: &Literal<T>, tree: &Tree, var_env: &mut Environment<()>, type_param_env: &mut Environment<()>, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, &Tree, &mut Environment<()>, &mut Environment<()>, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match literal {
            Literal::Tuple(field_others) => {
                for field_other in field_others {
                    f(self, &**field_other, tree, var_env, type_param_env, errs)?;
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, &**elem_other, tree, var_env, type_param_env, errs)?;
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, &**elem_other, tree, var_env, type_param_env, errs)?,
            _ => (),
        }
        Ok(())
    }

    fn check_idents_for_lambda_args(&self, lambda_args: &[LambdaArg], tree: &Tree, var_env: &mut Environment<()>, type_param_env: &mut Environment<()>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let mut lambda_arg_idents: BTreeSet<String> = BTreeSet::new();
        for lambda_arg in lambda_args {
            match lambda_arg {
                LambdaArg(ident, type_expr, _, pos) => {
                    if !lambda_arg_idents.contains(ident) {
                        var_env.add_var(ident.clone(), ());
                        lambda_arg_idents.insert(ident.clone());
                        match type_expr {
                            Some(type_expr) => self.check_idents_for_type_expr(&**type_expr, tree, type_param_env, false, true, errs)?,
                            None => (),
                        }
                    } else {
                        errs.push(FrontendError::Message(pos.clone(), format!("already defined argument {}", ident)));
                    }
                },
            }
        }
        Ok(())
    }
}
