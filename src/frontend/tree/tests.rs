//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::*;
use crate::frontend::lexer::*;
use crate::frontend::namer::*;
use crate::frontend::parser::*;
use crate::frontend::typer::*;
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

#[test]
fn test_local_types_set_defined_type_sets_defined_type()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Float, uniq u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: U<Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(2), local_types.set_defined_type(&typ));
                            assert_eq!(3, local_types.type_entries().len());
                            match local_types.type_entry(LocalType::new(0)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[0], type_param_entry));
                                    assert_eq!(LocalType::new(0), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(1)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[1], type_param_entry));
                                    assert_eq!(LocalType::new(1), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(2)) {
                                Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(typ.type_value(), type_value)),
                                _ => assert!(false),
                            }
                            assert_eq!(3, local_types.eq_type_param_entries().len());
                            match local_types.eq_type_param_entry(LocalType::new(0)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(1)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(2)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            // t1 t2 t3
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            //    t2 t3
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                            assert_eq!(typ.eq_type_param_set(), local_types.orig_eq_type_param_set());
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_local_types_set_defined_type_sets_defined_type_with_equal_parameters()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Float;
trait T<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t, u, v, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: T<Float>, v: T<Char>, t == u == v";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(3), local_types.set_defined_type(&typ));
                            assert_eq!(4, local_types.type_entries().len());
                            match local_types.type_entry(LocalType::new(0)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[0], type_param_entry));
                                    assert_eq!(LocalType::new(0), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(1)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[1], type_param_entry));
                                    assert_eq!(LocalType::new(1), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(2)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[2], type_param_entry));
                                    assert_eq!(LocalType::new(2), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(3)) {
                                Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(typ.type_value(), type_value)),
                                _ => assert!(false),
                            }
                            assert_eq!(4, local_types.eq_type_param_entries().len());
                            match local_types.eq_type_param_entry(LocalType::new(0)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(2, eq_type_param_entry.local_types.len());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(1)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(2, eq_type_param_entry.local_types.len());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(2)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(2, eq_type_param_entry.local_types.len());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(3)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            // t1 t2 t3 t 4
                            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
                            //    t2 t3 t4
                            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
                            //       t3 t4 
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
                            assert_eq!(typ.eq_type_param_set(), local_types.orig_eq_type_param_set());
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_local_types_set_defined_fun_types_sets_defined_function_types()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t, uniq u) -> Float";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: U<Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_defined_fun_types(&typ) {
                                Some(fun_local_types) => {
                                    assert_eq!(vec![LocalType::new(2), LocalType::new(3), LocalType::new(4)], fun_local_types);
                                },
                                None => assert!(false),
                            }
                            assert_eq!(5, local_types.type_entries().len());
                            match local_types.type_entry(LocalType::new(0)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[0], type_param_entry));
                                    assert_eq!(LocalType::new(0), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(1)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[1], type_param_entry));
                                    assert_eq!(LocalType::new(1), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match &**typ.type_value() {
                                TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values) => {
                                    match local_types.type_entry(LocalType::new(2)) {
                                        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&type_values[0], type_value)),
                                        _ => assert!(false),
                                    }
                                    match local_types.type_entry(LocalType::new(3)) {
                                        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&type_values[1], type_value)),
                                        _ => assert!(false),
                                    }
                                    match local_types.type_entry(LocalType::new(4)) {
                                        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&type_values[2], type_value)),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(5, local_types.eq_type_param_entries().len());
                            match local_types.eq_type_param_entry(LocalType::new(0)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(1)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(2)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(3)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(4)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            // t1 t2 t3 t4 t5
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(4)));
                            //    t2 t3 t4 t5
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(4)));
                            //       t3 t4 t5
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(4)));
                            //          t4 t5
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(3), LocalType::new(4)));
                            assert_eq!(typ.eq_type_param_set(), local_types.orig_eq_type_param_set());
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_local_types_set_defined_fun_types_sets_defined_function_types_with_equal_type_parameters()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Float;
trait T<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t, u, v) -> Float";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: T<Float>, v: T<Char>, t == u == v";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_defined_fun_types(&typ) {
                                Some(fun_local_types) => {
                                    assert_eq!(vec![LocalType::new(3), LocalType::new(4), LocalType::new(5), LocalType::new(6)], fun_local_types);
                                },
                                None => assert!(false),
                            }
                            assert_eq!(7, local_types.type_entries().len());
                            match local_types.type_entry(LocalType::new(0)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[0], type_param_entry));
                                    assert_eq!(LocalType::new(0), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(1)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[1], type_param_entry));
                                    assert_eq!(LocalType::new(1), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(2)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[2], type_param_entry));
                                    assert_eq!(LocalType::new(2), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match &**typ.type_value() {
                                TypeValue::Type(UniqFlag::None, TypeValueName::Fun, type_values) => {
                                    match local_types.type_entry(LocalType::new(3)) {
                                        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&type_values[0], type_value)),
                                        _ => assert!(false),
                                    }
                                    match local_types.type_entry(LocalType::new(4)) {
                                        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&type_values[1], type_value)),
                                        _ => assert!(false),
                                    }
                                    match local_types.type_entry(LocalType::new(5)) {
                                        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&type_values[2], type_value)),
                                        _ => assert!(false),
                                    }
                                    match local_types.type_entry(LocalType::new(6)) {
                                        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&type_values[3], type_value)),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(7, local_types.eq_type_param_entries().len());
                            match local_types.eq_type_param_entry(LocalType::new(0)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(2, eq_type_param_entry.local_types.len());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(1)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(2, eq_type_param_entry.local_types.len());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(2)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(2, eq_type_param_entry.local_types.len());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(3)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(4)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(5)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(6)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            // t1 t2 t3 t4 t5 t6 t7
                            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(4)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(5)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(6)));
                            //    t2 t3 t4 t5 t6 t7
                            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(4)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(5)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(6)));
                            //       t3 t4 t5 t6 t7
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(4)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(5)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(6)));
                            //          t4 t5 t6 t7
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(3), LocalType::new(4)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(3), LocalType::new(5)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(3), LocalType::new(6)));
                            //             t5 t6 t7
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(4), LocalType::new(5)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(4), LocalType::new(6)));
                            //                t6 t7
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(5), LocalType::new(6)));
                            assert_eq!(typ.eq_type_param_set(), local_types.orig_eq_type_param_set());
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_local_types_add_type_param_adds_type_parameters_after_defined_type_setting()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t1, Float, uniq t2)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t1: T <Int>, t2: U<Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(2), local_types.set_defined_type(&typ));
                            assert_eq!(3, local_types.type_entries().len());
                            match local_types.type_entry(LocalType::new(0)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[0], type_param_entry));
                                    assert_eq!(LocalType::new(0), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(1)) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[1], type_param_entry));
                                    assert_eq!(LocalType::new(1), *local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(2)) {
                                Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(typ.type_value(), type_value)),
                                _ => assert!(false),
                            }
                            assert_eq!(3, local_types.eq_type_param_entries().len());
                            match local_types.eq_type_param_entry(LocalType::new(0)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(1)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(true, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            match local_types.eq_type_param_entry(LocalType::new(2)) {
                                Some(eq_type_param_entry) => {
                                    assert_eq!(None, eq_type_param_entry.type_value_name);
                                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                                    assert_eq!(false, eq_type_param_entry.is_defined);
                                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                                },
                                None => assert!(false),
                            }
                            // t1 t2 t3
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            //    t2 t3
                            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                            assert_eq!(typ.eq_type_param_set(), local_types.orig_eq_type_param_set());
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let mut type_param_entry1 = TypeParamEntry::new();
    type_param_entry1.trait_names.insert(TraitName::Shared);
    type_param_entry1.trait_names.insert(TraitName::Name(String::from("U")));
    type_param_entry1.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
    let expected_type_param_entry1 = type_param_entry1.clone();
    let new_type_param_entry1 = Rc::new(RefCell::new(type_param_entry1));
    assert_eq!(LocalType::new(3), local_types.add_type_param(new_type_param_entry1.clone()));
    let mut type_param_entry2 = TypeParamEntry::new();
    type_param_entry2.trait_names.insert(TraitName::Fun);
    type_param_entry2.trait_names.insert(TraitName::Name(String::from("V")));
    type_param_entry2.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new())));
    type_param_entry2.closure_local_types.insert(LocalType::new(3));    
    let expected_type_param_entry2 = type_param_entry2.clone();
    let new_type_param_entry2 = Rc::new(RefCell::new(type_param_entry2));
    assert_eq!(LocalType::new(4), local_types.add_type_param(new_type_param_entry2.clone()));
    assert_eq!(5, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(3)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry1.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry1.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry1.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(Some(3), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(3), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(4)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry2.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry2.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry2.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(Some(4), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(4), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(5, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(3)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(4)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2 t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(4)));
    //    t2 t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(4)));
    //       t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(4)));
    //          t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(3), LocalType::new(4)));
    assert_eq!(2, local_types.orig_eq_type_param_set().len());
}

#[test]
fn test_local_types_add_type_param_adds_type_parameters()
{
    let mut local_types = LocalTypes::new();
    let mut type_param_entry1 = TypeParamEntry::new();
    type_param_entry1.trait_names.insert(TraitName::Shared);
    type_param_entry1.trait_names.insert(TraitName::Name(String::from("T")));
    type_param_entry1.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
    let expected_type_param_entry1 = type_param_entry1.clone();
    let new_type_param_entry1 = Rc::new(RefCell::new(type_param_entry1));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let mut type_param_entry2 = TypeParamEntry::new();
    type_param_entry2.trait_names.insert(TraitName::Fun);
    type_param_entry2.trait_names.insert(TraitName::Name(String::from("U")));
    type_param_entry2.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new())));
    type_param_entry2.closure_local_types.insert(LocalType::new(0));
    let expected_type_param_entry2 = type_param_entry2.clone();
    let new_type_param_entry2 = Rc::new(RefCell::new(type_param_entry2));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let mut type_param_entry3 = TypeParamEntry::new();
    type_param_entry3.trait_names.insert(TraitName::Name(String::from("V")));
    type_param_entry3.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new())));
    let expected_type_param_entry3 = type_param_entry3.clone();
    let new_type_param_entry3 = Rc::new(RefCell::new(type_param_entry3));
    assert_eq!(LocalType::new(2), local_types.add_type_param(new_type_param_entry3.clone()));
    assert_eq!(3, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry1.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry1.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry1.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(Some(1), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry2.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry2.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry2.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(Some(2), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(1), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(2)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry3, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry3.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry3.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry3.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(Some(3), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(2), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(3, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(2)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2 t3
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
    //    t2 t3
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_add_type_value_adds_type_values()
{
    let mut local_types = LocalTypes::new();
    let new_type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_value(new_type_value1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    let new_type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    assert_eq!(LocalType::new(2), local_types.add_type_value(new_type_value3.clone()));
    assert_eq!(3, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value1, type_value)),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value2, type_value)),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(2)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value3, type_value)),
        _ => assert!(false),
    }
    assert_eq!(3, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(2)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2 t3
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
    //    t2 t3
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_sets_type()
{
    let s = "
builtin type Int;
builtin type Slice;
trait T<t1> {};
trait U<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Slice<u>>, u: U<v>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(1), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(5, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => {
            assert_eq!(String::from("(t3, Int)"), type_value.to_string_without_fun()); 
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(2)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len());
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
            assert_eq!(1, type_param_entry_r.type_values.len());
            assert_eq!(String::from("Slice<t4>"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
            assert_eq!(Some(3), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(2), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(3)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len());
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
            assert_eq!(1, type_param_entry_r.type_values.len());
            assert_eq!(String::from("t5"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
            assert_eq!(Some(4), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(3), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(4)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(true, type_param_entry_r.trait_names.is_empty());
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
            assert_eq!(Some(5), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(4), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(5, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(2)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(3)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(4)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2 t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(4)));
    //    t2 t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(4)));
    //       t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(4)));
    //          t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(3), LocalType::new(4)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_sets_type_with_equal_type_parameters()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Float;
trait T<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let new_type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_value(new_type_value1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    let s3 = "(t, u, v)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: T<Char>, v: T<Float>, t == u == v";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(0), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(5, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Type(type_value)) => {
            assert_eq!(String::from("(t3, t4, t5)"), type_value.to_string_without_fun()); 
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value2, type_value)),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(2)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len());
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
            assert_eq!(1, type_param_entry_r.type_values.len());
            assert_eq!(String::from("Int"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
            assert_eq!(Some(1), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(2), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(3)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len());
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
            assert_eq!(1, type_param_entry_r.type_values.len());
            assert_eq!(String::from("Char"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
            assert_eq!(Some(2), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(3), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(4)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len());
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
            assert_eq!(1, type_param_entry_r.type_values.len());
            assert_eq!(String::from("Float"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
            assert_eq!(Some(3), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(4), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(5, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(2)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(2, eq_type_param_entry.local_types.len());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(3)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(2, eq_type_param_entry.local_types.len());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(4)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(2, eq_type_param_entry.local_types.len());
        },
        None => assert!(false),
    }
    // t1 t2 t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(4)));
    //    t2 t3 t4 t5
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(4)));
    //       t3 t4 t5
    assert_eq!(true, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
    assert_eq!(true, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(4)));
    //          t4 t5
    assert_eq!(true, local_types.has_eq_type_params(LocalType::new(3), LocalType::new(4)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_param_sets_type_parameter_after_type_parameter_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let mut type_param_entry3 = TypeParamEntry::new();
    type_param_entry3.trait_names.insert(TraitName::Name(String::from("T")));
    type_param_entry3.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
    let expected_type_param_entry3 = type_param_entry3.clone();
    let new_type_param_entry3 = Rc::new(RefCell::new(type_param_entry3));
    assert_eq!(true, local_types.set_type_param(LocalType::new(1), new_type_param_entry3.clone()));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry3, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry3.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry3.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry3.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(Some(2), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(1), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_param_sets_type_parameter_after_type_value_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_value(new_type_value1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    let mut type_param_entry3 = TypeParamEntry::new();
    type_param_entry3.trait_names.insert(TraitName::Name(String::from("T")));
    type_param_entry3.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
    let expected_type_param_entry3 = type_param_entry3.clone();
    let new_type_param_entry3 = Rc::new(RefCell::new(type_param_entry3));
    assert_eq!(true, local_types.set_type_param(LocalType::new(0), new_type_param_entry3.clone()));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry3, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry3.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry3.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry3.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(Some(1), type_param_entry_r.number);
            assert_eq!(None, type_param_entry_r.ident);
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value2, type_value)),
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_param_entry_sets_type_parameter_entry_after_type_parameter_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let mut type_param_entry3 = TypeParamEntry::new();
    type_param_entry3.trait_names.insert(TraitName::Name(String::from("T")));
    type_param_entry3.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
    type_param_entry3.number = Some(1);
    type_param_entry3.ident = Some(String::from("t1"));
    let expected_type_param_entry3 = type_param_entry3.clone();
    let new_type_param_entry3 = Rc::new(RefCell::new(type_param_entry3));
    assert_eq!(true, local_types.set_type_param_entry(LocalType::new(1), new_type_param_entry3.clone(), DefinedFlag::Undefined));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry3, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry3.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry3.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry3.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(expected_type_param_entry3.number, type_param_entry_r.number);
            assert_eq!(expected_type_param_entry3.ident, type_param_entry_r.ident);
            assert_eq!(LocalType::new(1), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_param_entry_sets_type_parameter_entry_after_type_value_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_value(new_type_value1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    let mut type_param_entry3 = TypeParamEntry::new();
    type_param_entry3.trait_names.insert(TraitName::Name(String::from("T")));
    type_param_entry3.type_values.push(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new())));
    type_param_entry3.number = Some(2);
    type_param_entry3.ident = Some(String::from("t2"));
    let expected_type_param_entry3 = type_param_entry3.clone();
    let new_type_param_entry3 = Rc::new(RefCell::new(type_param_entry3));
    assert_eq!(true, local_types.set_type_param_entry(LocalType::new(0), new_type_param_entry3.clone(), DefinedFlag::Defined));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry3, type_param_entry));
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(expected_type_param_entry3.trait_names, type_param_entry_r.trait_names); 
            assert_eq!(true, expected_type_param_entry3.type_values.iter().zip(type_param_entry_r.type_values.iter()).all(|p| Rc::ptr_eq(p.0, p.1))); 
            assert_eq!(expected_type_param_entry3.closure_local_types, type_param_entry_r.closure_local_types);
            assert_eq!(expected_type_param_entry3.number, type_param_entry_r.number);
            assert_eq!(expected_type_param_entry3.ident, type_param_entry_r.ident);
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value2, type_value)),
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_value_sets_type_value_after_type_parameter_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let new_type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(true, local_types.set_type_value(LocalType::new(1), new_type_value3.clone()));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value3, type_value)),
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(Some(TypeValueName::Name(String::from("Int"))), eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_type_value_sets_type_value_after_type_value_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_value(new_type_value1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    let new_type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Float")), Vec::new()));
    assert_eq!(true, local_types.set_type_value(LocalType::new(0), new_type_value3.clone()));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value3, type_value)),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value2, type_value)),
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(Some(TypeValueName::Name(String::from("Float"))), eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_uniq_sets_unique_flag_after_type_parameter_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    assert_eq!(true, local_types.set_uniq(LocalType::new(1)));
    assert_eq!(3, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => {
            assert_eq!(String::from("uniq t3"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(2)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
            assert_eq!(LocalType::new(2), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(3, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2 t3
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
    //    t2 t3
    assert_eq!(true, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_uniq_sets_unique_flag_after_type_value_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_value1 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_value(new_type_value1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    assert_eq!(true, local_types.set_uniq(LocalType::new(0)));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Type(type_value)) => {
            assert!(!Rc::ptr_eq(&new_type_value1, type_value));
            assert_eq!(String::from("uniq Int"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value2, type_value)),
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
}

#[test]
fn test_local_types_set_in_non_uniq_lambda_sets_in_non_unique_lambda_after_type_parameter_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    assert_eq!(true, local_types.set_in_non_uniq_lambda(LocalType::new(1), true));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
            assert_eq!(LocalType::new(1), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(true, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
    assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
    assert_eq!(true, local_types.has_in_non_uniq_lambda(LocalType::new(1)));
    assert_eq!(true, local_types.set_in_non_uniq_lambda(LocalType::new(1), false));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
            assert_eq!(LocalType::new(1), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
    assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
    assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(1)));
}

#[test]
fn test_local_types_set_defined_type_type_param_eq_sets_defined_type_parameter_equalation_after_type_parameter_additions()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    assert_eq!(true, local_types.set_defined_type_param_eq(LocalType::new(1), true));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
            assert_eq!(LocalType::new(1), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(true, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
    assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(0)));
    assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(1)));
    assert_eq!(true, local_types.set_defined_type_param_eq(LocalType::new(1), false));
    assert_eq!(2, local_types.type_entries().len());
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
            assert_eq!(LocalType::new(0), *local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
            assert_eq!(LocalType::new(1), *local_type);
        },
        _ => assert!(false),
    }
    assert_eq!(2, local_types.eq_type_param_entries().len());
    match local_types.eq_type_param_entry(LocalType::new(0)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    match local_types.eq_type_param_entry(LocalType::new(1)) {
        Some(eq_type_param_entry) => {
            assert_eq!(None, eq_type_param_entry.type_value_name);
            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
            assert_eq!(false, eq_type_param_entry.is_defined);
            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
        },
        None => assert!(false),
    }
    // t1 t2
    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
    assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(0)));
    assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(1)));
}

#[test]
fn test_local_types_join_local_types_joins_types()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let new_type_param_entry3 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(2), local_types.add_type_param(new_type_param_entry3.clone()));
    let new_type_param_entry4 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(3), local_types.add_type_param(new_type_param_entry4.clone()));
    match local_types.join_local_types(LocalType::new(0), LocalType::new(1)) {
        Some((root_local_type, eq_root_local_type)) => {
            match local_types.join_local_types(LocalType::new(0), LocalType::new(2)) {
                Some((root_local_type2, eq_root_local_type2)) => {
                    assert!(root_local_type.index() <= 1);
                    assert!(eq_root_local_type.index() <= 1);
                    assert!(root_local_type2.index() <= 2);
                    assert!(eq_root_local_type2.index() <= 2);
                    assert_eq!(true, local_types.set_in_non_uniq_lambda(LocalType::new(0), true));
                    let new_type_param_entry5 = match root_local_type2.index() {
                        0 => new_type_param_entry1.clone(),
                        1 => new_type_param_entry2.clone(),
                        _ => new_type_param_entry3.clone(),
                    };
                    assert_eq!(4, local_types.type_entries().len());
                    match local_types.type_entry(LocalType::new(0)) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry5, type_param_entry));
                            assert_eq!(root_local_type2, *local_type);
                        },
                        _ => assert!(false),
                    }
                    match local_types.type_entry(LocalType::new(1)) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry5, type_param_entry));
                            assert_eq!(root_local_type2, *local_type);
                        },
                        _ => assert!(false),
                    }
                    match local_types.type_entry(LocalType::new(2)) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry5, type_param_entry));
                            assert_eq!(root_local_type2, *local_type);
                        },
                        _ => assert!(false),
                    }
                    match local_types.type_entry(LocalType::new(3)) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry4, type_param_entry));
                            assert_eq!(LocalType::new(3), *local_type);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(4, local_types.eq_type_param_entries().len());
                    match local_types.eq_type_param_entry(LocalType::new(0)) {
                        Some(eq_type_param_entry) => {
                            assert_eq!(None, eq_type_param_entry.type_value_name);
                            assert_eq!(true, eq_type_param_entry.is_in_non_uniq_lambda);
                            assert_eq!(false, eq_type_param_entry.is_defined);
                            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                        },
                        None => assert!(false),
                    }
                    match local_types.eq_type_param_entry(LocalType::new(1)) {
                        Some(eq_type_param_entry) => {
                            assert_eq!(None, eq_type_param_entry.type_value_name);
                            assert_eq!(true, eq_type_param_entry.is_in_non_uniq_lambda);
                            assert_eq!(false, eq_type_param_entry.is_defined);
                            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                        },
                        None => assert!(false),
                    }
                    match local_types.eq_type_param_entry(LocalType::new(2)) {
                        Some(eq_type_param_entry) => {
                            assert_eq!(None, eq_type_param_entry.type_value_name);
                            assert_eq!(true, eq_type_param_entry.is_in_non_uniq_lambda);
                            assert_eq!(false, eq_type_param_entry.is_defined);
                            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                        },
                        None => assert!(false),
                    }
                    match local_types.eq_type_param_entry(LocalType::new(3)) {
                        Some(eq_type_param_entry) => {
                            assert_eq!(None, eq_type_param_entry.type_value_name);
                            assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                            assert_eq!(false, eq_type_param_entry.is_defined);
                            assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                        },
                        None => assert!(false),
                    }
                    // t1 t2 t3 t4
                    assert_eq!(true, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                    assert_eq!(true, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
                    //    t2 t3 t4
                    assert_eq!(true, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
                    //       t3 t4
                    assert_eq!(false, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
                    assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
                },
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_local_types_join_local_types_joins_types_for_equal_type_parameters()
{
    let s = "
builtin type Char;
builtin type Int;
trait T<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let s3 = "(t, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: T<Char>, t == u";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(0), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let new_type_param_entry3 = match local_types.type_entry(LocalType::new(2)) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => type_param_entry.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    match local_types.join_local_types(LocalType::new(1), LocalType::new(2)) {
        Some((root_local_type, eq_root_local_type)) => {
            assert!(root_local_type.index() >= 1 && root_local_type.index() <= 2);
            assert!(eq_root_local_type.index() >= 1 && eq_root_local_type.index() <= 3);
            assert_eq!(true, local_types.set_in_non_uniq_lambda(LocalType::new(1), true));
            let new_type_param_entry4 = match root_local_type.index() {
                1 => new_type_param_entry2.clone(),
                _ => new_type_param_entry3.clone(),
            };
            assert_eq!(4, local_types.type_entries().len());
            match local_types.type_entry(LocalType::new(0)) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert_eq!(String::from("(t3, t4)"), type_value.to_string_without_fun()); 
                },
                _ => assert!(false),
            }
            match local_types.type_entry(LocalType::new(1)) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                    assert!(Rc::ptr_eq(&new_type_param_entry4, type_param_entry));
                    assert_eq!(root_local_type, *local_type);
                },
                _ => assert!(false),
            }
            match local_types.type_entry(LocalType::new(2)) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                    assert!(Rc::ptr_eq(&new_type_param_entry4, type_param_entry));
                    assert_eq!(root_local_type, *local_type);
                },
                _ => assert!(false),
            }
            match local_types.type_entry(LocalType::new(3)) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                    assert!(!Rc::ptr_eq(&new_type_param_entry1, type_param_entry));
                    assert!(!Rc::ptr_eq(&new_type_param_entry2, type_param_entry));
                    assert!(!Rc::ptr_eq(&new_type_param_entry3, type_param_entry));
                    assert!(!Rc::ptr_eq(&new_type_param_entry4, type_param_entry));
                    assert_eq!(LocalType::new(3), *local_type);
                },
                _ => assert!(false),
            }
            assert_eq!(4, local_types.eq_type_param_entries().len());
            match local_types.eq_type_param_entry(LocalType::new(0)) {
                Some(eq_type_param_entry) => {
                    assert_eq!(None, eq_type_param_entry.type_value_name);
                    assert_eq!(false, eq_type_param_entry.is_in_non_uniq_lambda);
                    assert_eq!(false, eq_type_param_entry.is_defined);
                    assert_eq!(true, eq_type_param_entry.local_types.is_empty());
                },
                None => assert!(false),
            }
            match local_types.eq_type_param_entry(LocalType::new(1)) {
                Some(eq_type_param_entry) => {
                    assert_eq!(None, eq_type_param_entry.type_value_name);
                    assert_eq!(true, eq_type_param_entry.is_in_non_uniq_lambda);
                    assert_eq!(false, eq_type_param_entry.is_defined);
                    assert_eq!(1, eq_type_param_entry.local_types.len());
                },
                None => assert!(false),
            }
            match local_types.eq_type_param_entry(LocalType::new(2)) {
                Some(eq_type_param_entry) => {
                    assert_eq!(None, eq_type_param_entry.type_value_name);
                    assert_eq!(true, eq_type_param_entry.is_in_non_uniq_lambda);
                    assert_eq!(false, eq_type_param_entry.is_defined);
                    assert_eq!(1, eq_type_param_entry.local_types.len());
                },
                None => assert!(false),
            }
            match local_types.eq_type_param_entry(LocalType::new(3)) {
                Some(eq_type_param_entry) => {
                    assert_eq!(None, eq_type_param_entry.type_value_name);
                    assert_eq!(true, eq_type_param_entry.is_in_non_uniq_lambda);
                    assert_eq!(false, eq_type_param_entry.is_defined);
                    assert_eq!(1, eq_type_param_entry.local_types.len());
                },
                None => assert!(false),
            }
            // t1 t2 t3 t4
            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
            assert_eq!(false, local_types.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
            //    t2 t3 t4
            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
            //       t3 t4
            assert_eq!(true, local_types.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
            assert_eq!(true, local_types.orig_eq_type_param_set().is_empty());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_local_types_type_entry_for_type_value_returns_local_type_entries_for_defined_type_parameters()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Float, uniq u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: U<Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(2), local_types.set_defined_type(&typ));
                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[0], &type_param_entry));
                                    assert_eq!(LocalType::new(0), local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)))) {
                                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::Uniq, type_param_entry, local_type)) => {
                                    assert!(Rc::ptr_eq(&typ.type_param_entries()[1], &type_param_entry));
                                    assert_eq!(LocalType::new(1), local_type);
                                },
                                _ => assert!(false),
                            }
                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)))) {
                                Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(typ.type_value(), &type_value)),
                                _ => assert!(false),
                            }
                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(2)))) {
                                Some(LocalTypeEntry::Type(type_value)) => {
                                    assert_eq!(String::from("uniq (t1, Float, uniq t2)"), type_value.to_string_without_fun());
                                },
                                _ => assert!(false),
                            }
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_local_types_type_entry_for_type_value_returns_local_type_entries()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let new_type_value3 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    assert_eq!(LocalType::new(2), local_types.add_type_value(new_type_value3.clone()));
    let new_type_value4 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    assert_eq!(LocalType::new(3), local_types.add_type_value(new_type_value4.clone()));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, &type_param_entry));
            assert_eq!(LocalType::new(0), local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)))) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry2, &type_param_entry));
            assert_eq!(LocalType::new(1), local_type);
        },
        _ => assert!(false),
    }
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)))) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value3, &type_value)),
        _ => assert!(false),
    }
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(3)))) {
        Some(LocalTypeEntry::Type(type_value)) => {
            assert_eq!(String::from("uniq Char"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_local_types_type_entry_for_type_value_returns_local_type_entry_for_type_references_and_none_and_uniq()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(0)));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    let new_type_value3 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)));
    assert_eq!(LocalType::new(2), local_types.add_type_value(new_type_value3.clone()));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)))) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, &type_param_entry));
            assert_eq!(LocalType::new(0), local_type);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_local_types_type_entry_for_type_value_returns_local_type_entry_for_type_references_and_uniq_and_none()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    assert_eq!(LocalType::new(1), local_types.add_type_value(new_type_value2.clone()));
    let new_type_value3 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)));
    assert_eq!(LocalType::new(2), local_types.add_type_value(new_type_value3.clone()));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)))) {
        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry, local_type)) => {
            assert!(Rc::ptr_eq(&new_type_param_entry1, &type_param_entry));
            assert_eq!(LocalType::new(0), local_type);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_local_types_type_entry_for_type_value_returns_local_type_entries_for_joining()
{
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_param_entry2 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(1), local_types.add_type_param(new_type_param_entry2.clone()));
    let new_type_param_entry3 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(2), local_types.add_type_param(new_type_param_entry3.clone()));
    let new_type_param_entry4 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(3), local_types.add_type_param(new_type_param_entry4.clone()));
    match local_types.join_local_types(LocalType::new(0), LocalType::new(1)) {
        Some((root_local_type, eq_root_local_type)) => {
            match local_types.join_local_types(LocalType::new(0), LocalType::new(2)) {
                Some((root_local_type2, eq_root_local_type2)) => {
                    assert!(root_local_type.index() <= 1);
                    assert!(eq_root_local_type.index() <= 1);
                    assert!(root_local_type2.index() <= 2);
                    assert!(eq_root_local_type2.index() <= 2);
                    let new_type_param_entry5 = match root_local_type2.index() {
                        0 => new_type_param_entry1.clone(),
                        1 => new_type_param_entry2.clone(),
                        _ => new_type_param_entry3.clone(),
                    };
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry5, &type_param_entry));
                            assert_eq!(root_local_type2, local_type);
                        },
                        _ => assert!(false),
                    }
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry5, &type_param_entry));
                            assert_eq!(root_local_type2, local_type);
                        },
                        _ => assert!(false),
                    }
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry5, &type_param_entry));
                            assert_eq!(root_local_type2, local_type);
                        },
                        _ => assert!(false),
                    }
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(3)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type)) => {
                            assert!(Rc::ptr_eq(&new_type_param_entry4, &type_param_entry));
                            assert_eq!(LocalType::new(3), local_type);
                        },
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
        },
        None => assert!(false),
    }
}

#[test]
fn test_local_types_type_entry_for_type_value_returns_local_type_entries_for_type_value_setting()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Float;
trait T<t1, t2> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(0), local_types.add_type_param(new_type_param_entry1.clone()));
    let s3 = "(t, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int, Char>, u: T<Char, Float>, t == u";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(0), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let new_type_value21 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let new_type_value22 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("U")), vec![new_type_value21, new_type_value22]));
    assert_eq!(true, local_types.set_type_value(LocalType::new(1), new_type_value2.clone()));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => {
            assert_eq!(String::from("U<Int, Char>"), type_value.to_string_without_fun()); 
        },
        _ => assert!(false),
    }
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)))) {
        Some(LocalTypeEntry::Type(type_value)) => {
            assert_eq!(String::from("U<Char, Float>"), type_value.to_string_without_fun()); 
        },
        _ => assert!(false),
    }
}

#[test]
fn test_local_types_type_entry_for_type_value_returns_local_type_entry_for_type_value()
{
    let local_types = LocalTypes::new();
    let new_type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Char")), Vec::new()));
    match local_types.type_entry_for_type_value(&new_type_value) {
        Some(LocalTypeEntry::Type(type_value)) => assert!(Rc::ptr_eq(&new_type_value, &type_value)),
        _ => assert!(false),
    }
}

#[test]
fn test_local_types_type_value_to_string_returns_string_for_type_paremeter()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Float, uniq u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: U<Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => assert_eq!(LocalType::new(2), local_types.set_defined_type(&typ)),
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(3), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_value21 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let new_type_value22 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)));
    let new_type_value23 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(3)));
    let new_type_value24 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(3)));
    let new_type_value25 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, vec![new_type_value21, new_type_value22, new_type_value23, new_type_value24, new_type_value25]));
    assert_eq!(LocalType::new(4), local_types.add_type_value(new_type_value2.clone()));
    assert_eq!(String::from("(t, uniq u, t1, uniq t1, Int)"), local_types.type_value_to_string(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(4)))));
}

#[test]
fn test_local_types_type_value_to_string_returns_string_for_type()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1> {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let typer = Typer::new();
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Float, uniq u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>, u: U<Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => assert_eq!(LocalType::new(2), local_types.set_defined_type(&typ)),
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let new_type_param_entry1 = Rc::new(RefCell::new(TypeParamEntry::new()));
    assert_eq!(LocalType::new(3), local_types.add_type_param(new_type_param_entry1.clone()));
    let new_type_value21 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)));
    let new_type_value22 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)));
    let new_type_value23 = Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(3)));
    let new_type_value24 = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(3)));
    let new_type_value25 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()));
    let new_type_value2 = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, vec![new_type_value21, new_type_value22, new_type_value23, new_type_value24, new_type_value25]));
    assert_eq!(String::from("(t, uniq u, t1, uniq t1, Int)"), local_types.type_value_to_string(&new_type_value2));
}
