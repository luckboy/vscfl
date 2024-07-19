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

fn pattern_local_type(pattern: &Pattern) -> FrontendResultWithErrors<LocalType>
{
    match pattern {
        Pattern::Literal(_, Some(local_type), _) => Ok(*local_type),
        Pattern::As(_, _, _, Some(local_type), _) => Ok(*local_type),
        Pattern::Const(_, Some(local_type), _) => Ok(*local_type),
        Pattern::UnnamedFieldCon(_, _, _, Some(local_type), _) => Ok(*local_type),
        Pattern::NamedFieldCon(_, _, _, Some(local_type), _) => Ok(*local_type),
        Pattern::Var(_, _, Some(local_type), _) => Ok(*local_type),
        Pattern::At(_, _, _, Some(local_type), _) => Ok(*local_type),
        Pattern::Wildcard(Some(local_type), _) => Ok(*local_type),
        Pattern::Alt(_, Some(local_type), _) => Ok(*local_type),
        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("pattern_local_type: no local type"))])),
    }
}

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
                                                            match impl_vars.var(ident) {
                                                                Some(impl_var) => {
                                                                    let impl_var_r = impl_var.borrow();
                                                                    match &*impl_var_r {
                                                                        ImplVar::Builtin(_) => return Ok(()),
                                                                        ImplVar::Var(_, _, _, _, _) => Some(type_name.clone()),
                                                                        _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_var_key: implementation variable is function"))])),
                                                                    }
                                                                },
                                                                None => None,
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

fn named_fields_for_con_ident_in<T, F>(ident: &String, tree: &Tree, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&NamedFields) -> FrontendResultWithErrors<T>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, _) => {
                    match &**fun {
                        Fun::Con(con) => {
                            let con_r = con.borrow();
                            match &*con_r {
                                Con::NamedField(_, _, _, Some(named_fields), _) => f(&**named_fields),
                                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("named_fields_for_con_ident_in: constructor isn't named field contructor or no named fields"))])),
                            }
                        },
                        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("named_fields_for_con_ident_in: function isn't contructor"))])),
                    }
                },
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("named_fields_for_con_ident_in: variable isn't function or no type"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("named_fields_for_con_ident_in: no variable"))])),
    }
}

fn pattern_max_for_local_type(local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<Option<usize>>
{
    Ok(None)
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum PatternId
{
    Bool(bool),
    Char(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Uchar(u8),
    Ushort(u16),
    Uint(u32),
    Ulong(u64),
    Half(u32),
    Float(u32),
    Double(u64),
    SizeT(u64),
    PtrdiffT(i64),
    IntptrT(i64),
    UintptrT(u64),
    CharN(Vec<i8>),
    ShortN(Vec<i16>),
    IntN(Vec<i32>),
    LongN(Vec<i64>),
    UcharN(Vec<u8>),
    UshortN(Vec<u16>),
    UintN(Vec<u32>),
    UlongN(Vec<u64>),
    FloatN(Vec<u32>),
    DoubleN(Vec<u64>),
    String(Vec<u8>),
    Tuple(usize),
    Array(usize),
    Data(String),
}

pub struct Evaluator
{
    evals: Evals,
}

impl Evaluator
{
    pub fn new() -> Self
    { Evaluator { evals: Evals::new(), } }

    fn value_for_ident_and_type_name(&self, ident: &String, type_name: &Option<TypeName>, pos: Pos, tree: &Tree, are_errs: bool, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<Value>>
    {
        match tree.var(ident) {
            Some(var) => {
                let var_r = var.borrow();
                let (trait_ident, value) = match &*var_r {
                    Var::Builtin(tmp_trait_ident, _) => {
                        let tmp_value = match self.evals.fun(&(ident.clone(), None)) {
                            Some(fun) => Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::EvalFun(ident.clone(), None, fun))))),
                            None => Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Builtin(ident.clone(), None))))),
                        };
                        (tmp_trait_ident, tmp_value)
                    },
                    Var::Var(_, _, _, _, tmp_trait_ident, _, _, _, Some(tmp_value)) => (tmp_trait_ident, Some(tmp_value.clone())),
                    Var::Var(_, _, _, _, tmp_trait_ident, _, _, _, None) => (tmp_trait_ident, None),
                    Var::Fun(_, tmp_trait_ident, _) => (tmp_trait_ident, Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Fun(ident.clone(), None)))))),
                };
                let value3 = match type_name {
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
                                                        match impl_vars.var(ident) {
                                                            Some(impl_var) => {
                                                                let impl_var_r = impl_var.borrow();
                                                                match &*impl_var_r {
                                                                    ImplVar::Builtin(_) => {
                                                                        match self.evals.fun(&(ident.clone(), Some(type_name.clone()))) {
                                                                            Some(fun) => Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::EvalFun(ident.clone(), Some(type_name.clone()), fun))))),
                                                                            None => Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Builtin(ident.clone(), Some(type_name.clone())))))),
                                                                        }
                                                                    },
                                                                    ImplVar::Var(_, _, _, _, Some(value2)) => Some(value2.clone()),
                                                                    ImplVar::Var(_, _, _, _, None) => None,
                                                                    ImplVar::Fun(_, _) => Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Fun(ident.clone(), None))))),
                                                                }
                                                            },
                                                            None => value,
                                                        }
                                                    },
                                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name_in: no implementation"))])),
                                                }
                                            },
                                            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name_in: no trait variables"))])),
                                        }
                                    },
                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name_in: no trait"))])),
                                }
                            },
                            None => value,
                        }
                    },
                    None => value,
                };
                if value3.is_none() {
                    if are_errs  {
                        errs.push(FrontendError::Message(pos, format!("unevaluated variable {}", ident)));
                    }
                }
                Ok(value3)
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name_in: no variable"))])),
        }
    }
    
    fn normalize_pattern_forest(&self, forest: &mut PatternForest<PatternId>) -> FrontendResultWithErrors<()>
    {
        match forest.normalize() {
            Ok(()) => Ok(()),
            Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("normalize_pattern_forest: {}", err))])),
        }
    }
    
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
            Expr::NamedFieldConApp(_, expr_named_field_pairs, _, _, _) => {
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
                let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                add_var_key(ident, &type_name, pos.clone(), tree, keys, processed_keys, errs)?;
            },
            Pattern::UnnamedFieldCon(_, patterns, _, _, _) => {
                for pattern2 in patterns {
                    self.add_var_keys_for_pattern(&**pattern2, tree, var_env, type_stack, local_types, keys, processed_keys, errs)?;
                }
            },
            Pattern::NamedFieldCon(_, pattern_named_field_pairs, _, _, _) => {
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

    fn has_one_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<bool>
    {
        Ok(false)
    }

    fn convert_pattern_ids_for_type_value(&self, node: &mut PatternNode<PatternId>, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<()>
    {
        Ok(())
    }
    
    fn add_pattern_node_for_value(&self, value: &Value, forest: &mut PatternForest<PatternId>)
    {}
    
    fn check_pattern_exhaustions_for_expr(&self, expr: &Expr, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.do_literal(&**literal, errs, |evaluator, expr, errs| evaluator.check_pattern_exhaustions_for_expr(expr, tree, type_stack, local_types, errs))?,
            Expr::Lambda(_, _, body, _, _, _, _, _) => self.check_pattern_exhaustions_for_expr(&**body, tree, type_stack, local_types, errs)?,
            Expr::Var(_, _, _) => (),
            Expr::NamedFieldConApp(_, expr_named_field_pairs, _, _, _) => {
                self.do_named_field_pairs(expr_named_field_pairs.as_slice(), errs, |evaluator, expr, errs| evaluator.check_pattern_exhaustions_for_expr(expr, tree, type_stack, local_types, errs))?
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
                for expr3 in exprs {
                    self.check_pattern_exhaustions_for_expr(&**expr3, tree, type_stack, local_types, errs)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?,
            Expr::Get2Field(expr2, _, _, _) => self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
                self.check_pattern_exhaustions_for_expr(&**expr3, tree, type_stack, local_types, errs)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
                self.check_pattern_exhaustions_for_expr(&**expr3, tree, type_stack, local_types, errs)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
                self.check_pattern_exhaustions_for_expr(&**expr3, tree, type_stack, local_types, errs)?;
            },
            Expr::Uniq(expr2, _, _) => self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?,
            Expr::Shared(expr2, _, _) => self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?,
            Expr::Typed(expr2, _, _, _) => self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?,
            Expr::As(expr2, _, _, _) => self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
                self.check_pattern_exhaustions_for_expr(&**expr3, tree, type_stack, local_types, errs)?;
                self.check_pattern_exhaustions_for_expr(&**expr4, tree, type_stack, local_types, errs)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            self.check_pattern_exhaustions_for_expr(&**expr3, tree, type_stack, local_types, errs)?;
                            let mut forest: PatternForest<PatternId> = PatternForest::Alt(Vec::new(), pattern_max_for_local_type(pattern_local_type(&pattern)?, tree, local_types)?);
                            self.add_pattern_nodes_for_pattern(&**pattern, tree, type_stack, local_types, &mut forest, errs)?;
                            self.normalize_pattern_forest(&mut forest)?;
                            match forest {
                                PatternForest::All(_) => (),
                                _ => errs.push(FrontendError::Message(pattern_pos(&**pattern).clone(), String::from("non-exhaustive patterns"))),
                            }
                        },
                    }
                }
                self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
            },
            Expr::Match(expr2, cases, Some(local_type), pos) => {
                self.check_pattern_exhaustions_for_expr(&**expr2, tree, type_stack, local_types, errs)?;
                let mut forest: PatternForest<PatternId> = PatternForest::Alt(Vec::new(), pattern_max_for_local_type(*local_type, tree, local_types)?);
                for case in cases {
                    match case {
                        Case(pattern, expr3) => {
                            self.add_pattern_nodes_for_pattern(&**pattern, tree, type_stack, local_types, &mut forest, errs)?;
                            self.check_pattern_exhaustions_for_expr(&**expr3, tree, type_stack, local_types, errs)?;
                        },
                    }
                }
                self.normalize_pattern_forest(&mut forest)?;
                match forest {
                    PatternForest::All(_) => (),
                    _ => errs.push(FrontendError::Message(pos.clone(), String::from("non-exhaustive patterns"))),
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("check_pattern_exhaustions_for_expr: no local type"))])),
        }
        Ok(())
    }

    fn add_pattern_nodes_for_pattern(&self, pattern: &Pattern, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes, forest: &mut PatternForest<PatternId>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match pattern {
            Pattern::Literal(literal, _, _) => self.add_pattern_nodes_for_pattern_literal(&**literal, tree, type_stack, local_types, forest, errs)?,
            Pattern::As(literal, _, _, Some(local_type), _) => {
                self.add_pattern_nodes_for_pattern_literal(&**literal, tree, type_stack, local_types, forest, errs)?;
                match forest {
                    PatternForest::Alt(nodes, _) => {
                        match nodes.last() {
                            Some(node) => {
                                let mut node_r = node.borrow_mut();
                                self.convert_pattern_ids_for_type_value(&mut *node_r, &Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), tree, local_types)?; 
                            },
                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_pattern_nodes_for_pattern: no last pattern node"))])),
                        }
                    },
                    _ => (),
                }
            },
            Pattern::Const(ident, Some(local_type), pos) => {
                let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                match self.value_for_ident_and_type_name(ident, &type_name, pos.clone(), tree, true, errs)? {
                    Some(value) => self.add_pattern_node_for_value(&value, forest),
                    None => (),
                }
            },
            Pattern::UnnamedFieldCon(ident, patterns, _, _, _) => {
                let mut forests: Vec<PatternForest<PatternId>> = Vec::new();
                for pattern2 in patterns {
                    let mut forest2: PatternForest<PatternId> = PatternForest::Alt(Vec::new(), pattern_max_for_local_type(pattern_local_type(&**pattern2)?, tree, local_types)?);
                    self.add_pattern_nodes_for_pattern(&**pattern2, tree, type_stack, local_types, &mut forest2, errs)?;
                    forests.push(forest2);
                }
                forest.add_node(PatternNode::new(PatternId::Data(ident.clone()), PatternForests::Unfilled(forests)));
            },
            Pattern::NamedFieldCon(ident, pattern_named_field_pairs, _, _, _) => {
                named_fields_for_con_ident_in(ident, tree, |named_fields| {
                        let mut forests: Vec<PatternForest<PatternId>> = vec![PatternForest::Alt(Vec::new(), None); pattern_named_field_pairs.len()];
                        for pattern_named_field_pair in pattern_named_field_pairs {
                            match pattern_named_field_pair {
                                NamedFieldPair(field_ident, pattern2, _) => {
                                    match named_fields.field_index(field_ident) {
                                        Some(field_idx) => {
                                            let mut forest2: PatternForest<PatternId> = PatternForest::Alt(Vec::new(), pattern_max_for_local_type(pattern_local_type(&**pattern2)?, tree, local_types)?);
                                            self.add_pattern_nodes_for_pattern(&**pattern2, tree, type_stack, local_types, &mut forest2, errs)?;
                                            forests[field_idx] = forest2;
                                        },
                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_pattern_nodes_for_pattern: no field index"))])),
                                    }
                                },
                            }
                        }
                        forest.add_node(PatternNode::new(PatternId::Data(ident.clone()), PatternForests::Unfilled(forests)));
                        Ok(())
                })?;
            },
            Pattern::Var(_, _, Some(local_type), _) => forest.set_all(self.has_one_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), tree, local_types)?),
            Pattern::At(_, _, pattern2, _, _) => self.add_pattern_nodes_for_pattern(&**pattern2, tree, type_stack, local_types, forest, errs)?,
            Pattern::Wildcard(Some(local_type), _) => forest.set_all(self.has_one_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), tree, local_types)?),
            Pattern::Alt(patterns, _, _) => {
                for pattern2 in patterns {
                    self.add_pattern_nodes_for_pattern(&**pattern2, tree, type_stack, local_types, forest, errs)?;
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_pattern_nodes_for_pattern: no local type"))])),
        }
        Ok(())
    }

    fn add_pattern_nodes_for_pattern_literal(&self, literal: &Literal<Pattern>, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes, forest: &mut PatternForest<PatternId>, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
    {
        match literal {
            Literal::Bool(b) => {
                forest.add_node(PatternNode::new(PatternId::Bool(*b), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Char(c) => {
                forest.add_node(PatternNode::new(PatternId::Char(*c), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Int(n) => {
                forest.add_node(PatternNode::new(PatternId::Int(*n), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Long(n) => {
                forest.add_node(PatternNode::new(PatternId::Long(*n), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Uint(n) => {
                forest.add_node(PatternNode::new(PatternId::Uint(*n), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Ulong(n) => {
                forest.add_node(PatternNode::new(PatternId::Ulong(*n), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Float(n) => {
                forest.add_node(PatternNode::new(PatternId::Float(n.to_bits()), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Double(n) => {
                forest.add_node(PatternNode::new(PatternId::Double(n.to_bits()), PatternForests::Unfilled(Vec::new())));
            },
            Literal::String(bs) => {
                forest.add_node(PatternNode::new(PatternId::String(bs.clone()), PatternForests::Unfilled(Vec::new())));
            },
            Literal::Tuple(field_patterns) => {
                let mut forests: Vec<PatternForest<PatternId>> = Vec::new();
                for field_pattern in field_patterns {
                    let mut forest2: PatternForest<PatternId> = PatternForest::Alt(Vec::new(), pattern_max_for_local_type(pattern_local_type(&**field_pattern)?, tree, local_types)?);
                    self.add_pattern_nodes_for_pattern(&**field_pattern, tree, type_stack, local_types, &mut forest2, errs)?;
                    forests.push(forest2);
                }
                forest.add_node(PatternNode::new(PatternId::Tuple(field_patterns.len()), PatternForests::Unfilled(forests)));
            },
            Literal::Array(elem_patterns) => {
                let mut forests: Vec<PatternForest<PatternId>> = Vec::new();
                for elem_pattern in elem_patterns {
                    let mut forest2: PatternForest<PatternId> = PatternForest::Alt(Vec::new(), pattern_max_for_local_type(pattern_local_type(&**elem_pattern)?, tree, local_types)?);
                    self.add_pattern_nodes_for_pattern(&**elem_pattern, tree, type_stack, local_types, &mut forest2, errs)?;
                    forests.push(forest2);
                }
                forest.add_node(PatternNode::new(PatternId::Array(elem_patterns.len()), PatternForests::Unfilled(forests)));
            },
            Literal::FilledArray(elem_pattern, len) => {
                let mut forest2: PatternForest<PatternId> = PatternForest::Alt(Vec::new(), pattern_max_for_local_type(pattern_local_type(&**elem_pattern)?, tree, local_types)?);
                self.add_pattern_nodes_for_pattern(&**elem_pattern, tree, type_stack, local_types, &mut forest2, errs)?;
                forest.add_node(PatternNode::new(PatternId::Array(*len), PatternForests::Filled(forest2, *len)));
            },
        }
        Ok(())
    }
}
