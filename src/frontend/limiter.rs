//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::frontend::error::*;
use crate::frontend::tree::*;

fn check_local_var_modifier(var_modifier: VarModifier, ident: &String, pos: Pos, is_in_var: bool, errs: &mut Vec<FrontendError>)
{
    if is_in_var {
        if var_modifier != VarModifier::None {
            errs.push(FrontendError::Message(pos, format!("variable {} has variable modifier", ident)));
        }
    } else {
        if var_modifier == VarModifier::Constant {
            errs.push(FrontendError::Message(pos, format!("variable {} mustn't be constant", ident)));
        }
    }
}

fn check_global_var_modifier(var_modifier: VarModifier, ident: &String, pos: Pos, errs: &mut Vec<FrontendError>)
{
    match var_modifier {
        VarModifier::Private => errs.push(FrontendError::Message(pos, format!("variable {} mustn't be private", ident))),
        VarModifier::Local => errs.push(FrontendError::Message(pos, format!("variable {} mustn't be local", ident))),
        _ => (),
    }
}

fn check_fun_modifier(fun_modifier: FunModifier, ident: &String, trait_ident: &Option<String>, typ: &Type, pos: Pos, errs: &mut Vec<FrontendError>)
{
    if fun_modifier == FunModifier::Kernel {
        match trait_ident {
            Some(trait_ident) => {
                let are_only_type_params_with_trait = typ.type_param_entries().iter().all(|tpe| {
                        let type_param_entry_r = tpe.borrow();
                        type_param_entry_r.trait_names.contains(&TraitName::Name(trait_ident.clone()))
                });
                if !are_only_type_params_with_trait {
                    errs.push(FrontendError::Message(pos, format!("kernel {} mustn't have type parameters without trait {}", ident, trait_ident)));
                }
            },
            None => {
                if !typ.type_param_entries().is_empty() {
                    errs.push(FrontendError::Message(pos, format!("kernel {} mustn't have type parameters", ident)));
                }
            },
        }
    }
}

pub struct Limiter
{}

impl Limiter
{
    pub fn new() -> Self
    { Limiter {} }

    pub fn check_limits(&self, tree: &Tree) -> FrontendResultWithErrors<()>
    {
        let mut errs: Vec<FrontendError> = Vec::new();
        self.check_limits_for_defs(tree, &mut errs)?;
        if errs.is_empty() {
            Ok(())
        } else {
            Err(FrontendErrors::new(errs))
        }
    }

    fn check_limits_for_defs(&self, tree: &Tree, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for def in tree.defs() {
            match &**def {
                Def::Var(ident, var, pos) => {
                    let var_r = var.borrow();
                    self.check_limits_for_var(ident, &*var_r, pos.clone(), errs)?;
                },
                Def::Trait(_, trait1, _) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, trait_defs, _) => {
                            for trait_def in trait_defs {
                                match &**trait_def {
                                    TraitDef(ident, var, pos) => {
                                        let var_r = var.borrow();
                                        self.check_limits_for_var(ident, &*var_r, pos.clone(), errs)?;
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
                                        self.check_limits_for_impl_var(&*impl_var_r, errs)?;
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
    
    fn check_limits_for_named_field_pairs<T, F>(&self, named_field_pairs: &[NamedFieldPair<T>], is_in_var: bool, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, bool, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        for named_field_pair in named_field_pairs {
            match named_field_pair {
                NamedFieldPair(_, other, _) => f(self, &**other, is_in_var, errs)?,
            }
        }
        Ok(())
    }

    fn check_limits_for_var(&self, ident: &String, var: &Var, pos: Pos, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match var {
            Var::Builtin(_, _) => (),
            Var::Var(var_modifier, _, _, Some(expr), _, _, _, _, _) => {
                check_global_var_modifier(*var_modifier, ident, pos, errs);
                self.check_limits_for_expr(&**expr, true, errs)?;
            }
            Var::Var(var_modifier, _, _, None, _, _, _, _, _) => check_global_var_modifier(*var_modifier, ident, pos, errs),
            Var::Fun(fun, trait_name, Some(typ)) => {
                match &**fun {
                    Fun::Fun(fun_modifier, _, _, _, Some(body), _, _) => {
                        check_fun_modifier(*fun_modifier, ident, trait_name, &**typ, pos, errs);
                        self.check_limits_for_expr(&**body, false, errs)?;
                    },
                    Fun::Fun(fun_modifier, _, _, _, None, _, _) => check_fun_modifier(*fun_modifier, ident, trait_name, &**typ, pos, errs),
                    Fun::Con(_) => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_limits_for_var: variable is contructor"))])),
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_limits_for_var: no type"))])),
        }
        Ok(())
    }
    
    fn check_limits_for_expr(&self, expr: &Expr, is_in_var: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.check_limits_for_literal(&**literal, is_in_var, errs, Self::check_limits_for_expr)?,
            Expr::Lambda(_, _, body, _, _, _, _, _) => self.check_limits_for_expr(&**body, false, errs)?,
            Expr::Var(_, _, _) => (),
            Expr::NamedFieldConApp(_, expr_named_field_pairs, _, _, _) => self.check_limits_for_named_field_pairs(expr_named_field_pairs.as_slice(), is_in_var, errs, Self::check_limits_for_expr)?,
            Expr::PrintfApp(exprs, _, pos) => {
                match exprs.first() {
                    Some(expr2) => {
                        match &**expr2 {
                            Expr::Literal(_, _, _) => (),
                            _ => errs.push(FrontendError::Message(pos.clone(), String::from("printf takes first argument that must be literal"))),
                        }
                    },
                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_limits_for_expr: no frist expression"))]))
                }
                for expr2 in exprs {
                    self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
                for expr3 in exprs {
                    self.check_limits_for_expr(&**expr3, is_in_var, errs)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.check_limits_for_expr(&**expr2, is_in_var, errs)?,
            Expr::Get2Field(expr2, _, _, _) => self.check_limits_for_expr(&**expr2, is_in_var, errs)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
                self.check_limits_for_expr(&**expr3, is_in_var, errs)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
                self.check_limits_for_expr(&**expr3, is_in_var, errs)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
                self.check_limits_for_expr(&**expr3, is_in_var, errs)?;
            },
            Expr::Uniq(expr2, _, _) => self.check_limits_for_expr(&**expr2, is_in_var, errs)?,
            Expr::Shared(expr2, _, _) => self.check_limits_for_expr(&**expr2, is_in_var, errs)?,
            Expr::Typed(expr2, _, _, _) => self.check_limits_for_expr(&**expr2, is_in_var, errs)?,
            Expr::As(expr2, _, _, _) => self.check_limits_for_expr(&**expr2, is_in_var, errs)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
                self.check_limits_for_expr(&**expr3, is_in_var, errs)?;
                self.check_limits_for_expr(&**expr4, is_in_var, errs)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.check_limits_for_expr(&**expr3, is_in_var, errs)?;
                            self.check_limits_for_pattern(&**pattern, is_in_var, errs)?;
                        },
                    }
                }
                self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
            },
            Expr::Match(expr2, cases, _, _) => {
                self.check_limits_for_expr(&**expr2, is_in_var, errs)?;
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            self.check_limits_for_pattern(&**pattern, is_in_var, errs)?;
                            self.check_limits_for_expr(&**expr3, is_in_var, errs)?;
                        },
                    }
                }
            },
        }
        Ok(())
    }

    fn check_limits_for_pattern(&self, pattern: &Pattern, is_in_var: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, _, _) => self.check_limits_for_literal(&**literal, is_in_var, errs, Self::check_limits_for_pattern)?,
            Pattern::As(_, _, _, _, _) => (),
            Pattern::Const(_, _, _) => (),
            Pattern::UnnamedFieldCon(_, patterns, _, _, _) => {
                for pattern2 in patterns {
                    self.check_limits_for_pattern(&**pattern2, is_in_var, errs)?;
                }
            },
            Pattern::NamedFieldCon(_, pattern_named_field_pairs, _, _, _) => self.check_limits_for_named_field_pairs(pattern_named_field_pairs.as_slice(), is_in_var, errs, Self::check_limits_for_pattern)?,
            Pattern::Var(var_modifier, ident, _, pos) => check_local_var_modifier(*var_modifier, ident, pos.clone(), is_in_var, errs),
            Pattern::At(var_modifier, ident, pattern2, _, pos) => {
                check_local_var_modifier(*var_modifier, ident, pos.clone(), is_in_var, errs);
                self.check_limits_for_pattern(&**pattern2, is_in_var, errs)?;
            },
            Pattern::Wildcard(_, _) => (),
            Pattern::Alt(_, _, _) => (),
        }
        Ok(())
    }
    
    fn check_limits_for_literal<T, F>(&self, literal: &Literal<T>, is_in_var: bool, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &T, bool, &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>,
    {
        match literal {
            Literal::Tuple(field_others) => {
                for field_other in field_others {
                    f(self, &**field_other, is_in_var, errs)?
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, &**elem_other, is_in_var, errs)?
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, &**elem_other, is_in_var, errs)?,
            _ => (),
        }
        Ok(())
    }


    fn check_limits_for_impl_var(&self, impl_var: &ImplVar, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match impl_var {
            ImplVar::Builtin(_) => (),
            ImplVar::Var(expr, _, _, _, _) => self.check_limits_for_expr(&**expr, true, errs)?,
            ImplVar::Fun(impl_fun, _) => {
                match &**impl_fun {
                    ImplFun(_, body, _, _) => self.check_limits_for_expr(&**body, false, errs)?,
                }
            },
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
