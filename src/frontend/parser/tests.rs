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
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(10, pos.column);
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
        Err(_) => assert!(false),
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
        Err(_) => assert!(false),
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

#[test]
fn test_parser_parse_parses_builtin_var_definitions()
{
    let s = "
builtin A;
builtin a;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(2, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("A"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Builtin(None) => assert!(true),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Var(ident, var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Builtin(None) => assert!(true),
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_variable_definitions()
{
    let s = "
A: Int = 1;
a: Int = 2;
private b: Int = 3;
local c: Int = 4;
global d: Int = 5;
constant e: Int = 6;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(6, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("A"), *ident);
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
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(10, pos.column);
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
    match &*tree.defs()[1] {
        Def::Var(ident, var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_expr_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_expr_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(10, pos.column);
                                    match &**literal {
                                        Literal::Int(n) => assert_eq!(2, *n),
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
    match &*tree.defs()[2] {
        Def::Var(ident, var, pos) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::Private, type_expr, where_tuples, expr, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_expr_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(12, pos.column);
                            assert_eq!(String::from("Int"), *type_expr_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**literal {
                                        Literal::Int(n) => assert_eq!(3, *n),
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
    match &*tree.defs()[3] {
        Def::Var(ident, var, pos) => {
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("c"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::Local, type_expr, where_tuples, expr, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_expr_ident, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("Int"), *type_expr_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(16, pos.column);
                                    match &**literal {
                                        Literal::Int(n) => assert_eq!(4, *n),
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
    match &*tree.defs()[4] {
        Def::Var(ident, var, pos) => {
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("d"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::Global, type_expr, where_tuples, expr, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_expr_ident, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(11, pos.column);
                            assert_eq!(String::from("Int"), *type_expr_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(5, pos.line);
                                    assert_eq!(17, pos.column);
                                    match &**literal {
                                        Literal::Int(n) => assert_eq!(5, *n),
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
    match &*tree.defs()[5] {
        Def::Var(ident, var, pos) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("e"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::Constant, type_expr, where_tuples, expr, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_expr_ident, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(13, pos.column);
                            assert_eq!(String::from("Int"), *type_expr_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(6, pos.line);
                                    assert_eq!(19, pos.column);
                                    match &**literal {
                                        Literal::Int(n) => assert_eq!(6, *n),
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
fn test_parser_parse_parses_function_definitions()
{
    let s = "
F() -> Int = 1;
f() -> Int = 2;
g(x: Int, y: Int) -> Int = x + y;
inline h(x: Int, y: Int) -> Int = x - y;
kernel i() -> () = ();
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(5, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("F"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, InlineModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(true, args.is_empty());
                            match &**ret_type_expr {
                                TypeExpr::Var(type_expr_ident, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(8, pos.column);
                                    assert_eq!(String::from("Int"), *type_expr_ident);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, where_tuples.is_empty());
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(14, pos.column);
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Var(ident, var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("f"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, InlineModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(true, args.is_empty());
                            match &**ret_type_expr {
                                TypeExpr::Var(type_expr_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(8, pos.column);
                                    assert_eq!(String::from("Int"), *type_expr_ident);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, where_tuples.is_empty());
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(14, pos.column);
                                            match &**literal {
                                                Literal::Int(n) => assert_eq!(2, *n),
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Var(ident, var, pos) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("g"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, InlineModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(2, args.len());
                            match &args[0] {
                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(3, pos.column);
                                    assert_eq!(String::from("x"), *arg_ident);
                                    match &**arg_type_expr {
                                        TypeExpr::Var(type_expr_ident, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(6, pos.column);
                                            assert_eq!(String::from("Int"), *type_expr_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &args[1] {
                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("y"), *arg_ident);
                                    match &**arg_type_expr {
                                        TypeExpr::Var(type_expr_ident, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(14, pos.column);
                                            assert_eq!(String::from("Int"), *type_expr_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &**ret_type_expr {
                                TypeExpr::Var(type_expr_ident, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(22, pos.column);
                                    assert_eq!(String::from("Int"), *type_expr_ident);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, where_tuples.is_empty());
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::App(expr, arg_exprs, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(30, pos.column);
                                            match &**expr {
                                                Expr::Var(ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(30, pos.column);
                                                    assert_eq!(String::from("op_add"), *ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, arg_exprs.len());
                                            match &*arg_exprs[0] {
                                                Expr::Var(ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(28, pos.column);
                                                    assert_eq!(String::from("x"), *ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*arg_exprs[1] {
                                                Expr::Var(ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(32, pos.column);
                                                    assert_eq!(String::from("y"), *ident);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Var(ident, var, pos) => {
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("h"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, InlineModifier::Inline, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(2, args.len());
                            match &args[0] {
                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(10, pos.column);
                                    assert_eq!(String::from("x"), *arg_ident);
                                    match &**arg_type_expr {
                                        TypeExpr::Var(type_expr_ident, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(13, pos.column);
                                            assert_eq!(String::from("Int"), *type_expr_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &args[1] {
                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(18, pos.column);
                                    assert_eq!(String::from("y"), *arg_ident);
                                    match &**arg_type_expr {
                                        TypeExpr::Var(type_expr_ident, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(21, pos.column);
                                            assert_eq!(String::from("Int"), *type_expr_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &**ret_type_expr {
                                TypeExpr::Var(type_expr_ident, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(29, pos.column);
                                    assert_eq!(String::from("Int"), *type_expr_ident);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, where_tuples.is_empty());
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::App(expr, arg_exprs, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(37, pos.column);
                                            match &**expr {
                                                Expr::Var(ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(37, pos.column);
                                                    assert_eq!(String::from("op_sub"), *ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, arg_exprs.len());
                                            match &*arg_exprs[0] {
                                                Expr::Var(ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(35, pos.column);
                                                    assert_eq!(String::from("x"), *ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*arg_exprs[1] {
                                                Expr::Var(ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(39, pos.column);
                                                    assert_eq!(String::from("y"), *ident);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[4] {
        Def::Var(ident, var, pos) => {
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("i"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::Kernel, InlineModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(true, args.is_empty());
                            match &**ret_type_expr {
                                TypeExpr::Tuple(type_exprs, pos) => {
                                    assert_eq!(5, pos.line);
                                    assert_eq!(15, pos.column);
                                    assert_eq!(true, type_exprs.is_empty());
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(true, where_tuples.is_empty());
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(20, pos.column);
                                            match &**literal {
                                                Literal::Tuple(exprs) => assert_eq!(true, exprs.is_empty()),
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
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_trait_definitions()
{
    let s = "
trait T {};
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
    match &*tree.defs()[0] {
        Def::Trait(ident, trait1, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(type_args, trait_defs, None) => {
                    assert_eq!(true, type_args.is_empty());
                    assert_eq!(true, trait_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Trait(ident, trait1, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("U"), *ident);
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(type_args, trait_defs, None) => {
                    assert_eq!(2, type_args.len());
                    match &type_args[0] {
                        TypeArg(type_arg_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(9, pos.column);
                            assert_eq!(String::from("t1"), *type_arg_ident);
                        },
                    }
                    match &type_args[1] {
                        TypeArg(type_arg_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(13, pos.column);
                            assert_eq!(String::from("t2"), *type_arg_ident);
                        },
                    }
                    assert_eq!(true, trait_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_builtin_implementation_definition()
{
    let s = "
builtin impl T for U;
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
        Def::Impl(impl1, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Builtin(trait_ident, type_name, None) => {
                    assert_eq!(String::from("T"), *trait_ident);
                    assert_eq!(TypeName::Name(String::from("U")), *type_name);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_implementation_definition()
{
    let s = "
impl T for U {};
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
        Def::Impl(impl1, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("T"), *trait_ident);
                    assert_eq!(TypeName::Name(String::from("U")), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
