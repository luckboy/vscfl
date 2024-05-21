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
use super::*;

#[test]
fn test_typer_evaluate_types_for_type_vars_evaluates_type_synonym()
{
    let s = "
builtin type Char;
builtin type Int;
type T = (Int, Char);
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
    assert_eq!(3, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(Int, Char)"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_evaluates_type_arguments_for_builtin_type()
{
    let s = "
builtin type Slice;
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
    assert_eq!(1, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(1, type_args.type_arg_idents().len());
                    assert_eq!(String::from("t"), type_args.type_arg_idents()[0]);
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_evaluates_type_expressions()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Float;
data T = C();
type U = (Float, Int, T);
type V = (Int, Char) -> Float;
type W = [Int; 10];
type X = [Float; _];
type Y<t1, t2> = (t1, t2);
type Z = C;
type A = D<Int, Char>;
type B = uniq Int;
type C = T;
type D<t1, t2> = (t2, t1);
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
    assert_eq!(14, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("C")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("() -> T"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[4] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(Float, Int, T)"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[5] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(Int, Char) -> Float"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[6] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("[Int; 10]"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[7] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("[Float; _]"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[8] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(t1, t2)"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[9] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("T"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[10] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(Char, Int)"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[11] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("uniq Int"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[12] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("T"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[13] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(t2, t1)"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_evaluates_type_synonyms()
{
    let s = "
builtin type Char;
builtin type Int;
data T<t1, t2> = C(t1, t2);
data U<t1, t2> = D(t1, t2);
type V<t1, t2, t3> = W<t1, U<t2, t3>>;
type W<t1, t2> = (t2, X<t1, Char>, X<Char, Int>, Y<t1>, Z<Char>);
type X<t1, t2> = T<t2, t1>;
type Y<t1> = U<t1, t1>;
type Z<t1> = t1;
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
    assert_eq!(9, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("C")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(t1, t2) -> T<t1, t2>"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[3] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("D")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(t1, t2) -> U<t1, t2>"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[4] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(U<t2, t3>, T<Char, t1>, T<Int, Char>, U<t1, t1>, Char)"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[5] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("(t2, T<Char, t1>, T<Int, Char>, U<t1, t1>, Char)"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[6] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("T<t2, t1>"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[7] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("U<t1, t1>"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[8] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Synonym(_, _, Some(type_value)) => {
                    assert_eq!(String::from("t1"), type_value.to_string_without_fun());
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
