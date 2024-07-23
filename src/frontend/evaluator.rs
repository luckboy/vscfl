//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeMap;
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

fn expr_local_type(expr: &Expr) -> FrontendResultWithErrors<LocalType>
{
    match expr {
        Expr::Literal(_, Some(local_type), _) => Ok(*local_type),
        Expr::Lambda(_, _, _, _, Some(local_type), _, _, _) => Ok(*local_type),
        Expr::Var(_, Some(local_type), _) => Ok(*local_type),
        Expr::NamedFieldConApp(_, _, _, Some(local_type), _) => Ok(*local_type),
        Expr::PrintfApp(_, Some(local_type), _) => Ok(*local_type),
        Expr::App(_, _, Some(local_type), _) => Ok(*local_type),
        Expr::GetField(_, _, Some(local_type), _) => Ok(*local_type),
        Expr::Get2Field(_, _, Some(local_type), _) => Ok(*local_type),
        Expr::SetField(_, _, _, Some(local_type), _) => Ok(*local_type),
        Expr::UpdateField(_, _, _, Some(local_type), _) => Ok(*local_type),
        Expr::UpdateGet2Field(_, _, _, Some(local_type), _) => Ok(*local_type),
        Expr::Uniq(_, Some(local_type), _) => Ok(*local_type),
        Expr::Shared(_, Some(local_type), _) => Ok(*local_type),
        Expr::Typed(_, _, Some(local_type), _) => Ok(*local_type),
        Expr::As(_, _, Some(local_type), _) => Ok(*local_type),
        Expr::Let(_, _, Some(local_type), _) => Ok(*local_type),
        Expr::If(_, _, _, Some(local_type), _) => Ok(*local_type),
        Expr::Match(_, _, Some(local_type), _) => Ok(*local_type),
        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("expr_local_type: no local type"))])),
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

fn shared_flag_for_local_type(local_type: LocalType, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes) -> FrontendResultWithErrors<SharedFlag>
{
    match type_stack.push_type_entries_for_local_type(local_type, local_types) {
        Ok(new_local_type) => {
            match type_stack.shared_flag_for_local_type(new_local_type, tree) {
                Ok(shared_flag) => {
                    type_stack.pop_type_entries();
                    Ok(shared_flag)
                },
                Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("{}", err))])),
            }
        },
        Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("{}", err))])),
    }
}

fn type_for_ident_and_type_name_in<T, F>(ident: &String, type_name: &Option<TypeName>, tree: &Tree, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&Type) -> FrontendResultWithErrors<T>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            let (trait_ident, typ) = match &*var_r {
                Var::Builtin(tmp_trait_ident, Some(tmp_type)) => (tmp_trait_ident, tmp_type),
                Var::Var(_, _, _, _, tmp_trait_ident, _, _, Some(tmp_type), _) => (tmp_trait_ident, tmp_type),
                Var::Fun(_, tmp_trait_ident, Some(tmp_type)) => (tmp_trait_ident, tmp_type),
                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_ident_and_type_name_in: no type"))])),
            };
            match type_name {
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
                                                                ImplVar::Builtin(Some(type2)) => f(type2),
                                                                ImplVar::Var(_, _, _, Some(type2), _) => f(type2),
                                                                ImplVar::Fun(_, Some(type2)) => f(type2),
                                                                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_ident_and_type_name_in: no type"))])),
                                                            }
                                                        },
                                                        None => f(typ),
                                                    }
                                                },
                                                None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_ident_and_type_name_in: no implementation"))])),
                                            }
                                        },
                                        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_ident_and_type_name_in: no trait variables"))])),
                                    }
                                },
                                None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_ident_and_type_name_in: no trait"))])),
                            }
                        },
                        None => f(typ),
                    }
                },
                None => f(typ),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_ident_and_type_name_in: no variable"))])),
    }
}


fn type_for_fun_ident_in<T, F>(ident: &String, tree: &Tree, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&Type) -> FrontendResultWithErrors<T>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(_, _, Some(typ)) => f(typ),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_fun_ident_in: variable isn't function or no type"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_fun_ident_in: no variable"))])),
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

fn pattern_max_for_type_ident(ident: &String, tree: &Tree) -> FrontendResultWithErrors<Option<usize>>
{
    match tree.type_var(ident) {
        Some(type_var) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(_, _, _) => {
                    if ident == &String::from("Bool") {
                        Ok(Some(2))
                    } else if ident == &String::from("Char") || ident == &String::from("Uchar") {
                        Ok(Some(((u8::MAX as u64) + 1) as usize))
                    } else if ident == &String::from("Short") || ident == &String::from("Ushort") {
                        if (u16::MAX as u64) < (usize::MAX as u64) {
                            Ok(Some(((u16::MAX as u64) + 1) as usize))
                        } else {
                            Ok(None)
                        }
                    } else if ident == &String::from("Int") || ident == &String::from("Uint")  || ident == &String::from("Half") || ident == &String::from("Float") {
                        if (u32::MAX as u64) < (usize::MAX as u64) {
                            Ok(Some(((u32::MAX as u64) + 1) as usize))
                        } else {
                            Ok(None)
                        }
                    } else {
                        Ok(None)
                    }
                },
                TypeVar::Data(_, cons, _) => Ok(Some(cons.len())),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("pattern_max_for_type_ident: type variable is type synonym"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("pattern_max_for_type_ident: no type variable"))])),
    }
}

fn pattern_max_for_local_type(local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<Option<usize>>
{
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type))) {
        Some(LocalTypeEntry::Param(_, _, _, _)) => Ok(None),
        Some(LocalTypeEntry::Type(type_value)) => {
            match &*type_value {
                TypeValue::Param(_, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("pattern_max_for_local_type: type parameter in local type entry"))])),
                TypeValue::Type(_, TypeValueName::Tuple | TypeValueName::Array(_), _) => Ok(Some(1)),
                TypeValue::Type(_, TypeValueName::Fun, _) => Ok(None),
                TypeValue::Type(_, TypeValueName::Name(ident), _) => pattern_max_for_type_ident(ident, tree),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("pattern_max_for_local_type: no local type entry"))])),
    }
}

fn pattern_max_for_type(typ: &Type, tree: &Tree) -> FrontendResultWithErrors<Option<usize>>
{
    match &**typ.type_value() {
        TypeValue::Param(_, _) => Ok(None),
        TypeValue::Type(_, TypeValueName::Tuple | TypeValueName::Array(_), _) => Ok(Some(1)),
        TypeValue::Type(_, TypeValueName::Fun, _) => Ok(None),
        TypeValue::Type(_, TypeValueName::Name(ident), _) => pattern_max_for_type_ident(ident, tree),
    }
}

fn add_error_for_object(object: &Object, pos: Pos, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<()>
{
    match object {
        Object::Builtin(_, _) => {
            errs.push(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector")));
            Ok(())
        },
        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_error_for_object: invalid object"))])),
    }
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

#[derive(Clone, Debug)]
enum PatternObject
{
    String(Vec<u8>),
    CharN(Vec<i8>),
    ShortN(Vec<i16>),
    IntN(Vec<i32>),
    LongN(Vec<i64>),
    UcharN(Vec<u8>),
    UshortN(Vec<u16>),
    UintN(Vec<u32>),
    UlongN(Vec<u64>),
    FloatN(Vec<f32>),
    DoubleN(Vec<f64>),
    Tuple(Vec<PatternValue>),
    Array(Vec<PatternValue>),
    Data(String, Vec<PatternValue>),
    Var(String),
    At(String, PatternValue),
    Alt(Vec<PatternValue>),
}

#[derive(Clone, Debug)]
enum PatternValue
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
    Float(f32),
    Double(f64),
    SizeT(u64),
    PtrdiffT(i64),
    IntptrT(i64),
    UintptrT(u64),
    Wildcard,
    Object(Rc<RefCell<PatternObject>>),
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
                    Var::Fun(fun, tmp_trait_ident, _) => {
                        let tmp_value = match &**fun {
                            Fun::Fun(_, _, _, _, _, _, _) => Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Fun(ident.clone(), None))))),
                            Fun::Con(_) => Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Con(ident.clone()))))),
                        };
                        (tmp_trait_ident, tmp_value)
                    },
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
                                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name: no implementation"))])),
                                                }
                                            },
                                            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name: no trait variables"))])),
                                        }
                                    },
                                    None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name: no trait"))])),
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
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_ident_and_type_name: no variable"))])),
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
                    f(self, &**field_other, errs)?;
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, &**elem_other, errs)?;
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, &**elem_other, errs)?,
            _ => (),
        }
        Ok(())
    }

    fn do_named_field_pairs_mut_for_setting<T, F>(&self, named_field_pairs: &mut [NamedFieldPair<T>], mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &mut T) -> FrontendResultWithErrors<()>
    {
        for named_field_pair in named_field_pairs {
            match named_field_pair {
                NamedFieldPair(_, other, _) => f(self, other)?,
            }
        }
        Ok(())
    }    

    fn do_literal_mut_for_setting<T, F>(&self, literal: &mut Literal<T>, mut f: F) -> FrontendResultWithErrors<()>
        where F: FnMut(&Self, &mut T) -> FrontendResultWithErrors<()>,
    {
        match literal {
            Literal::Tuple(field_others) => {
                for field_other in field_others {
                    f(self, &mut **field_other)?;
                }
            },
            Literal::Array(elem_others) => {
                for elem_other in elem_others {
                    f(self, &mut **elem_other)?;
                }
            },
            Literal::FilledArray(elem_other, _) => f(self, &mut **elem_other)?,
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
                self.do_named_field_pairs(pattern_named_field_pairs.as_slice(), errs, |evaluator, pattern, errs| evaluator.add_var_keys_for_pattern(pattern, tree, var_env, type_stack, local_types, keys, processed_keys, errs))?;
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
        match local_types.type_entry_for_type_value(type_value) {
            Some(LocalTypeEntry::Param(_, _, _, _)) => Ok(false),
            Some(LocalTypeEntry::Type(type_value)) => {
                match &*type_value {
                    TypeValue::Param(_, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_one_for_type_value: type parameter in local type entry"))])),
                    TypeValue::Type(_, TypeValueName::Tuple | TypeValueName::Array(_), type_values) => {
                        let mut is_one = true;
                        for type_value2 in type_values {
                            is_one &= self.has_one_for_type_value(type_value2, tree, local_types)?;
                        }
                        Ok(is_one)
                    },
                    TypeValue::Type(_, TypeValueName::Fun, _) => Ok(false),
                    TypeValue::Type(_, TypeValueName::Name(ident), type_values) => {
                        match tree.type_var(ident) {
                            Some(type_var) => {
                                let type_var_r = type_var.borrow();
                                match &*type_var_r {
                                    TypeVar::Builtin(_, _, _) => Ok(false),
                                    TypeVar::Data(_, cons, _) => {
                                        if cons.len() == 0 {
                                            Ok(true)
                                        } else if cons.len() == 1 {
                                            let con_r = cons[0].borrow();
                                            let con_ident = match &*con_r {
                                                Con::UnnamedField(tmp_con_ident, _, _, _) => tmp_con_ident,
                                                Con::NamedField(tmp_con_ident, _, _, _, _) => tmp_con_ident,
                                            };
                                            type_for_fun_ident_in(con_ident, tree, |typ| {
                                                    match &**typ.type_value() {
                                                        TypeValue::Type(_, TypeValueName::Fun, type_values2) => {
                                                            if type_values2.len() >= 1 {
                                                                let mut is_one = true;
                                                                for type_value2 in type_values2 {
                                                                    let new_type_value = match type_value2.substitute(type_values) {
                                                                        Ok(Some(tmp_type_value)) => tmp_type_value,
                                                                        Ok(None) => type_value2.clone(),
                                                                        Err(err) => return Err(FrontendErrors::new(vec![FrontendError::Internal(format!("has_one_for_type_value: {}", err))])),
                                                                    };
                                                                    is_one &= self.has_one_for_type_value(&new_type_value, tree, local_types)?;
                                                                }
                                                                Ok(is_one)
                                                            } else {
                                                                Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_one_for_type_value: too few argument type values"))]))
                                                            }
                                                        },
                                                        _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_one_for_type_value: type value isn't function type"))]))
                                                    }
                                            })?;
                                            Ok(true)
                                        } else {
                                            Ok(false)
                                        }
                                    },
                                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_one_for_type_value: type variable is type synonym"))])),
                                }
                            },
                            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_one_for_type_value: no type variable"))])),
                        }
                    },
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_one_for_type_value: no local type entry"))])),
        }
    }

    fn convert_pattern_ids_for_type_value(&self, node: &mut PatternNode<PatternId>, max: &mut Option<usize>, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendResultWithErrors<()>
    {
        match local_types.type_entry_for_type_value(type_value) {
            Some(LocalTypeEntry::Param(_, _, _, _)) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: local type entry is type parameter"))])),
            Some(LocalTypeEntry::Type(type_value)) => {
                match &*type_value {
                    TypeValue::Param(_, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("has_one_for_type_value: type parameter in local type entry"))])),
                    TypeValue::Type(_, TypeValueName::Tuple, type_values) => {
                        match node.forests_mut() {
                            PatternForests::Unfilled(forests) => {
                                for (forest, type_value2) in forests.iter_mut().zip(type_values.iter()) {
                                    match forest {
                                        PatternForest::Alt(nodes, max2) => {
                                            for node2 in nodes {
                                                let mut node2_r = node2.borrow_mut();
                                                self.convert_pattern_ids_for_type_value(&mut *node2_r, max2, type_value2, tree, local_types)?
                                            }
                                        },
                                        PatternForest::All(_) => (),
                                    }
                                }
                                Ok(())
                            },
                            _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: filled pattern forests"))])),
                        }
                    },
                    TypeValue::Type(_, TypeValueName::Array(_), type_values) => {
                        match type_values.first() {
                            Some(type_value2) => {
                                match node.forests_mut() {
                                    PatternForests::Unfilled(forests) => {
                                        for forest in forests {
                                            match forest {
                                                PatternForest::Alt(nodes, max2) => {
                                                    for node2 in nodes {
                                                        let mut node2_r = node2.borrow_mut();
                                                        self.convert_pattern_ids_for_type_value(&mut *node2_r, max2, type_value2, tree, local_types)?
                                                    }
                                                },
                                                PatternForest::All(_) => (),
                                            }
                                        }
                                        Ok(())
                                    },
                                    PatternForests::Filled(forest, _) => {
                                        match forest {
                                            PatternForest::Alt(nodes, max2) => {
                                                for node2 in nodes {
                                                    let mut node2_r = node2.borrow_mut();
                                                    self.convert_pattern_ids_for_type_value(&mut *node2_r, max2, type_value2, tree, local_types)?
                                                }
                                            },
                                            PatternForest::All(_) => (),
                                        }
                                        Ok(())
                                    },
                                }
                            },
                            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: no type value"))])),
                        }
                    },
                    TypeValue::Type(_, TypeValueName::Fun, _) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: type value is function type"))])),
                    TypeValue::Type(_, TypeValueName::Name(ident), _) => {
                        match tree.type_var(ident) {
                            Some(type_var) => {
                                *max = pattern_max_for_type_ident(ident, tree)?;                                
                                let type_var_r = type_var.borrow();
                                match &*type_var_r {
                                    TypeVar::Builtin(_, _, _) => {
                                        if ident == &String::from("Char") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Short(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Int(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Long(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::Half(n) => node.set_id(PatternId::Char(f32::from_bits(*n) as i8)),
                                                PatternId::Float(n) => node.set_id(PatternId::Char(f32::from_bits(*n) as i8)),
                                                PatternId::Double(n) => node.set_id(PatternId::Char(f64::from_bits(*n) as i8)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Char(*n as i8)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Char(*n as i8)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Short") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Short(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Int(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Long(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::Half(n) => node.set_id(PatternId::Short(f32::from_bits(*n) as i16)),
                                                PatternId::Float(n) => node.set_id(PatternId::Short(f32::from_bits(*n) as i16)),
                                                PatternId::Double(n) => node.set_id(PatternId::Short(f64::from_bits(*n) as i16)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Short(*n as i16)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Short(*n as i16)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Int") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Short(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Int(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Long(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::Half(n) => node.set_id(PatternId::Int(f32::from_bits(*n) as i32)),
                                                PatternId::Float(n) => node.set_id(PatternId::Int(f32::from_bits(*n) as i32)),
                                                PatternId::Double(n) => node.set_id(PatternId::Int(f64::from_bits(*n) as i32)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Int(*n as i32)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Int(*n as i32)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Long") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Short(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Int(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Long(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::Half(n) => node.set_id(PatternId::Long(f32::from_bits(*n) as i64)),
                                                PatternId::Float(n) => node.set_id(PatternId::Long(f32::from_bits(*n) as i64)),
                                                PatternId::Double(n) => node.set_id(PatternId::Long(f64::from_bits(*n) as i64)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Long(*n as i64)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Long(*n as i64)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Uchar") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Short(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Int(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Long(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::Half(n) => node.set_id(PatternId::Uchar(f32::from_bits(*n) as u8)),
                                                PatternId::Float(n) => node.set_id(PatternId::Uchar(f32::from_bits(*n) as u8)),
                                                PatternId::Double(n) => node.set_id(PatternId::Uchar(f64::from_bits(*n) as u8)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Uchar(*n as u8)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Ushort") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Short(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Int(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Long(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::Half(n) => node.set_id(PatternId::Ushort(f32::from_bits(*n) as u16)),
                                                PatternId::Float(n) => node.set_id(PatternId::Ushort(f32::from_bits(*n) as u16)),
                                                PatternId::Double(n) => node.set_id(PatternId::Ushort(f64::from_bits(*n) as u16)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Ushort(*n as u16)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Uint") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Short(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Int(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Long(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::Half(n) => node.set_id(PatternId::Uint(f32::from_bits(*n) as u32)),
                                                PatternId::Float(n) => node.set_id(PatternId::Uint(f32::from_bits(*n) as u32)),
                                                PatternId::Double(n) => node.set_id(PatternId::Uint(f64::from_bits(*n) as u32)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Uint(*n as u32)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Ulong") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Short(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Int(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Long(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Uint(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::Half(n) => node.set_id(PatternId::Ulong(f32::from_bits(*n) as u64)),
                                                PatternId::Float(n) => node.set_id(PatternId::Ulong(f32::from_bits(*n) as u64)),
                                                PatternId::Double(n) => node.set_id(PatternId::Ulong(f64::from_bits(*n) as u64)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Ulong(*n as u64)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Half") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Short(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Int(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Long(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Uint(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::Half(n) => node.set_id(PatternId::Half(*n)),
                                                PatternId::Float(n) => node.set_id(PatternId::Half(*n)),
                                                PatternId::Double(n) => node.set_id(PatternId::Half((f64::from_bits(*n) as f32).to_bits())),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Half((*n as f32).to_bits())),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Float") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Short(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Int(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Long(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Uint(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::Half(n) => node.set_id(PatternId::Float(*n)),
                                                PatternId::Float(n) => node.set_id(PatternId::Float(*n)),
                                                PatternId::Double(n) => node.set_id(PatternId::Float((f64::from_bits(*n) as f32).to_bits())),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Float((*n as f32).to_bits())),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("Double") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Short(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Int(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Long(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Uchar(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Ushort(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Uint(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Ulong(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::Half(n) => node.set_id(PatternId::Double((f32::from_bits(*n) as f64).to_bits())),
                                                PatternId::Float(n) => node.set_id(PatternId::Double((f32::from_bits(*n) as f64).to_bits())),
                                                PatternId::Double(n) => node.set_id(PatternId::Double(*n)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::Double((*n as f64).to_bits())),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("SizeT") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Short(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Int(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Long(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Uint(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::Half(n) => node.set_id(PatternId::SizeT(f32::from_bits(*n) as u64)),
                                                PatternId::Float(n) => node.set_id(PatternId::SizeT(f32::from_bits(*n) as u64)),
                                                PatternId::Double(n) => node.set_id(PatternId::SizeT(f64::from_bits(*n) as u64)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::SizeT(*n as u64)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("PtrdiffT") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Short(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Int(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Long(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Uint(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::Half(n) => node.set_id(PatternId::PtrdiffT(f32::from_bits(*n) as i64)),
                                                PatternId::Float(n) => node.set_id(PatternId::PtrdiffT(f32::from_bits(*n) as i64)),
                                                PatternId::Double(n) => node.set_id(PatternId::PtrdiffT(f64::from_bits(*n) as i64)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::PtrdiffT(*n as i64)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("IntptrT") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Short(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Int(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Long(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Uint(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::Half(n) => node.set_id(PatternId::IntptrT(f32::from_bits(*n) as i64)),
                                                PatternId::Float(n) => node.set_id(PatternId::IntptrT(f32::from_bits(*n) as i64)),
                                                PatternId::Double(n) => node.set_id(PatternId::IntptrT(f64::from_bits(*n) as i64)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::IntptrT(*n as i64)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        } else if ident == &String::from("UintptrT") {
                                            match node.id() {
                                                PatternId::Char(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Short(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Int(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Long(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Uchar(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Ushort(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Uint(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Ulong(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::Half(n) => node.set_id(PatternId::UintptrT(f32::from_bits(*n) as u64)),
                                                PatternId::Float(n) => node.set_id(PatternId::UintptrT(f32::from_bits(*n) as u64)),
                                                PatternId::Double(n) => node.set_id(PatternId::UintptrT(f64::from_bits(*n) as u64)),
                                                PatternId::SizeT(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::PtrdiffT(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::IntptrT(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                PatternId::UintptrT(n) => node.set_id(PatternId::UintptrT(*n as u64)),
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: invalid pattern identifier"))]))
                                            }
                                        }
                                        Ok(())
                                    },
                                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: type variable isn't built-in type"))])),
                                }
                            },
                            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: no type variable"))])),
                        }
                    },
                }
            },
            None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("convert_pattern_ids_for_type_value: no local type entry"))])),
        }
    }
    
    fn add_pattern_node_for_value(&self, value: &Value, tree: &Tree, forest: &mut PatternForest<PatternId>) -> FrontendResultWithErrors<()>
    {
        match value {
            Value::Bool(b) => {
                forest.add_node(PatternNode::new(PatternId::Bool(*b), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Bool"), tree)?);
            },
            Value::Char(c) => {
                forest.add_node(PatternNode::new(PatternId::Char(*c), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Char"), tree)?);
            },
            Value::Short(n) => {
                forest.add_node(PatternNode::new(PatternId::Short(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Short"), tree)?);
            },
            Value::Int(n) => {
                forest.add_node(PatternNode::new(PatternId::Int(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Int"), tree)?);
            },
            Value::Long(n) => {
                forest.add_node(PatternNode::new(PatternId::Long(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Long"), tree)?);
            },
            Value::Uchar(c) => {
                forest.add_node(PatternNode::new(PatternId::Uchar(*c), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Uchar"), tree)?);
            },
            Value::Ushort(n) => {
                forest.add_node(PatternNode::new(PatternId::Ushort(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Ushort"), tree)?);
            },
            Value::Uint(n) => {
                forest.add_node(PatternNode::new(PatternId::Uint(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Uint"), tree)?);
            },
            Value::Ulong(n) => {
                forest.add_node(PatternNode::new(PatternId::Ulong(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Ulong"), tree)?);
            },
            Value::Float(n) => {
                forest.add_node(PatternNode::new(PatternId::Float(n.to_bits()), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Float"), tree)?);
            },
            Value::Double(n) => {
                forest.add_node(PatternNode::new(PatternId::Double(n.to_bits()), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("Double"), tree)?);
            },
            Value::SizeT(n) => {
                forest.add_node(PatternNode::new(PatternId::SizeT(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("SizeT"), tree)?);
            },
            Value::PtrdiffT(n) => {
                forest.add_node(PatternNode::new(PatternId::PtrdiffT(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("PtrdiffT"), tree)?);
            },
            Value::IntptrT(n) => {
                forest.add_node(PatternNode::new(PatternId::IntptrT(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("IntptrT"), tree)?);
            },
            Value::UintptrT(n) => {
                forest.add_node(PatternNode::new(PatternId::UintptrT(*n), PatternForests::Unfilled(Vec::new())));
                forest.set_max(pattern_max_for_type_ident(&String::from("UintptrT"), tree)?);
            },
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::String(bs) => {
                        forest.add_node(PatternNode::new(PatternId::String(bs.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&String::from("ConstantSlice"), tree)?);
                    },
                    Object::CharN(cs) => {
                        forest.add_node(PatternNode::new(PatternId::CharN(cs.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Char{}", cs.len()), tree)?);
                    },
                    Object::ShortN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::ShortN(ns.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Short{}", ns.len()), tree)?);
                    },
                    Object::IntN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::IntN(ns.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Int{}", ns.len()), tree)?);
                    },
                    Object::LongN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::LongN(ns.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Long{}", ns.len()), tree)?);
                    },
                    Object::UcharN(cs) => {
                        forest.add_node(PatternNode::new(PatternId::UcharN(cs.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Uchar{}", cs.len()), tree)?);
                    },
                    Object::UshortN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::UshortN(ns.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Ushort{}", ns.len()), tree)?);
                    },
                    Object::UintN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::UintN(ns.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Uint{}", ns.len()), tree)?);
                    },
                    Object::UlongN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::UlongN(ns.clone()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Ulong{}", ns.len()), tree)?);
                    },
                    Object::FloatN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::FloatN(ns.iter().map(|n| n.to_bits()).collect()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Float{}", ns.len()), tree)?);
                    },
                    Object::DoubleN(ns) => {
                        forest.add_node(PatternNode::new(PatternId::DoubleN(ns.iter().map(|n| n.to_bits()).collect()), PatternForests::Unfilled(Vec::new())));
                        forest.set_max(pattern_max_for_type_ident(&format!("Double{}", ns.len()), tree)?);
                    },
                    Object::Tuple(field_values) => {
                        let mut forests: Vec<PatternForest<PatternId>> = Vec::new();
                        for field_value in field_values {
                            let mut forest2 = PatternForest::Alt(Vec::new(), None);
                            self.add_pattern_node_for_value(field_value, tree, &mut forest2)?;
                            forests.push(forest2);
                        }
                        forest.add_node(PatternNode::new(PatternId::Tuple(field_values.len()), PatternForests::Unfilled(forests)));
                        forest.set_max(Some(1));
                    },
                    Object::Array(elem_values) => {
                        let mut forests: Vec<PatternForest<PatternId>> = Vec::new();
                        for elem_value in elem_values {
                            let mut forest2 = PatternForest::Alt(Vec::new(), None);
                            self.add_pattern_node_for_value(elem_value, tree, &mut forest2)?;
                            forests.push(forest2);
                        }
                        forest.add_node(PatternNode::new(PatternId::Array(elem_values.len()), PatternForests::Unfilled(forests)));
                        forest.set_max(Some(1));
                    },
                    Object::Data(con_ident, field_values) => {
                        let mut forests: Vec<PatternForest<PatternId>> = Vec::new();
                        for field_value in field_values {
                            let mut forest2 = PatternForest::Alt(Vec::new(), None);
                            self.add_pattern_node_for_value(field_value, tree, &mut forest2)?;
                            forests.push(forest2);
                        }
                        forest.add_node(PatternNode::new(PatternId::Array(field_values.len()), PatternForests::Unfilled(forests)));
                        type_for_fun_ident_in(con_ident, tree, |typ| {
                                match &**typ.type_value() {
                                    TypeValue::Type(_, TypeValueName::Fun, type_values) => {
                                        match type_values.last() {
                                            Some(type_value) => {
                                                match &**type_value {
                                                    TypeValue::Type(_, TypeValueName::Name(ident), _) => {
                                                        forest.set_max(pattern_max_for_type_ident(ident, tree)?);
                                                    },
                                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_pattern_node_for_value: type value isn't built-in type and data type"))])),
                                                }
                                            },
                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_pattern_node_for_value: no type value"))])),
                                        }
                                    },
                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("add_pattern_node_for_value: type isn't function type"))])),
                                }
                                Ok(())
                        })?;
                    },
                    Object::Builtin(ident, type_name) => {
                        type_for_ident_and_type_name_in(ident, type_name, tree, |typ| {
                                forest.set_max(pattern_max_for_type(typ, tree)?);
                                Ok(())
                        })?;
                    },
                    _ => (),
                }
            },
        }
        Ok(())
    }
    
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
                    PatternForest::Alt(nodes, max) => {
                        match nodes.last() {
                            Some(node) => {
                                let mut node_r = node.borrow_mut();
                                self.convert_pattern_ids_for_type_value(&mut *node_r, max, &Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), tree, local_types)?; 
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
                    Some(value) => self.add_pattern_node_for_value(&value, tree, forest)?,
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
    
    fn set_local_funs_for_expr(&self, expr: &mut Expr, local_fun_counter: &mut usize) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.do_literal_mut_for_setting(&mut **literal, |evaluator, expr| evaluator.set_local_funs_for_expr(expr, local_fun_counter))?,
            Expr::Lambda(_, _, _, _, _, local_fun, _, _) => {
                *local_fun = Some(LocalFun::new(*local_fun_counter));
                *local_fun_counter += 1;
            },
            Expr::Var(_, _, _) => (),
            Expr::NamedFieldConApp(_, expr_named_field_pairs, _, _, _) => {
                self.do_named_field_pairs_mut_for_setting(expr_named_field_pairs.as_mut_slice(), |evaluator, expr| evaluator.set_local_funs_for_expr(expr, local_fun_counter))?;
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
                for expr3 in exprs {
                    self.set_local_funs_for_expr(&mut **expr3, local_fun_counter)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?,
            Expr::Get2Field(expr2, _, _, _) => self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
                self.set_local_funs_for_expr(&mut **expr3, local_fun_counter)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
                self.set_local_funs_for_expr(&mut **expr3, local_fun_counter)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
                self.set_local_funs_for_expr(&mut **expr3, local_fun_counter)?;
            },
            Expr::Uniq(expr2, _, _) => self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?,
            Expr::Shared(expr2, _, _) => self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?,
            Expr::Typed(expr2, _, _, _) => self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?,
            Expr::As(expr2, _, _, _) => self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
                self.set_local_funs_for_expr(&mut **expr3, local_fun_counter)?;
                self.set_local_funs_for_expr(&mut **expr4, local_fun_counter)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                for bind in binds {
                    match bind {
                        Bind(_, expr3) => self.set_local_funs_for_expr(&mut **expr3, local_fun_counter)?,
                    }
                }
                self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
            },
            Expr::Match(expr2, cases, _, _) => {
                self.set_local_funs_for_expr(&mut **expr2, local_fun_counter)?;
                for case in cases {
                    match case {
                        Case(_, expr3) => self.set_local_funs_for_expr(&mut **expr3, local_fun_counter)?,
                    }
                }
            },
        }
        Ok(())
    }

    fn value_for_fields_with_ref_fun_in<F>(&self, value: &mut Value, local_type: LocalType, fields: &[Field], pos: &Pos, tree: &Tree, local_types: &LocalTypes, are_settings: bool, errs: &mut Vec<FrontendError>, f: &mut F) -> FrontendResultWithErrors<bool>
        where F: FnMut(&mut Value, &mut Vec<FrontendError>) -> FrontendResultWithErrors<bool>
    {
        match fields.first() {
            Some(field) => {
                let next_local_type: LocalType;
                let field_idx = match field {
                    Field::Unnamed(tmp_field_idx, Some(field_local_type)) => {
                        next_local_type = *field_local_type;
                        *tmp_field_idx
                    },
                    Field::Named(field_ident, Some(field_local_type)) => {
                        next_local_type = *field_local_type;
                        match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type))) {
                            Some(LocalTypeEntry::Type(type_value)) => {
                                match &*type_value {
                                    TypeValue::Type(_, TypeValueName::Name(type_ident), _) => {
                                        match tree.type_var(type_ident) {
                                            Some(type_var) => {
                                                let type_var_r = type_var.borrow();
                                                match &*type_var_r {
                                                    TypeVar::Builtin(_, Some(fields2), _) => {
                                                        match fields2.field_index(field_ident) {
                                                            Some(tmp_field_idx) => tmp_field_idx,
                                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: type variable hasn't field"))])),
                                                        }
                                                    },
                                                    TypeVar::Data(_, cons, _) => {
                                                        match cons.first() {
                                                            Some(con) => {
                                                                let con_r = con.borrow();
                                                                match &*con_r {
                                                                    Con::NamedField(_, _, _, Some(named_fields), _) => {
                                                                        match named_fields.field_index(field_ident) {
                                                                            Some(tmp_field_idx) => tmp_field_idx,
                                                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: type variable hasn't field"))])),
                                                                        }
                                                                    },
                                                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: type variable isn't type or no fields"))])),
                                                                }
                                                            },
                                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: type variable hasn't constructor"))])),
                                                        }
                                                    },
                                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: type variable isn't type or no fields"))])),
                                                }
                                            },
                                            None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: no type variable"))])),
                                        }
                                    },
                                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: type value isn't type or type value hasn't field"))]))
                                }
                            },
                            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: no local type entry"))])),
                        }
                    },
                    _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: no local type"))])),
                };
                match value {
                    Value::Object(shared_flag, object) => {
                        if *shared_flag == SharedFlag::Shared && are_settings {
                            let tmp_object = object.clone();
                            let tmp_object_r = tmp_object.borrow();
                            *object = Rc::new(RefCell::new(tmp_object_r.clone()));
                        }
                        let mut object_r = object.borrow_mut();
                        match &mut *object_r {
                            Object::CharN(cs) => {
                                match cs.get_mut(field_idx) {
                                    Some(c) => {
                                        let mut vec_field_value = Value::Char(*c);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Char(c2) => *c = c2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::ShortN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Short(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Short(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::IntN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Int(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Int(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::LongN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Long(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Long(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::UcharN(cs) => {
                                match cs.get_mut(field_idx) {
                                    Some(c) => {
                                        let mut vec_field_value = Value::Uchar(*c);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Uchar(c2) => *c = c2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::UshortN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Ushort(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Ushort(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::UintN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Uint(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Uint(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::UlongN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Ulong(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Ulong(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },    
                            Object::FloatN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Float(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Float(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::DoubleN(ns) => {
                                match ns.get_mut(field_idx) {
                                    Some(n) => {
                                        let mut vec_field_value = Value::Double(*n);
                                        let is_set = self.value_for_fields_with_ref_fun_in(&mut vec_field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f)?;
                                        if are_settings && is_set {
                                            match vec_field_value {
                                                Value::Double(n2) => *n = n2,
                                                Value::Object(_, object2) => {
                                                    let object2_r = object2.borrow();
                                                    add_error_for_object(&*object2_r, pos.clone(), errs)?;
                                                    return Ok(false);
                                                },
                                                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: invalid value"))])),
                                            }
                                        }
                                        Ok(is_set)
                                    },
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::Tuple(field_values) => {
                                match field_values.get_mut(field_idx) {
                                    Some(field_value) => self.value_for_fields_with_ref_fun_in(field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f),
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            Object::Data(_, field_values) => {
                                match field_values.get_mut(field_idx) {
                                    Some(field_value) => self.value_for_fields_with_ref_fun_in(field_value, next_local_type, &fields[1..], pos, tree, local_types, are_settings, errs, f),
                                    None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value hasn't field value"))])),
                                }
                            },
                            _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: object hasn't fields"))])),
                        }
                    },
                    _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("value_for_fields_with_ref_fun_in: value isn't object"))])),
                }
            },
            None => f(value, errs),
        }
    }

    fn value_for_fields_in<F>(&self, value: &mut Value, local_type: LocalType, fields: &[Field], pos: &Pos, tree: &Tree, local_types: &LocalTypes, are_settings: bool, errs: &mut Vec<FrontendError>, mut f: F) -> FrontendResultWithErrors<bool>
        where F: FnMut(&mut Value, &mut Vec<FrontendError>) -> FrontendResultWithErrors<bool>
    { self.value_for_fields_with_ref_fun_in(value, local_type, fields, pos, tree, local_types, are_settings, errs, &mut f) }

    fn convert_value_for_type_value(&self, value: &Value, type_value: &Rc<TypeValue>, pos: &Pos, tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<Value>>
    { Ok(None) }

    fn convert_pattern_value_for_type_value(&self, pattern_value: &PatternValue, type_value: &Rc<TypeValue>, pos: &Pos, tree: &Tree, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<PatternValue>>
    { Ok(None) }
    
    fn value_to_pattern_value(&self, value: &Value, pos: &Pos, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<PatternValue>>
    { Ok(None) }
    
    fn match_value_with_pattern_value(&self, value: &Value, pattern_value: &PatternValue, var_env: &mut Environment<Value>) -> FrontendResultWithErrors<bool>
    { Ok(false) }
    
    fn evaluate_value_for_expr(&self, expr: &Expr, tree: &Tree, var_env: &mut Environment<Value>, type_stack: &mut TypeStack, local_types: &LocalTypes, closures: &mut BTreeMap<LocalFun, Closure>, var_key: &(String, Option<TypeName>), errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<Value>>
    {
        match expr {
            Expr::Literal(literal, Some(local_type), _) => {
                match self.evaluate_value_for_expr_literal(&**literal, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(mut value) => {
                        value.set_shared_flag(shared_flag_for_local_type(*local_type, tree, type_stack, local_types)?);
                        Ok(Some(value))
                    },
                    None => Ok(None),
                }
            },
            Expr::Lambda(args, _, body, _, Some(local_type), Some(local_fun), _, _) => {
                let mut lambda_var_env = Environment::new();
                lambda_var_env.push_new_vars();
                for arg in &*args {
                    match arg {
                        LambdaArg(ident, _, _, _) => {
                            lambda_var_env.add_var(ident.clone(), ());
                        },
                    }
                }
                let mut closure = Closure::new();
                self.add_closure_vars_for_expr(&**body, var_env, &mut lambda_var_env, &mut closure);
                closures.insert(*local_fun, closure);
                lambda_var_env.pop_vars();
                let shared_flag = shared_flag_for_local_type(*local_type, tree, type_stack, local_types)?;
                Ok(Some(Value::Object(shared_flag, Rc::new(RefCell::new(Object::Lambda(var_key.0.clone(), var_key.1.clone(), *local_fun))))))
            },
            Expr::Var(ident, Some(local_type), pos) => {
                match var_env.var(ident) {
                    Some(value) => Ok(Some(value.clone())),
                    None => {
                        let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                        self.value_for_ident_and_type_name(ident, &type_name, pos.clone(), tree, true, errs)
                    },
                }
            },
            Expr::NamedFieldConApp(ident, expr_named_field_pairs, _, Some(local_type), pos) => {
                named_fields_for_con_ident_in(ident, tree, |named_fields| {
                        let mut field_values = vec![Value::Bool(false); expr_named_field_pairs.len()];
                        for expr_named_field_pair in expr_named_field_pairs {
                            match expr_named_field_pair {
                                NamedFieldPair(field_ident, expr2, _) => {
                                    match named_fields.field_index(field_ident) {
                                        Some(field_idx) => {
                                            match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                                                Some(field_value) => field_values[field_idx] = field_value,
                                                None => return Ok(None),
                                            }
                                        },
                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_value_for_expr: no field index"))])),
                                    }
                                },
                            }
                        }
                        let shared_flag = shared_flag_for_local_type(*local_type, tree, type_stack, local_types)?;
                        Ok(Some(Value::Object(shared_flag, Rc::new(RefCell::new(Object::Data(ident.clone(), field_values))))))
                })
            },
            Expr::PrintfApp(_, _, pos) => {
                errs.push(FrontendError::Message(pos.clone(), String::from("printf is unsupported for evaluation of variable values")));
                Ok(None)
            },
            Expr::App(expr2, exprs, Some(local_type), pos) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(Value::Object(_, object)) => {
                        let object_r = object.borrow();
                        match &*object_r {
                            Object::Con(con_ident) => {
                                let mut field_values: Vec<Value> = Vec::new();
                                for expr3 in exprs {
                                    match self.evaluate_value_for_expr(&**expr3, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                                        Some(field_value) => field_values.push(field_value),
                                        None => return Ok(None),
                                    }
                                }
                                let shared_flag = shared_flag_for_local_type(*local_type, tree, type_stack, local_types)?;
                                Ok(Some(Value::Object(shared_flag, Rc::new(RefCell::new(Object::Data(con_ident.clone(), field_values))))))
                            },
                            Object::EvalFun(_, _, fun) => {
                                let mut arg_values: Vec<Value> = Vec::new();
                                for expr3 in exprs {
                                    match self.evaluate_value_for_expr(&**expr3, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                                        Some(arg_value) => arg_values.push(arg_value),
                                        None => return Ok(None),
                                    }
                                }
                                match fun(arg_values.as_slice(), pos) {
                                    Ok(value) => Ok(Some(value)),
                                    Err(err @ FrontendError::Internal(_)) => Err(FrontendErrors::new(vec![err])),
                                    Err(err) => {
                                        errs.push(err);
                                        Ok(None)
                                    },
                                }
                            },
                            _ => {
                                errs.push(FrontendError::Message(pos.clone(), String::from("value isn't evaluable function")));
                                Ok(None)
                            },
                        }
                    },
                    Some(_) => {
                        errs.push(FrontendError::Message(pos.clone(), String::from("value isn't evaluable function")));
                        Ok(None)
                    }
                    None => Ok(None),
                }
            },
            Expr::GetField(expr2, fields, _, pos) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(mut value) => {
                        let mut value2: Option<Value> = None;
                        self.value_for_fields_in(&mut value, expr_local_type(&**expr2)?, fields.as_slice(), pos, tree, local_types, false, errs, |value, _| {
                                value2 = Some(value.clone());
                                Ok(false)
                        })?;
                        Ok(value2)
                    },
                    None => Ok(None),
                }
            },
            Expr::Get2Field(expr2, fields, Some(local_type), pos) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(mut value) => {
                        let mut value2: Option<Value> = None;
                        self.value_for_fields_in(&mut value, expr_local_type(&**expr2)?, fields.as_slice(), pos, tree, local_types, false, errs, |value, _| {
                                let shared_flag = shared_flag_for_local_type(*local_type, tree, type_stack, local_types)?;
                                value2 = Some(value.clone());
                                Ok(false)
                        })?;
                        match value2 {
                            Some(value2) => {
                                let shared_flag = shared_flag_for_local_type(*local_type, tree, type_stack, local_types)?;
                                Ok(Some(Value::Object(shared_flag, Rc::new(RefCell::new(Object::Tuple(vec![value2, value]))))))
                            },
                            None => Ok(None),
                        }
                    },
                    None => Ok(None),
                }
            },
            Expr::SetField(expr2, fields, expr3, _, pos) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(mut value) => {
                        let is_success = self.value_for_fields_in(&mut value, expr_local_type(&**expr2)?, fields.as_slice(), pos, tree, local_types, true, errs, |value, errs| {
                                match self.evaluate_value_for_expr(&**expr3, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                                    Some(value2) => {
                                        *value = value2;
                                        Ok(true)
                                    },
                                    None => Ok(false),
                                }
                        })?;
                        if is_success {
                            Ok(Some(value))
                        } else {
                            Ok(None)
                        }
                    },
                    None => Ok(None),
                }
            },
            Expr::UpdateField(_, _, _, _, pos) => {
                errs.push(FrontendError::Message(pos.clone(), String::from("opterator <-> is unsupported for evaluation of variable values")));
                Ok(None)
            },
            Expr::UpdateGet2Field(_, _, _, _, pos) => {
                errs.push(FrontendError::Message(pos.clone(), String::from("opterator <-> -> is unsupported for evaluation of variable values")));
                Ok(None)
            },
            Expr::Uniq(expr2, _, _) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(Value::Object(SharedFlag::Shared, object)) => {
                        let object_r = object.borrow();
                        Ok(Some(Value::Object(SharedFlag::None, Rc::new(RefCell::new(object_r.clone())))))
                    },
                    Some(value) => Ok(Some(value)),
                    None => Ok(None),
                }
            },
            Expr::Shared(expr2, _, _) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(Value::Object(SharedFlag::None, object)) => {
                        Ok(Some(Value::Object(SharedFlag::Shared, object.clone())))
                    },
                    Some(value) => Ok(Some(value)),
                    None => Ok(None),
                }
            },
            Expr::Typed(expr2, _, _, _) => self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs),
            Expr::As(expr2, _, Some(local_type), pos) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(value) => self.convert_value_for_type_value(&value, &Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), pos, tree, local_types, errs),
                    None => Ok(None),
                }
            },
            Expr::If(expr2, expr3, expr4, _, pos) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(Value::Bool(true)) => self.evaluate_value_for_expr(&**expr3, tree, var_env, type_stack, local_types, closures, var_key, errs),
                    Some(Value::Bool(false)) => self.evaluate_value_for_expr(&**expr4, tree, var_env, type_stack, local_types, closures, var_key, errs),
                    Some(Value::Object(_, object)) => {
                        let object_r = object.borrow();
                        match &*object_r {
                            Object::Builtin(_, _) => {
                                errs.push(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be condition")));
                                Ok(None)
                            },
                            _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_value_for_expr: invalid object"))])),
                        }
                    },
                    Some(_) => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_value_for_expr: invalid value"))])),
                    None => Ok(None),
                }
            },
            Expr::Let(binds, expr2, _, _) => {
                var_env.push_new_vars();
                for bind in binds {
                    match bind {
                        Bind(pattern, expr3) => {
                            match self.evaluate_value_for_expr(&**expr3, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                                Some(value) => {
                                    match self.evaluate_pattern_value_for_pattern(pattern, tree, type_stack, local_types, errs)? {
                                        Some(pattern_value) => {
                                            if !self.match_value_with_pattern_value(&value, &pattern_value, var_env)? {
                                                return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_value_for_expr: can't match value with pattern value"))]));
                                            }
                                        },
                                        None => return Ok(None),
                                    }
                                },
                                None => return Ok(None),
                            }
                        },
                    }
                }
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(value) => {
                        var_env.pop_vars();
                        Ok(Some(value))
                    },
                    None => Ok(None),
                }
            },
            Expr::Match(expr2, cases, _, _) => {
                match self.evaluate_value_for_expr(&**expr2, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(value) => {
                        for case in cases {
                            match case {
                                Case(pattern, expr3) => {
                                    var_env.push_new_vars();
                                    match self.evaluate_pattern_value_for_pattern(pattern, tree, type_stack, local_types, errs)? {
                                        Some(pattern_value) => {
                                            if self.match_value_with_pattern_value(&value, &pattern_value, var_env)? {
                                                return self.evaluate_value_for_expr(&**expr3, tree, var_env, type_stack, local_types, closures, var_key, errs);
                                            }
                                        },
                                        None => return Ok(None),
                                    }
                                    var_env.pop_vars();
                                },
                            }
                        }
                        Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_value_for_expr: can't match value with all pattern values"))]))
                    },
                    None => Ok(None),
                }
            },
            _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_value_for_expr: no local type or no local function"))])),
        }
    }

    fn evaluate_pattern_value_for_pattern(&self, pattern: &Pattern, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<PatternValue>>
    {
        match pattern {
            Pattern::Literal(literal, _, _) => self.evaluate_pattern_value_for_pattern_literal(&**literal, tree, type_stack, local_types, errs),
            Pattern::As(literal, _, _, Some(local_type), pos) => {
                match self.evaluate_pattern_value_for_pattern_literal(&**literal, tree, type_stack, local_types, errs)? {
                    Some(pattern_value) => self.convert_pattern_value_for_type_value(&pattern_value, &Rc::new(TypeValue::Param(UniqFlag::None, *local_type)), pos, tree, local_types, errs),
                    None => Ok(None),
                }
            },
            Pattern::Const(ident, Some(local_type), pos) => {
                let type_name = type_name_for_var_ident_and_local_type(ident, *local_type, tree, type_stack, local_types)?;
                match self.value_for_ident_and_type_name(ident, &type_name, pos.clone(), tree, false, errs)? {
                    Some(value) => self.value_to_pattern_value(&value, pos, errs),
                    None => Ok(None),
                }
            },
            Pattern::UnnamedFieldCon(ident, patterns, _, _, _) => {
                let mut field_pattern_values: Vec<PatternValue> = Vec::new();
                for pattern2 in patterns {
                    match self.evaluate_pattern_value_for_pattern(pattern2, tree, type_stack, local_types, errs)? {
                        Some(field_pattern_value) => field_pattern_values.push(field_pattern_value),
                        None => return Ok(None),
                    }
                }
                Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::Data(ident.clone(), field_pattern_values))))))
            },
            Pattern::NamedFieldCon(ident, pattern_named_field_pairs, _, _, _) => {
                named_fields_for_con_ident_in(ident, tree, |named_fields| {
                        let mut field_pattern_values = vec![PatternValue::Bool(false); pattern_named_field_pairs.len()];
                        for pattern_named_field_pair in pattern_named_field_pairs {
                            match pattern_named_field_pair {
                                NamedFieldPair(field_ident, pattern2, _) => {
                                    match named_fields.field_index(field_ident) {
                                        Some(field_idx) => {
                                            match self.evaluate_pattern_value_for_pattern(&**pattern2, tree, type_stack, local_types, errs)? {
                                                Some(field_pattern_value) => field_pattern_values[field_idx] = field_pattern_value,
                                                None => return Ok(None),
                                            }
                                        },
                                        None => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_pattern_value_for_pattern: no field index"))])),
                                    }
                                },
                            }
                        }
                        Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::Data(ident.clone(), field_pattern_values))))))
                })
            },
            Pattern::Var(_, ident, _, _) => Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::Var(ident.clone())))))),
            Pattern::At(_, ident, pattern2, _, _) => {
                match self.evaluate_pattern_value_for_pattern(&**pattern2, tree, type_stack, local_types, errs)? {
                    Some(pattern_value) => Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::At(ident.clone(), pattern_value)))))),
                    None => Ok(None),
                }
            },
            Pattern::Wildcard(_, _) => Ok(Some(PatternValue::Wildcard)),
            Pattern::Alt(patterns, _, _) => {
                let mut pattern_values: Vec<PatternValue> = Vec::new();
                for pattern2 in patterns {
                    match self.evaluate_pattern_value_for_pattern(pattern2, tree, type_stack, local_types, errs)? {
                        Some(pattern_value) => pattern_values.push(pattern_value),
                        None => return Ok(None),
                    }
                }
                Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::Alt(pattern_values))))))
            },
            _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("evaluate_pattern_value_for_pattern: no local type"))])),
        }
    }

    fn evaluate_value_for_expr_literal(&self, literal: &Literal<Expr>, tree: &Tree, var_env: &mut Environment<Value>, type_stack: &mut TypeStack, local_types: &LocalTypes, closures: &mut BTreeMap<LocalFun, Closure>, var_key: &(String, Option<TypeName>), errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<Value>>
    {
        match literal {
            Literal::Bool(b) => Ok(Some(Value::Bool(*b))),
            Literal::Char(c) => Ok(Some(Value::Char(*c))),
            Literal::Int(n) => Ok(Some(Value::Int(*n))),
            Literal::Long(n) => Ok(Some(Value::Long(*n))),
            Literal::Uint(n) => Ok(Some(Value::Uint(*n))),
            Literal::Ulong(n) => Ok(Some(Value::Ulong(*n))),
            Literal::Float(n) => Ok(Some(Value::Float(*n))),
            Literal::Double(n) => Ok(Some(Value::Double(*n))),
            Literal::String(bs) => Ok(Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::String(bs.clone())))))),
            Literal::Tuple(field_exprs) => {
                let mut field_values: Vec<Value> = Vec::new();
                for field_expr in field_exprs {
                    match self.evaluate_value_for_expr(&**field_expr, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                        Some(field_value) => field_values.push(field_value),
                        None => return Ok(None),
                    }
                }
                Ok(Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Tuple(field_values))))))
            },
            Literal::Array(elem_exprs) => {
                let mut elem_values: Vec<Value> = Vec::new();
                for elem_expr in elem_exprs {
                    match self.evaluate_value_for_expr(&**elem_expr, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                        Some(elem_value) => elem_values.push(elem_value),
                        None => return Ok(None),
                    }
                }
                Ok(Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Array(elem_values))))))
            },
            Literal::FilledArray(elem_expr, len) => {
                match self.evaluate_value_for_expr(&**elem_expr, tree, var_env, type_stack, local_types, closures, var_key, errs)? {
                    Some(elem_value) => Ok(Some(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Array(vec![elem_value; *len])))))),
                    None => Ok(None),
                }
            },
        }
    }

    fn evaluate_pattern_value_for_pattern_literal(&self, literal: &Literal<Pattern>, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes, errs: &mut Vec<FrontendError>) -> FrontendResultWithErrors<Option<PatternValue>>
    {
        match literal {
            Literal::Bool(b) => Ok(Some(PatternValue::Bool(*b))),
            Literal::Char(c) => Ok(Some(PatternValue::Char(*c))),
            Literal::Int(n) => Ok(Some(PatternValue::Int(*n))),
            Literal::Long(n) => Ok(Some(PatternValue::Long(*n))),
            Literal::Uint(n) => Ok(Some(PatternValue::Uint(*n))),
            Literal::Ulong(n) => Ok(Some(PatternValue::Ulong(*n))),
            Literal::Float(n) => Ok(Some(PatternValue::Float(*n))),
            Literal::Double(n) => Ok(Some(PatternValue::Double(*n))),
            Literal::String(bs) => Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::String(bs.clone())))))),
            Literal::Tuple(field_patterns) => {
                let mut field_pattern_values: Vec<PatternValue> = Vec::new();
                for field_pattern in field_patterns {
                    match self.evaluate_pattern_value_for_pattern(field_pattern, tree, type_stack, local_types, errs)? {
                        Some(field_pattern_value) => field_pattern_values.push(field_pattern_value),
                        None => return Ok(None),
                    }
                }
                Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::Tuple(field_pattern_values))))))
            },
            Literal::Array(elem_patterns) => {
                let mut elem_pattern_values: Vec<PatternValue> = Vec::new();
                for elem_pattern in elem_patterns {
                    match self.evaluate_pattern_value_for_pattern(elem_pattern, tree, type_stack, local_types, errs)? {
                        Some(elem_pattern_value) => elem_pattern_values.push(elem_pattern_value),
                        None => return Ok(None),
                    }
                }
                Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::Array(elem_pattern_values))))))
            },
            Literal::FilledArray(elem_pattern, len) => {
                match self.evaluate_pattern_value_for_pattern(elem_pattern, tree, type_stack, local_types, errs)? {
                    Some(elem_pattern_value) => Ok(Some(PatternValue::Object(Rc::new(RefCell::new(PatternObject::Array(vec![elem_pattern_value; *len])))))),
                    None => Ok(None),
                }
            },
        }
    }

    fn add_closure_vars_for_expr(&self, expr: &Expr, closure_var_env: &Environment<Value>, var_env: &mut Environment<()>, closure: &mut Closure)
    {}

    fn add_vars_for_pattern(&self, expr: &Pattern, var_env: &mut Environment<()>)
    {}
    
    fn set_closures_for_expr(&self, expr: &mut Expr, closures: &mut BTreeMap<LocalFun, Closure>) -> FrontendResultWithErrors<()>
    {
        match expr {
            Expr::Literal(literal, _, _) => self.do_literal_mut_for_setting(&mut **literal, |evaluator, expr| evaluator.set_closures_for_expr(expr, closures))?,
            Expr::Lambda(_, _, _, _, _, Some(local_fun), closure, _) => {
                match closures.remove(local_fun) {
                    Some(closure2) => *closure = Some(Box::new(closure2)),
                    None => (),
                }
            },
            Expr::Var(_, _, _) => (),
            Expr::NamedFieldConApp(_, expr_named_field_pairs, _, _, _) => {
                self.do_named_field_pairs_mut_for_setting(expr_named_field_pairs.as_mut_slice(), |evaluator, expr| evaluator.set_closures_for_expr(expr, closures))?;
            },
            Expr::PrintfApp(exprs, _, _) => {
                for expr2 in exprs {
                    self.set_closures_for_expr(&mut **expr2, closures)?;
                }
            },
            Expr::App(expr2, exprs, _, _) => {
                self.set_closures_for_expr(&mut **expr2, closures)?;
                for expr3 in exprs {
                    self.set_closures_for_expr(&mut **expr3, closures)?;
                }
            },
            Expr::GetField(expr2, _, _, _) => self.set_closures_for_expr(&mut **expr2, closures)?,
            Expr::Get2Field(expr2, _, _, _) => self.set_closures_for_expr(&mut **expr2, closures)?,
            Expr::SetField(expr2, _, expr3, _, _) => {
                self.set_closures_for_expr(&mut **expr2, closures)?;
                self.set_closures_for_expr(&mut **expr3, closures)?;
            },
            Expr::UpdateField(expr2, _, expr3, _, _) => {
                self.set_closures_for_expr(&mut **expr2, closures)?;
                self.set_closures_for_expr(&mut **expr3, closures)?;
            },
            Expr::UpdateGet2Field(expr2, _, expr3, _, _) => {
                self.set_closures_for_expr(&mut **expr2, closures)?;
                self.set_closures_for_expr(&mut **expr3, closures)?;
            },
            Expr::Uniq(expr2, _, _) => self.set_closures_for_expr(&mut **expr2, closures)?,
            Expr::Shared(expr2, _, _) => self.set_closures_for_expr(&mut **expr2, closures)?,
            Expr::Typed(expr2, _, _, _) => self.set_closures_for_expr(&mut **expr2, closures)?,
            Expr::As(expr2, _, _, _) => self.set_closures_for_expr(&mut **expr2, closures)?,
            Expr::If(expr2, expr3, expr4, _, _) => {
                self.set_closures_for_expr(&mut **expr2, closures)?;
                self.set_closures_for_expr(&mut **expr3, closures)?;
                self.set_closures_for_expr(&mut **expr4, closures)?;
            },
            Expr::Let(binds, expr2, _, _) => {
                for bind in binds {
                    match bind {
                        Bind(_, expr3) => self.set_closures_for_expr(&mut **expr3, closures)?,
                    }
                }
                self.set_closures_for_expr(&mut **expr2, closures)?;
            },
            Expr::Match(expr2, cases, _, _) => {
                self.set_closures_for_expr(&mut **expr2, closures)?;
                for case in cases {
                    match case {
                        Case(_, expr3) => self.set_closures_for_expr(&mut **expr3, closures)?,
                    }
                }
            },
            _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("set_closures_for_expr: no local function"))])),
        }
        Ok(())
    }
}
