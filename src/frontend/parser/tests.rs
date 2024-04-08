//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::*;
use super::*;

#[test]
fn test_parser_parse_parses_variable()
{
    let s = "
x: Int = 1;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(1, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("x"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_expr_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_expr_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr1) => {
                            match &**expr1 {
                                Expr::Literal(literal, None, pos2) => {
                                    assert_eq!(1, pos2.line);
                                    assert_eq!(10, pos2.column);
                                    match &**literal {
                                        Literal::Int(n) => assert_eq!(1, *n),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
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
fn test_parser_parse_parses_builtin_type_definition()
{
    let s = "
builtin type T;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(1, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(None, None) => assert!(true),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_data_definitions()
{
    let s = "
data T;
data U = C();
data V<t1, t2> = D() | E(t1, t2);
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(err) => {
            println!("{}", err);
            assert!(false);
        },
    }
    assert_eq!(3, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(type_args, cons, None) => {
                    assert_eq!(true, type_args.is_empty());
                    assert_eq!(true, cons.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("U"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(type_args, cons, None) => {
                    assert_eq!(true, type_args.is_empty());
                    assert_eq!(1, cons.len());
                    let con1_r = cons[0].borrow();
                    match &*con1_r {
                        Con::UnnamedField(con_ident, type_exprs, data_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("C"), *con_ident);
                            assert_eq!(true, type_exprs.is_empty());
                            assert_eq!(String::from("U"), *data_ident);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("V"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(type_args, cons, None) => {
                    assert_eq!(2, type_args.len());
                    match &type_args[0] {
                        TypeArg(type_arg_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(8, pos.column);
                            assert_eq!(String::from("t1"), *type_arg_ident);
                        },
                    }
                    match &type_args[1] {
                        TypeArg(type_arg_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(12, pos.column);
                            assert_eq!(String::from("t2"), *type_arg_ident);
                        },
                    }
                    assert_eq!(2, cons.len());
                    let con1_r = cons[0].borrow();
                    match &*con1_r {
                        Con::UnnamedField(con_ident, type_exprs, data_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(18, pos.column);
                            assert_eq!(String::from("D"), *con_ident);
                            assert_eq!(true, type_exprs.is_empty());
                            assert_eq!(String::from("V"), *data_ident);
                        },
                        _ => assert!(false),
                    }
                    let con2_r = cons[1].borrow();
                    match &*con2_r {
                        Con::UnnamedField(con_ident, type_exprs, data_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(24, pos.column);
                            assert_eq!(String::from("E"), *con_ident);
                            assert_eq!(2, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(26, pos.column);
                                    assert_eq!(String::from("t1"), *type_param_ident);
                                }
                                _ => assert!(false),
                            }
                            match &*type_exprs[1] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(30, pos.column);
                                    assert_eq!(String::from("t2"), *type_param_ident);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("V"), *data_ident);
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
fn test_parser_parse_parses_type_definitions()
{
    let s = "
type T = Int;
type U<t1, t2> = (t1, t2);
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(err) => {
            println!("{}", err);
            assert!(false);
        },
    }
    assert_eq!(2, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("U"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr) => {
                    assert_eq!(2, type_args.len());
                    match &type_args[0] {
                        TypeArg(type_arg_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(8, pos.column);
                            assert_eq!(String::from("t1"), *type_arg_ident);
                        },
                    }
                    match &type_args[1] {
                        TypeArg(type_arg_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(12, pos.column);
                            assert_eq!(String::from("t2"), *type_arg_ident);
                        },
                    }
                    match &**type_expr {
                        TypeExpr::Tuple(type_exprs, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(18, pos.column);
                            assert_eq!(2, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(19, pos.column);
                                    assert_eq!(String::from("t1"), *type_param_ident);
                                }
                                _ => assert!(false),
                            }
                            match &*type_exprs[1] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(23, pos.column);
                                    assert_eq!(String::from("t2"), *type_param_ident);
                                },
                                _ => assert!(false),
                            }
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
