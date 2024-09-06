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
use crate::frontend::type_matcher::*;
use crate::frontend::typer::*;
use super::*;

#[test]
fn test_type_stack_set_first_type_values_for_type_sets_first_type_values()
{
    let s = "
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
    let mut type_stack = TypeStack::new();
    let s3 = "(t, Int, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Float>, u: shared";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(0)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(2, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared)); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
            assert_eq!(1, type_param_entry_r.type_values.len());
            assert_eq!(String::from("Float"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(1)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_stack_push_type_entries_for_local_type_pushes_type_entries()
{
    let s = "
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
    let mut type_stack = TypeStack::new();
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Int, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared, u: shared";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(0)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared)); 
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(1)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let type_matcher = TypeMatcher::new();
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(t, Float, u)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared, u: shared + T <Int>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(3), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(3), &local_types) {
                                Ok(local_type) => assert_eq!(LocalType::new(2), local_type),
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
    assert_eq!(1, type_stack.type_value_stack_len());
    assert_eq!(4, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(t1, Float, t4)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(3)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(2, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared)); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T")))); 
            assert_eq!(1, type_param_entry_r.type_values.len());
            assert_eq!(String::from("Int"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_stack_push_type_entries_for_local_type_pushes_type_entries_with_closure_local_types()
{
    let s = "
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
    let mut type_stack = TypeStack::new();
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Int, u) -> Float";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(0)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(true, type_param_entry_r.trait_names.is_empty()); 
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(1)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(true, type_param_entry_r.trait_names.is_empty()); 
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let type_matcher = TypeMatcher::new();
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(4), local_types.add_type_value(Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Name(String::from("Int")), Vec::new()))));
    assert_eq!(LocalType::new(5), local_types.add_type_value(Rc::new(TypeValue::Type(UniqFlag::Uniq, TypeValueName::Name(String::from("Float")), Vec::new()))));
    let s5 = "(u, t) -> ()";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: -> <Int, Float>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(3), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(6), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match local_types.type_entry(LocalType::new(7)) {
                                Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
                                    let mut type_param_entry_r = type_param_entry.borrow_mut();
                                    type_param_entry_r.closure_local_types.insert(LocalType::new(4));
                                    type_param_entry_r.closure_local_types.insert(LocalType::new(5));
                                },
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(3), &local_types) {
                                Ok(local_type) => assert_eq!(LocalType::new(2), local_type),
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
    assert_eq!(1, type_stack.type_value_stack_len());
    assert_eq!(6, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(t1, t4) -> ()"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(3)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Fun)); 
            assert_eq!(2, type_param_entry_r.type_values.len());
            assert_eq!(String::from("Int"), type_param_entry_r.type_values[0].to_string_without_fun());
            assert_eq!(String::from("Float"), type_param_entry_r.type_values[1].to_string_without_fun());
            assert_eq!(2, type_param_entry_r.closure_local_types.len());
            assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(4))); 
            assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(5))); 
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(4)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("Int"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(5)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("uniq Float"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_stack_pop_type_entries_pops_type_entries()
{
    let s = "
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
    let mut type_stack = TypeStack::new();
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Int, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared, u: shared";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    let type_matcher = TypeMatcher::new();
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(t, Float, u)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared, u: shared + T <Int>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(3), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(3), &local_types) {
                                Ok(local_type) => assert_eq!(LocalType::new(2), local_type),
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
    type_stack.pop_type_entries();
    assert_eq!(1, type_stack.type_value_stack_len());
    assert_eq!(2, type_stack.type_entries().len());
}

#[test]
fn test_type_stack_push_type_values_for_local_type_and_type_pushes_type_values()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1> = C(t1);
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
    let mut type_stack = TypeStack::new();
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Int, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared, u: shared";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(0)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared)); 
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(1)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let type_matcher = TypeMatcher::new();
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(T<t>, Float, T<u>)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared, u: shared";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(3), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(1), LocalType::new(5), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(3), &local_types) {
                                Ok(local_type) => assert_eq!(LocalType::new(2), local_type),
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
    assert_eq!(1, type_stack.type_value_stack_len());
    assert_eq!(3, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(T<t1>, Float, T<t2>)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    let s7 = "(t, u, v)";
    let mut cursor6 = Cursor::new(s7.as_bytes());
    let mut parser6 = Parser::new(Lexer::new(String::from("test6.vscfl"), &mut cursor6));
    match parser6.parse_type() {
        Ok(type_expr) => {
            let s8 = "t: shared, u: shared";
            let mut cursor7 = Cursor::new(s8.as_bytes());
            let mut parser7 = Parser::new(Lexer::new(String::from("test7.vscfl"), &mut cursor7));
            match parser7.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test6.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match type_stack.push_type_values_for_local_type_and_type(LocalType::new(2), &typ) {
                                Ok(()) => assert!(true),
                                Err(err) => {
                                    println!("{}", err);
                                    assert!(false)
                                },
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
    assert_eq!(2, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(3, idx);
            assert_eq!(3, type_values.len());
            assert_eq!(String::from("T<t1>"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("Float"), type_values[1].to_string_without_fun());
            assert_eq!(String::from("T<t2>"), type_values[2].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(3, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(T<t1>, Float, T<t2>)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_stack_push_type_values_for_local_type_and_type_doubly_pushes_type_values()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1> = C(t1);
data U<t2> = D(t2);
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
    let mut type_stack = TypeStack::new();
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Int, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared, u: shared";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(0)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared)); 
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(1)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let type_matcher = TypeMatcher::new();
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(T<t>, Float, T<u>)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared, u: shared";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(3), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(1), LocalType::new(5), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(3), &local_types) {
                                Ok(local_type) => assert_eq!(LocalType::new(2), local_type),
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
    assert_eq!(1, type_stack.type_value_stack_len());
    assert_eq!(3, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(T<t1>, Float, T<t2>)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    let mut local_types2 = LocalTypes::new();
    let s7 = "(t, u, v)";
    let mut cursor6 = Cursor::new(s7.as_bytes());
    let mut parser6 = Parser::new(Lexer::new(String::from("test6.vscfl"), &mut cursor6));
    match parser6.parse_type() {
        Ok(type_expr) => {
            let s8 = "t: shared, u: shared, v: shared";
            let mut cursor7 = Cursor::new(s8.as_bytes());
            let mut parser7 = Parser::new(Lexer::new(String::from("test7.vscfl"), &mut cursor7));
            match parser7.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test6.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(3), local_types2.set_defined_type(&typ));
                            match type_stack.push_type_values_for_local_type_and_type(LocalType::new(2), &typ) {
                                Ok(()) => assert!(true),
                                Err(err) => {
                                    println!("{}", err);
                                    assert!(false)
                                },
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
    assert_eq!(2, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(3, idx);
            assert_eq!(3, type_values.len());
            assert_eq!(String::from("T<t1>"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("Float"), type_values[1].to_string_without_fun());
            assert_eq!(String::from("T<t2>"), type_values[2].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(3, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(T<t1>, Float, T<t2>)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    assert_eq!(LocalType::new(4), local_types2.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s9 = "(U<t>, U<u>, v)";
    let mut cursor8 = Cursor::new(s9.as_bytes());
    let mut parser8 = Parser::new(Lexer::new(String::from("test8.vscfl"), &mut cursor8));
    match parser8.parse_type() {
        Ok(type_expr) => {
            let s10 = "t: shared, u: shared, v: shared";
            let mut cursor9 = Cursor::new(s10.as_bytes());
            let mut parser9 = Parser::new(Lexer::new(String::from("test7.vscfl"), &mut cursor9));
            match parser9.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test8.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types2.set_type(LocalType::new(4), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(5), &tree, &mut local_types2) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(2), LocalType::new(6), &tree, &mut local_types2) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(4), &local_types2) {
                                Ok(local_type) => assert_eq!(LocalType::new(3), local_type),
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
    assert_eq!(2, type_stack.type_value_stack_len());
    assert_eq!(5, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(3)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(U<T<t1>>, U<T<t2>>, t5)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(4)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let s11 = "(t, u, v)";
    let mut cursor10 = Cursor::new(s11.as_bytes());
    let mut parser10 = Parser::new(Lexer::new(String::from("test10.vscfl"), &mut cursor10));
    match parser10.parse_type() {
        Ok(type_expr) => {
            let s12 = "t: shared, u: shared, v: shared";
            let mut cursor11 = Cursor::new(s12.as_bytes());
            let mut parser11 = Parser::new(Lexer::new(String::from("test11.vscfl"), &mut cursor11));
            match parser11.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test10.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match type_stack.push_type_values_for_local_type_and_type(LocalType::new(3), &typ) {
                                Ok(()) => assert!(true),
                                Err(err) => {
                                    println!("{}", err);
                                    assert!(false)
                                },
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
    assert_eq!(3, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(5, idx);
            assert_eq!(3, type_values.len());
            assert_eq!(String::from("U<T<t1>>"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("U<T<t2>>"), type_values[1].to_string_without_fun());
            assert_eq!(String::from("t5"), type_values[2].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(5, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(3)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(U<T<t1>>, U<T<t2>>, t5)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(4)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_stack_pop_type_values_pops_type_values()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1> = C(t1);
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
    let mut type_stack = TypeStack::new();
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Int, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared, u: shared";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(0)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared)); 
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(1)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let type_matcher = TypeMatcher::new();
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(T<t>, Float, T<u>)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared, u: shared";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(3), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(1), LocalType::new(5), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(3), &local_types) {
                                Ok(local_type) => assert_eq!(LocalType::new(2), local_type),
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
    assert_eq!(1, type_stack.type_value_stack_len());
    assert_eq!(3, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(T<t1>, Float, T<t2>)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    let s7 = "(t, u, v)";
    let mut cursor6 = Cursor::new(s7.as_bytes());
    let mut parser6 = Parser::new(Lexer::new(String::from("test6.vscfl"), &mut cursor6));
    match parser6.parse_type() {
        Ok(type_expr) => {
            let s8 = "t: shared, u: shared";
            let mut cursor7 = Cursor::new(s8.as_bytes());
            let mut parser7 = Parser::new(Lexer::new(String::from("test7.vscfl"), &mut cursor7));
            match parser7.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test6.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match type_stack.push_type_values_for_local_type_and_type(LocalType::new(2), &typ) {
                                Ok(()) => assert!(true),
                                Err(err) => {
                                    println!("{}", err);
                                    assert!(false)
                                },
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
    type_stack.pop_type_values();
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((_, idx)) => assert_eq!(2, idx),
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
}

#[test]
fn test_type_stack_pop_type_values_doubly_pops_type_values()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1> = C(t1);
data U<t2> = D(t2);
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
    let mut type_stack = TypeStack::new();
    let mut local_types = LocalTypes::new();
    let s3 = "(t, Int, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared, u: shared";
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
                            type_stack.set_first_type_values_for_type(&typ);
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(2, idx);
            assert_eq!(2, type_values.len());
            assert_eq!(String::from("t1"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("t2"), type_values[1].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(0)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared)); 
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(1)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let type_matcher = TypeMatcher::new();
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(T<t>, Float, T<u>)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared, u: shared";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(3), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(1), LocalType::new(5), &tree, &mut local_types) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(3), &local_types) {
                                Ok(local_type) => assert_eq!(LocalType::new(2), local_type),
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
    assert_eq!(1, type_stack.type_value_stack_len());
    assert_eq!(3, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(T<t1>, Float, T<t2>)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    let mut local_types2 = LocalTypes::new();
    let s7 = "(t, u, v)";
    let mut cursor6 = Cursor::new(s7.as_bytes());
    let mut parser6 = Parser::new(Lexer::new(String::from("test6.vscfl"), &mut cursor6));
    match parser6.parse_type() {
        Ok(type_expr) => {
            let s8 = "t: shared, u: shared, v: shared";
            let mut cursor7 = Cursor::new(s8.as_bytes());
            let mut parser7 = Parser::new(Lexer::new(String::from("test7.vscfl"), &mut cursor7));
            match parser7.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test6.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(3), local_types2.set_defined_type(&typ));
                            match type_stack.push_type_values_for_local_type_and_type(LocalType::new(2), &typ) {
                                Ok(()) => assert!(true),
                                Err(err) => {
                                    println!("{}", err);
                                    assert!(false)
                                },
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
    assert_eq!(2, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((type_values, idx)) => {
            assert_eq!(3, idx);
            assert_eq!(3, type_values.len());
            assert_eq!(String::from("T<t1>"), type_values[0].to_string_without_fun());
            assert_eq!(String::from("Float"), type_values[1].to_string_without_fun());
            assert_eq!(String::from("T<t2>"), type_values[2].to_string_without_fun());
        },
        None => assert!(false),
    }
    assert_eq!(3, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(2)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(T<t1>, Float, T<t2>)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    assert_eq!(LocalType::new(4), local_types2.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s9 = "(U<t>, U<u>, v)";
    let mut cursor8 = Cursor::new(s9.as_bytes());
    let mut parser8 = Parser::new(Lexer::new(String::from("test8.vscfl"), &mut cursor8));
    match parser8.parse_type() {
        Ok(type_expr) => {
            let s10 = "t: shared, u: shared, v: shared";
            let mut cursor9 = Cursor::new(s10.as_bytes());
            let mut parser9 = Parser::new(Lexer::new(String::from("test7.vscfl"), &mut cursor9));
            match parser9.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test8.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types2.set_type(LocalType::new(4), &typ) {
                                Ok(true) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(0), LocalType::new(5), &tree, &mut local_types2) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_matcher.matches(LocalType::new(2), LocalType::new(6), &tree, &mut local_types2) {
                                Ok(TypeMatcherResult::Matched) => assert!(true),
                                _ => assert!(false),
                            }
                            match type_stack.push_type_entries_for_local_type(LocalType::new(4), &local_types2) {
                                Ok(local_type) => assert_eq!(LocalType::new(3), local_type),
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
    assert_eq!(2, type_stack.type_value_stack_len());
    assert_eq!(5, type_stack.type_entries().len());
    match type_stack.type_entry(LocalType::new(3)) {
        Some(TypeStackEntry::Type(type_value)) => {
            assert_eq!(String::from("(U<T<t1>>, U<T<t2>>, t5)"), type_value.to_string_without_fun());
        },
        _ => assert!(false),
    }
    match type_stack.type_entry(LocalType::new(4)) {
        Some(TypeStackEntry::Param(type_param_entry)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(1, type_param_entry_r.trait_names.len()); 
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
            assert_eq!(true, type_param_entry_r.type_values.is_empty());
            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
        },
        _ => assert!(false),
    }
    let s11 = "(t, u, v)";
    let mut cursor10 = Cursor::new(s11.as_bytes());
    let mut parser10 = Parser::new(Lexer::new(String::from("test10.vscfl"), &mut cursor10));
    match parser10.parse_type() {
        Ok(type_expr) => {
            let s12 = "t: shared, u: shared, v: shared";
            let mut cursor11 = Cursor::new(s12.as_bytes());
            let mut parser11 = Parser::new(Lexer::new(String::from("test11.vscfl"), &mut cursor11));
            match parser11.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test10.vscfl"), 1, 1);
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match type_stack.push_type_values_for_local_type_and_type(LocalType::new(3), &typ) {
                                Ok(()) => assert!(true),
                                Err(err) => {
                                    println!("{}", err);
                                    assert!(false)
                                },
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
    type_stack.pop_type_values();
    type_stack.pop_type_values();
    assert_eq!(1, type_stack.type_value_stack_len());
    match type_stack.type_values_and_type_entry_index() {
        Some((_, idx)) => assert_eq!(2, idx),
        None => assert!(false),
    }
    assert_eq!(2, type_stack.type_entries().len());
}
