//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::*;
use std::rc::*;
use crate::frontend::lexer::*;
use crate::frontend::parser::*;
use super::*;

#[test]
fn test_namer_check_idents_adds_type_variable_and_variable()
{
    let s = "
builtin type Int;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(2, tree.defs().len());
    assert_eq!(1, tree.type_vars().len());
    assert_eq!(1, tree.vars().len());
    assert_eq!(true, tree.traits().is_empty());
    match &*tree.defs()[0] {
        Def::Type(_, type_var, _) => {
            match tree.type_var(&String::from("Int")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Var(_, var, _) => {
            match tree.var(&String::from("x")) {
                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_adds_type_variables()
{
    let s = "
builtin type Int;
builtin type Float;
data T<t> = C() | D(t, Int);
type U = T<Float>;
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
    assert_eq!(4, tree.defs().len());
    assert_eq!(4, tree.type_vars().len());
    assert_eq!(2, tree.vars().len());
    assert_eq!(true, tree.traits().is_empty());
    match &*tree.defs()[0] {
        Def::Type(_, type_var, _) => {
            match tree.type_var(&String::from("Int")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Type(_, type_var, _) => {
            match tree.type_var(&String::from("Float")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Type(_, type_var, _) => {
            match tree.type_var(&String::from("T")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, None) => {
                    assert_eq!(2, cons.len());
                    match tree.var(&String::from("C")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, None, None) => {
                                    match &**fun {
                                        Fun::Con(con2) => assert!(Rc::ptr_eq(&cons[0], con2)),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match tree.var(&String::from("D")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(fun, None, None) => {
                                    match &**fun {
                                        Fun::Con(con2) => assert!(Rc::ptr_eq(&cons[1], con2)),
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Type(_, type_var, _) => {
            match tree.type_var(&String::from("U")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_adds_variables()
{
    let s = "
builtin type Int;
builtin type Float;
builtin op_add;
x: Int = 1;
f(x: Float, y: Float) -> Float = x + y;
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
    assert_eq!(5, tree.defs().len());
    assert_eq!(2, tree.type_vars().len());
    assert_eq!(3, tree.vars().len());
    assert_eq!(true, tree.traits().is_empty());
    match &*tree.defs()[0] {
        Def::Type(_, type_var, _) => {
            match tree.type_var(&String::from("Int")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Type(_, type_var, _) => {
            match tree.type_var(&String::from("Float")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Var(_, var, _) => {
            match tree.var(&String::from("op_add")) {
                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Var(_, var, _) => {
            match tree.var(&String::from("x")) {
                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[4] {
        Def::Var(_, var, _) => {
            match tree.var(&String::from("f")) {
                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                None => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
