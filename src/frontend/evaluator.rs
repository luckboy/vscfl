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
use crate::utils::dfs::*;
use crate::utils::env::*;
use crate::utils::pattern::*;

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
                    if !processed_keys.contains(&key) {
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

    fn do_named_field_pairs<T, F>(&self, named_field_pairs: &[NamedFieldPair<T>], errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for named_field_pair in named_field_pairs {
            match named_field_pair {
                NamedFieldPair(_, other, _) => f(self, other, errs)?,
            }
        }
        Ok(())
    }

    fn do_literal<T, F>(&self, literal: &Literal<T>, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>,
    {
        match literal {
            Literal::Tuple(field_others) => {
                for field_other in field_others {
                    f(self, &**field_other, errs)?
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, &**elem_other, errs)?
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, &**elem_other, errs)?,
            _ => (),
        }
        Ok(())
    }
    
    fn add_var_keys_for_expr(&self, expr: &Expr, tree: &Tree, var_env: &mut Environment<()>, type_stack: &mut TypeStack, local_types: &LocalTypes, keys: &mut Vec<(String, Option<TypeName>)>, processed_keys: &BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.do_literal(&**literal, errs, |evaluator, expr, errs| evaluator.add_var_keys_for_expr(expr, tree, var_env, type_stack, local_types, keys, processed_keys, errs))?,
            Expr::Lambda(args, _, body, _, _, _, _, _) => {
                var_env.push_new_vars();
                for arg in &*args {
                    match arg {
                        LambdaArg(ident, _, _, _) => {
                            var_env.add_var(ident.clone(), ());
                        },
                    }
                }
                self.add_var_keys_for_expr(&**body, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                var_env.pop_vars();
            },
            Expr::Var(ident, Some(local_type), pos) => {
                if var_env.var(ident).is_none() {
                    let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                    add_var_key(ident, &type_name, pos.clone(), tree, keys, processed_keys, errs)?;
                }
            },
            Expr::NamedFieldConApp(ident, expr_named_field_pairs, _, _, _) => {
                self.do_named_field_pairs(expr_named_field_pairs.as_slice(), errs, |evaluator, expr, errs| evaluator.add_var_keys_for_expr(expr, tree, var_env, type_stack, local_types, keys, processed_keys, errs))?
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                for expr3 in exprs {
                    self.add_var_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?,
            Expr::Get2Field(expr2, _, _, _) => self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                self.add_var_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                self.add_var_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                self.add_var_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
            },
            Expr::Uniq(expr2, _, _) => self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?,
            Expr::Shared(expr2, _, _) => self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?,
            Expr::Typed(expr2, _, _, _) => self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?,
            Expr::As(expr2, _, _, _) => self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                self.add_var_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                self.add_var_keys_for_expr(&**expr4, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.add_var_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                            self.add_var_keys_for_pattern(&**pattern, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                        },
                    }
                }
                self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                var_env.pop_vars();
            },
            Expr::Match(expr2, cases, _, _) => {
                self.add_var_keys_for_expr(&**expr2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_new_vars();
                            self.add_var_keys_for_pattern(&**pattern, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                            self.add_var_keys_for_expr(&**expr3, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                            var_env.pop_vars();
                        },
                    }
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_keys_for_expr: no local type"))])),
        }
        Ok(())
    }

    fn add_var_keys_for_pattern(&self, pattern: &Pattern, tree: &Tree, var_env: &mut Environment<()>, type_stack: &mut TypeStack, local_types: &LocalTypes, keys: &mut Vec<(String, Option<TypeName>)>, processed_keys: &BTreeSet<(String, Option<TypeName>)>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, _, _) => self.do_literal(&**literal, errs, |evaluator, pattern, errs| evaluator.add_var_keys_for_pattern(pattern, tree, var_env, type_stack, local_types, keys, processed_keys, errs))?,
            Pattern::As(literal, _, _, _, _) => self.do_literal(&**literal, errs, |evaluator, pattern, errs| evaluator.add_var_keys_for_pattern(pattern, tree, var_env, type_stack, local_types, keys, processed_keys, errs))?,
            Pattern::Const(ident, Some(local_type), pos) => {
                if var_env.var(ident).is_none() {
                    let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                    add_var_key(ident, &type_name, pos.clone(), tree, keys, processed_keys, errs)?;
                }
            },
            Pattern::UnnamedFieldCon(ident, patterns, _, _, _) => {
                for pattern2 in patterns {
                    self.add_var_keys_for_pattern(&**pattern2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                }
            },
            Pattern::NamedFieldCon(ident, pattern_named_field_pairs, _, _, _) => {
                self.do_named_field_pairs(pattern_named_field_pairs.as_slice(), errs, |evaluator, expr, errs| evaluator.add_var_keys_for_pattern(pattern, tree, var_env, type_stack, local_types, keys, processed_keys, errs))?;
            },
            Pattern::Var(_, ident, _, _) => {
                var_env.add_var(ident.clone(), ());
            },
            Pattern::At(_, ident, pattern2, _, _) => {
                var_env.add_var(ident.clone(), ());
                self.add_var_keys_for_pattern(&**pattern2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
            },
            Pattern::Wildcard(_, _) => (),
            Pattern::Alt(patterns, _, _) => {
                for pattern2 in patterns {
                    self.add_var_keys_for_pattern(&**pattern2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_keys_for_pattern: no local type"))])),
        }
        Ok(())
    }
}
