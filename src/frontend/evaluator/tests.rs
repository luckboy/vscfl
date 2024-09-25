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

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_implemented_variables()
{
    let s = "
trait T
{
    a: t where t: shared + T;
};
builtin type Int;
data U = C(Int);
data V = D(U);
impl T for V
{
    a = D(a);
};
impl T for U
{
    a = C(a);
};
impl T for Int
{
    a = 1;
};
x: V = a;
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
    assert_eq!(8, tree.defs().len());
    match &*tree.defs()[4] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(_, _, impl_defs, _) => {
                    assert_eq!(1, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(_, impl_var, _) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Var(_, _, _, _, Some(value)) => {
                                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("D"), vec![Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(1)]))))])))), *value);
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
    match &*tree.defs()[5] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(_, _, impl_defs, _) => {
                    assert_eq!(1, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(_, impl_var, _) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Var(_, _, _, _, Some(value)) => {
                                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(1)])))), *value);
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
    match &*tree.defs()[6] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(_, _, impl_defs, _) => {
                    assert_eq!(1, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(_, impl_var, _) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Var(_, _, _, _, Some(value)) => {
                                    assert_eq!(Value::Int(1), *value);
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
    match &*tree.defs()[7] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("D"), vec![Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(1)]))))])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_implemented_variables_with_default_value()
{
    let s = "
data T<t1> = C() | D(t1);
trait U
{
    a: T<t> where t: shared + U = C();
};
builtin type Int;
builtin type Float;
impl U for Int
{
    a = D(1);
};
impl U for Float {};
x: T<Int> = a;
y: T<Float> = a;
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
    assert_eq!(8, tree.defs().len());
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
                                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), Vec::new())))), *value);
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
    match &*tree.defs()[4] {
        Def::Impl(impl1, _) => {
            let impl_r = impl1.borrow();
            match &*impl_r {
                Impl::Impl(_, _, impl_defs, _) => {
                    assert_eq!(1, impl_defs.len());
                    match &*impl_defs[0] {
                        ImplDef(_, impl_var, _) => {
                            let impl_var_r = impl_var.borrow();
                            match &*impl_var_r {
                                ImplVar::Var(_, _, _, _, Some(value)) => {
                                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("D"), vec![Value::Int(1)])))), *value);
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
    match &*tree.defs()[6] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("D"), vec![Value::Int(1)])))), *value);
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
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), Vec::new())))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_lambdas_with_closures()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Bool;
builtin type Int;
builtin impl OpAdd for Int;
a: (Int) -> Int = let x = 1; y = 2; in |z| x + y + z;
b: (Int) -> Int = let x = 1; y = 2; in |z| let y = 3; in x + y + z;
c: (Int) -> Int = let x = 1; in if false then |y| 1 + y else |y| x + y;
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
    assert_eq!(7, tree.defs().len());
    match &*tree.defs()[4] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, _, _, _, Some(value)) => {
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(_, expr, _, _) => {
                                    match &**expr {
                                        Expr::Lambda(_, _, _, _, _, Some(local_fun), Some(closure), _) => {
                                            assert_eq!(LocalFun::new(0), *local_fun);
                                            assert_eq!(2, closure.values().len());
                                            match closure.value(&String::from("x")) {
                                                Some(value) => assert_eq!(Value::Int(1), *value),
                                                None => assert!(false),
                                            }
                                            match closure.value(&String::from("y")) {
                                                Some(value) => assert_eq!(Value::Int(2), *value),
                                                None => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
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
    match &*tree.defs()[5] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, _, _, _, Some(value)) => {
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(_, expr, _, _) => {
                                    match &**expr {
                                        Expr::Lambda(_, _, _, _, _, Some(local_fun), Some(closure), _) => {
                                            assert_eq!(LocalFun::new(0), *local_fun);
                                            assert_eq!(1, closure.values().len());
                                            match closure.value(&String::from("x")) {
                                                Some(value) => assert_eq!(Value::Int(1), *value),
                                                None => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
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
    match &*tree.defs()[6] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, expr, _, _, _, _, Some(value)) => {
                    match expr {
                        Some(expr) => {
                            match &**expr {
                                Expr::Let(_, expr, _, _) => {
                                    match &**expr {
                                        Expr::If(_, expr2, expr3, _, _) => {
                                            match &**expr2 {
                                                Expr::Lambda(_, _, _, _, _, Some(local_fun), None, _) => {
                                                    assert_eq!(LocalFun::new(0), *local_fun);
                                                },
                                                _ => assert!(false),
                                            }
                                            match &**expr3 {
                                                Expr::Lambda(_, _, _, _, _, Some(local_fun), Some(closure), _) => {
                                                    assert_eq!(LocalFun::new(1), *local_fun);
                                                    assert_eq!(1, closure.values().len());
                                                    match closure.value(&String::from("x")) {
                                                        Some(value) => assert_eq!(Value::Int(1), *value),
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
                        },
                        None => assert!(false),
                    }
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Lambda(String::from("c"), None, LocalFun::new(1))))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_builtin_values_and_function_values()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
trait Zero
{
    builtin zero;
};
builtin type Int;
builtin type Float;
builtin type Float2;
builtin impl OpAdd for Int;
builtin impl Zero for Int;
builtin float2;
builtin FLOAT_MAX_EXP;
a: (Int, Int) -> Int = op_add;
b: (Float, Float) -> Float2 = float2;
c: Int = FLOAT_MAX_EXP;
d: () -> Int = zero; 
e: (Int, Int) -> Int = g;
f: (Int) -> Int = h;
g(x: Int, y: Int) -> Int = x + y;
trait T
{
    h(x: t) -> t where t: T = x;
};
impl T for Int
{
    h(x) = x + 1;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
    assert_eq!(18, tree.defs().len());
    match &*tree.defs()[9] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    match evaluator.evals().fun(&(String::from("op_add"), Some(TypeName::Name(String::from("Int"))))) {
                        Some(fun) => {
                            assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::EvalFun(String::from("op_add"), Some(TypeName::Name(String::from("Int"))), fun)))), *value);
                        },
                        None => assert!(false),
                    }
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
                    match evaluator.evals().fun(&(String::from("float2"), None)) {
                        Some(fun) => {
                            assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::EvalFun(String::from("float2"), None, fun)))), *value);
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
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Builtin(String::from("FLOAT_MAX_EXP"), None)))), *value);
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
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Builtin(String::from("zero"), Some(TypeName::Name(String::from("Int"))))))), *value);
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
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Fun(String::from("g"), None)))), *value);
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
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Fun(String::from("h"), Some(TypeName::Name(String::from("Int"))))))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_castings_in_expressions()
{
    let s = "
builtin type Int;
builtin type Float;
a: Int = 1.5 as Int;
b: (Int, Float) = (1.5, 2) as (Int, Float);
c: [Int; 3] = [1.5, 2.5, 3.5] as [Int; 3];
d: [Float; 2] = [1; 2] as [Float; 2];
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
    assert_eq!(6, tree.defs().len());
    match &*tree.defs()[2] {
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
    match &*tree.defs()[3] {
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
    match &*tree.defs()[4] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }    
    match &*tree.defs()[5] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Array(vec![Value::Float(1.0), Value::Float(1.0)])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }    
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_castings_in_patterns()
{
    let s = "
builtin type Int;
builtin type Float;
a: Int = 1 match {
        1.5 as Int => 1;
        _ => 2;
    };
b: Int = (1, 2.0) match {
        (1.5, 2) as (Int, Float) => 1;
        _ => 2;
    };
c: Int = [1, 2, 3] match {
        [1.5, 2.5, 3.5] as [Int; 3] => 1;
        _ => 2;
    };
d: Int = [1.0, 1.0] match {
        [1; 2] as [Float; 2] => 1;
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
    assert_eq!(6, tree.defs().len());
    match &*tree.defs()[2] {
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
    match &*tree.defs()[3] {
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
    match &*tree.defs()[4] {
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
}

#[test]
fn test_evaluator_evaluate_values_evaluates_value_for_pattern_matchting_with_filled_array()
{
    let s = "
builtin type Int;
a: Int = [1, 2, 3] match {
        [_; 3] => 1;
        [1, 2, 3] => 2; 
        _ => 3;
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
    assert_eq!(2, tree.defs().len());
    match &*tree.defs()[1] {
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
fn test_evaluator_evaluate_values_evaluates_values_for_pattern_matchting_with_vector()
{
    let s = "
builtin type Int;
builtin type Float;
builtin type Float4;
builtin float4;
V: Float4 = float4(1.5, 2.5, 3.5, 4.5);
a: Int = float4(1.5, 2.5, 3.5, 4.5) match {
        V => 1;
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
    assert_eq!(6, tree.defs().len());
    match &*tree.defs()[4] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::FloatN(vec![1.5, 2.5, 3.5, 4.5])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
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
}

#[test]
fn test_evaluator_evaluate_values_evaluates_values_for_pattern_matchtings_with_constants()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C(Int, Float) | D();
A: T = C(1, 1.5);
B: (Int, Float) = (1, 1.5);
E: [Int; 2] = [1, 2];
a: Int = C(1, 1.5) match {
        A => 1;
        _ => 2;
    };
b: Int = (1, 1.5) match {
        B => 1;
        _ => 2;
    };
c: Int = [1, 2] match {
        E => 1;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
    assert_eq!(9, tree.defs().len());
    match &*tree.defs()[3] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Data(String::from("C"), vec![Value::Int(1), Value::Float(1.5)])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[4] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Tuple(vec![Value::Int(1), Value::Float(1.5)])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
    match &*tree.defs()[5] {
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
    match &*tree.defs()[6] {
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
    match &*tree.defs()[7] {
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
    match &*tree.defs()[8] {
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
fn test_evaluator_evaluate_values_evaluates_value_for_fields_with_shared_types()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin type Float;
builtin type Int4;
builtin impl OpAdd for Int;
builtin int4;
data T = C(U, Int4, (Int, Int));
data U = D(Int, Float, Int);
a: (Int, Int, Int) =
    let x = C(D(1, 2.5, 3), int4(1, 2, 3, 4), (1, 2));
        (y1, x) = x.0.0 ->;
        (z1, x) = x.0.2 ->;
        x = x.0.0 <- (y1 + z1);
        (y2, x) = x.1.x ->;
        (z2, x) = x.1.w ->;
        x = x.1.x <- (y2 + z2);
        (y3, x) = x.2.0 ->;
        (z3, x) = x.2.1 ->;
        x = x.2.0 <- (y3 + z3);
        (w1, x) = x.0.0 ->;
        (w2, x) = x.1.x ->;
        w3 = x.2.0;
    in  (w1, w2, w3);
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
    assert_eq!(9, tree.defs().len());
    match &*tree.defs()[8] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Tuple(vec![Value::Int(4), Value::Int(5), Value::Int(3)])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_evaluates_value_for_fields_with_unique_types()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin type Float;
builtin type Int4;
builtin impl OpAdd for Int;
builtin int4;
data T = C(uniq U, uniq Int4, uniq (Int, Int));
data U = D(Int, Float, Int);
a: (Int, Int, Int) =
    let x = C(uniq D(1, 2.5, 3), uniq int4(1, 2, 3, 4), uniq (1, 2));
        (y1, x) = x.0.0 ->;
        (z1, x) = x.0.2 ->;
        x = x.0.0 <- (y1 + z1);
        (y2, x) = x.1.x ->;
        (z2, x) = x.1.w ->;
        x = x.1.x <- (y2 + z2);
        (y3, x) = x.2.0 ->;
        (z3, x) = x.2.1 ->;
        x = x.2.0 <- (y3 + z3);
        (w1, x) = x.0.0 ->;
        (w2, x) = x.1.x ->;
        w3 = x.2.0;
    in  (w1, w2, w3);
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
    assert_eq!(9, tree.defs().len());
    match &*tree.defs()[8] {
        Def::Var(_, var, _) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Var(_, _, _, _, _, _, _, _, Some(value)) => {
                    assert_eq!(Value::Object(SharedFlag::Shared, Rc::new(RefCell::new(Object::Tuple(vec![Value::Int(4), Value::Int(5), Value::Int(3)])))), *value);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_checks_pattern_exhaustions_for_variables()
{
    let s = "
builtin type Bool;
builtin type Int;
builtin type Float;
data T = C(Bool, Int) | D(Int, Int) | E(Float);
a: Int = C(true, 1) match {
        C(true, 1) => 1;
        C(true, _) => 2;
        C(false, _) => 3;
        D(_, _) => 4;
        E(_) => 5;
    };
b: Int = let (x, _) = (1, 2.5); in x;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
    assert_eq!(6, tree.defs().len());
    match &*tree.defs()[4] {
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
}

#[test]
fn test_evaluator_evaluate_values_checks_pattern_exhaustions_for_functions()
{
    let s = "
builtin type Bool;
builtin type Int;
builtin type Float;
data T = C(Bool, Int) | D(Int, Int) | E(Float);
f(x: T) -> Int = x match {
        C(true, 1) => 1;
        C(true, _) => 2;
        C(false, _) => 3;
        D(_, _) => 4;
        E(_) => 5;
    };
g(t: (Int, Float)) -> Int = let (x, _) = t; in x;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
}

#[test]
fn test_evaluator_evaluate_values_complains_on_definition_of_variable_is_recursive_for_little_recursion()
{
    let s = "
builtin type Int;
a: Int = a;
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
        Err(errs) => {
            assert_eq!(2, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("definition of variable a is recursive"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("unevaluated variable a"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_definition_of_variable_is_recursive()
{
    let s = "
builtin type Int;
a: Int = b;
b: Int = a;
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
        Err(errs) => {
            assert_eq!(3, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("definition of variable a is recursive"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[1] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("unevaluated variable a"), *msg);
                },
                _ => assert!(false),
            }
            match &errs.errors()[2] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("unevaluated variable b"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_non_exhaustive_patterns_for_variable()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C(Int, Float) | D(Int) | E();
a: Int = C(1, 1.5) match {
        C(1, _) => 1;
        C(_, _) => 2;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("non-exhaustive patterns"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_non_exhaustive_pattern_for_variable()
{
    let s = "
builtin type Int;
builtin type Float;
a: Int = let (x, 2.5) = (1, 2.5); in x;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(14, pos.column);
                    assert_eq!(String::from("non-exhaustive pattern"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_non_exhaustive_patterns_for_function()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C(Int, Float) | D(Int) | E();
f(x: T) -> Int = x match {
        C(1, _) => 1;
        C(_, _) => 2;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(18, pos.column);
                    assert_eq!(String::from("non-exhaustive patterns"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_non_exhaustive_pattern_for_function()
{
    let s = "
builtin type Int;
builtin type Float;
f(t: (Int, Float)) -> Int = let (x, 2.5) = t; in x;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(33, pos.column);
                    assert_eq!(String::from("non-exhaustive pattern"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_value_is_not_evaluable_function()
{
    let s = "
builtin type Int;
a: Int = f(1);
f(x: Int) -> Int = x;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("value isn't evaluable function"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_division_by_zero()
{
    let s = "
trait OpDiv
{
    op_div(x: t, y: t) -> t where t: OpDiv;
};
builtin type Int;
builtin impl OpDiv for Int;
a: Int = 1 / 0;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(7, pos.line);
                    assert_eq!(12, pos.column);
                    assert_eq!(String::from("division by zero"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_can_not_cast_value_to_type_half_for_evaluation_of_variable_values()
{
    let s = "
builtin type Half;
builtin type Float;
a: Half = 1.5 as Half;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(15, pos.column);
                    assert_eq!(String::from("can't cast value to type Half for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_can_not_cast_pattern_to_type_half_for_evaluation_of_variable_values()
{
    let s = "
builtin type Int;
builtin type Half;
builtin type Float;
data T = C(Float, Half) | D();
a: Int = D() match {
        C(1.5, 1.5 as Half) => 1;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(16, pos.column);
                    assert_eq!(String::from("can't cast pattern to type Half for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_value_of_built_in_variable_must_not_be_in_vector_for_evaluation_of_variable_values()
{
    let s = "
builtin type Int;
builtin type Int4;
builtin FLOAT_MAX_EXP;
builtin int4;
a: Int =
    let v = int4(1, 2, 3, 4);
        v = v.w <- FLOAT_MAX_EXP;  
    in  v.w;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(7, pos.line);
                    assert_eq!(13, pos.column);
                    assert_eq!(String::from("value of built-in variable mustn't be in vector for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_reference_fields_are_unsupported_for_evaluation_of_variable_values_for_tuple()
{
    let s = "
builtin type Int;
builtin type Float;
builtin type Ref;
builtin ref;
a: Ref<Int> =
    let x = ref((1, 1.5));
    in  x.0;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(7, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("reference fields are unsupported for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_reference_fields_are_unsupported_for_evaluation_of_variable_values_for_data_type()
{
    let s = "
builtin type Int;
builtin type Float;
builtin type Ref;
builtin ref;
data T = C(Int, Float);
a: Ref<Int> =
    let x = ref(C(1, 1.5));
    in  x.0;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(8, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("reference fields are unsupported for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_printf_is_unsupported_for_evaluation_of_variable_values()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type ConstantSlice;
a: Int = printf(\"%d\\n\", 2);
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(10, pos.column);
                    assert_eq!(String::from("printf is unsupported for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_operator_darrow_is_unsupported_for_evaluation_of_variable_values()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C(Int, Float);
a: T = let abc2 = abc.0 <-> fu in abc2;
abc: T = C(1, 2.5);
fu(x: Int) -> Int = x;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(19, pos.column);
                    assert_eq!(String::from("operator <-> is unsupported for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_evaluator_evaluate_values_complains_on_operator_darrow_rarrow_is_unsupported_for_evaluation_of_variable_values()
{
    let s = "
builtin type Int;
builtin type Float;
data T = C(Int, Float);
l: Float = let (x, _) = abc.0 <-> fug2 -> in x;
abc: T = C(1, 2.5);
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(25, pos.column);
                    assert_eq!(String::from("operator <-> -> is unsupported for evaluation of variable values"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
