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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
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

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_shared_and_defined_type_parameter_with_traits()
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
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T + U <Int, Float>";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared";
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
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    let expected_type_param_entry = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => type_param_entry.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_and_defined_type_parameter_with_traits()
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
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U <Int, Float>";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    let expected_type_param_entry = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => type_param_entry.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_trait_and_defined_type_parameter_with_traits()
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
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U + V <Int, uniq Float>";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    let expected_type_param_entry = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => type_param_entry.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_unique_type_parameter_and_unique_defined_type_parameter()
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
    let s3 = "uniq t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U <Int, Float>";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "uniq t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U <Int, Float>";
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
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    let expected_type_param_entry = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => type_param_entry.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq t"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::Uniq, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::Uniq, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq t"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::Uniq, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::Uniq, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_and_defined_type_parameter()
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    let expected_type_param_entry = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => type_param_entry.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(0)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_shared_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
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
    let s5 = "T<Int, Float>";
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
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(uniq Int, t1, t2);
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
    let s5 = "T<Int, Float>";
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
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_traits_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
trait U<t1, t2> {};
trait V<t1, t2> {};
impl U for T {};
impl V for T {};
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
            let s4 = "t: U + V <Int, Float>";
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
    let s5 = "T<Int, Float>";
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
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_traits_and_array_type()
{
    let s = "
builtin type Int;
trait T<t1> {};
trait U<t1> {};
impl T for [_; _] {};
impl U for [_; 10] {};
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
            let s4 = "t: T + U <Int>";
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
    let s5 = "[Int; 10]";
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
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("[Int; 10]"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("[Int; 10]"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_with_closure_local_types_and_function_type()
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
    let s5 = "(Int) -> Float";
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
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(4), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let mut type_param_entry_r = type_param_entry.borrow_mut();
            type_param_entry_r.closure_local_types.insert(LocalType::new(3));
            type_param_entry_r.closure_local_types.insert(LocalType::new(4));
        }
        _ => assert!(false),
    }
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("(Int) -> Float"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(3)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            let type_param_entry2_r = type_param_entry2.borrow();
                            assert_eq!(true, type_param_entry2_r.trait_names.contains(&TraitName::Shared));
                        },
                        _ => assert!(false),
                    }
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(4)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            let type_param_entry2_r = type_param_entry2.borrow();
                            assert_eq!(true, type_param_entry2_r.trait_names.contains(&TraitName::Shared));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("(Int) -> Float"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(3)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            let type_param_entry2_r = type_param_entry2.borrow();
                            assert_eq!(true, type_param_entry2_r.trait_names.contains(&TraitName::Shared));
                        },
                        _ => assert!(false),
                    }
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(4)))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry2, _)) => {
                            let type_param_entry2_r = type_param_entry2.borrow();
                            assert_eq!(true, type_param_entry2_r.trait_names.contains(&TraitName::Shared));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_unique_type_parameter_and_unique_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
trait U<t1, t2> {};
trait V<t1, t2> {};
impl U for T {};
impl V for T {};
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
            let s4 = "t: U + V <Int, Float>";
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
    let s5 = "uniq T<Int, Float>";
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
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_and_unique_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(uniq Int, t1, t2);
trait U<t1, t2> {};
trait V<t1, t2> {};
impl U for T {};
impl V for T {};
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
            let s4 = "t: U + V <Int, Float>";
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
    let s5 = "uniq T<Int, Float>";
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
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_defined_type_parameter_and_defined_type_parameter()
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
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
    match type_matcher.matches(LocalType::new(1), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_unique_defined_type_parameter_and_unique_defined_type_parameter()
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
    let s3 = "uniq t";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
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
    match type_matcher.matches(LocalType::new(1), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
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
    let s3 = "T<Int, Float>";
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
    let s5 = "T<Int, Float>";
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type1));
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_unique_type_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(uniq Int, t1, t2);
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
    let s3 = "uniq T<Int, Float>";
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
    let s5 = "T<Int, Float>";
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types.local_type_to_string(local_type1));
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_unique_type_and_unique_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
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
    let s3 = "uniq T<Int, Float>";
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
    let s5 = "uniq T<Int, Float>";
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types.local_type_to_string(local_type1));
            assert_eq!(String::from("uniq T<Int, Float>"), local_types.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("uniq T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            assert_eq!(String::from("uniq T<Int, Float>"), local_types2.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_and_type_parameter_with_nested_type_parameters()
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
            let s4 = "t: T + U <Int, u>";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U + V <u, uniq Float>";
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
fn test_type_matcher_matches_matches_type_parameter_and_type_with_nested_type_paremeters()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
trait U<t1, t2> {};
trait V<t1, t2> {};
impl U for T {};
impl V for T {};
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
            let s4 = "t: U + V <Int, u>";
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
    let s5 = "T<t, Float>";
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
    let expected_type_value = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(1)))) {
        Some(LocalTypeEntry::Type(type_value)) => type_value.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Type(type_value)) => {
                    assert!(Rc::ptr_eq(&expected_type_value, &type_value));
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_and_type_with_nested_type_parameters()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
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
    let s3 = "T<Int, t>";
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
    let s5 = "T<t, Float>";
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type1));
            assert_eq!(String::from("T<Int, Float>"), local_types.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(0);
            let local_type2 = LocalType::new(1);
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type1));
            assert_eq!(String::from("T<Int, Float>"), local_types2.local_type_to_string(local_type2));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_and_type_parameter_for_type_parameter_equations()
{
    let s = "
builtin type Char;
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
    let s3 = "(t, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int, Float>, u: T <Char, Float>, t == u";
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
    let s5 = "(t, u)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U <Int, Float>, u: U <Int, Char>, t == u";
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
    match type_matcher.matches(LocalType::new(2), LocalType::new(4), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(4);
            let local_type3 = LocalType::new(3);
            let local_type4 = LocalType::new(5);
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
                    assert_eq!(String::from("Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t4"), local_types.local_type_to_string(local_type3));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type3))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t6"), local_types.local_type_to_string(local_type4));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type4))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(4)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(4)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(5)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(5)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(4), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(4);
            let local_type3 = LocalType::new(3);
            let local_type4 = LocalType::new(5);
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
                    assert_eq!(String::from("Float"), local_types2.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t4"), local_types2.local_type_to_string(local_type3));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type3))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t6"), local_types2.local_type_to_string(local_type4));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type4))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(4)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(4)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(5)));
            assert_eq!(false, local_types2.has_defined_type_param_eq(LocalType::new(5)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_matches_type_parameter_and_defined_type_parameter_for_type_parameter_equation()
{
    let s = "
builtin type Char;
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
    let s3 = "(t, u)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U <Int, Float>, u: T + U <Int, Char>, t == u";
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
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(t, u)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U <Int, Float>, u: U <Char, Float>, t == u";
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
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    let expected_type_param_entry = match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => type_param_entry.clone(),
        _ => {
            assert!(false);
            return;
        },
    };
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(4), LocalType::new(0), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(4);
            let local_type2 = LocalType::new(0);
            let local_type3 = LocalType::new(5);
            let local_type4 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types.local_type_to_string(local_type1));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type3));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type3))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("u"), local_types.local_type_to_string(local_type4));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type4))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(4)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(4)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(0)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(5)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(5)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(1)));
            assert_eq!(true, local_types.has_defined_type_param_eq(LocalType::new(1)));
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(4);
            let local_type2 = LocalType::new(0);
            let local_type3 = LocalType::new(5);
            let local_type4 = LocalType::new(1);
            assert_eq!(String::from("t"), local_types2.local_type_to_string(local_type1));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type1))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry));
                    match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&expected_type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t3"), local_types2.local_type_to_string(local_type3));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type3))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Float"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("u"), local_types2.local_type_to_string(local_type4));
            match local_types2.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type4))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry, _)) => {
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(2, type_param_entry_r.trait_names.len());
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                    assert_eq!(2, type_param_entry_r.type_values.len());
                    assert_eq!(String::from("Int"), local_types.type_value_to_string(&type_param_entry_r.type_values[0]));
                    assert_eq!(String::from("Char"), local_types.type_value_to_string(&type_param_entry_r.type_values[1]));
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(4)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(4)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(0)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(0)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(5)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(5)));
            assert_eq!(false, local_types2.has_in_non_uniq_lambda(LocalType::new(1)));
            assert_eq!(true, local_types2.has_defined_type_param_eq(LocalType::new(1)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_type_parameter_and_type_parameter()
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
            let s4 = "t: T + U <Int, Int>";
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U + V <Int, Float>";
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
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_type_parameter_and_defined_type_parameter()
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
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U <Int, Float>";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: T <Int, Int>";
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
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_type_parameter_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
trait U<t1, t2> {};
impl U for T {}; 
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
            let s4 = "t: U <Int, Int>";
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
    let s5 = "T<Int, Float>";
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_defined_type_parameter_and_defined_type_parameter()
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
    let s3 = "(t, u)";
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
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_defined_type_parameter_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
trait U<t1, t2> {};
impl U for T {};
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
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: U <Int, Float>";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "T<Int, Float>";
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_type_and_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
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
    let s3 = "T<Int, Int>";
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
    let s5 = "T<Int, Float>";
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => assert_eq!(true, infos.is_empty()),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_param()
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
    let s3 = "t";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int, Float>";
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
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: U <Int, Float>";
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
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::Param(local_type1, trait_name, local_type2) => {
                    assert_eq!(LocalType::new(0), *local_type1);
                    assert_eq!(TraitName::Name(String::from("U")), trait_name.clone());
                    assert_eq!(LocalType::new(3), *local_type2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::Param(local_type1, trait_name, local_type2) => {
                    assert_eq!(LocalType::new(0), *local_type1);
                    assert_eq!(TraitName::Name(String::from("U")), trait_name.clone());
                    assert_eq!(LocalType::new(3), *local_type2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_type()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(Int, t1, t2);
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
            let s4 = "t: U <Int, Float>";
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
    let s5 = "T<Int, Float>";
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
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::Type(type_name, trait_name, local_type2) => {
                    assert_eq!(TypeName::Name(String::from("T")), type_name.clone());
                    assert_eq!(TraitName::Name(String::from("U")), trait_name.clone());
                    assert_eq!(LocalType::new(2), *local_type2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::Type(type_name, trait_name, local_type2) => {
                    assert_eq!(TypeName::Name(String::from("T")), type_name.clone());
                    assert_eq!(TraitName::Name(String::from("U")), trait_name.clone());
                    assert_eq!(LocalType::new(2), *local_type2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_eq()
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
    let s3 = "(t, u)";
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
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    let s5 = "(t, u)";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t == u";
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
    match type_matcher.matches(LocalType::new(4), LocalType::new(0), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(5), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::Eq(local_type1, local_type, local_type2) => {
                    assert_eq!(LocalType::new(5), *local_type1);
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(LocalType::new(1), *local_type2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(0), LocalType::new(4), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Matched) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(5), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::Eq(local_type1, local_type, local_type2) => {
                    assert_eq!(LocalType::new(5), *local_type1);
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(LocalType::new(1), *local_type2);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_shared_param()
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: T <Int, uniq Float>";
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
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::SharedParam(local_type) => {
                    assert_eq!(LocalType::new(3), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::SharedParam(local_type) => {
                    assert_eq!(LocalType::new(3), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_shared_closure_and_type_parameter()
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
    let s5 = "t";
    let mut cursor4 = Cursor::new(s5.as_bytes());
    let mut parser4 = Parser::new(Lexer::new(String::from("test4.vscfl"), &mut cursor4));
    match parser4.parse_type() {
        Ok(type_expr) => {
            let s6 = "t: shared";
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
    assert_eq!(LocalType::new(5), local_types.add_type_value(Rc::new(TypeValue::Type(UniqFlag::Uniq, TypeValueName::Name(String::from("Int")), Vec::new()))));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let mut type_param_entry_r = type_param_entry.borrow_mut();
            type_param_entry_r.closure_local_types.insert(LocalType::new(4));
            type_param_entry_r.closure_local_types.insert(LocalType::new(5));
        }
        _ => assert!(false),
    }
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::SharedClosure(local_type) => {
                    assert_eq!(LocalType::new(5), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::SharedClosure(local_type) => {
                    assert_eq!(LocalType::new(5), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_shared_closure_and_type()
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
    let s5 = "(Int) -> Float";
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
    assert_eq!(LocalType::new(3), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
    assert_eq!(LocalType::new(4), local_types.add_type_value(Rc::new(TypeValue::Type(UniqFlag::Uniq, TypeValueName::Name(String::from("Int")), Vec::new()))));
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(0)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let mut type_param_entry_r = type_param_entry.borrow_mut();
            type_param_entry_r.closure_local_types.insert(LocalType::new(3));
            type_param_entry_r.closure_local_types.insert(LocalType::new(4));
        }
        _ => assert!(false),
    }
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::SharedClosure(local_type) => {
                    assert_eq!(LocalType::new(4), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::SharedClosure(local_type) => {
                    assert_eq!(LocalType::new(4), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_no_closure()
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(LocalType::new(1), local_types.set_defined_type(&typ));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
    assert_eq!(LocalType::new(2), local_types.add_type_param(Rc::new(RefCell::new(TypeParamEntry::new()))));
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
                    match typer.evaluate_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            match local_types.set_type(LocalType::new(2), &typ) {
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
    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(2)))) {
        Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
            let mut type_param_entry_r = type_param_entry.borrow_mut();
            type_param_entry_r.closure_local_types.insert(LocalType::new(4));
            type_param_entry_r.closure_local_types.insert(LocalType::new(5));
        }
        _ => assert!(false),
    }
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(2), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(2, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::NoClosure(closure_local_type, local_type) => {
                    assert_eq!(LocalType::new(4), *closure_local_type);
                    assert_eq!(LocalType::new(0), *local_type);
                },
                _ => assert!(false),
            }
            match &infos[1] {
                MismatchedTypeInfo::NoClosure(closure_local_type, local_type) => {
                    assert_eq!(LocalType::new(5), *closure_local_type);
                    assert_eq!(LocalType::new(0), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(2), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(2, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::NoClosure(closure_local_type, local_type) => {
                    assert_eq!(LocalType::new(4), *closure_local_type);
                    assert_eq!(LocalType::new(0), *local_type);
                },
                _ => assert!(false),
            }
            match &infos[1] {
                MismatchedTypeInfo::NoClosure(closure_local_type, local_type) => {
                    assert_eq!(LocalType::new(5), *closure_local_type);
                    assert_eq!(LocalType::new(0), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_in_non_uniq_lambda()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t1, t2> = C(uniq Int, t1, t2);
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
    let s5 = "T<Int, Float>";
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
    local_types.set_in_non_uniq_lambda(LocalType::new(2), true);
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::InNonUniqLambda => assert!(true),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::InNonUniqLambda => assert!(true),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_matches_does_not_match_types_for_defined_type_param_eq()
{
    let s = "
builtin type Int;
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
    let s5 = "Int";
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
    local_types.set_defined_type_param_eq(LocalType::new(2), true);
    let mut local_types2 = local_types.clone();
    let type_matcher = typer.type_matcher();
    match type_matcher.matches(LocalType::new(0), LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::DefinedTypeParamEq => assert!(true),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match type_matcher.matches(LocalType::new(1), LocalType::new(0), &tree, &mut local_types2) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::DefinedTypeParamEq => assert!(true),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_first_pattern_type_matches_type_parameters_for_non_variable()
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_first_pattern_type(LocalType::new(2), false, LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(2);
            let local_type2 = LocalType::new(1);
            let local_type3 = LocalType::new(4);
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
                    assert_eq!(true, type_param_entry_r.trait_names.is_empty());
                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type3));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type3))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(true, type_param_entry_r.trait_names.is_empty());
                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(4)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(4)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_first_pattern_type_does_not_match_type_parameter_with_shared_and_unique_type_parameter_for_variable()
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
    let s5 = "uniq t";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_first_pattern_type(LocalType::new(2), true, LocalType::new(1), &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::UniqParam(local_type) => {
                    assert_eq!(LocalType::new(2), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_second_pattern_type_matches_type_parameters_for_non_variable()
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
    let s3 = "uniq t";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_second_pattern_type(LocalType::new(0), LocalType::new(3), false, &tree, &mut local_types) {
        Ok(TypeMatcherResult::Matched) => {
            let local_type1 = LocalType::new(3);
            let local_type2 = LocalType::new(0);
            let local_type3 = LocalType::new(4);
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
                    assert_eq!(true, type_param_entry_r.trait_names.is_empty());
                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(String::from("t3"), local_types.local_type_to_string(local_type3));
            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type3))) {
                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type2))) {
                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::Uniq, type_param_entry2, _)) => {
                            assert!(Rc::ptr_eq(&type_param_entry, &type_param_entry2));
                        },
                        _ => assert!(false),
                    }
                    let type_param_entry_r = type_param_entry.borrow();
                    assert_eq!(true, type_param_entry_r.trait_names.is_empty());
                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                },
                _ => assert!(false),
            }
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(2)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(2)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(3)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(3)));
            assert_eq!(false, local_types.has_in_non_uniq_lambda(LocalType::new(4)));
            assert_eq!(false, local_types.has_defined_type_param_eq(LocalType::new(4)));
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_second_pattern_type_does_not_match_unique_type_parameter_and_type_parameter_with_shared_for_variable()
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
    let s3 = "uniq t";
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
            let s6 = "t: shared";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_second_pattern_type(LocalType::new(0), LocalType::new(3), true, &tree, &mut local_types) {
        Ok(TypeMatcherResult::Mismatched(infos)) => {
            assert_eq!(1, infos.len());
            match &infos[0] {
                MismatchedTypeInfo::UniqParam(local_type) => {
                    assert_eq!(LocalType::new(3), *local_type);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_matches_primitive_types()
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
    let s3 = "Int";
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
    let s5 = "Float";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_matches_unique_primitive_types()
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
    let s3 = "uniq Int";
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
    let s5 = "uniq Float";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_matches_tuple_types()
{
    let s = "
builtin type Char;
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
    let s3 = "(Int, Float, Char)";
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
    let s5 = "(Float, Char, Int)";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_matches_array_types()
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
    let s3 = "[Int; 10]";
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
    let s5 = "[Float; 10]";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_matches_tuple_types_with_nested_array_types()
{
    let s = "
builtin type Char;
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
    let s3 = "(Int, [Float; 10], Char)";
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
    let s5 = "(Float, [Char; 10], Int)";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(true) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_does_not_match_primitive_type_and_bool_type()
{
    let s = "
builtin type Bool;
builtin type Int;
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
    let s3 = "Int";
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
    let s5 = "Bool";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.match_for_casting(LocalType::new(1), LocalType::new(0), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_does_not_match_primitive_type_and_data_type()
{
    let s = "
builtin type Int;
data T = C(Int);
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
    let s3 = "Int";
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
    let s5 = "T";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.match_for_casting(LocalType::new(1), LocalType::new(0), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_does_not_match_array_types_with_different_lengths()
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
    let s3 = "[Int; 10]";
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
    let s5 = "[Float; 5]";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.match_for_casting(LocalType::new(1), LocalType::new(0), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_type_matcher_match_for_casting_does_not_match_array_types_without_lengths()
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
    let s3 = "[Int; _]";
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
    let s5 = "[Float; _]";
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
    let type_matcher = typer.type_matcher();
    match type_matcher.match_for_casting(LocalType::new(0), LocalType::new(1), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
    match type_matcher.match_for_casting(LocalType::new(1), LocalType::new(0), &tree, &local_types, typer.builtins()) {
        Ok(false) => assert!(true),
        _ => assert!(false),
    }
}
