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

#[test]
fn test_namer_check_idents_adds_traits()
{
    let s = "
builtin type Int;

trait T
{
    x: t where t: T;
    f(x: t) -> t where t: T;
    g(x: t) -> Int where t: T = 1;
};

trait U
{
   y: t where t: U;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(3, tree.defs().len());
    assert_eq!(1, tree.type_vars().len());
    assert_eq!(4, tree.vars().len());
    assert_eq!(2, tree.traits().len());
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
        Def::Trait(_, trait1, _) => {
            match tree.trait1(&String::from("T")) {
                Some(trait2) => assert!(Rc::ptr_eq(trait1, trait2)),
                None => assert!(false),
            }
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(_, trait_defs, Some(trait_vars)) => {
                    assert_eq!(3, trait_defs.len());
                    assert_eq!(3, trait_vars.vars().len());
                    match &*trait_defs[0] {
                        TraitDef(_, var, _) => {
                            match trait_vars.var(&String::from("x")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                            match tree.var(&String::from("x")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[1] {
                        TraitDef(_, var, _) => {
                            match trait_vars.var(&String::from("f")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                            match tree.var(&String::from("f")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[2] {
                        TraitDef(_, var, _) => {
                            match trait_vars.var(&String::from("g")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                            match tree.var(&String::from("g")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
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
    match &*tree.defs()[2] {
        Def::Trait(_, trait1, _) => {
            match tree.trait1(&String::from("U")) {
                Some(trait2) => assert!(Rc::ptr_eq(trait1, trait2)),
                None => assert!(false),
            }
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(_, trait_defs, Some(trait_vars)) => {
                    assert_eq!(1, trait_defs.len());
                    assert_eq!(1, trait_vars.vars().len());
                    match &*trait_defs[0] {
                        TraitDef(_, var, _) => {
                            match trait_vars.var(&String::from("y")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                            match tree.var(&String::from("y")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
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
fn test_namer_check_idents_adds_implementations()
{
    let s = "
builtin type Int;
builtin type Float;

data U = C();

trait T
{
    x: t where t: T;
    f(x: t) -> t where t: T;
    g(x: t) -> Int where t: T = 1;
};

builtin impl T for Float;

impl T for Int
{
    x = 1;
    f(x) = x;
};

impl T for U
{
    builtin x;
    f(x) = C();
    g(x) = 2;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(7, tree.defs().len());
    assert_eq!(3, tree.type_vars().len());
    assert_eq!(4, tree.vars().len());
    assert_eq!(1, tree.traits().len());
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
            match tree.type_var(&String::from("U")) {
                Some(type_var2) => assert!(Rc::ptr_eq(type_var, type_var2)),
                None => assert!(false),
            }
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, None) => {
                    assert_eq!(1, cons.len());
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
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Trait(_, trait1, _) => {
            match tree.trait1(&String::from("T")) {
                Some(trait2) => assert!(Rc::ptr_eq(trait1, trait2)),
                None => assert!(false),
            }
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(_, trait_defs, Some(trait_vars)) => {
                    assert_eq!(3, trait_defs.len());
                    assert_eq!(3, trait_vars.vars().len());
                    match &*trait_defs[0] {
                        TraitDef(_, var, _) => {
                            match trait_vars.var(&String::from("x")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                            match tree.var(&String::from("x")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[1] {
                        TraitDef(_, var, _) => {
                            match trait_vars.var(&String::from("f")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                            match tree.var(&String::from("f")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                        },
                    }
                    match &*trait_defs[2] {
                        TraitDef(_, var, _) => {
                            match trait_vars.var(&String::from("g")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
                                _ => assert!(false),
                            }
                            match tree.var(&String::from("g")) {
                                Some(var2) => assert!(Rc::ptr_eq(var, var2)),
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
    match &*tree.defs()[4] {
        Def::Impl(impl1, _) => {
            match tree.trait1(&String::from("T")) {
                Some(trait1) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, _, Some(trait_vars)) => {
                            match trait_vars.impl1(&TypeName::Name(String::from("Float"))) {
                                Some(impl2) => assert!(Rc::ptr_eq(impl1, impl2)),
                                None => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Builtin(_, _, Some(impl_vars)) => {
                    match impl_vars.var(&String::from("x")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match impl_vars.var(&String::from("f")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match impl_vars.var(&String::from("g")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
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
    match &*tree.defs()[5] {
        Def::Impl(impl1, _) => {
            match tree.trait1(&String::from("T")) {
                Some(trait1) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, _, Some(trait_vars)) => {
                            match trait_vars.impl1(&TypeName::Name(String::from("Int"))) {
                                Some(impl2) => assert!(Rc::ptr_eq(impl1, impl2)),
                                None => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(_, _, impl_defs, Some(impl_vars)) => {
                    assert_eq!(2, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(_, impl_var, _) => {
                            match impl_vars.var(&String::from("x")) {
                                Some(impl_var2) => assert!(Rc::ptr_eq(impl_var, impl_var2)),
                                None => assert!(false),
                            }
                        },
                    }
                    match &*impl_defs[1] {
                        ImplDef(_, impl_var, _) => {
                            match impl_vars.var(&String::from("f")) {
                                Some(impl_var2) => assert!(Rc::ptr_eq(impl_var, impl_var2)),
                                None => assert!(false),
                            }
                        },
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[4] {
        Def::Impl(impl1, _) => {
            match tree.trait1(&String::from("T")) {
                Some(trait1) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, _, Some(trait_vars)) => {
                            match trait_vars.impl1(&TypeName::Name(String::from("Float"))) {
                                Some(impl2) => assert!(Rc::ptr_eq(impl1, impl2)),
                                None => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Builtin(_, _, Some(impl_vars)) => {
                    match impl_vars.var(&String::from("x")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match impl_vars.var(&String::from("f")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match impl_vars.var(&String::from("g")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(None) => assert!(true),
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
    match &*tree.defs()[6] {
        Def::Impl(impl1, _) => {
            match tree.trait1(&String::from("T")) {
                Some(trait1) => {
                    let trait_r = trait1.borrow();
                    match &*trait_r {
                        Trait(_, _, Some(trait_vars)) => {
                            match trait_vars.impl1(&TypeName::Name(String::from("U"))) {
                                Some(impl2) => assert!(Rc::ptr_eq(impl1, impl2)),
                                None => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                None => assert!(false),
            }
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(_, _, impl_defs, Some(impl_vars)) => {
                    assert_eq!(3, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(_, impl_var, _) => {
                            match impl_vars.var(&String::from("x")) {
                                Some(impl_var2) => assert!(Rc::ptr_eq(impl_var, impl_var2)),
                                None => assert!(false),
                            }
                        },
                    }
                    match &*impl_defs[1] {
                        ImplDef(_, impl_var, _) => {
                            match impl_vars.var(&String::from("f")) {
                                Some(impl_var2) => assert!(Rc::ptr_eq(impl_var, impl_var2)),
                                None => assert!(false),
                            }
                        },
                    }
                    match &*impl_defs[2] {
                        ImplDef(_, impl_var, _) => {
                            match impl_vars.var(&String::from("g")) {
                                Some(impl_var2) => assert!(Rc::ptr_eq(impl_var, impl_var2)),
                                None => assert!(false),
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
fn test_namer_check_idents_adds_named_fields()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C { x: Int, y: Float, z: Int, };
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
    assert_eq!(3, tree.defs().len());
    assert_eq!(3, tree.type_vars().len());
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
                    assert_eq!(1, cons.len());
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
                    let con_r = cons[0].borrow();
                    match &*con_r {
                        Con::NamedField(_, _, _, Some(named_fields), _) => {
                            assert_eq!(3, named_fields.field_indices().len());
                            match named_fields.field_index(&String::from("x")) {
                                Some(field_idx) => assert_eq!(0, field_idx),
                                None => assert!(false),
                            }
                            match named_fields.field_index(&String::from("y")) {
                                Some(field_idx) => assert_eq!(1, field_idx),
                                None => assert!(false),
                            }
                            match named_fields.field_index(&String::from("z")) {
                                Some(field_idx) => assert_eq!(2, field_idx),
                                None => assert!(false),
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
fn test_namer_check_idents_checks_identifiers_for_type_expressions()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C();
type U = (Int, Float, T);
type V = (Int, Int) -> Float;
type W = [Int; 10];
type X = [Float; _];
type Y<t, u> = (t, u);
type Z = C;
type A = D<Int, T>;
type B = uniq Int;
type C = T;
type D<t, u> = Y<t, u>;
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
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_where_tuples()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t, u> {};
f(x: (t, u, v)) -> Int
    where t: shared + -> + T <Int, Float>,
          u: shared + -> + T <Int, Float>,
          v: T<w, x>,
          w: shared,
          t == u = 1;
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
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_expressions()
{
    let s = "
builtin type Bool;
builtin type Int;
builtin type Float;
data T = C(Int, Float) | D { x: Int, y: Float, z: Int, };
data U = E(T);
builtin op_add;
builtin op_sub;
a: Int = 1;
b: (Int, Float) = (x, 2.0);
c: [Int; 2] = [x, 2];
d: [Int; 10] = [y; 10];
e: (Int, Int) -> Int = |x, y: Int| x + y;
f: (Float, Float) -> Float = |x: Float, y| -> Float x - y;
g: Int = x;
h: Int = let z = 1 in z;
i: T = C(x, 2.5);
j: T = D { y: 1.5f, z: 1, x: x, };
k: Int = printf(\"%d\n\", x);
l: Int = e(x, y);
m: Int =
    let _ = abc.0.z;
        _ = abc.0.z ->;
        _ = abc.0.z <- x;
        _ = abc.0.z <-> e;
        _ = abc.0.z <-> e ->;
    in  1;
n() -> uniq Int = uniq x;
o() -> Int = shared m();
p(x: t) -> (t, Float) = (x, 1.5): (t, Float);
q: Float = x as Float;
r: Int = if def then x else y;
s: Int =
    let z1 = 1;
        z2 = 2;
    in  z1 + z2;
t: Int = C(1, 2.5) match {
        C(z1, z2) => z1 + (z2 as Int);
        D { z: z, x: x, y: _ } => z - x;
    };
x: Int = 2;
y: Int = 3;
abc: U = E(D { x: 1, y: 2.0, z: 2, });
def: Bool = true;
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
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_patterns()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C(Int, Float) | D { x: Int, y: Float, z: Int, };
builtin op_add;
builtin op_sub;
a: Int = C(1, 2.5) match {
        (x, y) => x + y;
        [x, y] => x - y;
        [x; 2] => x;
        1.5 as Int => 4;
        X => 5;
        C(x, y) => x + (x as Int);
        D { y: _, z: y, x: x, } => x + y;
        x => x.0;
        x @ D { x: 1, y: 2.5, z: 2, } => x.0;
        C(1, 2) | D { x: 1, y: 2.5, z: 2, } => 2;
        _ => 3;
    };
X: Int = 1;
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
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_nested_let_expressions()
{
    let s = "
builtin type Int;
builtin op_add;
builtin op_sub;
a: Int =
    let x = 1;
        y = let z = x;
            in  z + x;
        y = y - x;
    in  let z = 1;
            x = 2;
        in  x + y - z;
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
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_nested_match_expressions()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C(Int, Float);
builtin op_add;
builtin op_sub;
a: Int =
    C(1, 1.5) match {
        C(1, x) => x match { 1.5 => 1; y => y as Int };
        C(2, x) => x match { 2.5 => 3; y => y as Int };
        _ => 3.5;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_type_expressions_in_expressions()
{
    let s = "
data T<t, u> = C() | D(t, u);
x: T<t, u> = C(): T<t, u>;
f(x: T<t, u>) -> (T<t, u>, u) = (x, x.0);
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
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_implementation_variable_with_type_parameters()
{
    let s = "
builtin type Int;
builtin z;
trait T<t, u>
{
    x: (t, u) where t: T<u, Int>;
};
data U<t, u> = C();
impl T for U
{
    x = (C(): U<u, Int>, z);
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_checks_identifiers_for_implementation_function_with_type_parameters()
{
    let s = "
builtin type Int;
trait T<t, u>
{
    f(x: t, y: u) -> t where t: T<u, Int>;
};
data U<t, u> = C();
impl T for U
{
    f(x, y) = x: U<u, Int>;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_defined_errors_for_types()
{
    let s = "
builtin type Int;
builtin type Int;
data T = C();
data T = D();
type U = Int;
type U = Int;
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
        Err(errs) => {
            assert_eq!(3, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("already defined built-in type Int"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("already defined type T"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[2] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("already defined type synonym U"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_defined_errors_for_variable()
{
    let s = "
builtin type Int;
builtin op_add;
builtin op_sub;
builtin op_sub;
x: Int = 1;
x: Int = 2;
f(x: Int, y: Int) -> Int = x + y;
f(x: Int, y: Int) -> Int = x - y;
data T = C();
data U = C();
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
        Err(errs) => {
            assert_eq!(4, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("already defined built-in variable op_sub"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("already defined variable x"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[2] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(8, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("already defined function f"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[3] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(10, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("already defined constructor C"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_function_must_be_variable_in_implementation()
{
    let s = "
builtin type Int;
trait T
{
    x: t where t: T;
};
impl T for Int
{
    x(y) = y;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(2, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(8, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("function x must be variable in implementation T"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("undefined required variable x in implementation T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_variable_must_be_function_in_implementation()
{
    let s = "
builtin type Int;
trait T
{
    f(x: t) -> Int where t: T;
};
impl T for Int
{
    f = 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(2, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(8, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("variable f must be function in implementation T"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("undefined required function f in implementation T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_required_variable_in_implementation()
{
    let s = "
builtin type Int;
trait T
{
    x: t where t: T;
};
impl T for Int
{};
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("undefined required variable x in implementation T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_required_function_in_implementation()
{
    let s = "
builtin type Int;
trait T
{
    f(x: t) -> Int where t: T;
};
impl T for Int
{};
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("undefined required function f in implementation T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_trait_for_implementation()
{
    let s = "
builtin type Int;
impl T for Int
{};
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("undefined trait T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_too_few_arguments_for_implementation()
{
    let s = "
builtin type Int;
trait T
{
    f(x: t, y: t) -> Int where t: T;
};
impl T for Int
{
    f(x) = 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(8, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("too few arguments"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_too_many_arguments_for_implementation()
{
    let s = "
builtin type Int;
trait T
{
    f(x: t, y: t) -> Int where t: T;
};
impl T for Int
{
    f(x, y, z) = 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(8, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("too many arguments"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_defined_field()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C { x: Int, y: Float, x: Int, };
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(32, pos.column);
                    assert_eq!(String::from("already defined field x"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_defined_type_argument()
{
    let s = "
data T<t, t> = C(t);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(String::from("already defined type argument t"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_defined_type_argument_for_type_synonym()
{
    let s = "
builtin type Int;
type T<t, t> = Int;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(String::from("already defined type argument t"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}


#[test]
fn test_namer_check_idents_complains_on_undefined_type_parameter()
{
    let s = "
type T<t> = u;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(13, pos.column);
                    assert_eq!(String::from("undefined type parameter u"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_type_variable()
{
    let s = "
type T = Int;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("undefined type variable Int"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_type_variable_for_type_application()
{
    let s = "
type T = U<Int>;
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
        Err(errs) => {
            assert_eq!(2, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("undefined type variable U"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(12, pos.column);
                    assert_eq!(String::from("undefined type variable Int"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_trait_for_where()
{
    let s = "
builtin type Int;
f(x: t) -> Int where t: T = 1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(22, pos.column);
                    assert_eq!(String::from("undefined trait T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_defined_argument()
{
    let s = "
builtin type Int;
f(x: Int, x: Int) -> Int = x;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(String::from("already defined argument x"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_defined_argument_for_lambda()
{
    let s = "
builtin type Int;
x: (Int, Int)-> Int = |x, x| x;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(27, pos.column);
                    assert_eq!(String::from("already defined argument x"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_variable()
{
    let s = "
builtin type Int;
x: Int = y;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("undefined variable y"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_constructor()
{
    let s = "
builtin type Int;
x: Int = C { x: 1, y: 2, };
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("undefined constructor C"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_field()
{
    let s = "
builtin type Int;
data T = C { x: Int, y: Int, };
x: Int = C { x: 1, y: 2, z: 3, };
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(26, pos.column);
                    assert_eq!(String::from("undefined field z"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_variable_for_pattern()
{
    let s = "
builtin type Int;
x: Int =
    1 match {
        X => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("undefined variable X"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_constructor_for_pattern()
{
    let s = "
builtin type Int;
x: Int =
    1 match {
        C(x, y) => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("undefined constructor C"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_too_few_fields()
{
    let s = "
builtin type Int;
data T = C(Int, Int);
x: Int =
    1 match {
        C(x) => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("too few fields"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_too_many_fields()
{
    let s = "
builtin type Int;
data T = C(Int, Int);
x: Int =
    1 match {
        C(x, y, z) => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("too many fields"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_constructor_for_pattern_and_named_field_constructor()
{
    let s = "
builtin type Int;
x: Int =
    1 match {
        C { x: x, y: y, } => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("undefined constructor C"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_field_for_pattern()
{
    let s = "
builtin type Int;
data T = C { x: Int, y: Int, };
x: Int = 
    1 match {
        C { x: x, y: y, z: z, } => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(25, pos.column);
                    assert_eq!(String::from("undefined field z"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_used_field()
{
    let s = "
builtin type Int;
data T = C { x: Int, y: Int, };
x: Int = 
    1 match {
        C { x: x, y: y, y: 1, } => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(25, pos.column);
                    assert_eq!(String::from("already used field y"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_already_variable_in_pattern()
{
    let s = "
builtin type Int;
data T = C(Int, Int);
x: Int = 
    1 match {
        C(x, x) => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(14, pos.column);
                    assert_eq!(String::from("already variable x in pattern"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_variable_pattern_must_not_be_in_alternative_pattern()
{
    let s = "
builtin type Int;
data T = C(Int) | D(Int);
x: Int = 
    1 match {
        C(x) | D(y) => 1;
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
    let namer = Namer::new();
    match namer.check_idents(&mut tree) {
        Err(errs) => {
            assert_eq!(2, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(String::from("variable pattern mustn't be in alternative pattern"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(18, pos.column);
                    assert_eq!(String::from("variable pattern mustn't be in alternative pattern"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_type_variable_for_as_expression()
{
    let s = "
builtin type Int;
x: Int = 1 as Float;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(15, pos.column);
                    assert_eq!(String::from("undefined type variable Float"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_type_variable_for_typed_expression()
{
    let s = "
builtin type Int;
f() -> Int = 1.5: Float;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(19, pos.column);
                    assert_eq!(String::from("undefined type variable Float"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_namer_check_idents_complains_on_undefined_type_parameter_for_typed_expression()
{
    let s = "
builtin type Int;
f(x: t) -> t = x: u;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(19, pos.column);
                    assert_eq!(String::from("undefined type parameter u"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
