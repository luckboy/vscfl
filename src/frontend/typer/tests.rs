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

//
// Evaluation of types for type variables.
//

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
fn test_typer_evaluate_types_for_type_vars_evaluates_types_for_type_synonyms()
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

#[test]
fn test_typer_evaluate_types_for_type_vars_evaluates_types_for_builtin_types()
{
    let s = "
builtin type Int;
builtin type Float;
builtin type Int2;
builtin type Float4;
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
    assert_eq!(4, tree.defs().len());
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
                    assert_eq!(2, fields.field_type_values().len());
                    assert_eq!(String::from("Int"), fields.field_type_values()[0].to_string_without_fun());
                    assert_eq!(String::from("Int"), fields.field_type_values()[1].to_string_without_fun());
                    assert_eq!(4, fields.field_indices().len());
                    assert_eq!(Some(0), fields.field_index(&String::from("x")));
                    assert_eq!(Some(1), fields.field_index(&String::from("y")));
                    assert_eq!(Some(0), fields.field_index(&String::from("s0")));
                    assert_eq!(Some(1), fields.field_index(&String::from("s1")));
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
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(true, type_args.type_arg_idents().is_empty());
                    assert_eq!(4, fields.field_type_values().len());
                    assert_eq!(String::from("Float"), fields.field_type_values()[0].to_string_without_fun());
                    assert_eq!(String::from("Float"), fields.field_type_values()[1].to_string_without_fun());
                    assert_eq!(String::from("Float"), fields.field_type_values()[2].to_string_without_fun());
                    assert_eq!(String::from("Float"), fields.field_type_values()[3].to_string_without_fun());
                    assert_eq!(8, fields.field_indices().len());
                    assert_eq!(Some(0), fields.field_index(&String::from("x")));
                    assert_eq!(Some(1), fields.field_index(&String::from("y")));
                    assert_eq!(Some(2), fields.field_index(&String::from("z")));
                    assert_eq!(Some(3), fields.field_index(&String::from("w")));
                    assert_eq!(Some(0), fields.field_index(&String::from("s0")));
                    assert_eq!(Some(1), fields.field_index(&String::from("s1")));
                    assert_eq!(Some(2), fields.field_index(&String::from("s2")));
                    assert_eq!(Some(3), fields.field_index(&String::from("s3")));
                    assert_eq!(SharedFlag::Shared, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_evaluates_types_for_data_types()
{
    let s = "
builtin type Int;
data T<t1> = C(Int) | D(V<t1, t1>, Int); 
data U<t1, t2> = E { x: Int, y: V<t1, Int>, z: t2, };
type V<t1, t2> = (t1, t2);
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
    assert_eq!(4, tree.defs().len());
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
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(2, cons.len());
                    match tree.var(&String::from("C")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(Int) -> T<t1>"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match tree.var(&String::from("D")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("((t1, t1), Int) -> T<t1>"), typ.to_string()),
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
    match &*tree.defs()[2] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("E")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(Int, (t1, Int), t2) -> U<t1, t2>"), typ.to_string()),
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
}

#[test]
fn test_typer_evaluate_types_for_type_vars_evaluates_shared_flags_for_data_types()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Ref;
builtin type UniqRef;
data T = C() | D(Char, Ref<U>);
data U = E(T);
data V = F() | G(Int, UniqRef<W>);
data W = H(V);
data X = I { x: uniq Int, y: Int };
data Y = J((uniq Int) -> uniq Int);
data Z = K(W);
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
    assert_eq!(11, tree.defs().len());
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
    match &*tree.defs()[3] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Builtin(Some(type_args), Some(fields), Some(shared_flag)) => {
                    assert_eq!(1, type_args.type_arg_idents().len());
                    assert_eq!(String::from("t"), type_args.type_arg_idents()[0]);
                    assert_eq!(true, fields.field_type_values().is_empty());
                    assert_eq!(true, fields.field_indices().is_empty());
                    assert_eq!(SharedFlag::None, *shared_flag);
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
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(2, cons.len());
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
                    match tree.var(&String::from("D")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(Char, Ref<U>) -> T"), typ.to_string()),
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
    match &*tree.defs()[5] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("E")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(T) -> U"), typ.to_string()),
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
    match &*tree.defs()[6] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(2, cons.len());
                    match tree.var(&String::from("F")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("() -> V"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    match tree.var(&String::from("G")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(Int, UniqRef<W>) -> V"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(SharedFlag::None, *shared_flag);
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
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("H")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(V) -> W"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(SharedFlag::None, *shared_flag);
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
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("I")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(uniq Int, Int) -> X"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(SharedFlag::None, *shared_flag);
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
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("J")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("((uniq Int) -> uniq Int) -> Y"), typ.to_string()),
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
    match &*tree.defs()[10] {
        Def::Type(_, type_var, _) => {
            let type_var_r = type_var.borrow();
            match &*type_var_r {
                TypeVar::Data(_, cons, Some(shared_flag)) => {
                    assert_eq!(1, cons.len());
                    match tree.var(&String::from("K")) {
                        Some(var) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => assert_eq!(String::from("(W) -> Z"), typ.to_string()),
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(SharedFlag::None, *shared_flag);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_checks_type_recursions_for_data_types()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Ref;
builtin type Slice;
data T = C() | D(Char, Ref<U>) | E(Slice<V>);
data U = F(T);
data V = G(W);
data W = H(T, Int);
data X = I() | J(Int, Ref<X>);
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
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_type_variable_has_type_arguments()
{
    let s = "
builtin type Int;
builtin type Slice;
type T = (Slice, Int);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(String::from("type variable Slice has type arguments"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_too_few_type_arguments()
{
    let s = "
builtin type Int;
data T<t1, t2> = C(t1, t2);
type U<t1> = (T<t1>, Int);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(15, pos.column);
                    assert_eq!(String::from("too few type arguments"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_too_many_type_arguments()
{
    let s = "
builtin type Int;
data T<t1, t2> = C(t1, t2);
type U<t1, t2, t3> = (T<t1, t2, t3>, Int);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(23, pos.column);
                    assert_eq!(String::from("too many type arguments"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_too_few_type_arguments_for_data_type()
{
    let s = "
builtin type Int;
data T<t1, t2> = C(t1, t2);
data U<t1> = D(T<t1>, Int);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(16, pos.column);
                    assert_eq!(String::from("too few type arguments"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_too_many_type_arguments_for_data_type()
{
    let s = "
builtin type Int;
data T<t1, t2> = C(t1, t2);
data U<t1, t2, t3> = D(T<t1, t2, t3>, Int);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(24, pos.column);
                    assert_eq!(String::from("too many type arguments"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_unevaluated_type_synonym()
{
    let s = "
builtin type Int;
data T<t1, t2> = C(t1, t2);
type U<t1> = (T<t1>, Int);
type V = U<Int>;
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
        Err(errs) => {
            assert_eq!(2, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(15, pos.column);
                    assert_eq!(String::from("too few type arguments"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("unevaluated type synonym U"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_definition_of_type_synonym_is_recursive()
{
    let s = "
builtin type Int;
type T = U;
type U = (T, Int);
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
        Err(errs) => {
            assert_eq!(3, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(String::from("definition of type synonym T is recursive"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(11, pos.column);
                    assert_eq!(String::from("unevaluated type synonym T"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[2] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("unevaluated type synonym U"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_definition_of_type_synonym_is_recursive_for_little_recursion()
{
    let s = "
type T = T;
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
        Err(errs) => {
            assert_eq!(2, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("definition of type synonym T is recursive"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(1, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("unevaluated type synonym T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_types_for_type_vars_complains_on_recursive_type_must_be_in_reference_type()
{
    let s = "
data T = C(U);
data U = D(T);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(12, pos.column);
                    assert_eq!(String::from("recursive type T must be in reference type"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}