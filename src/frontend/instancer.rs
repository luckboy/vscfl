//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;
use crate::utils::env::*;

fn is_inst_for_type_value(type_value: &Rc<TypeValue>, local_types: &LocalTypes) -> FrontendResultWithErrors<bool>
{
    match local_types.type_entry_for_type_value(type_value) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, _, type_param_entry, _)) => {
            let type_param_entry_r = type_param_entry.borrow();
            if type_param_entry_r.trait_names.is_empty() || (type_param_entry_r.trait_names.len() == 1 && (type_param_entry_r.trait_names.contains(&TraitName::Shared) || type_param_entry_r.trait_names.contains(&TraitName::Fun))) || (type_param_entry_r.trait_names.len() == 2 && type_param_entry_r.trait_names.contains(&TraitName::Shared) && type_param_entry_r.trait_names.contains(&TraitName::Fun)) {
                let mut is_inst = true;
                for type_value2 in &type_param_entry_r.type_values {
                    is_inst &= is_inst_for_type_value(type_value2, local_types)?;
                }
                Ok(is_inst)
            } else {
                Ok(false)
            }
        },
        Some(LocalTypeEntry::Param(DefinedFlag::Defined, _, _, _)) => Ok(true),
        Some(LocalTypeEntry::Type(type_value)) => {
            match &*type_value {
                TypeValue::Param(_, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("is_inst_for_type_value: type parameter in local type entry"))])),
                TypeValue::Type(_, _, type_values) => {
                    let mut is_inst = true;
                    for type_value2 in type_values {
                        is_inst &= is_inst_for_type_value(type_value2, local_types)?;
                    }
                    Ok(is_inst)
                },
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("is_inst_for_type_value: no local type entry"))])),
    }
}

fn check_inst_for_var_ident_and_local_type(ident: &String, local_type: LocalType, pos: Pos, tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    if !is_inst_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type)), local_types)? {
        match tree.var(ident) {
            Some(var) => {
                let var_r = var.borrow();
                match &*var_r {
                    Var::Builtin(_, _) => errs.push(FrontendError::Message(pos, format!("no instance of built-in variable {} with type {} with traits", ident, LocalTypeWithLocalTypes(local_type, local_types)))),
                    Var::Var(_, _, _, _, _, _, _, _, _) => errs.push(FrontendError::Message(pos, format!("no instance of variable {} with type {} with traits", ident, LocalTypeWithLocalTypes(local_type, local_types)))),
                    Var::Fun(fun, _, _) => {
                        match &**fun {
                            Fun::Fun(_, _, _, _, _, _, _) => errs.push(FrontendError::Message(pos, format!("no instance of function {} with type {} with traits", ident, LocalTypeWithLocalTypes(local_type, local_types)))),
                            Fun::Con(_) => errs.push(FrontendError::Message(pos, format!("no instance of constructor {} with type {} with traits", ident, LocalTypeWithLocalTypes(local_type, local_types)))),
                        }
                    },
                }
            },
            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_inst_for_var_ident_and_local_type: no variable"))])),
        }
    }
    Ok(())
}

pub struct Instancer
{}

impl Instancer
{
    pub fn new() -> Self
    { Instancer {} }

    pub fn check_insts(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.check_insts_for_defs(tree, &mut errs)?;
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }

    fn check_insts_for_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Var(_, var, _) => {
                    let var_r = var.borrow();
                    self.check_insts_for_var(&*var_r, tree, errs)?;
                },
                Def::Trait(_, trait1, _) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, trait_defs, _) => {
                            for trait_def in trait_defs {
                                match &**trait_def {
                                    TraitDef(_, var, _) => {
                                        let var_r = var.borrow();
                                        self.check_insts_for_var(&*var_r, tree, errs)?;
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
                                    ImplDef(_, impl_var, _) => {
                                        let impl_var_r = impl_var.borrow();
                                        self.check_insts_for_impl_var(&*impl_var_r, tree, errs)?;
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
    
    fn check_insts_for_named_field_pairs<T, F>(&self, named_field_pairs: &[NamedFieldPair<T>], tree: &Tree, var_env: &mut Environment<()>, local_types: &LocalTypes, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, &Tree, &mut Environment<()>, &LocalTypes, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for named_field_pair in named_field_pairs {
            match named_field_pair {
                NamedFieldPair(_, other, _) => f(self, other, tree, var_env, local_types, errs)?,
            }
        }
        Ok(())
    }

    fn check_insts_for_var(&self, var: &Var, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match var {
            Var::Builtin(_, _) => (),
            Var::Var(_, _, _, Some(expr), _, _, Some(local_types), _, _) => {
                let mut var_env: Environment<()> = Environment::new();
                self.check_insts_for_expr(&**expr, tree, &mut var_env, &**local_types, errs)?;
            },
            Var::Var(_, _, _, None, _, _, _, _, _) => (),
            Var::Fun(fun, _, _) => {
                match &**fun {
                    Fun::Fun(_, args, _, _, Some(body), _, Some(local_types)) => {
                        let mut var_env: Environment<()> = Environment::new();
                        var_env.push_new_vars();
                        for arg in args {
                            match arg {
                                Arg(ident, _, _, _) => {
                                    var_env.add_var(ident.clone(), ());
                                },
                            }
                        }
                        self.check_insts_for_expr(&**body, tree, &mut var_env, &**local_types, errs)?;
                    },
                    Fun::Fun(_, _, _, _, None, _, _) => (),
                    Fun::Con(_) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_insts_for_var: variable is contructor"))])),
                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_insts_for_var: no local types"))])),
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_insts_for_var: no local types"))])),
        }
        Ok(())
    }
    
    fn check_insts_for_expr(&self, expr: &Expr, tree: &Tree, var_env: &mut Environment<()>, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.check_insts_for_literal(&**literal, tree, var_env, local_types, errs, Self::check_insts_for_expr)?,
            Expr::Lambda(args, _, body, _, _, _, _, _) => {
                var_env.push_new_vars();
                for arg in &*args {
                    match arg {
                        LambdaArg(ident, _, _, _) => {
                            var_env.add_var(ident.clone(), ());
                        },
                    }
                }
                self.check_insts_for_expr(&**body, tree, var_env, local_types, errs)?;
                var_env.pop_vars();
            },
            Expr::Var(ident, Some(local_type), pos) => {
                if var_env.var(ident).is_none() {
                    check_inst_for_var_ident_and_local_type(ident, *local_type, pos.clone(), tree, local_types, errs)?;
                }
            },
            Expr::NamedFieldConApp(ident, expr_named_field_pairs, Some(con_local_type), _, pos) => {
                check_inst_for_var_ident_and_local_type(ident, *con_local_type, pos.clone(), tree, local_types, errs)?;
                self.check_insts_for_named_field_pairs(expr_named_field_pairs.as_slice(), tree, var_env, local_types, errs, Self::check_insts_for_expr)?
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                for expr3 in exprs {
                    self.check_insts_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::Get2Field(expr2, _, _, _) => self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                self.check_insts_for_expr(&**expr3, tree, var_env, local_types, errs)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                self.check_insts_for_expr(&**expr3, tree, var_env, local_types, errs)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                self.check_insts_for_expr(&**expr3, tree, var_env, local_types, errs)?;
            },
            Expr::Uniq(expr2, _, _) => self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::Shared(expr2, _, _) => self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::Typed(expr2, _, _, _) => self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::As(expr2, _, _, _) => self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                self.check_insts_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                self.check_insts_for_expr(&**expr4, tree, var_env, local_types, errs)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.check_insts_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                            self.check_insts_for_pattern(&**pattern, tree, var_env, local_types, errs)?;
                        },
                    }
                }
                self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                var_env.pop_vars();
            },
            Expr::Match(expr2, cases, _, _) => {
                self.check_insts_for_expr(&**expr2, tree, var_env, local_types, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            var_env.push_new_vars();
                            self.check_insts_for_pattern(&**pattern, tree, var_env, local_types, errs)?;
                            self.check_insts_for_expr(&**expr3, tree, var_env, local_types, errs)?;
                            var_env.pop_vars();
                        },
                    }
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_insts_for_expr: no local type"))])),
        }
        Ok(())
    }
        
    fn check_insts_for_pattern(&self, pattern: &Pattern, tree: &Tree, var_env: &mut Environment<()>, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, _, _) => self.check_insts_for_literal(&**literal, tree, var_env, local_types, errs, Self::check_insts_for_pattern)?,
            Pattern::As(literal, _, _, _, _) => self.check_insts_for_literal(&**literal, tree, var_env, local_types, errs, Self::check_insts_for_pattern)?,
            Pattern::Const(ident, Some(local_type), pos) => check_inst_for_var_ident_and_local_type(ident, *local_type, pos.clone(), tree, local_types, errs)?,
            Pattern::UnnamedFieldCon(ident, patterns, Some(con_local_type), _, pos) => {
                check_inst_for_var_ident_and_local_type(ident, *con_local_type, pos.clone(), tree, local_types, errs)?;
                for pattern2 in patterns {
                    self.check_insts_for_pattern(&**pattern2, tree, var_env, local_types, errs)?;
                }
            },
            Pattern::NamedFieldCon(ident, pattern_named_field_pairs, Some(con_local_type), _, pos) => {
                check_inst_for_var_ident_and_local_type(ident, *con_local_type, pos.clone(), tree, local_types, errs)?;
                self.check_insts_for_named_field_pairs(pattern_named_field_pairs.as_slice(), tree, var_env, local_types, errs, Self::check_insts_for_pattern)?
            },
            Pattern::Var(_, ident, _, _) => {
                var_env.add_var(ident.clone(), ());
            },
            Pattern::At(_, ident, pattern2, _, _) => {
                var_env.add_var(ident.clone(), ());
                self.check_insts_for_pattern(&**pattern2, tree, var_env, local_types, errs)?;
            },
            Pattern::Wildcard(_, _) => (),
            Pattern::Alt(patterns, _, _) => {
                for pattern2 in patterns {
                    self.check_insts_for_pattern(&**pattern2, tree, var_env, local_types, errs)?;
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_insts_for_pattern: no local type"))])),
        }
        Ok(())
    }

    fn check_insts_for_literal<T, F>(&self, literal: &Literal<T>, tree: &Tree, var_env: &mut Environment<()>, local_types: &LocalTypes, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, &Tree, &mut Environment<()>, &LocalTypes, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>,
    {
        match literal {
            Literal::Tuple(field_others) => {
                for field_other in field_others {
                    f(self, &**field_other, tree, var_env, local_types, errs)?
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, &**elem_other, tree, var_env, local_types, errs)?
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, &**elem_other, tree, var_env, local_types, errs)?,
            _ => (),
        }
        Ok(())
    }

    fn check_insts_for_impl_var(&self, impl_var: &ImplVar, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match impl_var {
            ImplVar::Builtin(_) => (),
            ImplVar::Var(expr, _, Some(local_types), _, _) => {
                let mut var_env: Environment<()> = Environment::new();
                self.check_insts_for_expr(&**expr, tree, &mut var_env, &**local_types, errs)?;
            },
            ImplVar::Fun(impl_fun, _) => {
                match &**impl_fun {
                    ImplFun(args, body, _, Some(local_types)) => {
                        let mut var_env: Environment<()> = Environment::new();
                        var_env.push_new_vars();
                        for arg in args {
                            match arg {
                                ImplArg(ident, _, _) => {
                                    var_env.add_var(ident.clone(), ());
                                },
                            }
                        }
                        self.check_insts_for_expr(&**body, tree, &mut var_env, &**local_types, errs)?;
                    },
                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_insts_for_impl_var: no local types"))])),
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_insts_for_impl_var: no local types"))])),
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
