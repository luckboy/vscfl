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

//
// Evaluation of types with where tuples.
//

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type()
{
    let s = "
builtin type Int;
builtin type Float;
trait T<t> {};
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
    let s3 = "(t, Float)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>";
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
                            assert_eq!(String::from("(t, Float)"), typ.to_string());
                            assert_eq!(1, typ.type_param_entries().len());
                            assert_eq!(1, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(1, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Int"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
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
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_with_shared_and_function()
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Float)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + -> + T <Int, Float>";
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
                            assert_eq!(String::from("(t, Float)"), typ.to_string());
                            assert_eq!(1, typ.type_param_entries().len());
                            assert_eq!(1, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(3, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Fun));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(2, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Int"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(String::from("Float"), type_param_entry_r.type_values[1].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
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
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_with_nested_type_parameters()
{
    let s = "
builtin type Int;
builtin type Float;
builtin type Slice;
trait T<t1> {};
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
    let s3 = "(t, Float)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Slice<u>>, u: U <v>";
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
                            assert_eq!(String::from("(t, Float)"), typ.to_string());
                            assert_eq!(3, typ.type_param_entries().len());
                            assert_eq!(3, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(1, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Slice<t2>"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(1)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(1, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("t3"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("u")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(2)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(true, type_param_entry_r.trait_names.is_empty());
                                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("v")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            // t u v
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            //   u v
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_with_equal_type_parameters()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type Float;
trait T<t1> {};
trait U<t1> {};
trait V<t1> {};
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
    let s3 = "(t, u, v) -> w";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U <Char>, u: T + U <Int>, v: T + U <Float>, w: V <Char>, t == u == v";
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
                            assert_eq!(String::from("(t, u, v) -> w"), typ.to_string());
                            assert_eq!(4, typ.type_param_entries().len());
                            assert_eq!(4, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(2, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Char"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(1)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(2, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Int"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("u")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(2)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(2, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Float"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("v")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(3)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(1, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("V"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Char"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("w")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            // t u v w
                            assert_eq!(true, typ.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(true, typ.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(0), LocalType::new(3)));
                            //   u v w
                            assert_eq!(true, typ.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(1), LocalType::new(3)));
                            //     v w
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(2), LocalType::new(3)));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_with_shared_type_parameters()
{
    let s = "
builtin type Int;
builtin type Slice;
trait T<t1, t2> {};
trait U {};
trait V {};
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
    let s3 = "(t, Int) -> u";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <Slice<u>, v>, u: shared + U, v: shared + V";
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
                            assert_eq!(String::from("(t, Int) -> u"), typ.to_string());
                            assert_eq!(3, typ.type_param_entries().len());
                            assert_eq!(3, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(2, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(2, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("Slice<t2>"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(String::from("t3"), type_param_entry_r.type_values[1].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(1)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(2, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("U"))));
                                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("u")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(2)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(2, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("V"))));
                                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("v")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            // t u v
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            //   u v
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_with_shared_type_parameter_for_data_type_and_function_type()
{
    let s = "
builtin type Char;
builtin type Int;
data T<t1, t2> = C(t1, t2);
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <T<Int, Char>, (uniq Int) -> uniq Char>";
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
                            assert_eq!(String::from("(t, Int)"), typ.to_string());
                            assert_eq!(1, typ.type_param_entries().len());
                            assert_eq!(1, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(2, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(2, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("T<Int, Char>"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(String::from("(uniq Int) -> uniq Char"), type_param_entry_r.type_values[1].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
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
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_with_shared_type_parameter_for_function_trait()
{
    let s = "
builtin type Char;
builtin type Int;
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + -> + T <uniq Int, uniq Char>";
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
                            assert_eq!(String::from("(t, Int)"), typ.to_string());
                            assert_eq!(1, typ.type_param_entries().len());
                            assert_eq!(1, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(3, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Fun));
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(2, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("uniq Int"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(String::from("uniq Char"), type_param_entry_r.type_values[1].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t")), type_param_entry_r.ident);
                                    assert_eq!(None, type_param_entry_r.number);
                                },
                                None => assert!(false),
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
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_without_type_parameters()
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
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
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &None, &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(String::from("Int"), typ.to_string());
                            assert_eq!(true, typ.type_param_entries().is_empty());
                            assert_eq!(true, typ.eq_type_param_set().is_empty());
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_evaluates_type_for_trait_identifier()
{
    let s = "
builtin type Int;
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
    let s3 = "(t1, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t1: T <t2>, t2: T <t3>, t1 == t2";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &Some(String::from("T")), &pos, &tree) {
                        Ok(typ) => {
                            assert_eq!(String::from("(t1, Int)"), typ.to_string());
                            assert_eq!(3, typ.type_param_entries().len());
                            assert_eq!(3, typ.eq_type_param_set().len());
                            match typ.type_param_entry(LocalType::new(0)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(1, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("t2"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t1")), type_param_entry_r.ident);
                                    assert_eq!(Some(1), type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(1)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(1, type_param_entry_r.trait_names.len());
                                    assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("T"))));
                                    assert_eq!(1, type_param_entry_r.type_values.len());
                                    assert_eq!(String::from("t3"), type_param_entry_r.type_values[0].to_string_without_fun());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t2")), type_param_entry_r.ident);
                                    assert_eq!(Some(2), type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            match typ.type_param_entry(LocalType::new(2)) {
                                Some(type_param_entry) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    assert_eq!(true, type_param_entry_r.trait_names.is_empty());
                                    assert_eq!(true, type_param_entry_r.type_values.is_empty());
                                    assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                    assert_eq!(Some(String::from("t3")), type_param_entry_r.ident);
                                    assert_eq!(Some(3), type_param_entry_r.number);
                                },
                                None => assert!(false),
                            }
                            // t u v
                            assert_eq!(true, typ.has_eq_type_params(LocalType::new(0), LocalType::new(1)));
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(0), LocalType::new(2)));
                            //   u v
                            assert_eq!(false, typ.has_eq_type_params(LocalType::new(1), LocalType::new(2)));
                        },
                        Err(_) => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_number_of_type_arguments_of_trait_is_not_equal_to_number_of_type_expressions_of_type_parameter()
{
    let s = "
builtin type Int;
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <Int>";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("number of type arguments of trait T isn't equal to number of type expressions of type parameter t"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_no_type_expressions_of_type_parameter_for_trait_right_arrow()
{
    let s = "
builtin type Int;
trait T {};
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
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + ->";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("no type expressions of type parameter t for trait ->"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_trait_definition_of_type_parameter_is_recursive()
{
    let s = "
builtin type Int;
builtin type Slice;
trait T<t1> {};
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
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <u>, u: U <Slice<t>>";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(11, pos.column);
                                    assert_eq!(String::from("trait definition of type parameter t is recursive"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_trait_definition_of_type_parameter_is_recursive_for_little_recursion()
{
    let s = "
builtin type Int;
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
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <t>";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("trait definition of type parameter t is recursive"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_parameter_must_not_be_shared()
{
    let s = "
builtin type Int;
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <u, v>, u: shared";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("type parameter t mustn't be shared"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_parameter_must_not_be_shared_for_unique_data_type()
{
    let s = "
builtin type Int;
data T<t1> = C(uniq Int, t1);
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <T<u>, Int>, u: shared";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("type parameter t mustn't be shared"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_parameter_must_not_be_shared_for_unique_type()
{
    let s = "
builtin type Int;
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <uniq Int, u>, u: shared";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("type parameter t mustn't be shared"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_parameter_must_not_be_shared_for_two_unique_types()
{
    let s = "
builtin type Char;
builtin type Int;
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
    match typer.evaluate_types_for_type_vars(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let s3 = "(t, Int)";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: shared + T <uniq Int, uniq Char>";
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
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("type parameter t mustn't be shared"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_parameter_has_not_same_traits_as_type_parameter()
{
    let s = "
builtin type Int;
trait T<t1> {};
trait U<t1> {};
trait V<t1> {};
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
    let s3 = "(t, Int) -> u";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T + U <v>, u: T + V <v>, t == u";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &Some(String::from("T")), &pos, &tree) {
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(34, pos.column);
                                    assert_eq!(String::from("type parameter u hasn't same traits as type parameter t"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_of_variable_has_type_parameters_with_trait_which_are_not_equal()
{
    let s = "
builtin type Int;
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
    let s3 = "(t, Int) -> u";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: T <v>, u: T <v>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &Some(String::from("T")), &pos, &tree) {
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("type of variable test has type parameters with trait T which aren't equal"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_of_variable_has_not_type_parameters_with_trait()
{
    let s = "
builtin type Int;
trait T<t1> {};
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
    let s3 = "(t, Int) -> u";
    let mut cursor2 = Cursor::new(s3.as_bytes());
    let mut parser2 = Parser::new(Lexer::new(String::from("test2.vscfl"), &mut cursor2));
    match parser2.parse_type() {
        Ok(type_expr) => {
            let s4 = "t: U <v>, u: U <v>";
            let mut cursor3 = Cursor::new(s4.as_bytes());
            let mut parser3 = Parser::new(Lexer::new(String::from("test3.vscfl"), &mut cursor3));
            match parser3.parse_where() {
                Ok(where_tuples) => {
                    match namer.check_idents_for_type_with_where(&type_expr, where_tuples.as_slice(), &tree) {
                        Ok(()) => assert!(true),
                        Err(_) => assert!(false),
                    }
                    let pos = Pos::new(String::from("test2.vscfl"), 1, 1);
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &Some(String::from("T")), &pos, &tree) {
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("type of variable test hasn't type parameters with trait T"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_typer_evaluate_type_with_where_complains_on_type_of_variable_must_have_type_parameter_with_trait()
{
    let s = "
builtin type Int;
trait T {};
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
                    match typer.evalute_type_with_where("test", &type_expr, where_tuples.as_slice(), &Some(String::from("T")), &pos, &tree) {
                        Err(errs) => {
                            assert_eq!(1, errs.errors().len());
                            match &errs.errors()[0] {
                                FrontendError::Message(pos, msg) => {
                                    assert_eq!(1, pos.line);
                                    assert_eq!(1, pos.column);
                                    assert_eq!(String::from("type of variable test must have type parameter with trait T"), *msg);
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                Err(_) => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

//
// Evaluation of types for variables.
//

#[test]
fn test_typer_evaluate_types_evaluates_types_for_variable()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin impl OpAdd for Int;
a: Int = 1 + 2;
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
    assert_eq!(4, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Trait(_, trait1, _) => {
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(_, trait_defs, _) => {
                    assert_eq!(1, trait_defs.len());
                    match &*trait_defs[0] {
                        TraitDef(_, var, _) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => {
                                    assert_eq!(String::from("(t, t) -> t"), typ.to_string());
                                    assert_eq!(1, typ.type_param_entries().len());
                                    match typ.type_param_entry(LocalType::new(0)) {
                                        Some(type_param_entry) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(1, type_param_entry_r.trait_names.len());
                                            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("OpAdd"))));
                                            assert_eq!(true, type_param_entry_r.type_values.is_empty());
                                            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                        },
                                        None => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                },
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[2] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Builtin(_, _, Some(impl_vars)) => {
                    assert_eq!(1, impl_vars.vars().len());
                    match impl_vars.var(&String::from("op_add")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(Some(typ)) => {
                                    assert_eq!(String::from("(Int, Int) -> Int"), typ.to_string());
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
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, Some(local_type), _) => {
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::Literal(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(2), *local_type);
                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::Literal(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(3), *local_type);
                                            assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(4), *local_type);
                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
}

#[test]
fn test_typer_evaluate_types_checks_type_arguments_for_implementation()
{
    let s = "
trait T<t1, t2> {};
data U<t1, t2> = C(t1, t2);
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
}

#[test]
fn test_typer_evaluate_types_evaluates_types_for_expressions()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
trait OpSub
{
    op_sub(x: t, y: t) -> t where t: OpSub;
};
builtin type Bool;
builtin type Int;
builtin type Float;
builtin impl OpAdd for Int;
builtin impl OpSub for Int;
builtin impl OpSub for Float;
data T = C { x: Int, y: Float, z: Int, }; 
data U = D(Int, Float);
data V = E(T);
a: (Int, Int) -> Int = |x: Int, y| x + y;
b: (Float, Float) -> Float = |x, y: Float| -> Float x - y;
c: Int = x;
d: Int = let z = 1 in z;
e: T = C { x: 1, y: 1.5, z: 2, };
f: Int = printf(\"%d\\n\", x);
g: Int = a(x, y);
h: Int = abc.0.z;
i: Int = let (z, _) = abc.0.z -> in z;
j: V = let abc2 = abc.0.z <- 1 in abc2;
k: V = let abc2 = abc.0.z <-> fu in abc2;
l: Float = let (z, _) = abc.0.z <-> fug2 -> in z;
m() -> uniq Int = uniq x;
n() -> Int = shared m();
o(x: t) -> (t, Float) = (x, 1.5): (t, Float);
p: Float = x as Float;
q: Int = if true then x else y;
r: Int =
    let z1 = 1;
        z2 = 2;
    in  z1 + z2;
s: Int = C { x: 1, y: 2.5, z: 3, } match {
        C { z: z, x: x, y: _, } => z - x;
    };
x: Int = 1;
y: Int = 2;
abc: V = E(C { x: 2, y: 2.5, z: 3, });
fu(x: Int) -> Int = x;
fug2(x: Int) -> (Float, Int) = (1.5, x); 
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
    assert_eq!(35, tree.defs().len());
    match &*tree.defs()[0] {
        Def::Trait(_, trait1, _) => {
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(_, trait_defs, _) => {
                    assert_eq!(1, trait_defs.len());
                    match &*trait_defs[0] {
                        TraitDef(_, var, _) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => {
                                    assert_eq!(String::from("(t, t) -> t"), typ.to_string());
                                    assert_eq!(1, typ.type_param_entries().len());
                                    match typ.type_param_entry(LocalType::new(0)) {
                                        Some(type_param_entry) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(1, type_param_entry_r.trait_names.len());
                                            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("OpAdd"))));
                                            assert_eq!(true, type_param_entry_r.type_values.is_empty());
                                            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                        },
                                        None => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                },
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[1] {
        Def::Trait(_, trait1, _) => {
            let trait_r = trait1.borrow();
            match &*trait_r {
                Trait(_, trait_defs, _) => {
                    assert_eq!(1, trait_defs.len());
                    match &*trait_defs[0] {
                        TraitDef(_, var, _) => {
                            let var_r = var.borrow();
                            match &*var_r {
                                Var::Fun(_, _, Some(typ)) => {
                                    assert_eq!(String::from("(t, t) -> t"), typ.to_string());
                                    assert_eq!(1, typ.type_param_entries().len());
                                    match typ.type_param_entry(LocalType::new(0)) {
                                        Some(type_param_entry) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(1, type_param_entry_r.trait_names.len());
                                            assert_eq!(true, type_param_entry_r.trait_names.contains(&TraitName::Name(String::from("OpSub"))));
                                            assert_eq!(true, type_param_entry_r.type_values.is_empty());
                                            assert_eq!(true, type_param_entry_r.closure_local_types.is_empty());
                                        },
                                        None => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                        },
                    }
                },
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[5] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Builtin(_, _, Some(impl_vars)) => {
                    assert_eq!(1, impl_vars.vars().len());
                    match impl_vars.var(&String::from("op_add")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(Some(typ)) => {
                                    assert_eq!(String::from("(Int, Int) -> Int"), typ.to_string());
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
    match &*tree.defs()[6] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Builtin(_, _, Some(impl_vars)) => {
                    assert_eq!(1, impl_vars.vars().len());
                    match impl_vars.var(&String::from("op_sub")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(Some(typ)) => {
                                    assert_eq!(String::from("(Int, Int) -> Int"), typ.to_string());
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
    match &*tree.defs()[7] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Builtin(_, _, Some(impl_vars)) => {
                    assert_eq!(1, impl_vars.vars().len());
                    match impl_vars.var(&String::from("op_sub")) {
                        Some(impl_var) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Builtin(Some(typ)) => {
                                    assert_eq!(String::from("(Float, Float) -> Float"), typ.to_string());
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
    match &*tree.defs()[11] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("(Int, Int) -> Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("(Int, Int) -> Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Lambda(lambda_args, _, expr, Some(ret_local_type), Some(local_type), None, None, _) => {
                                    assert_eq!(2, lambda_args.len());
                                    match &lambda_args[0] {
                                        LambdaArg(_, _, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                                        },
                                        _ => assert!(false),
                                    }
                                    match &lambda_args[1] {
                                        LambdaArg(_, _, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(2), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(3), *ret_local_type);
                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*ret_local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *ret_local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr {
                                        Expr::App(expr, exprs, Some(local_type), _) => {
                                            match &**expr {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(4), *local_type);
                                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(1), *local_type);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(5), *local_type);
                                            assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(6), *local_type);
                                    assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[12] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("(Float, Float) -> Float"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("(Float, Float) -> Float"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Lambda(lambda_args, _, expr, Some(ret_local_type), Some(local_type), None, None, _) => {
                                    assert_eq!(2, lambda_args.len());
                                    match &lambda_args[0] {
                                        LambdaArg(_, _, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &lambda_args[1] {
                                        LambdaArg(_, _, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(2), *local_type);
                                            assert_eq!(String::from("Float"), local_types.local_type_to_string(*local_type));
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(3), *ret_local_type);
                                    assert_eq!(String::from("Float"), local_types.local_type_to_string(*ret_local_type));
                                    match &**expr {
                                        Expr::App(expr, exprs, Some(local_type), _) => {
                                            match &**expr {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(4), *local_type);
                                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(1), *local_type);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(5), *local_type);
                                            assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(6), *local_type);
                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[13] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Var(_, Some(local_type), _) => {
                                    assert_eq!(LocalType::new(1), *local_type);
                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[14] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, Some(local_type), _) => {
                                    assert_eq!(1, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(1), *local_type);
                                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(2), *local_type);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(3), *local_type);
                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[15] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("T"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("T"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::NamedFieldConApp(_, expr_named_field_pairs, Some(con_local_type), Some(local_type), _) => {
                                    assert_eq!(3, expr_named_field_pairs.len());
                                    match &expr_named_field_pairs[0] {
                                        NamedFieldPair(_, expr, _) => {
                                            match &**expr {
                                                Expr::Literal(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(1), *local_type);
                                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &expr_named_field_pairs[1] {
                                        NamedFieldPair(_, expr, _) => {
                                            match &**expr {
                                                Expr::Literal(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &expr_named_field_pairs[2] {
                                        NamedFieldPair(_, expr, _) => {
                                            match &**expr {
                                                Expr::Literal(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(3), *local_type);
                                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    assert_eq!(LocalType::new(4), *con_local_type);
                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*con_local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *con_local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(5), *local_type);
                                    assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[16] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::PrintfApp(exprs, Some(local_type), _) => {
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::Literal(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(2), *local_type);
                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(3), *local_type);
                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[17] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, Some(local_type), _) => {
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(2, exprs.len());
                                    match &*exprs[0] {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(2), *local_type);
                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &*exprs[1] {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(3), *local_type);
                                            assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(4), *local_type);
                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[18] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::GetField(expr, _, Some(local_type), _) => {
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(2), *local_type);
                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[19] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, Some(local_type), _) => {
                                    assert_eq!(1, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, Some(local_type), _) => {
                                                    match &**literal {
                                                        Literal::Tuple(patterns) => {
                                                            assert_eq!(2, patterns.len());
                                                            match &*patterns[0] {
                                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(3), *local_type);
                                                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*patterns[1] {
                                                                Pattern::Wildcard(Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(4), *local_type);
                                                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(5), *local_type);
                                                    assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Get2Field(expr, _, Some(local_type), _) => {
                                                    match &**expr {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(1), *local_type);
                                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(3), *local_type);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(6), *local_type);
                                    assert_eq!(String::from("t6"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[20] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("V"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("V"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, Some(local_type), _) => {
                                    assert_eq!(1, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(4), *local_type);
                                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::SetField(expr1, _, expr2, Some(local_type), _) => {
                                                    match &**expr1 {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(1), *local_type);
                                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &**expr2 {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(2), *local_type);
                                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(3), *local_type);
                                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(4), *local_type);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(5), *local_type);
                                    assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[21] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("V"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("V"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, Some(local_type), _) => {
                                    assert_eq!(1, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(4), *local_type);
                                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::UpdateField(expr1, _, expr2, Some(local_type), _) => {
                                                    match &**expr1 {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(1), *local_type);
                                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &**expr2 {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(2), *local_type);
                                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(3), *local_type);
                                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(4), *local_type);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(5), *local_type);
                                    assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[22] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Float"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Float"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, Some(local_type), _) => {
                                    assert_eq!(1, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Literal(literal, Some(local_type), _) => {
                                                    match &**literal {
                                                        Literal::Tuple(patterns) => {
                                                            assert_eq!(2, patterns.len());
                                                            match &*patterns[0] {
                                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(4), *local_type);
                                                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*patterns[1] {
                                                                Pattern::Wildcard(Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(5), *local_type);
                                                                    assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(6), *local_type);
                                                    assert_eq!(String::from("t6"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::UpdateGet2Field(expr1, _, expr2, Some(local_type), _) => {
                                                    match &**expr1 {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(1), *local_type);
                                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &**expr2 {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(2), *local_type);
                                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(3), *local_type);
                                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(4), *local_type);
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(7), *local_type);
                                    assert_eq!(String::from("t7"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[23] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, Some(typ)) => {
                    assert_eq!(String::from("() -> uniq Int"), typ.to_string());
                    match &**fun {
                        Fun::Fun(_, args, _, _, expr, Some(ret_local_type), Some(local_types)) => {
                            assert_eq!(true, args.is_empty());
                            assert_eq!(LocalType::new(0), *ret_local_type);
                            assert_eq!(String::from("uniq Int"), local_types.local_type_to_string(*ret_local_type));
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Uniq(expr, Some(local_type), _) => {
                                            match &**expr {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(1), *local_type);
                                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(2), *local_type);
                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[24] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, Some(typ)) => {
                    assert_eq!(String::from("() -> Int"), typ.to_string());
                    match &**fun {
                        Fun::Fun(_, args, _, _, expr, Some(ret_local_type), Some(local_types)) => {
                            assert_eq!(true, args.is_empty());
                            assert_eq!(LocalType::new(0), *ret_local_type);
                            assert_eq!(String::from("Int"), local_types.local_type_to_string(*ret_local_type));
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Shared(expr, Some(local_type), _) => {
                                            match &**expr {
                                                Expr::App(expr, exprs, Some(local_type), _) => {
                                                    match &**expr {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(1), *local_type);
                                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(true, exprs.is_empty());
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(3), *local_type);
                                            assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[25] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, Some(typ)) => {
                    assert_eq!(String::from("(t) -> (t, Float)"), typ.to_string());
                    match &**fun {
                        Fun::Fun(_, args, _, _, expr, Some(ret_local_type), Some(local_types)) => {
                            assert_eq!(String::from("t"), local_types.local_type_to_string(LocalType::new(0)));
                            assert_eq!(1, args.len());
                            match &args[0] {
                                Arg(_, _, Some(local_type), _) => {
                                    assert_eq!(LocalType::new(1), *local_type);
                                    assert_eq!(String::from("t"), local_types.local_type_to_string(*local_type));
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(LocalType::new(2), *ret_local_type);
                            assert_eq!(String::from("(t, Float)"), local_types.local_type_to_string(*ret_local_type));
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Typed(expr, _, Some(local_type), _) => {
                                            match &**expr {
                                                Expr::Literal(literal, Some(local_type), _) => {
                                                    match &**literal {
                                                        Literal::Tuple(exprs) => {
                                                            assert_eq!(2, exprs.len());
                                                            match &*exprs[0] {
                                                                Expr::Var(_, Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(1), *local_type);
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                            match &*exprs[1] {
                                                                Expr::Literal(_, Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(3), *local_type);
                                                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(4), *local_type);
                                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(5), *local_type);
                                            assert_eq!(String::from("(t, Float)"), local_types.local_type_to_string(*local_type));                                        },
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[26] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Float"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Float"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::As(expr, _, Some(local_type), _) => {
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(2), *local_type);
                                    assert_eq!(String::from("Float"), local_types.local_type_to_string(*local_type));
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
    match &*tree.defs()[27] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::If(expr1, expr2, expr3, Some(local_type), _) => {
                                    match &**expr1 {
                                        Expr::Literal(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr2 {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(2), *local_type);
                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    match &**expr3 {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(3), *local_type);
                                            assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(4), *local_type);
                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[28] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(binds, expr, Some(local_type), _) => {
                                    assert_eq!(2, binds.len());
                                    match &binds[0] {
                                        Bind(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                    assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(1), *local_type);
                                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
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
                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(4), *local_type);
                                                    assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::Literal(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(3), *local_type);
                                                    assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    match &**expr {
                                        Expr::App(expr, exprs, Some(local_type), _) => {
                                            match &**expr {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(5), *local_type);
                                                    assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(2, exprs.len());
                                            match &*exprs[0] {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(2), *local_type);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &*exprs[1] {
                                                Expr::Var(_, Some(local_type), _) => {
                                                    assert_eq!(LocalType::new(4), *local_type);
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(6), *local_type);
                                            assert_eq!(String::from("t6"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(7), *local_type);
                                    assert_eq!(String::from("t7"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[29] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Match(expr, cases, Some(local_type), _) => {
                                    match &**expr {
                                        Expr::NamedFieldConApp(_, expr_named_field_pairs, Some(con_local_type), Some(local_type), _) => {
                                            assert_eq!(3, expr_named_field_pairs.len());
                                            match &expr_named_field_pairs[0] {
                                                NamedFieldPair(_, expr, _) => {
                                                    match &**expr {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(1), *local_type);
                                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                            }
                                            match &expr_named_field_pairs[1] {
                                                NamedFieldPair(_, expr, _) => {
                                                    match &**expr {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(2), *local_type);
                                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                            }
                                            match &expr_named_field_pairs[2] {
                                                NamedFieldPair(_, expr, _) => {
                                                    match &**expr {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(3), *local_type);
                                                            assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                            }
                                            assert_eq!(LocalType::new(4), *con_local_type);
                                            assert_eq!(String::from("t4"), local_types.local_type_to_string(*con_local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(5), *local_type);
                                            assert_eq!(String::from("t5"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, cases.len());
                                    match &cases[0] {
                                        Case(pattern, expr) => {
                                            match &**pattern {
                                                Pattern::NamedFieldCon(_, pattern_named_field_pairs, Some(con_local_type), Some(local_type), _) => {
                                                    assert_eq!(3, pattern_named_field_pairs.len());
                                                    match &pattern_named_field_pairs[0] {
                                                        NamedFieldPair(_, pattern, _) => {
                                                            match &**pattern {
                                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(6), *local_type);
                                                                    assert_eq!(String::from("t6"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                    }
                                                    match &pattern_named_field_pairs[1] {
                                                        NamedFieldPair(_, pattern, _) => {
                                                            match &**pattern {
                                                                Pattern::Var(_, _, Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(7), *local_type);
                                                                    assert_eq!(String::from("t7"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                    }
                                                    match &pattern_named_field_pairs[2] {
                                                        NamedFieldPair(_, pattern, _) => {
                                                            match &**pattern {
                                                                Pattern::Wildcard(Some(local_type), _) => {
                                                                    assert_eq!(LocalType::new(8), *local_type);
                                                                    assert_eq!(String::from("t8"), local_types.local_type_to_string(*local_type));
                                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                            let type_param_entry_r = type_param_entry.borrow();
                                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                        },
                                                                        _ => assert!(false),
                                                                    }
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                    }
                                                    assert_eq!(LocalType::new(9), *con_local_type);
                                                    assert_eq!(String::from("t9"), local_types.local_type_to_string(*con_local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *con_local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(10), *local_type);
                                                    assert_eq!(String::from("t10"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr {
                                                Expr::App(expr, exprs, Some(local_type), _) => {
                                                    match &**expr {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(11), *local_type);
                                                            assert_eq!(String::from("t11"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(6), *local_type);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(7), *local_type);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    assert_eq!(LocalType::new(12), *local_type);
                                                    assert_eq!(String::from("t12"), local_types.local_type_to_string(*local_type));
                                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                            let type_param_entry_r = type_param_entry.borrow();
                                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                    }
                                    assert_eq!(LocalType::new(13), *local_type);
                                    assert_eq!(String::from("t13"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[30] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(_, Some(local_type), _) => {
                                    assert_eq!(LocalType::new(1), *local_type);
                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[31] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("Int"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Literal(_, Some(local_type), _) => {
                                    assert_eq!(LocalType::new(1), *local_type);
                                    assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[32] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, Some(local_type), Some(local_types), Some(typ), None) => {
                    assert_eq!(String::from("V"), typ.to_string());
                    assert_eq!(LocalType::new(0), *local_type);
                    assert_eq!(String::from("V"), local_types.local_type_to_string(*local_type));
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::App(expr, exprs, Some(local_type), _) => {
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(1), *local_type);
                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(1, exprs.len());
                                    match &*exprs[0] {
                                        Expr::NamedFieldConApp(_, expr_named_field_pairs, Some(con_local_type), Some(local_type), _) => {
                                            assert_eq!(3, expr_named_field_pairs.len());
                                            match &expr_named_field_pairs[0] {
                                                NamedFieldPair(_, expr, _) => {
                                                    match &**expr {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(2), *local_type);
                                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                            }
                                            match &expr_named_field_pairs[1] {
                                                NamedFieldPair(_, expr, _) => {
                                                    match &**expr {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(3), *local_type);
                                                            assert_eq!(String::from("t3"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                            }
                                            match &expr_named_field_pairs[2] {
                                                NamedFieldPair(_, expr, _) => {
                                                    match &**expr {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(4), *local_type);
                                                            assert_eq!(String::from("t4"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                            }
                                            assert_eq!(LocalType::new(5), *con_local_type);
                                            assert_eq!(String::from("t5"), local_types.local_type_to_string(*con_local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *con_local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(6), *local_type);
                                            assert_eq!(String::from("t6"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                    assert_eq!(LocalType::new(7), *local_type);
                                    assert_eq!(String::from("t7"), local_types.local_type_to_string(*local_type));
                                    match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                        Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                            let type_param_entry_r = type_param_entry.borrow();
                                            assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                        },
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
    match &*tree.defs()[33] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, Some(typ)) => {
                    assert_eq!(String::from("(Int) -> Int"), typ.to_string());
                    match &**fun {
                        Fun::Fun(_, args, _, _, expr, Some(ret_local_type), Some(local_types)) => {
                            assert_eq!(1, args.len());
                            match &args[0] {
                                Arg(_, _, Some(local_type), _) => {
                                    assert_eq!(LocalType::new(0), *local_type);
                                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(LocalType::new(1), *ret_local_type);
                            assert_eq!(String::from("Int"), local_types.local_type_to_string(*ret_local_type));
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Var(_, Some(local_type), _) => {
                                            assert_eq!(LocalType::new(0), *local_type);
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
        },
        _ => assert!(false),
    }
    match &*tree.defs()[34] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(fun, _, Some(typ)) => {
                    assert_eq!(String::from("(Int) -> (Float, Int)"), typ.to_string());
                    match &**fun {
                        Fun::Fun(_, args, _, _, expr, Some(ret_local_type), Some(local_types)) => {
                            assert_eq!(1, args.len());
                            match &args[0] {
                                Arg(_, _, Some(local_type), _) => {
                                    assert_eq!(LocalType::new(0), *local_type);
                                    assert_eq!(String::from("Int"), local_types.local_type_to_string(*local_type));
                                },
                                _ => assert!(false),
                            }
                            assert_eq!(LocalType::new(1), *ret_local_type);
                            assert_eq!(String::from("(Float, Int)"), local_types.local_type_to_string(*ret_local_type));
                            match expr {
                                Some(expr) => {
                                    match &**expr {
                                        Expr::Literal(literal, Some(local_type), _) => {
                                            match &**literal {
                                                Literal::Tuple(exprs) => {
                                                    assert_eq!(2, exprs.len());
                                                    match &*exprs[0] {
                                                        Expr::Literal(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(2), *local_type);
                                                            assert_eq!(String::from("t1"), local_types.local_type_to_string(*local_type));
                                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                                    let type_param_entry_r = type_param_entry.borrow();
                                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                                },
                                                                _ => assert!(false),
                                                            }
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                    match &*exprs[1] {
                                                        Expr::Var(_, Some(local_type), _) => {
                                                            assert_eq!(LocalType::new(0), *local_type);
                                                        },
                                                        _ => assert!(false),
                                                    }
                                                },
                                                _ => assert!(false),
                                            }
                                            assert_eq!(LocalType::new(3), *local_type);
                                            assert_eq!(String::from("t2"), local_types.local_type_to_string(*local_type));
                                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, *local_type))) {
                                                Some(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, _)) => {
                                                    let type_param_entry_r = type_param_entry.borrow();
                                                    assert_eq!(false, type_param_entry_r.trait_names.contains(&TraitName::Shared));
                                                },
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
        },
        _ => assert!(false),
    }
}
