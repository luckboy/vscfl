//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::rc::*;
use super::*;

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_type_value()
{
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new());
    assert_eq!(String::from("Int"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_tuple_type_value()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value4 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, vec![type_value1, type_value2, type_value3, type_value4]);
    assert_eq!(String::from("(Int, t1, Float, t2)"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_fun_type_value()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Double")), Vec::new()));
    let type_value4 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Fun, vec![type_value1, type_value2, type_value3, type_value4]);
    assert_eq!(String::from("(Float, t1, Double) -> t2"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_array_type_value_with_lenght()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Array(Some(10)), vec![type_value1]);
    assert_eq!(String::from("[Float; 10]"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_array_type_value_without_lenght()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Array(None), vec![type_value1]);
    assert_eq!(String::from("[Int; _]"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_named_type_value()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value4 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3, type_value4]);
    assert_eq!(String::from("T<Int, t1, Float, t2>"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_parameter_type_value()
{
    let type_value = TypeValue::Param(UniqFlag::None, LocalType::new(0));
    assert_eq!(String::from("t1"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_unique_tuple_type_value()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value4 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::Uniq, TypeValueName::Tuple, vec![type_value1, type_value2, type_value3, type_value4]);
    assert_eq!(String::from("uniq (Int, t1, Float, t2)"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_unique_fun_type_value()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Double")), Vec::new()));
    let type_value4 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::Uniq, TypeValueName::Fun, vec![type_value1, type_value2, type_value3, type_value4]);
    assert_eq!(String::from("uniq (Float, t1, Double) -> t2"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_unique_array_type_value_with_lenght()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value = TypeValue::Type(UniqFlag::Uniq, TypeValueName::Array(Some(10)), vec![type_value1]);
    assert_eq!(String::from("uniq [Float; 10]"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_unique_array_type_value_without_lenght()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value = TypeValue::Type(UniqFlag::Uniq, TypeValueName::Array(None), vec![type_value1]);
    assert_eq!(String::from("uniq [Int; _]"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_unique_named_type_value()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let type_value4 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::Uniq, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3, type_value4]);
    assert_eq!(String::from("uniq T<Int, t1, Float, t2>"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_to_string_without_fun_returns_string_for_unique_parameter_type_value()
{
    let type_value = TypeValue::Param(UniqFlag::Uniq, LocalType::new(0));
    assert_eq!(String::from("uniq t1"), type_value.to_string_without_fun());
}

#[test]
fn test_type_value_substitute_substitutes_type_values()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3]);
    let a_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let b_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    match type_value.substitute(&[a_type_value, b_type_value]) {
        Ok(Some(res_type_value)) => assert_eq!(String::from("T<Int, Float, Char>"), res_type_value.to_string_without_fun()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_value_substitute_substitutes_type_values_with_type_parameters()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)));
    let type_value4 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3, type_value4]);
    let a_type_value = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(3)));
    let b_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let c_type_value = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(4)));
    match type_value.substitute(&[a_type_value, b_type_value, c_type_value]) {
        Ok(Some(res_type_value)) => assert_eq!(String::from("T<Int, t4, t5, Float>"), res_type_value.to_string_without_fun()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_value_substitute_substitutes_unique_type_values()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3]);
    let a_type_value = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(2)));
    let b_type_value = Rc::new(TypeValue::Type(UniqFlag::Uniq, TypeValueName::Name(String::from("Float")), Vec::new()));
    match type_value.substitute(&[a_type_value, b_type_value]) {
        Ok(Some(res_type_value)) => assert_eq!(String::from("T<Int, uniq t3, uniq Float>"), res_type_value.to_string_without_fun()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_value_substitute_substitutes_type_values_for_unique_type_parameters()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3]);
    let a_type_value = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)));
    let b_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    match type_value.substitute(&[a_type_value, b_type_value]) {
        Ok(Some(res_type_value)) => assert_eq!(String::from("T<Int, uniq t3, uniq Float>"), res_type_value.to_string_without_fun()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_value_substitute_substitutes_unique_type_values_for_unique_type_parameters()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3]);
    let a_type_value = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(2)));
    let b_type_value = Rc::new(TypeValue::Type(UniqFlag::Uniq, TypeValueName::Name(String::from("Float")), Vec::new()));
    match type_value.substitute(&[a_type_value, b_type_value]) {
        Ok(Some(res_type_value)) => assert_eq!(String::from("T<Int, uniq t3, uniq Float>"), res_type_value.to_string_without_fun()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_value_substitute_substitutes_nested_type_values_for_nested_type_value()
{
    let type_value11 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value12 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("U")), vec![type_value11, type_value12]));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2]);
    let a_type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let a_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Double")), Vec::new()));
    let a_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("V")), vec![a_type_value1, a_type_value2]));
    let b_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    match type_value.substitute(&[a_type_value, b_type_value]) {
        Ok(Some(res_type_value)) => assert_eq!(String::from("T<U<Int, Char>, V<Float, Double>>"), res_type_value.to_string_without_fun()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_value_substitute_does_not_substitutes_type_values_for_type_value_without_parameters()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Long")), Vec::new()));
    let type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Short")), Vec::new()));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3]);
    let a_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    let b_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    match type_value.substitute(&[a_type_value, b_type_value]) {
        Ok(None) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_value_substitute_complains_on_no_type_value_for_type_parameter()
{
    let type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let type_value3 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    let type_value = TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("T")), vec![type_value1, type_value2, type_value3]);
    let a_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    match type_value.substitute(&[a_type_value]) {
        Err(_) => assert!(true),
        _ => assert!(false),
    }
}
