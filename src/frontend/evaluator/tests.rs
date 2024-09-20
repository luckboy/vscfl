//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::*;
use crate::frontend::instancer::*;
use crate::frontend::lexer::*;
use crate::frontend::limiter::*;
use crate::frontend::namer::*;
use crate::frontend::parser::*;
use crate::frontend::typer::*;
use super::*;

#[test]
fn test_evaluator_evaluate_values_evaluates_value_for_variable()
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
    match typer.check_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let instancer = Instancer::new();
    match instancer.check_insts(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let limiter = Limiter::new();
    match limiter.check_limits(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let evaluator = Evaluator::new();
    match evaluator.evaluate_values(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(4, tree.defs().len());
    match &*tree.defs()[3] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(3), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_expressions()
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
data U = D(T);
a: (Int, Int) -> Int = |x: Int, y| x + y;
b: (Float, Float) -> Float = |x, y: Float| -> Float x - y;
c: Int = x;
d: Int = let z = 1 in z;
e: T = C { x: 1, y: 1.5, z: 2, };
f: Int = abc.0.z;
g: Int = let (z, _) = abc.0.z -> in z;
h: U = let abc2 = abc.0.z <- 1 in abc2;
i: Int = let y = uniq x; in shared y;
j: Float = 1.5: Float;
k: Float = x as Float;
l: Int = if true then x else y;
m: Int =
    let z1 = 1;
        z2 = 2;
    in  z1 + z2;
n: Int = C { x: 1, y: 2.5, z: 3, } match {
        C { z: z, x: x, y: _, } => z - x;
    };
x: Int = 1;
y: Int = 2;
abc: U = D(C { x: 2, y: 2.5, z: 3, });
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
    match typer.check_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let instancer = Instancer::new();
    match instancer.check_insts(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let limiter = Limiter::new();
    match limiter.check_limits(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let evaluator = Evaluator::new();
    match evaluator.evaluate_values(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(27, tree.defs().len());
    match &*tree.defs()[10] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, _, _, _, Some(value)) => {
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Lambda(_, _, _, _, _, Some(local_fun), Some(closure), _) => {
                                    assert_eq!(LocalFun::new(0), *local_fun);
                                    assert_eq!(true, closure.values().is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Lambda(String::from("a"), None, LocalFun::new(0))))), *value);
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
                Var::Var(_, _, _, expr, _, _, _, _, Some(value)) => {
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Lambda(_, _, _, _, _, Some(local_fun), Some(closure), _) => {
                                    assert_eq!(LocalFun::new(0), *local_fun);
                                    assert_eq!(true, closure.values().is_empty());
                                },
                                _ => assert!(false),
                            }
                        },
                        None => assert!(false),
                    }
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Lambda(String::from("b"), None, LocalFun::new(0))))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(1), Value::Float(1.5), Value::Int(2)])))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(3), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(3), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("D"), vec![Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(2), Value::Float(2.5), Value::Int(1)]))))])))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Float(1.5), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Float(1.0), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(3), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(2), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(2), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("D"), vec![Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(2), Value::Float(2.5), Value::Int(3)]))))])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_patterns()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin type Float;
builtin impl OpAdd for Int;
data T = C(Int, Float) | D { x: Int, y: Float, z: Int, };
a: Int = 1 match {
        X => 1;
        _ => 2;
    };
b: Int = C(1, 2.5) match {
        C(x, y) => x + (y as Int);
        D { y: _, z: y, x: x, } => x + y;
    };
c: Int = 2 match { x => x; };
d: T = C(1, 2.5) match {
        x @ D { x: 1, y: 2.5, z: 2, } => x;
        _ => C(1, 2.5);
    };
e: Int = C(1, 2.5) match {
        C(1, 2.5) | D { x: 1, y: 2.5, z: 2, } => 1;
        _ => 2;
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
    let typer = Typer::new();
    match typer.check_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let instancer = Instancer::new();
    match instancer.check_insts(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let limiter = Limiter::new();
    match limiter.check_limits(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let evaluator = Evaluator::new();
    match evaluator.evaluate_values(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(11, tree.defs().len());
    match &*tree.defs()[5] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[6] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(3), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[7] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(2), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[8] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(1), Value::Float(2.5)])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[9] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[10] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_expression_literals()
{
    let s = "
builtin type Bool;
builtin type Char;
builtin type Int;
builtin type Long;
builtin type Uint;
builtin type Ulong;
builtin type Float;
builtin type Double;
builtin type ConstantSlice;
a: Bool = true;
b: Char = 'a';
c: Int = 1i;
d: Long = 1I;
e: Uint = 1u;
f: Ulong = 1U;
g: Float = 1.5f;
h: Double = 1.5F;
i: ConstantSlice<Char> = \"abc\";
j: (Int, Float) = (x, 2.0);
k: [Int; 2] = [x, 2];
l: [Int; 10] = [y; 10];
x: Int = 1;
y: Int = 2;
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
    match typer.check_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let instancer = Instancer::new();
    match instancer.check_insts(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let limiter = Limiter::new();
    match limiter.check_limits(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let evaluator = Evaluator::new();
    match evaluator.evaluate_values(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(23, tree.defs().len());
    match &*tree.defs()[9] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Bool(true), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[10] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Char(b'a' as i8), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Long(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Uint(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Ulong(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Float(1.5), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Double(1.5), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::String(b"abc".to_vec())))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Tuple(vec![Value::Int(1), Value::Float(2.0)])))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Array(vec![Value::Int(1), Value::Int(2)])))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Array(vec![Value::Int(2); 10])))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(2), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_pattern_literals()
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
builtin type Char;
builtin type Int;
builtin type Long;
builtin type Uint;
builtin type Ulong;
builtin type Float;
builtin type Double;
builtin type ConstantSlice;
builtin impl OpAdd for Int;
builtin impl OpSub for Int;
a: Int = true match {
        true => 1;
        _ => 2;
    };
b: Int = 'a' match {
        'a' => 1;
        _ => 2;
    };
c: Int = 1i match {
        1i => 1;
        _ => 2;
    };
d: Int = 1I match {
        1I => 1;
        _ => 2;
    };
e: Int = 1u match {
        1u => 1;
        _ => 2;
    };
f: Int = 1U match {
        1U => 1;
        _ => 2;
    };
g: Int = 1.5f match {
        1.5f => 1;
        _ => 2;
    };
h: Int = 1.5F match {
        1.5F => 1;
        _ => 2;
    };
i: Int = \"abc\" match {
        \"abc\" => 1;
        _ => 2;
    };
j: Int = (1, 1.5) match {
        (x, y) => x - (y as Int);
    };
k: Int = [1, 2] match {
        [x, y] => x + y;
    };
l: Int = [1; 2] match {
        [x; 2] => x;
        _ => 2;
    };
m: Int = 1 match {
        1.5 as Int => 1;
        _ => 2;
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
    let typer = Typer::new();
    match typer.check_types(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let instancer = Instancer::new();
    match instancer.check_insts(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let limiter = Limiter::new();
    match limiter.check_limits(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    let evaluator = Evaluator::new();
    match evaluator.evaluate_values(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    assert_eq!(26, tree.defs().len());
    match &*tree.defs()[13] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(0), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(3), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Int(1), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
