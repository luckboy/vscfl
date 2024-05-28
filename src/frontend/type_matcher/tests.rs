//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::io::*;
use std::rc::*;
use crate::frontend::lexer::*;
use crate::frontend::namer::*;
use crate::frontend::parser::*;
use crate::frontend::typer::*;
use super::*;

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_defined_type_parameters()
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
    let mut local_types = LocalTypes::new();
    let s3 = "(t, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>, u: T <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(2), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let type_value = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(0)));
    assert_eq!(LocalType::new(3), local_types.add_type_value(type_value.clone()));
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(0), &tree, &local_types) {
        Ok((UniqFlag::None, SharedFlag::Shared)) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(3), &tree, &local_types) {
        Ok((UniqFlag::Uniq, SharedFlag::None)) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(1), &tree, &local_types) {
        Ok((UniqFlag::None, SharedFlag::None)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_type_parameters()
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
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "(t, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>, u: T <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_value = Rc::new(TypeValue::Param(UniqFlag::Uniq, LocalType::new(1)));
    assert_eq!(LocalType::new(3), local_types.add_type_value(type_value.clone()));
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(1), &tree, &local_types) {
        Ok((UniqFlag::None, SharedFlag::Shared)) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(3), &tree, &local_types) {
        Ok((UniqFlag::Uniq, SharedFlag::None)) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(2), &tree, &local_types) {
        Ok((UniqFlag::None, SharedFlag::None)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_first_tuple_type()
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
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "(t, u, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>, u: shared + T <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(0), &tree, &local_types) {
        Ok((UniqFlag::None, SharedFlag::Shared)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_second_tuple_type()
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
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "(t, u, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>, u: T <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(0), &tree, &local_types) {
        Ok((UniqFlag::Uniq, SharedFlag::None)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_function_type()
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
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "(t, uniq Int) -> uniq Float";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(0), &tree, &local_types) {
        Ok((UniqFlag::None, SharedFlag::Shared)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_first_data_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t, u> = C(t, u, Int);
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
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "T<t, u>";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + U <Int>, u: shared + U <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(0), &tree, &local_types) {
        Ok((UniqFlag::None, SharedFlag::Shared)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_second_data_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t, u> = C(t, u, Int);
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
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "T<t, u>";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + U <Int>, u: U <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(0), &tree, &local_types) {
        Ok((UniqFlag::Uniq, SharedFlag::None)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_uniq_flag_and_shared_flag_returns_unique_flag_and_shared_flag_for_unique_data_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t, u> = C(t, u, uniq Int);
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
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "T<t, u>";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + U <Int>, u: shared + U <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.uniq_flag_and_shared_flag(LocalType::new(0), &tree, &local_types) {
        Ok((UniqFlag::Uniq, SharedFlag::None)) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_set_shared_sets_or_does_not_set_shared_for_defined_type_parameters()
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
    let mut local_types = LocalTypes::new();
    let s3 = "(t, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>, u: T <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(2), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let type_matcher = TypeMatcher::new();
    match type_matcher.set_shared(LocalType::new(0), &tree, &local_types) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(0)) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
        },
        _ => assert!(false),
    }
    match type_matcher.set_shared(LocalType::new(1), &tree, &local_types) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_set_shared_sets_or_does_not_set_shared_for_type_parameters()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1, t2> {};
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
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "(t, u, v)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>, u: T <Float>, v: U<Int, uniq Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.set_shared(LocalType::new(1), &tree, &local_types) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(1)) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
        },
        _ => assert!(false),
    }
    match type_matcher.set_shared(LocalType::new(2), &tree, &local_types) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(2)) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
        },
        _ => assert!(false),
    }
    match type_matcher.set_shared(LocalType::new(3), &tree, &local_types) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(3)) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_set_shared_sets_shared_for_data_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(t1, t2, Int);
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
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "T<t, u>";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + U <Int>, u: shared + U <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.set_shared(LocalType::new(0), &tree, &local_types) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_set_shared_does_not_set_shared_for_unique_data_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(t1, t2, uniq Int);
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
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "T<t, u>";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + U <Int>, u: shared + U <Float>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let type_matcher = TypeMatcher::new();
    match type_matcher.set_shared(LocalType::new(0), &tree, &local_types) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}
