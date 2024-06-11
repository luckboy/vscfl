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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
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
    let s3 = "(t, u, v, w)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Int>, u: T <Float>, v: U<Int, uniq Float>, w: -> <uniq Float, Int>";
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
    let type_matcher = typer.type_matcher();
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
    match type_matcher.set_shared(LocalType::new(4), &tree, &local_types) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
    match local_types.type_entry(LocalType::new(4)) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let type_param_entry_r = type_param_entry.borrow();
            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
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
    let type_matcher = typer.type_matcher();
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
    let type_matcher = typer.type_matcher();
    match type_matcher.set_shared(LocalType::new(0), &tree, &local_types) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_shared_and_type_parameter_with_traits()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1, t2> {};
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: T + U <Int, Float>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(3, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(3, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types2.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_without_shared_and_type_parameter_with_traits()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1, t2> {};
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "t";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: T + U <Int, uniq Float>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types2.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_shared_and_type_parameter_with_function_trait_and_trait()
{
    let s = "
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: -> + T <Int, uniq Float>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(3, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Fun));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(3, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Fun));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types2.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_traits_and_type_parameter_with_traits()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1, t2> {};
trait U<t1, t2> {};
trait V<t1, t2> {};
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U <Int, uniq Float>";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U + V <Int, uniq Float>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(3, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("V"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(3, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("V"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types2.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_closure_local_types_and_type_parameter_with_closure_local_types()
{
    let s = "
builtin type Int;
builtin type Float;
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: -> <Int, Float>";
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
    let s5 = "t";
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
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    assert_eq!(LocalType::new(4), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(5), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(6), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(7), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let mut type_param_entry_r = type_param_entry.borrow_mut();
            type_param_entry_r.closure_local_types.insert(LocalType::new(4));
            type_param_entry_r.closure_local_types.insert(LocalType::new(5));
        }
        _ => assert!(false),
    }
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let mut type_param_entry_r = type_param_entry.borrow_mut();
            type_param_entry_r.closure_local_types.insert(LocalType::new(6));
            type_param_entry_r.closure_local_types.insert(LocalType::new(7));
        }
        _ => assert!(false),
    }
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(1, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Fun));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(4, type_param_entry_r.closure_local_types.len());
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(4)));
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(5)));
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(6)));
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(7)));
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(1, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Fun));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types2.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(4, type_param_entry_r.closure_local_types.len());
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(4)));
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(5)));
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(6)));
                    assert_eq!(true, type_param_entry_r.closure_local_types.contains(&LocalType::new(7)));
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_in_in_non_unique_lambda_and_type_parameter_with_defined_equation()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1, t2> {};
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int, uniq Float>";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U <Int, uniq Float>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    local_types.set_in_non_uniq_lambda(LocalType::new(2), true);
    local_types.set_defined_type_param_eq(LocalType::new(3), true);
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(true, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(true, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types2.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(true, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(true, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_unique_type_parameter_and_unique_type_parameter()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t1, t2> {};
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "uniq t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int, uniq Float>";
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
    let s5 = "uniq t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U <Int, uniq Float>";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types2.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("uniq Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_shared_and_type_parameter()
{
    let s = "
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
    match typer.evaluate_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let mut local_types = LocalTypes::new();
    assert_eq!(LocalType::new(0), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(1), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "";
            let mut cursor5 = Cursor::new(s6.as_bytes());
            let mut parser5 = Parser::new(Lexer::new(String::from("test5.vscfl"), &mut cursor5));
            match parser5.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test4.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(1, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
        },
        _ => assert!(false),
    }
}
