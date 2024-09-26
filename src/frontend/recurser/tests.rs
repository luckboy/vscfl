//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::*;
use crate::frontend::evaluator::*;
use crate::frontend::instancer::*;
use crate::frontend::lexer::*;
use crate::frontend::limiter::*;
use crate::frontend::namer::*;
use crate::frontend::parser::*;
use crate::frontend::typer::*;
use super::*;

#[test]
fn test_recurser_check_recursions_checks_recursion_for_function()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin impl OpAdd for Int;
f(x: Int, y: Int) -> Int = x + y;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursions_for_expressions()
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
builtin type Float;
builtin type ConstantSlice;
builtin impl OpAdd for Int;
builtin impl OpSub for Int;
builtin impl OpSub for Float;
data T = C { x: Int, y: Float, z: Int, }; 
data U = D(T);
f() -> (Int, Int) -> Int = |x: Int, y| x + y;
g() -> (Float, Float) -> Float = |x, y: Float| -> Float x - y;
h() -> Int = x;
i() -> Int = let z = 1 in z;
j() -> T = C { x: 1, y: 1.5, z: 2, };
k() -> Int = printf(\"%d\\n\", x);
l() -> Int = f()(x, y);
m() -> Int = abc.0.z;
n() -> Int = let (z, _) = abc.0.z -> in z;
o() -> U = let abc2 = abc.0.z <- 1 in abc2;
p() -> U = let abc2 = abc.0.z <-> fu in abc2;
q() -> Float = let (z, _) = abc.0.z <-> fug2 -> in z;
r() -> uniq Int = uniq x;
s() -> Int = shared m();
t(x: t) -> (t, Float) = (x, 1.5): (t, Float);
u() -> Float = x as Float;
v() -> Int = if true then x else y;
w() -> Int =
    let z1 = 1;
        z2 = 2;
    in  z1 + z2;
z() -> Int = C { x: 1, y: 2.5, z: 3, } match {
        C { z: z, x: x, y: _, } => z - x;
    };
x: Int = 1;
y: Int = 2;
abc: U = D(C { x: 2, y: 2.5, z: 3, });
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursions_for_patterns()
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
f() -> Int = 1 match {
        X => 1;
        _ => 2;
    };
g() -> Int = C(1, 2.5) match {
        C(x, y) => x + (y as Int);
        D { y: _, z: y, x: x, } => x + y;
    };
h() -> Int = 2 match { x => x; };
i() -> T = C(1, 2.5) match {
        x @ D { x: 1, y: 2.5, z: 2, } => x;
        _ => C(1, 2.5);
    };
j() -> Int = C(1, 2.5) match {
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursions_for_expression_literals()
{
    let s = "
builtin type Int;
builtin type Float;
f() -> Int = 1;
g() -> (Int, Float) = (x, 2.0);
h() -> [Int; 2] = [x, 2];
i() -> [Int; 10] = [y; 10];
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursions_for_pattern_literals()
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
builtin type Int;
builtin type Float;
builtin impl OpAdd for Int;
builtin impl OpSub for Int;
f() -> Int = (1, 1.5) match {
        (x, y) => x - (y as Int);
    };
g() -> Int = [1, 2] match {
        [x, y] => x + y;
    };
h() -> Int = [1; 2] match {
        [x; 2] => x;
        _ => 1;
    };
i() -> Int = 1 match {
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursions_for_implemented_functions()
{
    let s = "
trait T
{
    f() -> t where t: T;
};
builtin type Int;
data U = C(Int);
data V = D(U);
impl T for V
{
    f() = D(f());
};
impl T for U
{
    f() = C(f());
};
impl T for Int
{
    f() = 1;
};
g() -> V = f();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursions_for_implemented_functions_with_default_value()
{
    let s = "
data T<t1> = C() | D(t1);
trait U
{
    f() -> T<t> where t: U = C();
};
builtin type Int;
builtin type Float;
impl U for Int
{
    f() = D(1);
};
impl U for Float {};
g() -> T<Int> = f();
h() -> T<Float> = f();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursion_for_tail_recursive_function()
{
    let s = "
builtin type Int;
f() -> Int = f();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursion_for_tail_recursive_function_and_let_clause()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin impl OpAdd for Int; 
f(x: Int) -> Int =
    let y = x + 1;
    in  f(y);
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursion_for_tail_recursive_function_and_if_clause()
{
    let s = "
trait OpMul
{
    op_mul(x: t, y: t) -> t where t: OpMul;
};
trait OpSub
{
    op_sub(x: t, y: t) -> t where t: OpSub;
};
trait Ord
{
    op_le(x: t, y: t) -> Bool where t: Ord;
};
builtin type Bool;
builtin type Int;
builtin impl OpMul for Int;
builtin impl OpSub for Int;
builtin impl Ord for Int;
f(x: Int, y: Int) -> Int = if x <= 0 then y else f(x - 1, x * y);
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursion_for_tail_recursive_function_and_match_clause()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin impl OpAdd for Int;
data T = C() | D() | E();
f(x: T, y: Int) -> Int =
    x match {
        C() => f(D(), y + 1);
        D() => f(E(), y + 1);
        E() => y;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursion_for_tail_recursive_function_and_typed_expression()
{
    let s = "
builtin type Int;
f() -> Int = f(): Int;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursion_for_tail_recursive_function_and_nested_clause()
{
    let s = "
trait OpMul
{
    op_mul(x: t, y: t) -> t where t: OpMul;
};
trait OpSub
{
    op_sub(x: t, y: t) -> t where t: OpSub;
};
trait Ord
{
    op_le(x: t, y: t) -> Bool where t: Ord;
};
builtin type Bool;
builtin type Int;
builtin impl OpMul for Int;
builtin impl OpSub for Int;
builtin impl Ord for Int;
f(x: Int, y: Int) -> Int =
    if x <= 0 then
        y
    else
        let y2 = x * y;
        in  f(x - 1, y2);
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_checks_recursion_for_implemented_recursive_function()
{
    let s = "
trait T
{
    f() -> t where t: T;
};
builtin type Int;
impl T for Int
{
    f() = f();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin impl OpAdd for Int; 
f() -> Int = f() + 1;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(7, pos.line);
                    assert_eq!(14, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_two_recursive_functions()
{
    let s = "
builtin type Int;
f() -> Int = g();
g() -> Int = f();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(14, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_printf_application()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type ConstantSlice;
f() -> Int = printf(\"%d\\n\", f());
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(29, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_application()
{
    let s = "
builtin type Int;
f() -> Int = g(f());
g(x: Int) -> Int = x;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(16, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_uniq_operator()
{
    let s = "
builtin type Int;
f() -> uniq Int = uniq f();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(24, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_shared_operator()
{
    let s = "
builtin type Int;
f() -> Int = shared f();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(21, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_casting_operator()
{
    let s = "
builtin type Int;
f() -> Int = f() as Int;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(14, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_if_clause()
{
    let s = "
builtin type Bool;
f() -> Bool = if f() then false else true;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(18, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_let_clause()
{
    let s = "
builtin type Int;
f() -> Int = let x = f() in x;
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(22, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_recurser_check_recursions_complains_on_recursive_function_can_use_only_tail_recursion_for_match_clause()
{
    let s = "
data T = C() | D() | E();
f() -> T =
    f() match {
        C() => D();
        D() => E();
        E() => C();
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
    let recurser = Recurser::new();
    match recurser.check_recursions(&tree) {
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("recursive function f can use only tail recursion"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
