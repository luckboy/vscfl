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
use crate::frontend::private::*;
use crate::frontend::tree::*;
use crate::frontend::type_stack::*;
use crate::utils::dfs::*;
use crate::utils::env::*;

fn add_fun_key(ident: &String, type_name: &Option<TypeName>, pos: Pos, tree: &Tree, keys: &mut Vec<(String, Option<TypeName>)>, processed_keys: &BTreeSet<(String, Option<TypeName>)>, rec_key: Option<&(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            let (trait_ident, is_builtin_var) = match &*var_r {
                Var::Builtin(tmp_trait_ident, _) => (tmp_trait_ident, true),
                Var::Var(_, _, _, _, _, _, _, _, _) => return Ok(()),
                Var::Fun(_, tmp_trait_ident, _) => (tmp_trait_ident, false),
            };
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
                                                        _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_fun_key: no implementation variables"))])),
                                                    };
                                                    match impl_vars.var(ident) {
                                                        Some(impl_var) => {
                                                            let impl_var_r = impl_var.borrow();
                                                            match &*impl_var_r {
                                                                ImplVar::Builtin(_) => return Ok(()),
                                                                ImplVar::Fun(_, _) => Some(type_name.clone()),
                                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_fun_key: implementation variable is variable"))])),
                                                            }
                                                        },
                                                        None => None,
                                                    }
                                                },
                                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_fun_key: no implementation"))])),
                                            }
                                        },
                                        _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_fun_key: no trait variables"))])),
                                    }
                                },
                                None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_fun_key: no trait"))])),
                            }
                        },
                        None => None,
                    }
                },
                None => None,
            };
            if !is_builtin_var || key_type_name.is_some() {
                let key = (ident.clone(), key_type_name);
                match rec_key {
                    Some(rec_key) if rec_key == &key => (),
                    _ => {
                        if !processed_keys.contains(&key) {
                            keys.push(key);
                        } else {
                            errs.push(FrontendError::Message(pos, format!("recursive function {} can use only tail recursion", ident)));
                        }
                    },
                }
            }
            Ok(())
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_fun_key: no variable"))])),
    }
}

fn do_fun_for_fun_key<T, F>(key: &(String, Option<TypeName>), tree: &Tree, z: T, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&[String], &Expr, &LocalTypes, &Type) -> FrontendResultWithErrors<T>
{
    match tree.var(&key.0) {
        Some(var) => {
            let var_r = var.borrow();
            let (trait_ident, fun_tuple) = match &*var_r {
                Var::Builtin(tmp_trait_ident, _) => (tmp_trait_ident, None),
                Var::Fun(fun, tmp_trait_ident, Some(tmp_type)) => {
                    match &**fun {
                        Fun::Fun(_, args, _, _, tmp_body, _, Some(tmp_local_types)) => {
                            let tmp_arg_idents: Vec<String> = args.iter().map(|a| {
                                    match a {
                                        Arg(ident, _, _, _) => ident.clone(),
                                    }
                            }).collect();
                            (tmp_trait_ident, Some((tmp_arg_idents, tmp_body, tmp_local_types, tmp_type)))
                        },
                        _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: function is constructor or no local types"))])),
                    }
                },
                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: variable is variable or no type"))])),
            };
            match &key.1 {
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
                                                    match impl_vars.var(&key.0) {
                                                        Some(impl_var) => {
                                                            let impl_var_r = impl_var.borrow();
                                                            match &*impl_var_r {
                                                                ImplVar::Fun(impl_fun, Some(type2)) => {
                                                                    match &**impl_fun {
                                                                        ImplFun(impl_args, body2, _, Some(local_types2)) => {
                                                                            let arg_idents2: Vec<String> =impl_args.iter().map(|ia| {
                                                                                    match ia {
                                                                                        ImplArg(ident, _, _) => ident.clone(),
                                                                                    }
                                                                            }).collect();
                                                                            f(arg_idents2.as_slice(), &**body2, &**local_types2, &**type2)
                                                                        },
                                                                        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: no local types no type"))])),
                                                                    }
                                                                },
                                                                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: implementation variable isn't variable or no type"))])),
                                                            }
                                                        },
                                                        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: implementation variable is variable"))])),
                                                    }
                                                },
                                                None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: no implementation"))])),
                                            }
                                        },
                                        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: no trait variables"))])),
                                    }
                                },
                                None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: no trait"))])),
                            }
                        },
                        None => {
                            match fun_tuple {
                                Some((arg_idents, body, local_types, typ)) => {
                                    match body {
                                        Some(body) => f(arg_idents.as_slice(), &**body, &**local_types, &**typ),
                                        None => Ok(z),
                                    }
                                },
                                None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: variable is built-in variable"))])),
                            }
                        },
                    }
                },
                None => {
                    match fun_tuple {
                        Some((arg_idents, body, local_types, typ)) => {
                            match body {
                                Some(body) => f(arg_idents.as_slice(), &**body, &**local_types, &**typ),
                                None => Ok(z),
                            }
                        },
                        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: variable is built-in variable"))])),
                    }
                },
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("do_fun_for_fun_key: no variable"))])),
    }
}

pub struct Recurser
{}

impl Recurser
{
    pub fn new() -> Self
    { Recurser {} }

    pub fn check_recursions(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.check_recursions_for_fun_defs(tree, &mut errs)?;
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }

    fn check_recursions_for_fun_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let mut visited_keys: BTreeSet<(String, Option<TypeName>)> = BTreeSet::new();
        for def in tree.defs() {
            match &**def {
                Def::Var(ident, var, _) => self.check_recursions_for_fun(ident, var, tree, &mut visited_keys, errs)?,
                Def::Trait(_, trait1, _) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, trait_defs, _) => {
                            for trait_def in trait_defs {
                                match &**trait_def {
                                    TraitDef(ident, var, _) => self.check_recursions_for_fun(ident, var, tree, &mut visited_keys, errs)?,
                                }
                            }
                        },
                    }
                },
                Def::Impl(impl1, _) => {
                    let impl_r = impl1.borrow();
                    match &*impl_r {
                        Impl::Builtin(_, _, _) => (),
                        Impl::Impl(_, type_name, impl_defs, _) => {
                            for impl_def in impl_defs {
                                match &**impl_def {
                                    ImplDef(ident, impl_var, _) => self.check_recursions_for_impl_fun(ident, type_name, impl_var, tree, &mut visited_keys, errs)?,
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
    
    fn check_recursions_for_fun(&self, ident: &String, var: &Rc<RefCell<Var>>, tree: &Tree, visited_keys: &mut BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let is_fun = {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(_, _, _) => true,
                _ => false,
            }
        };
        if is_fun {
            self.check_recursions_for_fun_key(&(ident.clone(), None), tree, visited_keys, errs)?;
        }
        Ok(())
    }

    fn check_recursions_for_fun_key(&self, key: &(String, Option<TypeName>), tree: &Tree, visited_keys: &mut BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        dfs_with_result(key, visited_keys, errs, |key, processed_keys, errs| {
                self.check_recursions_for_fun_key2(key, tree, processed_keys, errs)
        }, |_, _| Ok (()))
    }
    
    fn check_recursions_for_fun_key2(&self, key: &(String, Option<TypeName>), tree: &Tree, processed_keys: &BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Vec<(String, Option<TypeName>)>>
    {
        do_fun_for_fun_key(key, tree, Vec::new(), |arg_idents, body, local_types, typ| {
                let mut keys: Vec<(String, Option<TypeName>)> = Vec::new();
                let mut type_stack = TypeStack::new();
                let mut var_env: Environment<()> = Environment::new();
                type_stack.set_first_type_values_for_type(typ);
                var_env.push_new_vars();
                for arg_ident in arg_idents {
                    var_env.add_var(arg_ident.clone(), ());
                }
                self.add_fun_keys_for_expr(body, tree, &mut var_env, &mut type_stack, local_types, &mut keys, processed_keys, Some(key), errs)?;
                Ok(keys)
        })
    }
    
    fn add_fun_keys_for_expr(&self, expr: &Expr, tree: &Tree, var_env: &mut Environment<()>, type_stack: &mut TypeStack, local_types: &LocalTypes, keys: &mut Vec<(String, Option<TypeName>)>, processed_keys: &BTreeSet<(String, Option<TypeName>)>, rec_key: Option<&(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.add_fun_keys_for_expr_literal(&**literal, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?,
            Expr::Lambda(args, _, body, _, _, _, _, _) => {
                var_env.push_new_vars();
                for arg in &*args {
                    match arg {
                        LambdaArg(ident, _, _, _) => {
                            var_env.add_var(ident.clone(), ());
                        },
                    }
                }
                self.add_fun_keys_for_expr(&**body, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                var_env.pop_vars();
            },
            Expr::Var(ident, Some(local_type), pos) => {
                if var_env.var(ident).is_none() {
                    let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                    add_fun_key(ident, &type_name, pos.clone(), tree, keys, processed_keys, None, errs)?;
                }
            },
            Expr::NamedFieldConApp(_, expr_named_field_pairs, _, _, _) => {
                for expr_named_field_pair in expr_named_field_pairs {
                    match expr_named_field_pair {
                        NamedFieldPair(_, expr2, _) => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
                    }
                }
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                match rec_key {
                    Some(rec_key) => {
                        match &**expr2 {
                            Expr::Var(ident, Some(local_type), pos) => {
                                if var_env.var(ident).is_none() {
                                    let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                                    add_fun_key(ident, &type_name, pos.clone(), tree, keys, processed_keys, Some(rec_key), errs)?;
                                }
                            },
                            _ => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
                        }
                    },
                    None => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
                }
                for expr3 in exprs {
                    self.add_fun_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
            Expr::Get2Field(expr2, _, _, _) => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                self.add_fun_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                self.add_fun_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                self.add_fun_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
            },
            Expr::Uniq(expr2, _, _) => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
            Expr::Shared(expr2, _, _) => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
            Expr::Typed(expr2, _, _, _) => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, rec_key, errs)?,
            Expr::As(expr2, _, _, _) => self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                self.add_fun_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, rec_key, errs)?;
                self.add_fun_keys_for_expr(&**expr4, tree, var_env, type_stack, local_types, keys, processed_keys, rec_key, errs)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.add_fun_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                            self.add_vars_for_pattern(&**pattern, var_env);
                        },
                    }
                }
                self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, rec_key, errs)?;
                var_env.pop_vars();
            },
            Expr::Match(expr2, cases, _, _) => {
                self.add_fun_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_new_vars();
                            self.add_vars_for_pattern(&**pattern, var_env);
                            self.add_fun_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, rec_key, errs)?;
                            var_env.pop_vars();
                        },
                    }
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_fun_keys_for_expr: no local type"))])),
        }
        Ok(())
    }

    fn add_vars_for_pattern(&self, pattern: &Pattern, var_env: &mut Environment<()>)
    {
        match pattern {
            Pattern::Literal(literal, _, _) => self.add_vars_for_pattern_literal(&**literal, var_env),
            Pattern::As(_, _, _, _, _) => (),
            Pattern::Const(_, _, _) => (),
            Pattern::UnnamedFieldCon(_, patterns, _, _, _) => {
                for pattern2 in patterns {
                    self.add_vars_for_pattern(pattern2, var_env);
                }
            },
            Pattern::NamedFieldCon(_, pattern_named_field_pairs, _, _, _) => {
                for pattern_named_field_pair in pattern_named_field_pairs {
                    match pattern_named_field_pair {
                        NamedFieldPair(_, pattern2, _) => self.add_vars_for_pattern(pattern2, var_env),
                    }
                }
            },
            Pattern::Var(_, ident, _, _) => {
                var_env.add_var(ident.clone(), ());
            },
            Pattern::At(_, ident, pattern2, _, _) => {
                var_env.add_var(ident.clone(), ());
                self.add_vars_for_pattern(pattern2, var_env);
            },
            Pattern::Wildcard(_, _) => (),
            Pattern::Alt(_, _, _) => (),
        }
    }

    fn add_fun_keys_for_expr_literal(&self, literal: &Literal<Expr>, tree: &Tree, var_env: &mut Environment<()>, type_stack: &mut TypeStack, local_types: &LocalTypes, keys: &mut Vec<(String, Option<TypeName>)>, processed_keys: &BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match literal {
            Literal::Tuple(field_exprs) => {
                for field_expr in field_exprs {
                    self.add_fun_keys_for_expr(&**field_expr, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                }
            },
            Literal::Array(elem_exprs) => {
                for elem_expr in elem_exprs {
                    self.add_fun_keys_for_expr(&**elem_expr, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?;
                }
            },
            Literal::FilledArray(elem_expr, _) => self.add_fun_keys_for_expr(&**elem_expr, tree, var_env, type_stack, local_types, keys, processed_keys, None, errs)?,
            _ => (),
        }
        Ok(())
    }

    fn add_vars_for_pattern_literal(&self, literal: &Literal<Pattern>, var_env: &mut Environment<()>)
    {
        match literal {
            Literal::Tuple(field_patterns) => {
                for field_pattern in field_patterns {
                    self.add_vars_for_pattern(field_pattern, var_env);
                }
            },
            Literal::Array(elem_patterns) => {
                for elem_pattern in elem_patterns {
                    self.add_vars_for_pattern(elem_pattern, var_env);
                }
            },
            Literal::FilledArray(elem_pattern, _) => self.add_vars_for_pattern(elem_pattern, var_env),
            _ => (),
        }
    }

    fn check_recursions_for_impl_fun(&self, ident: &String, type_name: &TypeName, impl_var: &Rc<RefCell<ImplVar>>, tree: &Tree, visited_keys: &mut BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        let is_impl_fun = {
            let impl_var_r = impl_var.borrow();
            match &*impl_var_r {
                ImplVar::Fun(_, _) => true,
                _ => false,
            }
        };
        if is_impl_fun{
            self.check_recursions_for_fun_key(&(ident.clone(), Some(type_name.clone())), tree, visited_keys, errs)?;
        }
        Ok(())
    }
}
