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
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
                                },
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
                TypeVar::Synonym(type_args, type_expr, None) => {
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
                TypeVar::Synonym(type_args, type_expr, None) => {
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
                                },
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
fn test_parser_parse_parses_builtin_variable_definitions()
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
                Var::Builtin(None, None) => assert!(true),
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
                Var::Builtin(None, None) => assert!(true),
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
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Var(VarModifier::Private, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(12, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Var(VarModifier::Local, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Var(VarModifier::Global, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(11, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Var(VarModifier::Constant, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(13, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Fun(fun, None, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(true, args.is_empty());
                            match &**ret_type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(8, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Fun(fun, None, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(true, args.is_empty());
                            match &**ret_type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(8, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Fun(fun, None, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(2, args.len());
                            match &args[0] {
                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(3, pos.column);
                                    assert_eq!(String::from("x"), *arg_ident);
                                    match &**arg_type_expr {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(6, pos.column);
                                            assert_eq!(String::from("Int"), *type_var_ident);
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
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(14, pos.column);
                                            assert_eq!(String::from("Int"), *type_var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &**ret_type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(22, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Fun(fun, None, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::Inline, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(2, args.len());
                            match &args[0] {
                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(10, pos.column);
                                    assert_eq!(String::from("x"), *arg_ident);
                                    match &**arg_type_expr {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(13, pos.column);
                                            assert_eq!(String::from("Int"), *type_var_ident);
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
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(21, pos.column);
                                            assert_eq!(String::from("Int"), *type_var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &**ret_type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(29, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
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
                Var::Fun(fun, None, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::Kernel, args, ret_type_expr, where_tuples, expr, None, None) => {
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
    assert_eq!(2, tree.defs().len());
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

#[test]
fn test_parser_parse_parses_constructors()
{
    let s = "
data T = C()
       | D(Float, Int)
       | E {}
       | F { x: Int, y: Float, };
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
                TypeVar::Data(type_args, cons, None) => {
                    assert_eq!(true, type_args.is_empty());
                    assert_eq!(4, cons.len());
                    let con1_r = cons[0].borrow();
                    match &*con1_r {
                        Con::UnnamedField(con_ident, type_exprs, data_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("C"), *con_ident);
                            assert_eq!(true, type_exprs.is_empty());
                            assert_eq!(String::from("T"), *data_ident);
                        },
                        _ => assert!(false),
                    }
                    let con2_r = cons[1].borrow();
                    match &*con2_r {
                        Con::UnnamedField(con_ident, type_exprs, data_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("D"), *con_ident);
                            assert_eq!(2, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(12, pos.column);
                                    assert_eq!(String::from("Float"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                            match &*type_exprs[1] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(19, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(String::from("T"), *data_ident);
                        },
                        _ => assert!(false),
                    }
                    let con3_r = cons[2].borrow();
                    match &*con3_r {
                        Con::NamedField(con_ident, type_expr_named_field_pairs, data_ident, None, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("E"), *con_ident);
                            assert_eq!(true, type_expr_named_field_pairs.is_empty());
                            assert_eq!(String::from("T"), *data_ident);
                        },
                        _ => assert!(false),
                    }
                    let con4_r = cons[3].borrow();
                    match &*con4_r {
                        Con::NamedField(con_ident, type_expr_named_field_pairs, data_ident, None, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("F"), *con_ident);
                            assert_eq!(2, type_expr_named_field_pairs.len());
                            match &type_expr_named_field_pairs[0] {
                                NamedFieldPair(field_ident, type_expr, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(14, pos.column);
                                    assert_eq!(String::from("x"), *field_ident);
                                    match &**type_expr {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(17, pos.column);
                                            assert_eq!(String::from("Int"), *type_var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                            }
                            match &type_expr_named_field_pairs[1] {
                                NamedFieldPair(field_ident, type_expr, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(22, pos.column);
                                    assert_eq!(String::from("y"), *field_ident);
                                    match &**type_expr {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(25, pos.column);
                                            assert_eq!(String::from("Float"), *type_var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                            }
                            assert_eq!(String::from("T"), *data_ident);
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
fn test_parser_parse_parses_type_expressions()
{
    let s = "
type T = ();
type U = (Int, Char);
type V = () -> Int;
type W = (Float, Int) -> Char;
type X = [Int; _];
type Y = [Float; 12];
type Z<t1> = t1;
type A = Int;
type B = S<Int, S2<Float>>;
type C = uniq Array;
type D = uniq (Int);
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(11, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Tuple(type_exprs, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(true, type_exprs.is_empty());
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
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Tuple(type_exprs, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(2, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                            match &*type_exprs[1] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(16, pos.column);
                                    assert_eq!(String::from("Char"), *type_var_ident);
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
        Def::Type(ident, type_var, pos) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("V"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Fun(arg_type_exprs, ret_type_expr, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(true, arg_type_exprs.is_empty());
                            match &**ret_type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(16, pos.column);
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("W"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Fun(arg_type_exprs, ret_type_expr, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(2, arg_type_exprs.len());
                            match &*arg_type_exprs[0] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("Float"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                            match &*arg_type_exprs[1] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(18, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                            match &**ret_type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(26, pos.column);
                                    assert_eq!(String::from("Char"), *type_var_ident);
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
        Def::Type(ident, type_var, pos) => {
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("X"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Array(type_expr, None, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(10, pos.column);
                            match &**type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(5, pos.line);
                                    assert_eq!(11, pos.column);
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[5] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("Y"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Array(type_expr, Some(len), pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(10, pos.column);
                            match &**type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(6, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("Float"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(12, *len);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[6] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(7, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("Z"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(1, type_args.len());
                    match &type_args[0] {
                        TypeArg(type_arg_ident, pos) => {
                            assert_eq!(7, pos.line);
                            assert_eq!(8, pos.column);
                            assert_eq!(String::from("t1"), *type_arg_ident);
                        },
                    }
                    match &**type_expr {
                        TypeExpr::Param(type_param_ident, pos) => {
                            assert_eq!(7, pos.line);
                            assert_eq!(14, pos.column);
                            assert_eq!(String::from("t1"), *type_param_ident);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[7] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(8, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("A"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(8, pos.line);
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
    match &*tree.defs()[8] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(9, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("B"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::App(type_var_ident, type_exprs, pos) => {
                            assert_eq!(9, pos.line);
                            assert_eq!(10, pos.column);
                            assert_eq!(String::from("S"), *type_var_ident);
                            assert_eq!(2, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(9, pos.line);
                                    assert_eq!(12, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                            match &*type_exprs[1] {
                               TypeExpr::App(type_var_ident, type_exprs, pos) => {
                                    assert_eq!(9, pos.line);
                                    assert_eq!(17, pos.column);
                                    assert_eq!(String::from("S2"), *type_var_ident);
                                    assert_eq!(1, type_exprs.len());
                                    match &*type_exprs[0] {
                                       TypeExpr::Var(type_var_ident, pos) => {
                                           assert_eq!(9, pos.line);
                                           assert_eq!(20, pos.column);
                                           assert_eq!(String::from("Float"), *type_var_ident);
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
    match &*tree.defs()[9] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(10, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("C"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Uniq(type_expr, pos) => {
                            assert_eq!(10, pos.line);
                            assert_eq!(10, pos.column);
                            match &**type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(10, pos.line);
                                    assert_eq!(15, pos.column);
                                    assert_eq!(String::from("Array"), *type_var_ident);
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
    match &*tree.defs()[10] {
        Def::Type(ident, type_var, pos) => {
            assert_eq!(11, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("D"), *ident);
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(type_args, type_expr, None) => {
                    assert_eq!(true, type_args.is_empty());
                    match &**type_expr {
                        TypeExpr::Uniq(type_expr, pos) => {
                            assert_eq!(11, pos.line);
                            assert_eq!(10, pos.column);
                            match &**type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(11, pos.line);
                                    assert_eq!(16, pos.column);
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
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_where_tuples()
{
    let s = "
a: (t1, t2, t3)
    where t1: shared,
          t2: T + U <t1, Int>,
          t3: -> <Int> = g();
f() -> (t1, t2, t3)
    where t1: shared,
          t2: T + U <t1, Int>,
          t3: -> <Int> = g();
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Tuple(type_exprs, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(3, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(String::from("t1"), *type_param_ident);
                                },
                                _ => assert!(false),
                            }
                            match &*type_exprs[1] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(9, pos.column);
                                    assert_eq!(String::from("t2"), *type_param_ident);
                                },
                                _ => assert!(false),
                            }
                            match &*type_exprs[2] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(13, pos.column);
                                    assert_eq!(String::from("t3"), *type_param_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(3, where_tuples.len());
                    match &where_tuples[0] {
                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(11, pos.column);
                            assert_eq!(String::from("t1"), *type_param_ident);
                            assert_eq!(1, trait_names.len());
                            assert_eq!(TraitName::Shared, trait_names[0]);
                            assert_eq!(true, type_exprs.is_empty());
                        },
                    }
                    match &where_tuples[1] {
                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(11, pos.column);
                            assert_eq!(String::from("t2"), *type_param_ident);
                            assert_eq!(2, trait_names.len());
                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                            assert_eq!(TraitName::Name(String::from("U")), trait_names[1]);
                            assert_eq!(2, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Param(type_param_ident, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(22, pos.column);
                                    assert_eq!(String::from("t1"), *type_param_ident);
                                },
                                _ => assert!(false),
                            }
                            match &*type_exprs[1] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(26, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &where_tuples[2] {
                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(11, pos.column);
                            assert_eq!(String::from("t3"), *type_param_ident);
                            assert_eq!(1, trait_names.len());
                            assert_eq!(TraitName::Fun, trait_names[0]);
                            assert_eq!(1, type_exprs.len());
                            match &*type_exprs[0] {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(19, pos.column);
                                    assert_eq!(String::from("Int"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(26, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(26, pos.column);
                                            assert_eq!(String::from("g"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(true, exprs.is_empty());
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
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("f"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, None, None) => {
                    match &**fun {
                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                            assert_eq!(true, args.is_empty());
                            match &**ret_type_expr {
                                TypeExpr::Tuple(type_exprs, pos) => {
                                    assert_eq!(5, pos.line);
                                    assert_eq!(8, pos.column);
                                    assert_eq!(3, type_exprs.len());
                                    match &*type_exprs[0] {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(9, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*type_exprs[1] {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(13, pos.column);
                                            assert_eq!(String::from("t2"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*type_exprs[2] {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(17, pos.column);
                                            assert_eq!(String::from("t3"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(3, where_tuples.len());
                            match &where_tuples[0] {
                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                    assert_eq!(6, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("t1"), *type_param_ident);
                                    assert_eq!(1, trait_names.len());
                                    assert_eq!(TraitName::Shared, trait_names[0]);
                                    assert_eq!(true, type_exprs.is_empty());
                                },
                            }
                            match &where_tuples[1] {
                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                    assert_eq!(7, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("t2"), *type_param_ident);
                                    assert_eq!(2, trait_names.len());
                                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                    assert_eq!(TraitName::Name(String::from("U")), trait_names[1]);
                                    assert_eq!(2, type_exprs.len());
                                    match &*type_exprs[0] {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(7, pos.line);
                                            assert_eq!(22, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*type_exprs[1] {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(7, pos.line);
                                            assert_eq!(26, pos.column);
                                            assert_eq!(String::from("Int"), *type_var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                            }
                            match &where_tuples[2] {
                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                    assert_eq!(8, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("t3"), *type_param_ident);
                                    assert_eq!(1, trait_names.len());
                                    assert_eq!(TraitName::Fun, trait_names[0]);
                                    assert_eq!(1, type_exprs.len());
                                    match &*type_exprs[0] {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(8, pos.line);
                                            assert_eq!(19, pos.column);
                                            assert_eq!(String::from("Int"), *type_var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                            }
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(8, pos.line);
                                            assert_eq!(26, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(26, pos.column);
                                                    assert_eq!(String::from("g"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(true, exprs.is_empty());
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
fn test_parser_parse_parses_expressions1()
{
    let s = "
a: Int = if true then 1 else 2;
b: Int =
    let x = 1;
        y = 2;
    in  x + y;
c: Int =
    x match {
        Some(y) => y;
        _ => 1;
    };
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
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::If(expr1, expr2, expr3, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(10, pos.column);
                                    match &**expr1 {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**literal {
                                                Literal::Bool(b) => assert_eq!(true, *b),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr2 {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(23, pos.column);
                                            match &**literal {
                                                Literal::Int(n) => assert_eq!(1, *n),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(30, pos.column);
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
    match &*tree.defs()[1] {
        Def::Var(ident, var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(2, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(VarModifier::None, var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("x"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[1] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(VarModifier::None, var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(11, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    assert_eq!(String::from("op_add"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("x"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
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
    match &*tree.defs()[2] {
        Def::Var(ident, var, pos) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("c"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Match(expr, cases, None, pos) => {
                                    assert_eq!(7, pos.line);
                                    assert_eq!(5, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(7, pos.line);
                                            assert_eq!(5, pos.column);
                                            assert_eq!(String::from("x"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, cases.len());
                                    match &cases[0] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("Some"), *con_ident);
                                                    assert_eq!(1, patterns.len());
                                                    match &*patterns[0] {
                                                        Pattern::Var(VarModifier::None, var_ident, None, pos) => {
                                                            assert_eq!(8, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("y"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[1] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(9, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(9, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
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
fn test_parser_parse_parses_expressions2()
{
    let s = "
a: Int = shared 1: Int;
b: Int = shared 2.5 as Int;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Typed(expr, type_expr, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Shared(expr, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(10, pos.column);
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(17, pos.column);
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
                                    match &**type_expr {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(20, pos.column);
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
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::As(expr, type_expr, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(21, pos.column);
                                    match &**expr {
                                        Expr::Shared(expr, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(10, pos.column);
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    match &**literal {
                                                        Literal::Float(n) => assert_eq!(2.5, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**type_expr {
                                        TypeExpr::Var(type_var_ident, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(24, pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expressions3()
{
    let s = "
a: uniq Bool = uniq true | false;
b: Bool = shared false | true;
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
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Uniq(type_expr, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            match &**type_expr {
                                TypeExpr::Var(type_var_ident, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(9, pos.column);
                                    assert_eq!(String::from("Bool"), *type_var_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Uniq(expr, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(16, pos.column);
                                    match &**expr {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(26, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(26, pos.column);
                                                    assert_eq!(String::from("op_or"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(true, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(28, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(false, *b),
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Var(ident, var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Shared(expr, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(11, pos.column);
                                    match &**expr {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(24, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(24, pos.column);
                                                    assert_eq!(String::from("op_or"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(false, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(26, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(true, *b),
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
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression4()
{
    let s = "
a: Bool = true ^ false | false ^ true;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(24, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(24, pos.column);
                                            assert_eq!(String::from("op_or"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(16, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    assert_eq!(String::from("op_xor"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(true, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(false, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(32, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(32, pos.column);
                                                    assert_eq!(String::from("op_xor"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(26, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(false, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(34, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(true, *b),
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
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression5()
{
    let s = "
a: Bool = true & false ^ false & true;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(24, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(24, pos.column);
                                            assert_eq!(String::from("op_xor"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(16, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    assert_eq!(String::from("op_and"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(true, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(false, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(32, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(32, pos.column);
                                                    assert_eq!(String::from("op_and"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(26, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(false, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(34, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(true, *b),
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
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression6()
{
    let s = "
a: Bool = 1 == 2 & 3 != 4;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_and"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_eq"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(16, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(22, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(22, pos.column);
                                                    assert_eq!(String::from("op_ne"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(25, pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expressions7()
{
    let s = "
a: Bool = 1 << 2 == 3 >> 4;
b: Bool = 1 << 2 != 3 >> 4;
c: Bool = 1 << 2 < 3 >> 4;
d: Bool = 1 << 2 >= 3 >> 4;
e: Bool = 1 << 2 > 3 >> 4;
f: Bool = 1 << 2 <= 3 >> 4;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_eq"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_shl"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(16, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(23, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(23, pos.column);
                                                    assert_eq!(String::from("op_shr"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(26, pos.column);
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
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_ne"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_shl"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(16, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(23, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(23, pos.column);
                                                    assert_eq!(String::from("op_shr"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(26, pos.column);
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
            assert_eq!(String::from("c"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_lt"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_shl"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(16, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(22, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(22, pos.column);
                                                    assert_eq!(String::from("op_shr"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(25, pos.column);
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
            assert_eq!(String::from("d"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_ge"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_shl"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(16, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(23, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(23, pos.column);
                                                    assert_eq!(String::from("op_shr"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(26, pos.column);
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
            assert_eq!(String::from("e"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(5, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_gt"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_shl"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(16, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(22, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(22, pos.column);
                                                    assert_eq!(String::from("op_shr"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(25, pos.column);
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
            assert_eq!(String::from("f"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(6, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(6, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_le"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(6, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_shl"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(16, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(6, pos.line);
                                            assert_eq!(23, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(23, pos.column);
                                                    assert_eq!(String::from("op_shr"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(26, pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expressions8()
{
    let s = "
a: Int = 1 + 2 << 3 - 4;
b: Int = 1 + 2 >> 3 - 4;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(16, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(16, pos.column);
                                            assert_eq!(String::from("op_shl"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(12, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(12, pos.column);
                                                    assert_eq!(String::from("op_add"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
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
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(21, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    assert_eq!(String::from("op_sub"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(19, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(23, pos.column);
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
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(16, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(16, pos.column);
                                            assert_eq!(String::from("op_shr"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(12, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(12, pos.column);
                                                    assert_eq!(String::from("op_add"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(21, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    assert_eq!(String::from("op_sub"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(19, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(23, pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expressions9()
{
    let s = "
a: Int = 1 * 2 + 3 / 4;
b: Int = 1 * 2 - 3 / 4;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(16, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(16, pos.column);
                                            assert_eq!(String::from("op_add"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(12, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(12, pos.column);
                                                    assert_eq!(String::from("op_mul"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
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
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(20, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    assert_eq!(String::from("op_div"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(22, pos.column);
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
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(16, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(16, pos.column);
                                            assert_eq!(String::from("op_sub"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(12, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(12, pos.column);
                                                    assert_eq!(String::from("op_mul"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(20, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    assert_eq!(String::from("op_div"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(22, pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expressions10()
{
    let s = "
a: Int = -1 * !2;
b: Int = -1 / !2;
c: Int = -1 % !2;
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
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(13, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(13, pos.column);
                                            assert_eq!(String::from("op_mul"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(10, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    assert_eq!(String::from("op_neg"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(11, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(15, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    assert_eq!(String::from("op_not"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(16, pos.column);
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Var(ident, var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(13, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(13, pos.column);
                                            assert_eq!(String::from("op_div"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(10, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    assert_eq!(String::from("op_neg"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(11, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(15, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    assert_eq!(String::from("op_not"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(2, pos.line);
                                                    assert_eq!(16, pos.column);
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Var(ident, var, pos) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("c"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(13, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(13, pos.column);
                                            assert_eq!(String::from("op_rem"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(10, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    assert_eq!(String::from("op_neg"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(11, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(15, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    assert_eq!(String::from("op_not"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(16, pos.column);
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
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expression11()
{
    let s = "
a: Int = -!1;
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(10, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(10, pos.column);
                                            assert_eq!(String::from("op_neg"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(11, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    assert_eq!(String::from("op_not"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(12, pos.column);
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
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_expressions12()
{
    let s = "
a: Int = f(1, 2);
b: Int =
    let _ = x.0.y;
        _ = x.0.y ->;
        _ = x.0.y <- y;
        _ = x.0.y <-> f;
        _ = x.0.y <-> f ->;
        _ = x[i].0.y ->;
    in  1;
c: Int =
    let _ = x[i];
        _ = x[i] ->;
        _ = x[i] <- y;
        _ = x[i] <-> f;
        _ = x[i] <-> f ->;
        _ = x[i][j] ->;
    in  2;
d: Int =
    let _ = *x;
        _ = *x ->;
        _ = *x <- y;
        _ = *x <-> f;
        _ = *x <-> f ->;
        _ = *x[i] ->;
    in  3;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(4, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(10, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(10, pos.column);
                                            assert_eq!(String::from("f"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(12, pos.column);
                                            match &**literal {
                                                Literal::Int(n) => assert_eq!(1, *n),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(15, pos.column);
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
    match &*tree.defs()[1] {
        Def::Var(ident, var, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(6, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::GetField(expr1, fields, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr1 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(3, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(vec![Field::Unnamed(0), Field::Named(String::from("y"))], *fields);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[1] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Get2Field(expr1, fields, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr1 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(4, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(vec![Field::Unnamed(0), Field::Named(String::from("y"))], *fields);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[2] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::SetField(expr1, fields, expr2, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr1 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(vec![Field::Unnamed(0), Field::Named(String::from("y"))], *fields);
                                                    match &**expr2 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(22, pos.column);
                                                            assert_eq!(String::from("y"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[3] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::UpdateField(expr1, fields, expr2, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr1 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(6, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(vec![Field::Unnamed(0), Field::Named(String::from("y"))], *fields);
                                                    match &**expr2 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(6, pos.line);
                                                            assert_eq!(23, pos.column);
                                                            assert_eq!(String::from("f"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[4] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::UpdateGet2Field(expr1, fields, expr2, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr1 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(7, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(vec![Field::Unnamed(0), Field::Named(String::from("y"))], *fields);
                                                    match &**expr2 {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(7, pos.line);
                                                            assert_eq!(23, pos.column);
                                                            assert_eq!(String::from("f"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[5] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Get2Field(expr1, fields, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr1 {
                                                        Expr::App(expr, exprs, None, pos) => {
                                                            assert_eq!(8, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            match &**expr {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(8, pos.line);
                                                                    assert_eq!(13, pos.column);
                                                                    assert_eq!(String::from("op_get_nth"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            assert_eq!(2, exprs.len());
                                                            match &*exprs[0] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(8, pos.line);
                                                                    assert_eq!(13, pos.column);
                                                                    assert_eq!(String::from("x"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*exprs[1] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(8, pos.line);
                                                                    assert_eq!(15, pos.column);
                                                                    assert_eq!(String::from("i"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(vec![Field::Unnamed(0), Field::Named(String::from("y"))], *fields);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(9, pos.line);
                                            assert_eq!(9, pos.column);
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
    match &*tree.defs()[2] {
        Def::Var(ident, var, pos) => {
            assert_eq!(10, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("c"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(10, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, None, pos) => {
                                    assert_eq!(11, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(6, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(11, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(11, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(11, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_get_nth"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(11, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(11, pos.line);
                                                            assert_eq!(15, pos.column);
                                                            assert_eq!(String::from("i"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[1] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(12, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_get2_nth"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(12, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(12, pos.line);
                                                            assert_eq!(15, pos.column);
                                                            assert_eq!(String::from("i"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[2] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(13, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(13, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(13, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_set_nth"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(3, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(13, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(13, pos.line);
                                                            assert_eq!(15, pos.column);
                                                            assert_eq!(String::from("i"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[2] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(13, pos.line);
                                                            assert_eq!(21, pos.column);
                                                            assert_eq!(String::from("y"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[3] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(14, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_update_nth"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(3, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(14, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(14, pos.line);
                                                            assert_eq!(15, pos.column);
                                                            assert_eq!(String::from("i"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[2] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(14, pos.line);
                                                            assert_eq!(22, pos.column);
                                                            assert_eq!(String::from("f"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[4] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(15, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(15, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(15, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_update_get2_nth"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(3, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(15, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(15, pos.line);
                                                            assert_eq!(15, pos.column);
                                                            assert_eq!(String::from("i"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[2] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(15, pos.line);
                                                            assert_eq!(22, pos.column);
                                                            assert_eq!(String::from("f"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[5] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(16, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(16, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(16, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_get2_nth"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::App(expr, exprs, None, pos) => {
                                                            assert_eq!(16, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            match &**expr {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(16, pos.line);
                                                                    assert_eq!(13, pos.column);
                                                                    assert_eq!(String::from("op_get_nth"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            assert_eq!(2, exprs.len());
                                                            match &*exprs[0] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(16, pos.line);
                                                                    assert_eq!(13, pos.column);
                                                                    assert_eq!(String::from("x"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*exprs[1] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(16, pos.line);
                                                                    assert_eq!(15, pos.column);
                                                                    assert_eq!(String::from("i"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(16, pos.line);
                                                            assert_eq!(18, pos.column);
                                                            assert_eq!(String::from("j"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(17, pos.line);
                                            assert_eq!(9, pos.column);
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
    match &*tree.defs()[3] {
        Def::Var(ident, var, pos) => {
            assert_eq!(18, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("d"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(18, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, None, pos) => {
                                    assert_eq!(19, pos.line);
                                    assert_eq!(5, pos.column);
                                    assert_eq!(6, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(19, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(19, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(19, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_get"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(1, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(19, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[1] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(20, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(20, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(20, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_get2"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(1, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(20, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[2] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(21, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(21, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(21, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_set"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(21, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(21, pos.line);
                                                            assert_eq!(19, pos.column);
                                                            assert_eq!(String::from("y"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[3] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(22, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(22, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(22, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_update"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(22, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(22, pos.line);
                                                            assert_eq!(20, pos.column);
                                                            assert_eq!(String::from("f"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[4] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(23, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(23, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(23, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_update_get2"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(23, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(23, pos.line);
                                                            assert_eq!(20, pos.column);
                                                            assert_eq!(String::from("f"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &binds[5] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(24, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(24, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(24, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("op_get2"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(1, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::App(expr, exprs, None, pos) => {
                                                            assert_eq!(24, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            match &**expr {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(24, pos.line);
                                                                    assert_eq!(14, pos.column);
                                                                    assert_eq!(String::from("op_get_nth"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            assert_eq!(2, exprs.len());
                                                            match &*exprs[0] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(24, pos.line);
                                                                    assert_eq!(14, pos.column);
                                                                    assert_eq!(String::from("x"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*exprs[1] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(24, pos.line);
                                                                    assert_eq!(16, pos.column);
                                                                    assert_eq!(String::from("i"), *var_ident);
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
                                    }
                                    match &**expr {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(25, pos.line);
                                            assert_eq!(9, pos.column);
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
        },
        _ => assert!(false),
    }    
}

#[test]
fn test_parser_parse_parses_expressions13()
{
    let s = "
a: Int = (1 + 2) * (3 - 4);
b: Int = 1;
c: Fun = |x: Int, y| x + y;
d: Fun = |x, y: Int| -> Int x - y;
e: Int = X;
f: Int = x;
g: T = C { x: 1, y: 2, };
h: () = printf(\"%d\\n\", 2);
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(8, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(18, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("op_mul"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(13, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    assert_eq!(String::from("op_add"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(15, pos.column);
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
                                    match &*exprs[1] {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(1, pos.line);
                                            assert_eq!(23, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(23, pos.column);
                                                    assert_eq!(String::from("op_sub"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(1, pos.line);
                                                    assert_eq!(25, pos.column);
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
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
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
    match &*tree.defs()[2] {
        Def::Var(ident, var, pos) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("c"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Fun"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Lambda(lambda_args, None, expr, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(10, pos.column);
                                    assert_eq!(2, lambda_args.len());
                                    match &lambda_args[0] {
                                        LambdaArg(arg_ident, arg_type_expr, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(11, pos.column);
                                            assert_eq!(String::from("x"), *arg_ident);
                                            match arg_type_expr {
                                                Some(arg_type_expr) => {
                                                    match &**arg_type_expr {
                                                        TypeExpr::Var(type_var_ident, pos) => {
                                                            assert_eq!(3, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("Int"), *type_var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                }
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &lambda_args[1] {
                                        LambdaArg(arg_ident, None, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(19, pos.column);
                                            assert_eq!(String::from("y"), *arg_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(24, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(24, pos.column);
                                                    assert_eq!(String::from("op_add"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(22, pos.column);
                                                    assert_eq!(String::from("x"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(26, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
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
            assert_eq!(String::from("d"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Fun"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Lambda(lambda_args, ret_type_expr, expr, None, pos) => {
                                    assert_eq!(4, pos.line);
                                    assert_eq!(10, pos.column);
                                    assert_eq!(2, lambda_args.len());
                                    match &lambda_args[0] {
                                        LambdaArg(arg_ident, None, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(11, pos.column);
                                            assert_eq!(String::from("x"), *arg_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    match &lambda_args[1] {
                                        LambdaArg(arg_ident, arg_type_expr, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(14, pos.column);
                                            assert_eq!(String::from("y"), *arg_ident);
                                            match arg_type_expr {
                                                Some(arg_type_expr) => {
                                                    match &**arg_type_expr {
                                                        TypeExpr::Var(type_var_ident, pos) => {
                                                            assert_eq!(4, pos.line);
                                                            assert_eq!(17, pos.column);
                                                            assert_eq!(String::from("Int"), *type_var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                }
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match ret_type_expr {
                                        Some(ret_type_expr) => {
                                            match &**ret_type_expr {
                                                TypeExpr::Var(type_var_ident, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(25, pos.column);
                                                    assert_eq!(String::from("Int"), *type_var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        }
                                        _ => assert!(false),
                                    }
                                    match &**expr {
                                        Expr::App(expr, exprs, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(31, pos.column);
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(31, pos.column);
                                                    assert_eq!(String::from("op_sub"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(29, pos.column);
                                                    assert_eq!(String::from("x"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(33, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
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
            assert_eq!(String::from("e"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Var(var_ident, None, pos) => {
                                    assert_eq!(5, pos.line);
                                    assert_eq!(10, pos.column);
                                    assert_eq!(String::from("X"), *var_ident);
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
            assert_eq!(String::from("f"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Var(var_ident, None, pos) => {
                                    assert_eq!(6, pos.line);
                                    assert_eq!(10, pos.column);
                                    assert_eq!(String::from("x"), *var_ident);
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
    match &*tree.defs()[6] {
        Def::Var(ident, var, pos) => {
            assert_eq!(7, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("g"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(7, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("T"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::NamedFieldConApp(con_ident, expr_named_field_pairs, None, pos) => {
                                    assert_eq!(7, pos.line);
                                    assert_eq!(8, pos.column);
                                    assert_eq!(String::from("C"), *con_ident);
                                    assert_eq!(2, expr_named_field_pairs.len());
                                    match &expr_named_field_pairs[0] {
                                        NamedFieldPair(field_ident, expr, pos) => {
                                            assert_eq!(7, pos.line);
                                            assert_eq!(12, pos.column);
                                            assert_eq!(String::from("x"), *field_ident);
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &expr_named_field_pairs[1] {
                                        NamedFieldPair(field_ident, expr, pos) => {
                                            assert_eq!(7, pos.line);
                                            assert_eq!(18, pos.column);
                                            assert_eq!(String::from("y"), *field_ident);
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
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
    match &*tree.defs()[7] {
        Def::Var(ident, var, pos) => {
            assert_eq!(8, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("h"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Tuple(type_exprs, pos) => {
                            assert_eq!(8, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(true, type_exprs.is_empty());
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::PrintfApp(exprs, None, pos) => {
                                    assert_eq!(8, pos.line);
                                    assert_eq!(9, pos.column);
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(8, pos.line);
                                            assert_eq!(16, pos.column);
                                            match &**literal {
                                                Literal::String(bs) => assert_eq!("%d\n".as_bytes(), *bs),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::Literal(literal, None, pos) => {
                                            assert_eq!(8, pos.line);
                                            assert_eq!(24, pos.column);
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
}

#[test]
fn test_parser_parse_parses_pattern1()
{
    let s = "
a: Int =
    x match {
        C() | D() => 1;
    };
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Match(expr, cases, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(5, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(5, pos.column);
                                            assert_eq!(String::from("x"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, cases.len());
                                    match &cases[0] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Alt(patterns, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(2, patterns.len());
                                                    match &*patterns[0] {
                                                        Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                            assert_eq!(3, pos.line);
                                                            assert_eq!(9, pos.column);
                                                            assert_eq!(String::from("C"), *con_ident);
                                                            assert_eq!(0, patterns.len());
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*patterns[1] {
                                                        Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                            assert_eq!(3, pos.line);
                                                            assert_eq!(15, pos.column);
                                                            assert_eq!(String::from("D"), *con_ident);
                                                            assert_eq!(0, patterns.len());
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(22, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
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
fn test_parser_parse_parses_patterns2()
{
    let s = "
a: Int =
    x match {
        y @ (C() | D()) => 1;
        1 => 2;
        2 as Float => 3;
        X => 4;
        C() => 5;
        E(1, 2) => 6;
        C {} => 7;
        E { x: 1, y: 2, } => 8;
        y => 9;
        private y => 10;
        local y => 11;
        global y => 12;
        constant y => 13;
        y @ C() => 14;
        private y @ C() => 15;
        local y @ C() => 16;
        global y @ C() => 17;
        constant y @ C() => 18;
        _ => 19;
    };
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Match(expr, cases, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(5, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(5, pos.column);
                                            assert_eq!(String::from("x"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(19, cases.len());
                                    match &cases[0] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::At(VarModifier::None, var_ident, pattern, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                    match &**pattern {
                                                        Pattern::Alt(patterns, None, pos) => {
                                                            assert_eq!(3, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(2, patterns.len());
                                                            match &*patterns[0] {
                                                                Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                                    assert_eq!(3, pos.line);
                                                                    assert_eq!(14, pos.column);
                                                                    assert_eq!(String::from("C"), *con_ident);
                                                                    assert_eq!(0, patterns.len());
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*patterns[1] {
                                                                Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                                    assert_eq!(3, pos.line);
                                                                    assert_eq!(20, pos.column);
                                                                    assert_eq!(String::from("D"), *con_ident);
                                                                    assert_eq!(0, patterns.len());
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(28, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[1] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[2] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::As(literal, type_expr, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                        _ => assert!(false),
                                                    }
                                                    match &**type_expr {
                                                        TypeExpr::Var(type_var_ident, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(14, pos.column);
                                                            assert_eq!(String::from("Float"), *type_var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(23, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[3] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Const(con_ident, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("X"), *con_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(4, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[4] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("C"), *con_ident);
                                                    assert_eq!(true, patterns.is_empty());
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(5, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[5] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("E"), *con_ident);
                                                    assert_eq!(2, patterns.len());
                                                    match &*patterns[0] {
                                                        Pattern::Literal(literal, None, pos) => {
                                                            assert_eq!(8, pos.line);
                                                            assert_eq!(11, pos.column);
                                                            match &**literal {
                                                                Literal::Int(n) => assert_eq!(1, *n),
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*patterns[1] {
                                                        Pattern::Literal(literal, None, pos) => {
                                                            assert_eq!(8, pos.line);
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
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(6, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[6] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::NamedFieldCon(con_ident, pattern_named_field_pairs, None, pos) => {
                                                    assert_eq!(9, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("C"), *con_ident);
                                                    assert_eq!(true, pattern_named_field_pairs.is_empty());
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(9, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(7, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[7] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::NamedFieldCon(con_ident, pattern_named_field_pairs, None, pos) => {
                                                    assert_eq!(10, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("E"), *con_ident);
                                                    assert_eq!(2, pattern_named_field_pairs.len());
                                                    match &pattern_named_field_pairs[0] {
                                                        NamedFieldPair(field_ident, pattern, pos) => {
                                                            assert_eq!(10, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("x"), *field_ident);
                                                            match &**pattern {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(10, pos.line);
                                                                    assert_eq!(16, pos.column);
                                                                    match &**literal {
                                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                    }
                                                    match &pattern_named_field_pairs[1] {
                                                        NamedFieldPair(field_ident, pattern, pos) => {
                                                            assert_eq!(10, pos.line);
                                                            assert_eq!(19, pos.column);
                                                            assert_eq!(String::from("y"), *field_ident);
                                                            match &**pattern {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(10, pos.line);
                                                                    assert_eq!(22, pos.column);
                                                                    match &**literal {
                                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(10, pos.line);
                                                    assert_eq!(30, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(8, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[8] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(VarModifier::None, var_ident, None, pos) => {
                                                    assert_eq!(11, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(11, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(9, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[9] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(VarModifier::Private, var_ident, None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(22, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(10, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[10] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(VarModifier::Local, var_ident, None, pos) => {
                                                    assert_eq!(13, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(13, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(11, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[11] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(VarModifier::Global, var_ident, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(12, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[12] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(VarModifier::Constant, var_ident, None, pos) => {
                                                    assert_eq!(15, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(15, pos.line);
                                                    assert_eq!(23, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(13, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[13] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::At(VarModifier::None, var_ident, pattern, None, pos) => {
                                                    assert_eq!(16, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                    match &**pattern {
                                                        Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                            assert_eq!(16, pos.line);
                                                            assert_eq!(13, pos.column);
                                                            assert_eq!(String::from("C"), *con_ident);
                                                            assert_eq!(true, patterns.is_empty());
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(16, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(14, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[14] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::At(VarModifier::Private, var_ident, pattern, None, pos) => {
                                                    assert_eq!(17, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                    match &**pattern {
                                                        Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                            assert_eq!(17, pos.line);
                                                            assert_eq!(21, pos.column);
                                                            assert_eq!(String::from("C"), *con_ident);
                                                            assert_eq!(true, patterns.is_empty());
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(17, pos.line);
                                                    assert_eq!(28, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(15, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[15] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::At(VarModifier::Local, var_ident, pattern, None, pos) => {
                                                    assert_eq!(18, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                    match &**pattern {
                                                        Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                            assert_eq!(18, pos.line);
                                                            assert_eq!(19, pos.column);
                                                            assert_eq!(String::from("C"), *con_ident);
                                                            assert_eq!(true, patterns.is_empty());
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(18, pos.line);
                                                    assert_eq!(26, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(16, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[16] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::At(VarModifier::Global, var_ident, pattern, None, pos) => {
                                                    assert_eq!(19, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                    match &**pattern {
                                                        Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                            assert_eq!(19, pos.line);
                                                            assert_eq!(20, pos.column);
                                                            assert_eq!(String::from("C"), *con_ident);
                                                            assert_eq!(true, patterns.is_empty());
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(19, pos.line);
                                                    assert_eq!(27, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(17, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[17] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::At(VarModifier::Constant, var_ident, pattern, None, pos) => {
                                                    assert_eq!(20, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    assert_eq!(String::from("y"), *var_ident);
                                                    match &**pattern {
                                                        Pattern::UnnamedFieldCon(con_ident, patterns, None, pos) => {
                                                            assert_eq!(20, pos.line);
                                                            assert_eq!(22, pos.column);
                                                            assert_eq!(String::from("C"), *con_ident);
                                                            assert_eq!(true, patterns.is_empty());
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(20, pos.line);
                                                    assert_eq!(29, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(18, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[18] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Wildcard(None, pos) => {
                                                    assert_eq!(21, pos.line);
                                                    assert_eq!(9, pos.column);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(21, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(19, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
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
fn test_parser_parse_parses_expression_literals()
{
    let s = "
a: Bool = false;
b: Bool = true;
c: Char = 'a';
d: Int = 1i;
e: Long = 2I;
f: Uint = 3u;
g: Ulong = 4U;
h: Float = 1.5f;
i: Double = 2.5F;
j: S = \"abc\";
k: U = ();
l: T = (1, 2);
m: A1 = [];
n: A2 = [1, 2, 3];
o: A3 = [1; 10];
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(15, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Var(ident, var, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(11, pos.column);
                                    match &**literal {
                                        Literal::Bool(b) => assert_eq!(false, *b),
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
            assert_eq!(String::from("b"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(2, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Bool"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(11, pos.column);
                                    match &**literal {
                                        Literal::Bool(b) => assert_eq!(true, *b),
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
            assert_eq!(String::from("c"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Char"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(3, pos.line);
                                    assert_eq!(11, pos.column);
                                    match &**literal {
                                        Literal::Char(n) => assert_eq!(b'a' as i8, *n),
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
            assert_eq!(String::from("d"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(4, pos.line);
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
    match &*tree.defs()[4] {
        Def::Var(ident, var, pos) => {
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("e"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Long"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(5, pos.line);
                                    assert_eq!(11, pos.column);
                                    match &**literal {
                                        Literal::Long(n) => assert_eq!(2, *n),
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
            assert_eq!(String::from("f"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Uint"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(6, pos.line);
                                    assert_eq!(11, pos.column);
                                    match &**literal {
                                        Literal::Uint(n) => assert_eq!(3, *n),
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
    match &*tree.defs()[6] {
        Def::Var(ident, var, pos) => {
            assert_eq!(7, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("g"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(7, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Ulong"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(7, pos.line);
                                    assert_eq!(12, pos.column);
                                    match &**literal {
                                        Literal::Ulong(n) => assert_eq!(4, *n),
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
    match &*tree.defs()[7] {
        Def::Var(ident, var, pos) => {
            assert_eq!(8, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("h"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(8, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Float"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(8, pos.line);
                                    assert_eq!(12, pos.column);
                                    match &**literal {
                                        Literal::Float(n) => assert_eq!(1.5, *n),
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
    match &*tree.defs()[8] {
        Def::Var(ident, var, pos) => {
            assert_eq!(9, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("i"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(9, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Double"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(9, pos.line);
                                    assert_eq!(13, pos.column);
                                    match &**literal {
                                        Literal::Double(n) => assert_eq!(2.5, *n),
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
    match &*tree.defs()[9] {
        Def::Var(ident, var, pos) => {
            assert_eq!(10, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("j"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(10, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("S"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(10, pos.line);
                                    assert_eq!(8, pos.column);
                                    match &**literal {
                                        Literal::String(bs) => assert_eq!("abc".as_bytes(), *bs),
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
    match &*tree.defs()[10] {
        Def::Var(ident, var, pos) => {
            assert_eq!(11, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("k"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(11, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("U"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(11, pos.line);
                                    assert_eq!(8, pos.column);
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
    match &*tree.defs()[11] {
        Def::Var(ident, var, pos) => {
            assert_eq!(12, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("l"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(12, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("T"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(12, pos.line);
                                    assert_eq!(8, pos.column);
                                    match &**literal {
                                        Literal::Tuple(exprs) => {
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(12, pos.column);
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[12] {
        Def::Var(ident, var, pos) => {
            assert_eq!(13, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("m"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(13, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("A1"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(13, pos.line);
                                    assert_eq!(9, pos.column);
                                    match &**literal {
                                        Literal::Array(exprs) => assert_eq!(true, exprs.is_empty()),
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
    match &*tree.defs()[13] {
        Def::Var(ident, var, pos) => {
            assert_eq!(14, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("n"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(14, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("A2"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(14, pos.line);
                                    assert_eq!(9, pos.column);
                                    match &**literal {
                                        Literal::Array(exprs) => {
                                            assert_eq!(3, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(13, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[2] {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(16, pos.column);
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[14] {
        Def::Var(ident, var, pos) => {
            assert_eq!(15, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("o"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(15, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("A3"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(literal, None, pos) => {
                                    assert_eq!(15, pos.line);
                                    assert_eq!(9, pos.column);
                                    match &**literal {
                                        Literal::FilledArray(expr, len) => {
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(15, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(10, *len);
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
fn test_parser_parse_parses_pattern_literals()
{
    let s = "
a: Int =
    x match {
        false => 1;
        true => 2;
        'a' => 3;
        1i => 4;
        2I => 5;
        3u => 6;
        4U => 7;
        1.5f => 8;
        2.5F => 9;
        !false => 10;
        !true => 11;
        -!'a' => 12;
        -!1i => 13;
        -!2I => 14;
        !3u => 15;
        !4U => 16;
        -1.5f => 17;
        -2.5F => 18;
        \"abc\" => 19;
        () => 20;
        (1, 2) => 21;
        [] => 22;
        [1, 2, 3] => 23;
        [1; 10] => 24;
    };
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
            assert_eq!(String::from("a"), *ident);
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(VarModifier::None, type_expr, where_tuples, expr, None, None, None, None) => {
                    match &**type_expr {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(4, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                    assert_eq!(true, where_tuples.is_empty());
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Match(expr, cases, None, pos) => {
                                    assert_eq!(2, pos.line);
                                    assert_eq!(5, pos.column);
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(2, pos.line);
                                            assert_eq!(5, pos.column);
                                            assert_eq!(String::from("x"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(24, cases.len());
                                    match &cases[0] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(false, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[1] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(true, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[2] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Char(n) => assert_eq!(b'a' as i8, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[3] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(4, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[4] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Long(n) => assert_eq!(2, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(5, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[5] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Uint(n) => assert_eq!(3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(6, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[6] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(9, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Ulong(n) => assert_eq!(4, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(9, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(7, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[7] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(10, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Float(n) => assert_eq!(1.5, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(10, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(8, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[8] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(11, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Double(n) => assert_eq!(2.5, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(11, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(9, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }


                                    match &cases[9] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(!false, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(12, pos.line);
                                                    assert_eq!(19, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(10, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[10] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(13, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Bool(b) => assert_eq!(!true, *b),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(13, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(11, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[11] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Char(n) => assert_eq!(-!(b'a' as i8), *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(14, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(12, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[12] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(15, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(-!1, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(15, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(13, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[13] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(16, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Long(n) => assert_eq!(-!2, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(16, pos.line);
                                                    assert_eq!(17, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(14, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[14] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(17, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Uint(n) => assert_eq!(!3, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(17, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(15, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[15] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(18, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Ulong(n) => assert_eq!(!4, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(18, pos.line);
                                                    assert_eq!(16, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(16, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[16] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(19, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Float(n) => assert_eq!(-1.5, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(19, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(17, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[17] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(20, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Double(n) => assert_eq!(-2.5, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(20, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(18, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[18] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(21, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::String(bs) => assert_eq!("abc".as_bytes(), *bs),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(21, pos.line);
                                                    assert_eq!(18, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(19, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[19] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(22, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Tuple(patterns) => assert_eq!(true, patterns.is_empty()),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(22, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(20, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[20] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(23, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Tuple(patterns) => {
                                                            assert_eq!(2, patterns.len());
                                                            match &*patterns[0] {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(23, pos.line);
                                                                    assert_eq!(10, pos.column);
                                                                    match &**literal {
                                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*patterns[1] {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(23, pos.line);
                                                                    assert_eq!(13, pos.column);
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
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(23, pos.line);
                                                    assert_eq!(19, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(21, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[21] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(24, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Array(patterns) => assert_eq!(true, patterns.is_empty()),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(24, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(22, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[22] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(25, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::Array(patterns) => {
                                                            assert_eq!(3, patterns.len());
                                                            match &*patterns[0] {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(25, pos.line);
                                                                    assert_eq!(10, pos.column);
                                                                    match &**literal {
                                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*patterns[1] {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(25, pos.line);
                                                                    assert_eq!(13, pos.column);
                                                                    match &**literal {
                                                                        Literal::Int(n) => assert_eq!(2, *n),
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*patterns[2] {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(25, pos.line);
                                                                    assert_eq!(16, pos.column);
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
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(25, pos.line);
                                                    assert_eq!(22, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(23, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &cases[23] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, None, pos) => {
                                                    assert_eq!(26, pos.line);
                                                    assert_eq!(9, pos.column);
                                                    match &**literal {
                                                        Literal::FilledArray(pattern, len) => {
                                                            match &**pattern {
                                                                Pattern::Literal(literal, None, pos) => {
                                                                    assert_eq!(26, pos.line);
                                                                    assert_eq!(10, pos.column);
                                                                    match &**literal {
                                                                        Literal::Int(n) => assert_eq!(1, *n),
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            assert_eq!(10, *len);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(literal, None, pos) => {
                                                    assert_eq!(26, pos.line);
                                                    assert_eq!(20, pos.column);
                                                    match &**literal {
                                                        Literal::Int(n) => assert_eq!(24, *n),
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
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
fn test_parser_parse_parses_trait_builtin_variable_definitions()
{
    let s = "
trait T
{
    builtin A;
    builtin a;
};
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
        Def::Trait(ident, trait1, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(type_args, trait_defs, None) => {
                    assert_eq!(true, type_args.is_empty());
                    assert_eq!(2, trait_defs.len());
                    match &*trait_defs[0] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("A"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Builtin(Some(trait_ident), None) => assert_eq!(String::from("T"), *trait_ident),
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[1] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("a"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Builtin(Some(trait_ident), None) => assert_eq!(String::from("T"), *trait_ident),
                                _ => assert!(false),
                            }
                        },
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_trait_variable_definitions()
{
    let s = "
trait T
{
    A: t1 where t1: T = x1;
    a: t1 where t1: T = x2;
    private b: t1 where t1: T = x3;
    local c: t1 where t1: T = x4;
    global d: t1 where t1: T = x5;
    constant e: t1 where t1: T = x6;
    f: t1 where t1: T;
};
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
        Def::Trait(ident, trait1, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(type_args, trait_defs, None) => {
                    assert_eq!(true, type_args.is_empty());
                    assert_eq!(7, trait_defs.len());
                    match &*trait_defs[0] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("A"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Var(VarModifier::None, type_expr, where_tuples, expr, Some(trait_ident), None, None, None) => {
                                    match &**type_expr {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(8, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, where_tuples.len());
                                    match &where_tuples[0] {
                                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(17, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                            assert_eq!(1, trait_names.len());
                                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                            assert_eq!(true, type_exprs.is_empty());
                                        },
                                    }
                                    match expr {
                                        Some(expr) => {
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(25, pos.column);
                                                    assert_eq!(String::from("x1"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[1] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("a"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Var(VarModifier::None, type_expr, where_tuples, expr, Some(trait_ident), None, None, None) => {
                                    match &**type_expr {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(8, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, where_tuples.len());
                                    match &where_tuples[0] {
                                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(17, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                            assert_eq!(1, trait_names.len());
                                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                            assert_eq!(true, type_exprs.is_empty());
                                        },
                                    }
                                    match expr {
                                        Some(expr) => {
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(25, pos.column);
                                                    assert_eq!(String::from("x2"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[2] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("b"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Var(VarModifier::Private, type_expr, where_tuples, expr, Some(trait_ident), None, None, None) => {
                                    match &**type_expr {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(16, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, where_tuples.len());
                                    match &where_tuples[0] {
                                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                            assert_eq!(5, pos.line);
                                            assert_eq!(25, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                            assert_eq!(1, trait_names.len());
                                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                            assert_eq!(true, type_exprs.is_empty());
                                        },
                                    }
                                    match expr {
                                        Some(expr) => {
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(33, pos.column);
                                                    assert_eq!(String::from("x3"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[3] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("c"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Var(VarModifier::Local, type_expr, where_tuples, expr, Some(trait_ident), None, None, None) => {
                                    match &**type_expr {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(6, pos.line);
                                            assert_eq!(14, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, where_tuples.len());
                                    match &where_tuples[0] {
                                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                            assert_eq!(6, pos.line);
                                            assert_eq!(23, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                            assert_eq!(1, trait_names.len());
                                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                            assert_eq!(true, type_exprs.is_empty());
                                        },
                                    }
                                    match expr {
                                        Some(expr) => {
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(31, pos.column);
                                                    assert_eq!(String::from("x4"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[4] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(7, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("d"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Var(VarModifier::Global, type_expr, where_tuples, expr, Some(trait_ident), None, None, None) => {
                                    match &**type_expr {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(7, pos.line);
                                            assert_eq!(15, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, where_tuples.len());
                                    match &where_tuples[0] {
                                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                            assert_eq!(7, pos.line);
                                            assert_eq!(24, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                            assert_eq!(1, trait_names.len());
                                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                            assert_eq!(true, type_exprs.is_empty());
                                        },
                                    }
                                    match expr {
                                        Some(expr) => {
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(32, pos.column);
                                                    assert_eq!(String::from("x5"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[5] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(8, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("e"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Var(VarModifier::Constant, type_expr, where_tuples, expr, Some(trait_ident), None, None, None) => {
                                    match &**type_expr {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(8, pos.line);
                                            assert_eq!(17, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, where_tuples.len());
                                    match &where_tuples[0] {
                                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                            assert_eq!(8, pos.line);
                                            assert_eq!(26, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                            assert_eq!(1, trait_names.len());
                                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                            assert_eq!(true, type_exprs.is_empty());
                                        },
                                    }
                                    match expr {
                                        Some(expr) => {
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(34, pos.column);
                                                    assert_eq!(String::from("x6"), *var_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[6] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(9, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("f"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Var(VarModifier::None, type_expr, where_tuples, None, Some(trait_ident), None, None, None) => {
                                    match &**type_expr {
                                        TypeExpr::Param(type_param_ident, pos) => {
                                            assert_eq!(9, pos.line);
                                            assert_eq!(8, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, where_tuples.len());
                                    match &where_tuples[0] {
                                        WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                            assert_eq!(9, pos.line);
                                            assert_eq!(17, pos.column);
                                            assert_eq!(String::from("t1"), *type_param_ident);
                                            assert_eq!(1, trait_names.len());
                                            assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                            assert_eq!(true, type_exprs.is_empty());
                                        },
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);                                    
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_trait_function_definitions()
{
    let s = "
trait T
{
    F() -> t1 where t1: T = x1;
    f() -> t1 where t1: T = x2;
    g(x: t1, y: t1) -> t1 where t1: T = f1(x, y);
    inline h(x: t1, y: t1) -> t1 where t1: T = f2(x, y);
    kernel i(x: t1) -> () where t1: T = ();
    j() -> t1 where t1: T;
};
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
        Def::Trait(ident, trait1, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("T"), *ident);
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(type_args, trait_defs, None) => {
                    assert_eq!(true, type_args.is_empty());
                    assert_eq!(6, trait_defs.len());
                    match &*trait_defs[0] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("F"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, Some(trait_ident), None) => {
                                    match &**fun {
                                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                                            assert_eq!(true, args.is_empty());
                                            match &**ret_type_expr {
                                                TypeExpr::Param(type_param_ident, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(12, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, where_tuples.len());
                                            match &where_tuples[0] {
                                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                    assert_eq!(1, trait_names.len());
                                                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                                    assert_eq!(true, type_exprs.is_empty());
                                                },
                                            }
                                            match expr {
                                                Some(expr) => {
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(3, pos.line);
                                                            assert_eq!(29, pos.column);
                                                            assert_eq!(String::from("x1"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[1] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("f"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, Some(trait_ident), None) => {
                                    match &**fun {
                                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                                            assert_eq!(true, args.is_empty());
                                            match &**ret_type_expr {
                                                TypeExpr::Param(type_param_ident, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(12, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, where_tuples.len());
                                            match &where_tuples[0] {
                                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                    assert_eq!(1, trait_names.len());
                                                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                                    assert_eq!(true, type_exprs.is_empty());
                                                },
                                            }
                                            match expr {
                                                Some(expr) => {
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(4, pos.line);
                                                            assert_eq!(29, pos.column);
                                                            assert_eq!(String::from("x2"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[2] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("g"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, Some(trait_ident), None) => {
                                    match &**fun {
                                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, expr, None, None) => {
                                            assert_eq!(2, args.len());
                                            match &args[0] {
                                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(7, pos.column);
                                                    assert_eq!(String::from("x"), *arg_ident);
                                                    match &**arg_type_expr {
                                                        TypeExpr::Param(type_param_ident, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(10, pos.column);
                                                            assert_eq!(String::from("t1"), *type_param_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &args[1] {
                                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    assert_eq!(String::from("y"), *arg_ident);
                                                    match &**arg_type_expr {
                                                        TypeExpr::Param(type_param_ident, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(17, pos.column);
                                                            assert_eq!(String::from("t1"), *type_param_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**ret_type_expr {
                                                TypeExpr::Param(type_param_ident, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(24, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, where_tuples.len());
                                            match &where_tuples[0] {
                                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(33, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                    assert_eq!(1, trait_names.len());
                                                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                                    assert_eq!(true, type_exprs.is_empty());
                                                },
                                            }
                                            match expr {
                                                Some(expr) => {
                                                    match &**expr {
                                                        Expr::App(expr, exprs, None, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(41, pos.column);
                                                            match &**expr {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(5, pos.line);
                                                                    assert_eq!(41, pos.column);
                                                                    assert_eq!(String::from("f1"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            assert_eq!(2, exprs.len());
                                                            match &*exprs[0] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(5, pos.line);
                                                                    assert_eq!(44, pos.column);
                                                                    assert_eq!(String::from("x"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*exprs[1] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(5, pos.line);
                                                                    assert_eq!(47, pos.column);
                                                                    assert_eq!(String::from("y"), *var_ident);
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
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[3] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(6, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("h"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, Some(trait_ident), None) => {
                                    match &**fun {
                                        Fun::Fun(FunModifier::Inline, args, ret_type_expr, where_tuples, expr, None, None) => {
                                            assert_eq!(2, args.len());
                                            match &args[0] {
                                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    assert_eq!(String::from("x"), *arg_ident);
                                                    match &**arg_type_expr {
                                                        TypeExpr::Param(type_param_ident, pos) => {
                                                            assert_eq!(6, pos.line);
                                                            assert_eq!(17, pos.column);
                                                            assert_eq!(String::from("t1"), *type_param_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &args[1] {
                                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    assert_eq!(String::from("y"), *arg_ident);
                                                    match &**arg_type_expr {
                                                        TypeExpr::Param(type_param_ident, pos) => {
                                                            assert_eq!(6, pos.line);
                                                            assert_eq!(24, pos.column);
                                                            assert_eq!(String::from("t1"), *type_param_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**ret_type_expr {
                                                TypeExpr::Param(type_param_ident, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(31, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, where_tuples.len());
                                            match &where_tuples[0] {
                                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                                    assert_eq!(6, pos.line);
                                                    assert_eq!(40, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                    assert_eq!(1, trait_names.len());
                                                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                                    assert_eq!(true, type_exprs.is_empty());
                                                },
                                            }
                                            match expr {
                                                Some(expr) => {
                                                    match &**expr {
                                                        Expr::App(expr, exprs, None, pos) => {
                                                            assert_eq!(6, pos.line);
                                                            assert_eq!(48, pos.column);
                                                            match &**expr {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(6, pos.line);
                                                                    assert_eq!(48, pos.column);
                                                                    assert_eq!(String::from("f2"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            assert_eq!(2, exprs.len());
                                                            match &*exprs[0] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(6, pos.line);
                                                                    assert_eq!(51, pos.column);
                                                                    assert_eq!(String::from("x"), *var_ident);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*exprs[1] {
                                                                Expr::Var(var_ident, None, pos) => {
                                                                    assert_eq!(6, pos.line);
                                                                    assert_eq!(54, pos.column);
                                                                    assert_eq!(String::from("y"), *var_ident);
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
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[4] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(7, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("i"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, Some(trait_ident), None) => {
                                    match &**fun {
                                        Fun::Fun(FunModifier::Kernel, args, ret_type_expr, where_tuples, expr, None, None) => {
                                            assert_eq!(1, args.len());
                                            match &args[0] {
                                                Arg(arg_ident, arg_type_expr, None, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(14, pos.column);
                                                    assert_eq!(String::from("x"), *arg_ident);
                                                    match &**arg_type_expr {
                                                        TypeExpr::Param(type_param_ident, pos) => {
                                                            assert_eq!(7, pos.line);
                                                            assert_eq!(17, pos.column);
                                                            assert_eq!(String::from("t1"), *type_param_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**ret_type_expr {
                                                TypeExpr::Tuple(type_exprs, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(24, pos.column);
                                                    assert_eq!(true, type_exprs.is_empty());
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, where_tuples.len());
                                            match &where_tuples[0] {
                                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                                    assert_eq!(7, pos.line);
                                                    assert_eq!(33, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                    assert_eq!(1, trait_names.len());
                                                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                                    assert_eq!(true, type_exprs.is_empty());
                                                },
                                            }
                                            match expr {
                                                Some(expr) => {
                                                    match &**expr {
                                                        Expr::Literal(literal, None, pos) => {
                                                            assert_eq!(7, pos.line);
                                                            assert_eq!(41, pos.column);
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
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[5] {
                        TraitDef(ident, var, pos) => {
                            assert_eq!(8, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("j"), *ident);
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, Some(trait_ident), None) => {
                                    match &**fun {
                                        Fun::Fun(FunModifier::None, args, ret_type_expr, where_tuples, None, None, None) => {
                                            assert_eq!(true, args.is_empty());
                                            match &**ret_type_expr {
                                                TypeExpr::Param(type_param_ident, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(12, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(1, where_tuples.len());
                                            match &where_tuples[0] {
                                                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                                                    assert_eq!(8, pos.line);
                                                    assert_eq!(21, pos.column);
                                                    assert_eq!(String::from("t1"), *type_param_ident);
                                                    assert_eq!(1, trait_names.len());
                                                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                                                    assert_eq!(true, type_exprs.is_empty());
                                                },
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(String::from("T"), *trait_ident);
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_type_names()
{
    let s = "
impl T for () {};
impl U for (_, _) {};
impl V for () -> _ {};
impl W for (_, _) -> _ {};
impl X for [_; 10] {};
impl Y for [_; _] {};
impl Z for A {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(7, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Impl(impl1, pos) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("T"), *trait_ident);
                    assert_eq!(TypeName::Tuple(0), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Impl(impl1, pos) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("U"), *trait_ident);
                    assert_eq!(TypeName::Tuple(2), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Impl(impl1, pos) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("V"), *trait_ident);
                    assert_eq!(TypeName::Fun(0), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Impl(impl1, pos) => {
            assert_eq!(4, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("W"), *trait_ident);
                    assert_eq!(TypeName::Fun(2), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[4] {
        Def::Impl(impl1, pos) => {
            assert_eq!(5, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("X"), *trait_ident);
                    assert_eq!(TypeName::Array(Some(10)), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[5] {
        Def::Impl(impl1, pos) => {
            assert_eq!(6, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("Y"), *trait_ident);
                    assert_eq!(TypeName::Array(None), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[6] {
        Def::Impl(impl1, pos) => {
            assert_eq!(7, pos.line);
            assert_eq!(1, pos.column);
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(trait_ident, type_name, impl_defs, None) => {
                    assert_eq!(String::from("Z"), *trait_ident);
                    assert_eq!(TypeName::Name(String::from("A")), *type_name);
                    assert_eq!(true, impl_defs.is_empty());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_implementation_builtin_variable_definitions()
{
    let s = "
impl T for ()
{
    builtin A;
    builtin a;
};
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
                    assert_eq!(TypeName::Tuple(0), *type_name);
                    assert_eq!(2, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(ident, impl_var, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("A"), *ident);
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*impl_defs[1] {
                        ImplDef(ident, impl_var, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("a"), *ident);
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_implementation_variable_definitions()
{
    let s = "
impl T for ()
{
    A = x1;
    a = x2;
};
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
                    assert_eq!(TypeName::Tuple(0), *type_name);
                    assert_eq!(2, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(ident, impl_var, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("A"), *ident);
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Var(expr, None, None, None) => {
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(3, pos.line);
                                            assert_eq!(9, pos.column);
                                            assert_eq!(String::from("x1"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*impl_defs[1] {
                        ImplDef(ident, impl_var, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("a"), *ident);
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Var(expr, None, None, None) => {
                                    match &**expr {
                                        Expr::Var(var_ident, None, pos) => {
                                            assert_eq!(4, pos.line);
                                            assert_eq!(9, pos.column);
                                            assert_eq!(String::from("x2"), *var_ident);
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_parses_implementation_function_definitions()
{
    let s = "
impl T for ()
{
    F() = x1;
    f() = x2;
    g(x, y) = f1(x, y);
};
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
                    assert_eq!(TypeName::Tuple(0), *type_name);
                    assert_eq!(3, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(ident, impl_var, pos) => {
                            assert_eq!(3, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("F"), *ident);
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Fun(fun, None) => {
                                    match &**fun {
                                        ImplFun(impl_args, expr, None, None) => {
                                            assert_eq!(true, impl_args.is_empty());
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(3, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    assert_eq!(String::from("x1"), *var_ident);
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
                    }
                    match &*impl_defs[1] {
                        ImplDef(ident, impl_var, pos) => {
                            assert_eq!(4, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("f"), *ident);
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Fun(fun, None) => {
                                    match &**fun {
                                        ImplFun(impl_args, expr, None, None) => {
                                            assert_eq!(true, impl_args.is_empty());
                                            match &**expr {
                                                Expr::Var(var_ident, None, pos) => {
                                                    assert_eq!(4, pos.line);
                                                    assert_eq!(11, pos.column);
                                                    assert_eq!(String::from("x2"), *var_ident);
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
                    }
                    match &*impl_defs[2] {
                        ImplDef(ident, impl_var, pos) => {
                            assert_eq!(5, pos.line);
                            assert_eq!(5, pos.column);
                            assert_eq!(String::from("g"), *ident);
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Fun(fun, None) => {
                                    match &**fun {
                                        ImplFun(impl_args, expr, None, None) => {
                                            assert_eq!(2, impl_args.len());
                                            match &impl_args[0] {
                                                ImplArg(impl_arg_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(7, pos.column);
                                                    assert_eq!(String::from("x"), *impl_arg_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &impl_args[1] {
                                                ImplArg(impl_arg_ident, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(10, pos.column);
                                                    assert_eq!(String::from("y"), *impl_arg_ident);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, None, pos) => {
                                                    assert_eq!(5, pos.line);
                                                    assert_eq!(15, pos.column);
                                                    match &**expr {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(15, pos.column);
                                                            assert_eq!(String::from("f1"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(18, pos.column);
                                                            assert_eq!(String::from("x"), *var_ident);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(var_ident, None, pos) => {
                                                            assert_eq!(5, pos.line);
                                                            assert_eq!(21, pos.column);
                                                            assert_eq!(String::from("y"), *var_ident);
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
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_data_definition()
{
    let s = "
data T +
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(8, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_type_definition()
{
    let s = "
type T +
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(8, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_variable_definition()
{
    let s = "
x +
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(3, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_variable_definition_without_expression()
{
    let s = "
x: Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(7, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_function_definition()
{
    let s = "
f(x: Int) +
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(11, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_function_definition_without_body()
{
    let s = "
f(x: Int) -> Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(17, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_trait_definition()
{
    let s = "
trait T +
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_token_for_implementation_definition()
{
    let s = "
impl T +
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(8, pos.column);
            assert_eq!(String::from("unexpected token"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_data_definition()
{
    let s = "
data T<t,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_type_definition()
{
    let s = "
type T<t,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_function_definition()
{
    let s = "
f(x: Int,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_trait_definition()
{
    let s = "
trait T<t,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_trait_definition_and_brace()
{
    let s = "
trait T
{
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_implementation_definition_and_brace()
{
    let s = "
impl T for U
{
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(3, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_type_expression()
{
    let s = "
type T = U<Int,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_type_expression_and_parenthesis()
{
    let s = "
type T = (Int,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_type_expression_and_bracket()
{
    let s = "
type T = [Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_constructor()
{
    let s = "
data T = C(Int,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_constructor_and_brace()
{
    let s = "
data T = C { x: Int,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_expression()
{
    let s = "
a: Int = f(1, 
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_expression_and_brace()
{
    let s = "
a: T = C { x: 1,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unexpected_end_of_file_for_expression_and_bar_bracket()
{
    let s = "
a: Fun = |x: Int,
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(2, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("unexpected end of file"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_angle_bracket_for_type_definition()
{
    let s = "
type T<t;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unclosed angle bracket"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_angle_bracket_for_data_definition()
{
    let s = "
data T<t;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unclosed angle bracket"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_parenthesis_for_function_definition()
{
    let s = "
f(x: Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("unclosed parenthesis"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_angle_bracet_for_trait_definition()
{
    let s = "
trait T<t;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(10, pos.column);
            assert_eq!(String::from("unclosed angle bracket"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_parenthesis_for_type_expression()
{
    let s = "
type T = (Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(14, pos.column);
            assert_eq!(String::from("unclosed parenthesis"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_bracket_for_type_expression()
{
    let s = "
type T = [Int;10;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(17, pos.column);
            assert_eq!(String::from("unclosed bracket"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_angle_bracket_for_type_expression()
{
    let s = "
type T = U<Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(15, pos.column);
            assert_eq!(String::from("unclosed angle bracket"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_parenthesis_for_constructor()
{
    let s = "
data T = C(Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(15, pos.column);
            assert_eq!(String::from("unclosed parenthesis"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_brace_for_constructor()
{
    let s = "
data T = C { x: Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(20, pos.column);
            assert_eq!(String::from("unclosed brace"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_parethesis_for_expression()
{
    let s = "
a: Int = f(1; 
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("unclosed parenthesis"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_brace_for_expression()
{
    let s = "
a: T = C { x: 1; 
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(16, pos.column);
            assert_eq!(String::from("unclosed brace"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_unclosed_bar_brace_for_expression()
{
    let s = "
a: Fun = |x: Int; 
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(17, pos.column);
            assert_eq!(String::from("unclosed bar bracket"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_must_be_one_element_for_filled_array_for_expression()
{
    let s = "
a: A = [1, 2; 10];
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(13, pos.column);
            assert_eq!(String::from("must be one element for filled array"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_already_used_variable_modifier()
{
    let s = "
private local
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(9, pos.column);
            assert_eq!(String::from("already used variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_already_used_function_modifier()
{
    let s = "
kernel inline
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(8, pos.column);
            assert_eq!(String::from("already used function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_built_in_type_must_not_have_variable_modifier()
{
    let s = "
private builtin type T;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("built-in type mustn't have variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_built_in_type_must_not_have_function_modifier()
{
    let s = "
kernel builtin type T;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("built-in type mustn't have function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_type_must_not_have_variable_modifier()
{
    let s = "
private data T;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("type mustn't have variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_type_must_not_have_function_modifier()
{
    let s = "
kernel data T;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("type mustn't have function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_type_synonym_must_not_have_variable_modifier()
{
    let s = "
private type T = Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("type synonym mustn't have variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_type_synonym_must_not_have_function_modifier()
{
    let s = "
kernel type T = Int;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("type synonym mustn't have function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_variable_must_not_have_function_modifier()
{
    let s = "
kernel a: Int = 1;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("variable mustn't have function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_function_must_not_have_variable_modifier()
{
    let s = "
private f() -> Int = 1;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("function mustn't have variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_trait_must_not_have_variable_modifier()
{
    let s = "
private trait T {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("trait mustn't have variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_trait_must_not_have_function_modifier()
{
    let s = "
kernel trait T {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("trait mustn't have function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_built_in_implementation_must_not_have_variable_modifier()
{
    let s = "
private builtin impl T for U;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("built-in implementation mustn't have variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_built_in_implementation_must_not_have_function_modifier()
{
    let s = "
kernel builtin impl T for U;
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("built-in implementation mustn't have function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_implementation_must_not_have_variable_modifier()
{
    let s = "
private impl T for U {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("implementation mustn't have variable modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_complains_on_implementation_must_not_have_function_modifier()
{
    let s = "
kernel impl T for U {};
";
    let s2 = &s[1..];
    let mut cursor = Cursor::new(s2.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    let mut tree = Tree::new();
    match parser.parse(&mut tree) {
        Err(FrontendError::Message(pos, msg)) => {
            assert_eq!(1, pos.line);
            assert_eq!(1, pos.column);
            assert_eq!(String::from("implementation mustn't have function modifier"), msg);
        },
        _ => assert!(false),
    }
}

#[test]
fn test_parser_parse_type_args_parses_type_arguments()
{
    let s = "t1, t2, t3";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    match parser.parse_type_args() {
        Ok(type_args) => {
            assert_eq!(3, type_args.len());
            match &type_args[0] {
                TypeArg(type_arg_ident, pos) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("t1"), *type_arg_ident);
                },
            }
            match &type_args[1] {
                TypeArg(type_arg_ident, pos) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("t2"), *type_arg_ident);
                },
            }
            match &type_args[2] {
                TypeArg(type_arg_ident, pos) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("t3"), *type_arg_ident);
                },
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_type_parses_type_expression()
{
    let s = "T<Int>";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    match parser.parse_type() {
        Ok(type_expr) => {
            match &type_expr {
                TypeExpr::App(type_var_ident, type_exprs, pos) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("T"), *type_var_ident);
                    assert_eq!(1, type_exprs.len());
                    match &*type_exprs[0] {
                        TypeExpr::Var(type_var_ident, pos) => {
                            assert_eq!(1, pos.line);
                            assert_eq!(3, pos.column);
                            assert_eq!(String::from("Int"), *type_var_ident);
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_parser_parse_where_parses_where_tuples()
{
    let s = "t1: T + U, t2: V";
    let mut cursor = Cursor::new(s.as_bytes());
    let mut parser = Parser::new(Lexer::new(String::from("test.vscfl"), &mut cursor));
    match parser.parse_where() {
        Ok(where_tuples) => {
            assert_eq!(2, where_tuples.len());
            match &where_tuples[0] {
                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("t1"), *type_param_ident);
                    assert_eq!(2, trait_names.len());
                    assert_eq!(TraitName::Name(String::from("T")), trait_names[0]);
                    assert_eq!(TraitName::Name(String::from("U")), trait_names[1]);
                    assert_eq!(true, type_exprs.is_empty());
                },
            }
            match &where_tuples[1] {
                WhereTuple(type_param_ident, trait_names, type_exprs, pos) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(12, pos.column);
                    assert_eq!(String::from("t2"), *type_param_ident);
                    assert_eq!(1, trait_names.len());
                    assert_eq!(TraitName::Name(String::from("V")), trait_names[0]);
                    assert_eq!(true, type_exprs.is_empty());
                },
            }
        },
        Err(_) => assert!(false),
    }
}
