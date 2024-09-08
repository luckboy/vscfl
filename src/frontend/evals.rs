//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::cmp::max;
use std::cmp::min;
use std::collections::HashMap;
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;

fn char_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut cs: Vec<i8> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Char(c) => cs.push(*c),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("char_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("char_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::CharN(cs)))))
}

fn short_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<i16> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Short(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("short_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("short_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::ShortN(ns)))))
}

fn int_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<i32> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Int(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("int_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("int_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::IntN(ns)))))
}

fn long_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<i64> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Long(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("long_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("long_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::LongN(ns)))))
}

fn uchar_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut cs: Vec<u8> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Uchar(c) => cs.push(*c),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("uchar_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("uchar_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::UcharN(cs)))))
}

fn ushort_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<u16> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Ushort(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("ushort_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("ushort_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::UshortN(ns)))))
}

fn uint_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<u32> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Uint(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("uint_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("uint_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::UintN(ns)))))
}

fn ulong_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<u64> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Ulong(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("ulong_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("ulong_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::UlongN(ns)))))
}

fn float_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<f32> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Float(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("float_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("float_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::FloatN(ns)))))
}

fn double_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    let mut ns: Vec<f64> = Vec::new();
    for arg_value in arg_values {
        match arg_value {
            Value::Double(n) => ns.push(*n),
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("value of built-in variable mustn't be in vector for evaluation of variable values"))),
                    _ => return Err(FrontendError::Internal(String::from("double_n: invalid object"))),
                }
            },
            _ => return Err(FrontendError::Internal(String::from("double_n: invalid value"))),
        }
    }
    Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::DoubleN(ns)))))
}

fn reference(arg_values: &[Value], ref_values: &mut RefValues, _pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
        let idx = ref_values.add_value(RefValue(RefValueFlag::None, arg_values[0].clone()));
        Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Ref(idx, Vec::new())))))
    } else {
        Err(FrontendError::Internal(String::from("reference: too few or many arguments")))
    }
}

fn global_ref(arg_values: &[Value], ref_values: &mut RefValues, _pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
        let idx = ref_values.add_value(RefValue(RefValueFlag::Global, arg_values[0].clone()));
        Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Ref(idx, Vec::new())))))
    } else {
        Err(FrontendError::Internal(String::from("global_ref: too few or many arguments")))
    }
}

fn op_neg(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
        match &arg_values[0] {
            Value::Char(c) => Ok(Value::Char((-(*c as i16)) as i8)),
            Value::Short(n) => Ok(Value::Short((-(*n as i32)) as i16)),
            Value::Int(n) => Ok(Value::Int((-(*n as i64)) as i32)),
            Value::Long(n) => Ok(Value::Long((-(*n as i128)) as i64)),
            Value::Float(n) => Ok(Value::Float(-*n)),
            Value::Double(n) => Ok(Value::Double(-*n)),
            Value::Object(shared_flag, object) => {
                let new_object = if *shared_flag == SharedFlag::Shared {
                    let tmp_object = object.clone();
                    let tmp_object_r = tmp_object.borrow();
                    Rc::new(RefCell::new(tmp_object_r.clone()))
                } else {
                    object.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let tmp_object = match &*new_object_r {
                        Object::CharN(cs) => Object::CharN(cs.iter().map(|c| (-(*c as i16)) as i8).collect()),
                        Object::ShortN(ns) => Object::ShortN(ns.iter().map(|n| (-(*n as i32)) as i16).collect()),
                        Object::IntN(ns) => Object::IntN(ns.iter().map(|n| (-(*n as i64)) as i32).collect()),
                        Object::LongN(ns) => Object::LongN(ns.iter().map(|n| (-(*n as i128)) as i64).collect()),
                        Object::FloatN(ns) => Object::FloatN(ns.iter().map(|n| -*n).collect()),
                        Object::DoubleN(ns) => Object::DoubleN(ns.iter().map(|n| -*n).collect()),
                        Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_neg for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_neg: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag, new_object))
            },
            _ => Err(FrontendError::Internal(String::from("op_neg: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_neg: too few or many arguments")))
    }
}

fn op_not(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
        match &arg_values[0] {
            Value::Bool(b) => Ok(Value::Bool(!*b)),
            Value::Char(c) => Ok(Value::Char(!*c)),
            Value::Short(n) => Ok(Value::Short(!*n)),
            Value::Int(n) => Ok(Value::Int(!*n)),
            Value::Long(n) => Ok(Value::Long(!*n)),
            Value::Uchar(c) => Ok(Value::Uchar(!*c)),
            Value::Ushort(n) => Ok(Value::Ushort(!*n)),
            Value::Uint(n) => Ok(Value::Uint(!*n)),
            Value::Ulong(n) => Ok(Value::Ulong(!*n)),
            Value::Object(shared_flag, object) => {
                let new_object = if *shared_flag == SharedFlag::Shared {
                    let tmp_object = object.clone();
                    let tmp_object_r = tmp_object.borrow();
                    Rc::new(RefCell::new(tmp_object_r.clone()))
                } else {
                    object.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let tmp_object = match &*new_object_r {
                        Object::CharN(cs) => Object::CharN(cs.iter().map(|c| !*c).collect()),
                        Object::ShortN(ns) => Object::ShortN(ns.iter().map(|n| !*n).collect()),
                        Object::IntN(ns) => Object::IntN(ns.iter().map(|n| !*n).collect()),
                        Object::LongN(ns) => Object::LongN(ns.iter().map(|n| !*n).collect()),
                        Object::UcharN(cs) => Object::UcharN(cs.iter().map(|c| !*c).collect()),
                        Object::UshortN(ns) => Object::UshortN(ns.iter().map(|n| !*n).collect()),
                        Object::UintN(ns) => Object::UintN(ns.iter().map(|n| !*n).collect()),
                        Object::UlongN(ns) => Object::UlongN(ns.iter().map(|n| !*n).collect()),
                        Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_not for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_not: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag, new_object))
            },
            _ => Err(FrontendError::Internal(String::from("op_not: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_not: too few or many arguments")))
    }
}

fn op_mul(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Char(((*c1 as i16) * (*c2 as i16)) as i8)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Short(((*n1 as i32) * (*n2 as i32)) as i16)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Int(((*n1 as i64) * (*n2 as i64)) as i32)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Long(((*n1 as i128) * (*n2 as i128)) as i64)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Uchar(((*c1 as u16) * (*c2 as u16)) as u8)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Ushort(((*n1 as u32) * (*n2 as u32)) as u16)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(((*n1 as u64) * (*n2 as u64)) as u32)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Ulong(((*n1 as u128) * (*n2 as u128)) as u64)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Float(n1 * n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Double(n1 * n2)),
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => Object::CharN(cs1.iter().zip(cs2.iter()).map(|p| ((*p.0 as i16) * (*p.1 as i16)) as i8).collect()),
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i32) * (*p.1 as i32)) as i16).collect()),
                        (Object::IntN(ns1), Object::IntN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i64) * (*p.1 as i64)) as i32).collect()),
                        (Object::LongN(ns1), Object::LongN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i128) * (*p.1 as i128)) as i64).collect()),
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => Object::UcharN(cs1.iter().zip(cs2.iter()).map(|p| ((*p.0 as u16) * (*p.1 as u16)) as u8).collect()),
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as u32) * (*p.1 as u32)) as u16).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as u64) * (*p.1 as u64)) as u32).collect()),
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as u128) * (*p.1 as u128)) as u64).collect()),
                        (Object::FloatN(ns1), Object::FloatN(ns2)) => Object::FloatN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 * *p.1).collect()),
                        (Object::DoubleN(ns1), Object::DoubleN(ns2)) => Object::DoubleN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 * *p.1).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_mul for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_mul: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_mul for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_mul: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_mul for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_mul: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_mul: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_mul: too few or many arguments")))
    }
}

fn op_div(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Char(c1), Value::Char(c2)) => {
                if *c2 != 0 {
                    Ok(Value::Char(*c1 / *c2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Short(n1), Value::Short(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Short(*n1 / *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Int(n1), Value::Int(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Int(*n1 / *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Long(n1), Value::Long(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Long(*n1 / *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Uchar(c1), Value::Uchar(c2)) => {
                if *c2 != 0 {
                    Ok(Value::Uchar(*c1 / *c2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Ushort(n1), Value::Ushort(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Ushort(*n1 / *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Uint(*n1 / *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Ulong(n1), Value::Ulong(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Ulong(*n1 / *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Float(n1 / n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Double(n1 / n2)),
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => {
                            let mut cs: Vec<i8> = Vec::new();
                            for (c1, c2) in cs1.iter().zip(cs2.iter()) {
                                if *c2 != 0 {
                                    cs.push(*c1 / *c2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::CharN(cs)
                        },
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => {
                            let mut ns: Vec<i16> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 / *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::ShortN(ns)
                        },
                        (Object::IntN(ns1), Object::IntN(ns2)) => {
                            let mut ns: Vec<i32> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 / *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::IntN(ns)
                        },
                        (Object::LongN(ns1), Object::LongN(ns2)) => {
                            let mut ns: Vec<i64> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 / *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::LongN(ns)
                        },
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => {
                            let mut cs: Vec<u8> = Vec::new();
                            for (c1, c2) in cs1.iter().zip(cs2.iter()) {
                                if *c2 != 0 {
                                    cs.push(*c1 / *c2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UcharN(cs)
                        },
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => {
                            let mut ns: Vec<u16> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 / *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UshortN(ns)
                        },
                        (Object::UintN(ns1), Object::UintN(ns2)) => {
                            let mut ns: Vec<u32> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 / *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UintN(ns)
                        },
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => {
                            let mut ns: Vec<u64> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 / *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UlongN(ns)
                        },
                        (Object::FloatN(ns1), Object::FloatN(ns2)) => Object::FloatN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 * *p.1).collect()),
                        (Object::DoubleN(ns1), Object::DoubleN(ns2)) => Object::DoubleN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 * *p.1).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_div for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_div: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_div for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_div: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_div for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_div: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_div: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_div: too few or many arguments")))
    }
}

fn op_rem(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Char(c1), Value::Char(c2)) => {
                if *c2 != 0 {
                    Ok(Value::Char(*c1 % *c2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Short(n1), Value::Short(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Short(*n1 % *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Int(n1), Value::Int(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Int(*n1 % *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Long(n1), Value::Long(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Long(*n1 % *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Uchar(c1), Value::Uchar(c2)) => {
                if *c2 != 0 {
                    Ok(Value::Uchar(*c1 % *c2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Ushort(n1), Value::Ushort(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Ushort(*n1 % *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Uint(*n1 % *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Ulong(n1), Value::Ulong(n2)) => {
                if *n2 != 0 {
                    Ok(Value::Ulong(*n1 % *n2))
                } else {
                    Err(FrontendError::Message(pos.clone(), String::from("division by zero")))
                }
            },
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => {
                            let mut cs: Vec<i8> = Vec::new();
                            for (c1, c2) in cs1.iter().zip(cs2.iter()) {
                                if *c2 != 0 {
                                    cs.push(*c1 % *c2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::CharN(cs)
                        },
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => {
                            let mut ns: Vec<i16> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 % *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::ShortN(ns)
                        },
                        (Object::IntN(ns1), Object::IntN(ns2)) => {
                            let mut ns: Vec<i32> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 % *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::IntN(ns)
                        },
                        (Object::LongN(ns1), Object::LongN(ns2)) => {
                            let mut ns: Vec<i64> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 % *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::LongN(ns)
                        },
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => {
                            let mut cs: Vec<u8> = Vec::new();
                            for (c1, c2) in cs1.iter().zip(cs2.iter()) {
                                if *c2 != 0 {
                                    cs.push(*c1 % *c2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UcharN(cs)
                        },
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => {
                            let mut ns: Vec<u16> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 % *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UshortN(ns)
                        },
                        (Object::UintN(ns1), Object::UintN(ns2)) => {
                            let mut ns: Vec<u32> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 % *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UintN(ns)
                        },
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => {
                            let mut ns: Vec<u64> = Vec::new();
                            for (n1, n2) in ns1.iter().zip(ns2.iter()) {
                                if *n2 != 0 {
                                    ns.push(*n1 % *n2);
                                } else {
                                    return Err(FrontendError::Message(pos.clone(), String::from("division by zero")));
                                }
                            }
                            Object::UlongN(ns)
                        },
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_rem for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_rem: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_rem for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_rem: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_rem for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_rem: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_rem: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_rem: too few or many arguments")))
    }
}

fn op_add(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Char(((*c1 as i16) + (*c2 as i16)) as i8)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Short(((*n1 as i32) + (*n2 as i32)) as i16)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Int(((*n1 as i64) + (*n2 as i64)) as i32)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Long(((*n1 as i128) + (*n2 as i128)) as i64)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Uchar(((*c1 as u16) + (*c2 as u16)) as u8)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Ushort(((*n1 as u32) + (*n2 as u32)) as u16)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(((*n1 as u64) + (*n2 as u64)) as u32)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Ulong(((*n1 as u128) + (*n2 as u128)) as u64)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Float(n1 + n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Double(n1 + n2)),
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => Object::CharN(cs1.iter().zip(cs2.iter()).map(|p| ((*p.0 as i16) + (*p.1 as i16)) as i8).collect()),
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i32) + (*p.1 as i32)) as i16).collect()),
                        (Object::IntN(ns1), Object::IntN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i64) + (*p.1 as i64)) as i32).collect()),
                        (Object::LongN(ns1), Object::LongN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i128) + (*p.1 as i128)) as i64).collect()),
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => Object::UcharN(cs1.iter().zip(cs2.iter()).map(|p| ((*p.0 as u16) + (*p.1 as u16)) as u8).collect()),
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as u32) + (*p.1 as u32)) as u16).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as u64) + (*p.1 as u64)) as u32).collect()),
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as u128) + (*p.1 as u128)) as u64).collect()),
                        (Object::FloatN(ns1), Object::FloatN(ns2)) => Object::FloatN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 + *p.1).collect()),
                        (Object::DoubleN(ns1), Object::DoubleN(ns2)) => Object::DoubleN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 + *p.1).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_add for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_add: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_add for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_add: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_add for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_add: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_add: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_add: too few or many arguments")))
    }
}

fn op_sub(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Char(((*c1 as i16) - (*c2 as i16)) as i8)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Short(((*n1 as i32) - (*n2 as i32)) as i16)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Int(((*n1 as i64) - (*n2 as i64)) as i32)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Long(((*n1 as i128) - (*n2 as i128)) as i64)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Uchar(((u8::MAX as u16) + 1 + (*c1 as u16) - (*c2 as u16)) as u8)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Ushort(((u16::MAX as u32) + 1 + (*n1 as u32) - (*n2 as u32)) as u16)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(((u32::MAX as u64) + 1 + (*n1 as u64) - (*n2 as u64)) as u32)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Ulong(((u64::MAX as u128) + 1 + (*n1 as u128) - (*n2 as u128)) as u64)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Float(n1 + n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Double(n1 + n2)),
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => Object::CharN(cs1.iter().zip(cs2.iter()).map(|p| ((*p.0 as i16) + (*p.1 as i16)) as i8).collect()),
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i32) + (*p.1 as i32)) as i16).collect()),
                        (Object::IntN(ns1), Object::IntN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i64) + (*p.1 as i64)) as i32).collect()),
                        (Object::LongN(ns1), Object::LongN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| ((*p.0 as i128) + (*p.1 as i128)) as i64).collect()),
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => Object::UcharN(cs1.iter().zip(cs2.iter()).map(|p| ((u8::MAX as u16) + 1 + (*p.0 as u16) - (*p.1 as u16)) as u8).collect()),
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| ((u16::MAX as u32) + 1 + (*p.0 as u32) - (*p.1 as u32)) as u16).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| ((u16::MAX as u64) + 1 + (*p.0 as u64) - (*p.1 as u64)) as u32).collect()),
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| ((u64::MAX as u128) + 1 + (*p.0 as u128) - (*p.1 as u128)) as u64).collect()),
                        (Object::FloatN(ns1), Object::FloatN(ns2)) => Object::FloatN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 + *p.1).collect()),
                        (Object::DoubleN(ns1), Object::DoubleN(ns2)) => Object::DoubleN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 + *p.1).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_sub for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_sub: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_sub for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_sub: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_sub for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_sub: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_sub: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_sub: too few or many arguments")))
    }
}

fn op_shl(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Char(c1), Value::Uint(n2)) => Ok(Value::Char(*c1 << (*n2 & (i8::BITS - 1)))),
            (Value::Short(n1), Value::Uint(n2)) => Ok(Value::Short(*n1 << (*n2 & (i16::BITS - 1)))),
            (Value::Int(n1), Value::Uint(n2)) => Ok(Value::Int(*n1 << (*n2 & (i32::BITS - 1)))),
            (Value::Long(n1), Value::Uint(n2)) => Ok(Value::Long(*n1 << (*n2 & (i64::BITS - 1)))),
            (Value::Uchar(c1), Value::Uint(n2)) => Ok(Value::Uchar(*c1 << (*n2 & (u8::BITS - 1)))),
            (Value::Ushort(n1), Value::Uint(n2)) => Ok(Value::Ushort(*n1 << (*n2 & (u16::BITS - 1)))),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(*n1 << (*n2 & (u32::BITS - 1)))),
            (Value::Ulong(n1), Value::Uint(n2)) => Ok(Value::Ulong(*n1 << (*n2 & (u64::BITS - 1)))),
            (Value::Object(shared_flag1, object1), Value::Uint(n2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let tmp_object = match &*new_object_r {
                        Object::CharN(cs1) => Object::CharN(cs1.iter().map(|c1| *c1 << (*n2 & (i8::BITS - 1))).collect()),
                        Object::ShortN(ns1) => Object::ShortN(ns1.iter().map(|n1| *n1 << (*n2 & (i16::BITS - 1))).collect()),
                        Object::IntN(ns1) => Object::IntN(ns1.iter().map(|n1| *n1 << (*n2 & (i32::BITS - 1))).collect()),
                        Object::LongN(ns1) => Object::LongN(ns1.iter().map(|n1| *n1 << (*n2 & (i64::BITS - 1))).collect()),
                        Object::UcharN(cs1) => Object::UcharN(cs1.iter().map(|c1| *c1 << (*n2 & (u8::BITS - 1))).collect()),
                        Object::UshortN(ns1) => Object::UshortN(ns1.iter().map(|n1| *n1 << (*n2 & (u16::BITS - 1))).collect()),
                        Object::UintN(ns1) => Object::UintN(ns1.iter().map(|n1| *n1 << (*n2 & (u32::BITS - 1))).collect()),
                        Object::UlongN(ns1) => Object::UlongN(ns1.iter().map(|n1| *n1 << (*n2 & (u64::BITS - 1))).collect()),
                        Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_shl for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_shl: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            _ => Err(FrontendError::Internal(String::from("op_shl: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_shl: too few or many arguments")))
    }
}

fn op_shr(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Char(c1), Value::Uint(n2)) => Ok(Value::Char(*c1 >> (*n2 & (i8::BITS - 1)))),
            (Value::Short(n1), Value::Uint(n2)) => Ok(Value::Short(*n1 >> (*n2 & (i16::BITS - 1)))),
            (Value::Int(n1), Value::Uint(n2)) => Ok(Value::Int(*n1 >> (*n2 & (i32::BITS - 1)))),
            (Value::Long(n1), Value::Uint(n2)) => Ok(Value::Long(*n1 >> (*n2 & (i64::BITS - 1)))),
            (Value::Uchar(c1), Value::Uint(n2)) => Ok(Value::Uchar(*c1 >> (*n2 & (u8::BITS - 1)))),
            (Value::Ushort(n1), Value::Uint(n2)) => Ok(Value::Ushort(*n1 >> (*n2 & (u16::BITS - 1)))),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(*n1 >> (*n2 & (u32::BITS - 1)))),
            (Value::Ulong(n1), Value::Uint(n2)) => Ok(Value::Ulong(*n1 >> (*n2 & (u64::BITS - 1)))),
            (Value::Object(shared_flag1, object1), Value::Uint(n2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let tmp_object = match &*new_object_r {
                        Object::CharN(cs1) => Object::CharN(cs1.iter().map(|c1| *c1 >> (*n2 & (i8::BITS - 1))).collect()),
                        Object::ShortN(ns1) => Object::ShortN(ns1.iter().map(|n1| *n1 >> (*n2 & (i16::BITS - 1))).collect()),
                        Object::IntN(ns1) => Object::IntN(ns1.iter().map(|n1| *n1 >> (*n2 & (i32::BITS - 1))).collect()),
                        Object::LongN(ns1) => Object::LongN(ns1.iter().map(|n1| *n1 >> (*n2 & (i64::BITS - 1))).collect()),
                        Object::UcharN(cs1) => Object::UcharN(cs1.iter().map(|c1| *c1 >> (*n2 & (u8::BITS - 1))).collect()),
                        Object::UshortN(ns1) => Object::UshortN(ns1.iter().map(|n1| *n1 >> (*n2 & (u16::BITS - 1))).collect()),
                        Object::UintN(ns1) => Object::UintN(ns1.iter().map(|n1| *n1 >> (*n2 & (u32::BITS - 1))).collect()),
                        Object::UlongN(ns1) => Object::UlongN(ns1.iter().map(|n1| *n1 >> (*n2 & (u64::BITS - 1))).collect()),
                        Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_shr for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_shr: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            _ => Err(FrontendError::Internal(String::from("op_shr: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_shr: too few or many arguments")))
    }
}

fn op_eq(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 == b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Bool(c1 == c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Bool(c1 == c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Object(_, object1), Value::Object(_, object2)) => {
                let object1_r = object1.borrow();
                let object2_r = object2.borrow();
                match (&*object1_r, &*object2_r) {
                    (Object::CharN(cs1), Object::CharN(cs2)) => Ok(Value::Bool(cs1 == cs2)),
                    (Object::ShortN(ns1), Object::ShortN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::IntN(ns1), Object::IntN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::LongN(ns1), Object::LongN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::UcharN(cs1), Object::UcharN(cs2)) => Ok(Value::Bool(cs1 == cs2)),
                    (Object::UshortN(ns1), Object::UshortN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::UintN(ns1), Object::UintN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::UlongN(ns1), Object::UlongN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::FloatN(ns1), Object::FloatN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::DoubleN(ns1), Object::DoubleN(ns2)) => Ok(Value::Bool(ns1 == ns2)),
                    (Object::Builtin(_, _), Object::Builtin(_, _)) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_eq for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_eq: invalid object"))),
                }
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_eq for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_eq: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_eq for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_eq: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_eq: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_eq: too few or many arguments")))
    }
}

fn op_ne(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 != b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Bool(c1 != c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Bool(c1 != c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Bool(n1 != n2)),
            (Value::Object(_, object1), Value::Object(_, object2)) => {
                let object1_r = object1.borrow();
                let object2_r = object2.borrow();
                match (&*object1_r, &*object2_r) {
                    (Object::CharN(cs1), Object::CharN(cs2)) => Ok(Value::Bool(cs1 != cs2)),
                    (Object::ShortN(ns1), Object::ShortN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::IntN(ns1), Object::IntN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::LongN(ns1), Object::LongN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::UcharN(cs1), Object::UcharN(cs2)) => Ok(Value::Bool(cs1 != cs2)),
                    (Object::UshortN(ns1), Object::UshortN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::UintN(ns1), Object::UintN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::UlongN(ns1), Object::UlongN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::FloatN(ns1), Object::FloatN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::DoubleN(ns1), Object::DoubleN(ns2)) => Ok(Value::Bool(ns1 != ns2)),
                    (Object::Builtin(_, _), Object::Builtin(_, _)) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_ne for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_ne: invalid object"))),
                }
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_ne for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_ne: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_ne for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_ne: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_ne: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_ne: too few or many arguments")))
    }
}

fn op_lt(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 < b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Bool(c1 < c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Bool(c1 < c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_lt for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_lt: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_lt for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_lt: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_lt: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_lt: too few or many arguments")))
    }
}

fn op_ge(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 >= b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Bool(c1 >= c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Bool(c1 >= c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Bool(n1 >= n2)),
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_ge for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_ge: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_ge for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_ge: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_ge: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_ge: too few or many arguments")))
    }
}

fn op_gt(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 > b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Bool(c1 > c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Bool(c1 > c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_gt for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_gt: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_gt for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_gt: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_gt: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_gt: too few or many arguments")))
    }
}

fn op_le(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 <= b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Bool(c1 <= c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Bool(c1 <= c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Float(n1), Value::Float(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Double(n1), Value::Double(n2)) => Ok(Value::Bool(n1 <= n2)),
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_le for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_le: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_le for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_le: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_le: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_le: too few or many arguments")))
    }
}

fn op_and(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 & b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Char(c1 & c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Short(n1 & n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Int(n1 & n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Long(n1 & n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Uchar(c1 & c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Ushort(n1 & n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(n1 & n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Ulong(n1 & n2)),
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => Object::CharN(cs1.iter().zip(cs2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::IntN(ns1), Object::IntN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::LongN(ns1), Object::LongN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => Object::UcharN(cs1.iter().zip(cs2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| p.0 & p.1).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_and for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_xor: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_xor for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_xor: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_xor for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_xor: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_xor: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_xor: too few or many arguments")))
    }
}

fn op_xor(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 ^ b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Char(c1 ^ c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Short(n1 ^ n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Int(n1 ^ n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Long(n1 ^ n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Uchar(c1 ^ c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Ushort(n1 ^ n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(n1 ^ n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Ulong(n1 ^ n2)),
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => Object::CharN(cs1.iter().zip(cs2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::IntN(ns1), Object::IntN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::LongN(ns1), Object::LongN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => Object::UcharN(cs1.iter().zip(cs2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| p.0 ^ p.1).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_xor for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_xor: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_xor for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_xor: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_xor for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_xor: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_xor: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_xor: too few or many arguments")))
    }
}

fn op_or(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Bool(b1), Value::Bool(b2)) => Ok(Value::Bool(b1 | b2)),
            (Value::Char(c1), Value::Char(c2)) => Ok(Value::Char(c1 | c2)),
            (Value::Short(n1), Value::Short(n2)) => Ok(Value::Short(n1 | n2)),
            (Value::Int(n1), Value::Int(n2)) => Ok(Value::Int(n1 | n2)),
            (Value::Long(n1), Value::Long(n2)) => Ok(Value::Long(n1 | n2)),
            (Value::Uchar(c1), Value::Uchar(c2)) => Ok(Value::Uchar(c1 | c2)),
            (Value::Ushort(n1), Value::Ushort(n2)) => Ok(Value::Ushort(n1 | n2)),
            (Value::Uint(n1), Value::Uint(n2)) => Ok(Value::Uint(n1 | n2)),
            (Value::Ulong(n1), Value::Ulong(n2)) => Ok(Value::Ulong(n1 | n2)),
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::CharN(cs2)) => Object::CharN(cs1.iter().zip(cs2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::ShortN(ns1), Object::ShortN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::IntN(ns1), Object::IntN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::LongN(ns1), Object::LongN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::UcharN(cs1), Object::UcharN(cs2)) => Object::UcharN(cs1.iter().zip(cs2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::UshortN(ns1), Object::UshortN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::UlongN(ns1), Object::UlongN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| p.0 | p.1).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_or for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_or: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_or for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_or: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_or for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_or: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_or: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_or: too few or many arguments")))
    }
}

fn op_get_nth(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Object(_, object1), Value::Ulong(n2)) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Array(elem_values) => {
                        if *n2 < (elem_values.len() as u64) {
                            Ok(elem_values[*n2 as usize].clone())
                        } else {
                            Err(FrontendError::Message(pos.clone(), String::from("index out of bounds")))
                        }
                    },
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_get_nth for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_get_nth: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_get_nth: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_get_nth: too few or many arguments")))
    }
}

fn op_get2_nth(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Object(shared_flag, object1), Value::Ulong(n2)) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Array(elem_values) => {
                        if *n2 < (elem_values.len() as u64) {
                            Ok(Value::Object(*shared_flag, Rc::new(RefCell::new(Object::Tuple(vec![elem_values[*n2 as usize].clone(), arg_values[1].clone()])))))
                        } else {
                            Err(FrontendError::Message(pos.clone(), String::from("index out of bounds")))
                        }
                    },
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_get2_nth for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("op_get2_nth: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("op_get2_nth: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_get2_nth: too few or many arguments")))
    }
}

fn op_set_nth(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 3 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Object(shared_flag, object1), Value::Ulong(n2)) => {
                let new_object = if *shared_flag == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    match &mut *new_object_r {
                        Object::Array(elem_values) => {
                            if *n2 < (elem_values.len() as u64) {
                                elem_values[*n2 as usize] = arg_values[2].clone();
                            } else {
                                return Err(FrontendError::Message(pos.clone(), String::from("index out of bounds")))
                            }
                        },
                        Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function op_set_nth for value of built-in variable"))),
                        _ => return Err(FrontendError::Internal(String::from("op_set_nth: invalid object"))),
                    }
                }
                Ok(Value::Object(*shared_flag, new_object))
            },
            _ => Err(FrontendError::Internal(String::from("op_set_nth: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("op_set_nth: too few or many arguments")))
    }
}

fn slice(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
       match &arg_values[0] {
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Array(elem_values) => {
                        let idx = ref_values.add_value(RefValue(RefValueFlag::None, arg_values[0].clone()));
                        Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Slice(idx, vec![0], elem_values.len())))))
                    },
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function slice for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("slice: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("slice: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("slice: too few or many arguments")))
    }
}

fn slice_from_ref2(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos, s: &str) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
        match &arg_values[0] {
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Ref(idx, offs) => {
                        match ref_values.value(*idx) {
                            Some(RefValue(_, value)) => {
                                let mut tmp_value = value.clone();
                                for off in offs {
                                    let new_tmp_value = match tmp_value {
                                        Value::Object(_, object) => {
                                            let object_r = object.borrow();
                                            match &*object_r {
                                                Object::Array(elem_values) => {
                                                    if *off < elem_values.len() {
                                                        elem_values[*off].clone()
                                                    } else {
                                                        return Err(FrontendError::Internal(String::from("slice_from_ref2: index out of bounds")));
                                                    }
                                                },
                                                _ => return Err(FrontendError::Internal(String::from("slice_from_ref2: invalid object"))),
                                            }
                                        },
                                        _ => return Err(FrontendError::Internal(String::from("slice_from_ref2: invalid value"))),
                                    };
                                    tmp_value = new_tmp_value;
                                }
                                match &tmp_value {
                                    Value::Object(_, object) => {
                                        let object_r = object.borrow();
                                        match &*object_r {
                                            Object::Array(elem_values) => {
                                                let mut new_offs = offs.clone();
                                                new_offs.push(0);
                                                Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Slice(*idx, new_offs, elem_values.len())))))
                                            },
                                            _ => Err(FrontendError::Internal(String::from("slice_from_ref2: invalid object"))),
                                        }
                                    },
                                    _ => Err(FrontendError::Internal(String::from("slice_from_ref2: invalid value"))),
                                }  
                            },
                            _ => Err(FrontendError::Internal(String::from("slice_from_ref2: no reference values"))),
                        }
                    },
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), format!("can't evaluate function {} for value of built-in variable", s))),
                    _ => Err(FrontendError::Internal(String::from("slice_from_ref2: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("slice_from_ref2: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("slice_from_ref2: too few or many arguments")))
    }
}

fn slice_from_ref(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ slice_from_ref2(arg_values, ref_values, pos, "slice_from_ref") }

fn global_slice(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
        match &arg_values[0] {
            Value::Object(_, object) => {
                let object_r = object.borrow();
                match &*object_r {
                    Object::Array(elem_values) => {
                        let idx = ref_values.add_value(RefValue(RefValueFlag::None, arg_values[0].clone()));
                        Ok(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Slice(idx, vec![0], elem_values.len())))))
                    },
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function global_slice for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("global_slice: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("global_slice: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("slice: too few or many arguments")))
    }
}

fn global_slice_from_ref(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ slice_from_ref2(arg_values, ref_values, pos, "global_slice_from_ref") }

fn shl_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos, n: u32) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::UintN(ns2)) => Object::CharN(cs1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (i8::BITS - 1))).collect()),
                        (Object::ShortN(ns1), Object::UintN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (i16::BITS - 1))).collect()),
                        (Object::IntN(ns1), Object::UintN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (i32::BITS - 1))).collect()),
                        (Object::LongN(ns1), Object::UintN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (i64::BITS - 1))).collect()),
                        (Object::UcharN(cs1), Object::UintN(ns2)) => Object::UcharN(cs1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (u8::BITS - 1))).collect()),
                        (Object::UshortN(ns1), Object::UintN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (u16::BITS - 1))).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (u32::BITS - 1))).collect()),
                        (Object::UlongN(ns1), Object::UintN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 << (*p.1 & (u64::BITS - 1))).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), format!("can't evaluate function shl{} for value of built-in variable", n))),
                        _ => return Err(FrontendError::Internal(String::from("shl_n: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), format!("can't evaluate function shl{} for value of built-in variable", n))),
                    _ => Err(FrontendError::Internal(String::from("shl_n: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), format!("can't evaluate function shl{} for value of built-in variable", n))),
                    _ => Err(FrontendError::Internal(String::from("shl_n: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("shl_n: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("shl_n: too few or many arguments")))
    }
}

fn shl2(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shl_n(arg_values, ref_values, pos, 2) }

fn shl3(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shl_n(arg_values, ref_values, pos, 3) }

fn shl4(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shl_n(arg_values, ref_values, pos, 4) }

fn shl8(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shl_n(arg_values, ref_values, pos, 8) }

fn shl16(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shl_n(arg_values, ref_values, pos, 16) }

fn shr_n(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos, n: u32) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Object(shared_flag1, object1), Value::Object(_, object2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let object2_r = object2.borrow();
                    let tmp_object = match (&*new_object_r, &*object2_r) {
                        (Object::CharN(cs1), Object::UintN(ns2)) => Object::CharN(cs1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (i8::BITS - 1))).collect()),
                        (Object::ShortN(ns1), Object::UintN(ns2)) => Object::ShortN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (i16::BITS - 1))).collect()),
                        (Object::IntN(ns1), Object::UintN(ns2)) => Object::IntN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (i32::BITS - 1))).collect()),
                        (Object::LongN(ns1), Object::UintN(ns2)) => Object::LongN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (i64::BITS - 1))).collect()),
                        (Object::UcharN(cs1), Object::UintN(ns2)) => Object::UcharN(cs1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (u8::BITS - 1))).collect()),
                        (Object::UshortN(ns1), Object::UintN(ns2)) => Object::UshortN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (u16::BITS - 1))).collect()),
                        (Object::UintN(ns1), Object::UintN(ns2)) => Object::UintN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (u32::BITS - 1))).collect()),
                        (Object::UlongN(ns1), Object::UintN(ns2)) => Object::UlongN(ns1.iter().zip(ns2.iter()).map(|p| *p.0 >> (*p.1 & (u64::BITS - 1))).collect()),
                        (Object::Builtin(_, _), Object::Builtin(_, _)) => return Err(FrontendError::Message(pos.clone(), format!("can't evaluate function shr{} for value of built-in variable", n))),
                        _ => return Err(FrontendError::Internal(String::from("shr_n: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (Value::Object(_, object1), _) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), format!("can't evaluate function shr{} for value of built-in variable", n))),
                    _ => Err(FrontendError::Internal(String::from("shr_n: invalid object"))),
                }
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), format!("can't evaluate function shr{} for value of built-in variable", n))),
                    _ => Err(FrontendError::Internal(String::from("shr_n: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("shr_n: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("shr_n: too few or many arguments")))
    }
}

fn shr2(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shr_n(arg_values, ref_values, pos, 2) }

fn shr3(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shr_n(arg_values, ref_values, pos, 3) }

fn shr4(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shr_n(arg_values, ref_values, pos, 4) }

fn shr8(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shr_n(arg_values, ref_values, pos, 8) }

fn shr16(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ shr_n(arg_values, ref_values, pos, 16) }

fn len(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{
    if arg_values.len() == 1 {
        match &arg_values[0] {
            Value::Object(_, object1) => {
                let object1_r = object1.borrow();
                match &*object1_r {
                    Object::Array(elem_values) => Ok(Value::Ulong(elem_values.len() as u64)),
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function len for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("len: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("len: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("len: too few or many arguments")))
    }
}

fn get_ref2(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos, s: &str) -> FrontendResult<Value>
{
    if arg_values.len() == 2 {
        match (&arg_values[0], &arg_values[1]) {
            (Value::Object(shared_flag1, object1), Value::Ulong(n2)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let tmp_object = match &*new_object_r {
                        Object::Slice(idx, offs, len) => {
                            let mut new_offs = offs.clone();
                            match new_offs.last_mut() {
                                Some(new_off) => {
                                    if *n2 < (*len as u64) {
                                        *new_off += *n2 as usize;
                                    } else {
                                        return Err(FrontendError::Message(pos.clone(), String::from("index out of bounds")))
                                    }
                                },
                                None => return Err(FrontendError::Internal(String::from("get_ref2: no last offset"))),
                            }
                            Object::Ref(*idx, new_offs)
                        },
                        Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), format!("can't evaluate function {} for value of built-in variable", s))),
                        _ => return Err(FrontendError::Internal(String::from("get_ref2: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (_, Value::Object(_, object2)) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), format!("can't evaluate function {} for value of built-in variable", s))),
                    _ => Err(FrontendError::Internal(String::from("get_ref2: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("get_ref2: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("get_ref2: too few or many arguments")))
    }
}

fn get_ref(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ get_ref2(arg_values, ref_values, pos, "get_ref") }

fn get_global_ref(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ get_ref2(arg_values, ref_values, pos, "get_global_ref") }

fn get_slice2(arg_values: &[Value], _ref_values: &mut RefValues, pos: &Pos, s: &str) -> FrontendResult<Value>
{
    if arg_values.len() == 3 {
        match (&arg_values[0], &arg_values[1], &arg_values[2]) {
            (Value::Object(shared_flag1, object1), Value::Ulong(n2), Value::Ulong(n3)) => {
                let new_object = if *shared_flag1 == SharedFlag::Shared {
                    let tmp_object1 = object1.clone();
                    let tmp_object1_r = tmp_object1.borrow();
                    Rc::new(RefCell::new(tmp_object1_r.clone()))
                } else {
                    object1.clone()
                };
                {
                    let mut new_object_r = new_object.borrow_mut();
                    let tmp_object = match &*new_object_r {
                        Object::Slice(idx, offs, len) => {
                            let mut new_offs = offs.clone();
                            let new_len = match new_offs.last_mut() {
                                Some(new_off) => {
                                    let tmp_off = min(*n2, *len as u64);
                                    *new_off += tmp_off as usize;
                                    (min(max(*n3, tmp_off), *len as u64) - tmp_off) as usize
                                },
                                None => return Err(FrontendError::Internal(String::from("get_slice2: no last offset"))),
                            };
                            Object::Slice(*idx, new_offs, new_len)
                        },
                        Object::Builtin(_, _) => return Err(FrontendError::Message(pos.clone(), format!("can't evaluate function {} for value of built-in variable", s))),
                        _ => return Err(FrontendError::Internal(String::from("get_slice2: invalid object"))),
                    };
                    *new_object_r = tmp_object;
                }
                Ok(Value::Object(*shared_flag1, new_object))
            },
            (_, Value::Object(_, object2), _) => {
                let object2_r = object2.borrow();
                match &*object2_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), format!("can't evaluate function {} for value of built-in variable", s))),
                    _ => Err(FrontendError::Internal(String::from("get_slice2: invalid object"))),
                }
            },
            (_, _, Value::Object(_, object3)) => {
                let object3_r = object3.borrow();
                match &*object3_r {
                    Object::Builtin(_, _) => Err(FrontendError::Message(pos.clone(), String::from("can't evaluate function get_*slice for value of built-in variable"))),
                    _ => Err(FrontendError::Internal(String::from("get_slice2: invalid object"))),
                }
            },
            _ => Err(FrontendError::Internal(String::from("get_slice2: invalid value"))),
        }
    } else {
        Err(FrontendError::Internal(String::from("get_slice2: too few or many arguments")))
    }
}

fn get_slice(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ get_slice2(arg_values, ref_values, pos, "get_slice") }

fn get_global_slice(arg_values: &[Value], ref_values: &mut RefValues, pos: &Pos) -> FrontendResult<Value>
{ get_slice2(arg_values, ref_values, pos, "get_global_slice") }

#[derive(Clone, Debug)]
pub struct Evals
{
    funs: HashMap<(String, Option<TypeName>), fn(&[Value], &mut RefValues, &Pos) -> FrontendResult<Value>>,
}

impl Evals
{
    pub fn new() -> Self
    {
        let mut funs: HashMap<(String, Option<TypeName>), fn(&[Value], &mut RefValues, &Pos) -> FrontendResult<Value>> = HashMap::new();
        // charN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("char{}", n), None), char_n);
        }
        // shortN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("short{}", n), None), short_n);
        }
        // intN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("int{}", n), None), int_n);
        }
        // longN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("long{}", n), None), long_n);
        }
        // ucharN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("uchar{}", n), None), uchar_n);
        }
        // ushortN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("ushort{}", n), None), ushort_n);
        }
        // uintN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("uint{}", n), None), uint_n);
        }
        // ulongN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("ulong{}", n), None), ulong_n);
        }
        // floatN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("float{}", n), None), float_n);
        }
        // doubleN
        for n in [2, 3, 4, 8, 16] {
            funs.insert((format!("double{}", n), None), double_n);
        }
        // ref
        funs.insert((String::from("ref"), None), reference);
        // global_ref
        funs.insert((String::from("global_ref"), None), global_ref);
        // op_neg
        for s in ["Char", "Short", "Int", "Long", "Float", "Double", "PtrdiffT", "IntptrT"] {
            funs.insert((String::from("op_neg"), Some(TypeName::Name(String::from(s)))), op_neg);
        }
        for s in ["Char", "Short", "Int", "Long", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_neg"), Some(TypeName::Name(format!("{}{}", s, n)))), op_neg);
            }
        }
        // op_not
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_not"), Some(TypeName::Name(String::from(s)))), op_not);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_not"), Some(TypeName::Name(format!("{}{}", s, n)))), op_not);
            }
        }
        // op_mul
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_mul"), Some(TypeName::Name(String::from(s)))), op_mul);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_mul"), Some(TypeName::Name(format!("{}{}", s, n)))), op_mul);
            }
        }
        // op_div
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_div"), Some(TypeName::Name(String::from(s)))), op_div);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_div"), Some(TypeName::Name(format!("{}{}", s, n)))), op_div);
            }
        }
        // op_rem
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_rem"), Some(TypeName::Name(String::from(s)))), op_rem);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_rem"), Some(TypeName::Name(format!("{}{}", s, n)))), op_rem);
            }
        }
        // op_add
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_add"), Some(TypeName::Name(String::from(s)))), op_add);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_rem"), Some(TypeName::Name(format!("{}{}", s, n)))), op_add);
            }
        }
        // op_sub
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_sub"), Some(TypeName::Name(String::from(s)))), op_sub);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_sub"), Some(TypeName::Name(format!("{}{}", s, n)))), op_sub);
            }
        }
        // op_shl
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_shl"), Some(TypeName::Name(String::from(s)))), op_shl);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_shl"), Some(TypeName::Name(format!("{}{}", s, n)))), op_shl);
            }
        }
        // op_shl
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_shr"), Some(TypeName::Name(String::from(s)))), op_shr);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_shr"), Some(TypeName::Name(format!("{}{}", s, n)))), op_shr);
            }
        }
        // op_eq
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_eq"), Some(TypeName::Name(String::from(s)))), op_eq);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_eq"), Some(TypeName::Name(format!("{}{}", s, n)))), op_eq);
            }
        }
        // op_ne
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_ne"), Some(TypeName::Name(String::from(s)))), op_ne);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_ne"), Some(TypeName::Name(format!("{}{}", s, n)))), op_ne);
            }
        }
        // op_lt
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_lt"), Some(TypeName::Name(String::from(s)))), op_lt);
        }
        // op_ge
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_ge"), Some(TypeName::Name(String::from(s)))), op_ge);
        }
        // op_gt
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_gt"), Some(TypeName::Name(String::from(s)))), op_gt);
        }
        // op_le
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "Float", "Double", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_le"), Some(TypeName::Name(String::from(s)))), op_le);
        }
        // op_and
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_and"), Some(TypeName::Name(String::from(s)))), op_and);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_and"), Some(TypeName::Name(format!("{}{}", s, n)))), op_and);
            }
        }
        // op_xor
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_xor"), Some(TypeName::Name(String::from(s)))), op_xor);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_xor"), Some(TypeName::Name(format!("{}{}", s, n)))), op_xor);
            }
        }
        // op_or
        for s in ["Bool", "Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong", "SizeT", "PtrdiffT", "IntptrT", "UintptrT"] {
            funs.insert((String::from("op_or"), Some(TypeName::Name(String::from(s)))), op_or);
        }
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            for n in [2, 3, 4, 8, 16] {
                funs.insert((String::from("op_or"), Some(TypeName::Name(format!("{}{}", s, n)))), op_or);
            }
        }
        // op_get_nth
        funs.insert((String::from("op_get_nth"), Some(TypeName::Array(None))), op_get_nth);
        // op_get2_nth
        funs.insert((String::from("op_get2_nth"), Some(TypeName::Array(None))), op_get2_nth);
        // op_set_nth
        funs.insert((String::from("op_set_nth"), Some(TypeName::Array(None))), op_set_nth);
        // slice
        funs.insert((String::from("slice"), Some(TypeName::Array(None))), slice);
        // slice_from_ref
        funs.insert((String::from("slice_from_ref"), Some(TypeName::Array(None))), slice_from_ref);
        // global_slice
        funs.insert((String::from("global_slice"), Some(TypeName::Array(None))), global_slice);
        // global_slice_from_ref
        funs.insert((String::from("global_slice_from_ref"), Some(TypeName::Array(None))), global_slice_from_ref);
        // shl2
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shl2"), Some(TypeName::Name(format!("{}2", s)))), shl2);
        }
        // shl3
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shl3"), Some(TypeName::Name(format!("{}3", s)))), shl3);
        }
        // shl4
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shl4"), Some(TypeName::Name(format!("{}4", s)))), shl4);
        }
        // shl8
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shl8"), Some(TypeName::Name(format!("{}8", s)))), shl8);
        }
        // shl16
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shl16"), Some(TypeName::Name(format!("{}16", s)))), shl16);
        }
        // shr2
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shr2"), Some(TypeName::Name(format!("{}2", s)))), shr2);
        }
        // shr3
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shr3"), Some(TypeName::Name(format!("{}3", s)))), shr3);
        }
        // shr4
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shr4"), Some(TypeName::Name(format!("{}4", s)))), shr4);
        }
        // shr8
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shr8"), Some(TypeName::Name(format!("{}8", s)))), shr8);
        }
        // shr16
        for s in ["Char", "Short", "Int", "Long", "Uchar", "Ushort", "Uint", "Ulong"] {
            funs.insert((String::from("shr16"), Some(TypeName::Name(format!("{}16", s)))), shr16);
        }
        // len
        funs.insert((String::from("len"), Some(TypeName::Array(None))), len);
        // get_ref
        funs.insert((String::from("get_ref"), Some(TypeName::Name(String::from("Slice")))), get_ref);
        // get_global_ref
        funs.insert((String::from("get_global_ref"), Some(TypeName::Name(String::from("GlobalSlice")))), get_global_ref);
        // get_slice
        funs.insert((String::from("get_slice"), Some(TypeName::Name(String::from("Slice")))), get_slice);
        // get_global_slice
        funs.insert((String::from("get_global_slice"), Some(TypeName::Name(String::from("GlobalSlice")))), get_global_slice);
        Evals { funs, }
    }

    pub fn new_empty() -> Self
    { Evals { funs: HashMap::new(), } }
    
    pub fn funs(&self) -> &HashMap<(String, Option<TypeName>), fn(&[Value], &mut RefValues, &Pos) -> FrontendResult<Value>>
    { &self.funs }

    pub fn fun(&self, key: &(String, Option<TypeName>)) -> Option<fn(&[Value], &mut RefValues, &Pos) -> FrontendResult<Value>>
    {
        match self.funs.get(key) {
            Some(fun) => Some(*fun),
            None => None,
        }
    }
    
    pub fn add_fun(&mut self, key: (String, Option<TypeName>), fun: fn(&[Value], &mut RefValues, &Pos) -> FrontendResult<Value>)
    { self.funs.insert(key, fun); }

    pub fn remove_fun(&mut self, key: &(String, Option<TypeName>)) -> bool
    { self.funs.remove(key).is_some() }
}
