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
f(x: (t, u)) -> Int
    where t: shared + -> + T <Int, Float>,
          u: shared + -> + T <Int, Float>,
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
k: Int = e(x, y);
l: Int =
    let _ = abc.0.z;
        _ = abc.0.z ->;
        _ = abc.0.z <- x;
        _ = abc.0.z <-> e;
        _ = abc.0.z <-> e ->;
    in  1;
m() -> uniq Int = uniq x;
n() -> Int = shared m();
o(x: t) -> (t, Float) = (x, 1.5): (t, Float);
p: Float = x as Float;
q: Int = if def then x else y;
r: Int =
    let z1 = 1;
        z2 = 2;
    in  z1 + z2;
s: Int = C(1, 2.5) match {
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
