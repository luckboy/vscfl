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
use crate::frontend::namer::*;
use crate::frontend::parser::*;
use crate::frontend::typer::*;
use super::*;

#[test]
fn test_limiter_check_limits_checks_limits_for_variable()
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_expressions()
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
a: (Int, Int) -> Int = |x: Int, y| x + y;
b: (Float, Float) -> Float = |x, y: Float| -> Float x - y;
c: Int = x;
d: Int = let z = 1 in z;
e: T = C { x: 1, y: 1.5, z: 2, };
f: Int = printf(\"%d\\n\", x);
g: Int = a(x, y);
h: Int = abc.0.z;
i: Int = let (z, _) = abc.0.z -> in z;
j: U = let abc2 = abc.0.z <- 1 in abc2;
k: U = let abc2 = abc.0.z <-> fu in abc2;
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_patterns()
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_expression_literals()
{
    let s = "
builtin type Int;
builtin type Float;
a: Int = 1;
b: (Int, Float) = (x, 2.0);
c: [Int; 2] = [x, 2];
d: [Int; 10] = [y; 10];
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_pattern_literals()
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
a: Int = (1, 1.5) match {
        (x, y) => x - (y as Int);
    };
b: Int = [1, 2] match {
        [x, y] => x + y;
    };
c: Int = [1; 2] match {
        [x; 2] => x;
        _ => 1;
    };
d: Int = 1 match {
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_functions()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin impl OpAdd for Int;
f(x: Int, y: Int) -> Int = x + y;
g(x: t, y: u) -> (t, u) = (x, y);
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_implemented_variable_and_implemented_function()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin type Float;
builtin impl OpAdd for Int;
trait T
{
    a: t where t: shared + T;
    f(x: t, y: t) -> t where t: T;
};
impl T for Int
{
    a = 1.5 as t;
    f(x, y) = x + y;
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_variables_with_modifiers_in_function()
{
    let s = "
trait OpAdd
{
    op_add(x: t, y: t) -> t where t: OpAdd;
};
builtin type Int;
builtin impl OpAdd for Int;
f(x: Int) -> Int =
    let private a = 2;
        local b = 3;
        global c = 4;
    in  x + a + b + c;
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_variables_with_modifiers()
{
    let s = "
builtin type Int;
global a: Int = 1;
constant b: Int = 2;
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_variables_with_modifiers_in_trait()
{
    let s = "
trait T
{
    global a: t where t: shared + T;
    constant b: t where t: shared + T;
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_kernel()
{
    let s = "
builtin type Int;
kernel f(x: Int, y: Int) -> () = ();
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
}

#[test]
fn test_limiter_check_limits_checks_limits_for_kernel_in_trait()
{
    let s = "
builtin type Int;
trait T<t1> {
    kernel f(x: t, y: u) -> () where t: T <Int>, u: T <Int>, t == u;
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
        Err(errs) => {
            println!("{}", errs);
            assert!(false)
        },
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_has_variable_modifier_for_private_modifier()
{
    let s = "
builtin type Int;
a: Int =
    let private x = 1;
    in  1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(17, pos.column);
                    assert_eq!(String::from("variable x has variable modifier"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_has_variable_modifier_for_local_modifier()
{
    let s = "
builtin type Int;
a: Int =
    let local x = 1;
    in  1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(15, pos.column);
                    assert_eq!(String::from("variable x has variable modifier"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_has_variable_modifier_for_global_modifier()
{
    let s = "
builtin type Int;
a: Int =
    let global x = 1;
    in  1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(16, pos.column);
                    assert_eq!(String::from("variable x has variable modifier"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_has_variable_modifier_for_constant_modifier()
{
    let s = "
builtin type Int;
a: Int =
    let constant x = 1;
    in  1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(18, pos.column);
                    assert_eq!(String::from("variable x has variable modifier"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_must_not_be_constant()
{
    let s = "
builtin type Int;
f() -> Int =
    let constant x = 1;
    in  1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(18, pos.column);
                    assert_eq!(String::from("variable x mustn't be constant"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_must_not_be_private()
{
    let s = "
builtin type Int;
private a: Int = 1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("variable a mustn't be private"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_must_not_be_local()
{
    let s = "
builtin type Int;
local a: Int = 1;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("variable a mustn't be local"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_must_not_be_private_for_variable_in_trait()
{
    let s = "
trait T
{
    private a: t where t: shared + T;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("variable a mustn't be private"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_must_not_be_local_for_variable_in_trait()
{
    let s = "
trait T
{
    local a: t where t: shared + T;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("variable a mustn't be local"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_must_not_have_type_parameters()
{
    let s = "
builtin type Int;
data T<t1> = C() | D(t1);
a: T<t> where t: shared = C();
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(3, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("variable a mustn't have type parameters"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_variable_must_not_have_type_parameters_without_trait()
{
    let s = "
builtin type Int;
data U<t1, t2> = C() | D(t1, t2);
trait T<t1>
{
    a: U<t, u> where t: shared + T <Int>, u: shared;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(5, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("variable a mustn't have type parameters without trait T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_kernel_must_not_have_type_parameters()
{
    let s = "
builtin type Int;
kernel f(x: Int, y: t) -> () = ();
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(2, pos.line);
                    assert_eq!(1, pos.column);
                    assert_eq!(String::from("kernel f mustn't have type parameters"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_kernel_must_not_have_type_parameters_without_trait()
{
    let s = "
builtin type Int;
trait T<t1>
{
    kernel f(x: t, y: u) -> () where t: T <Int>;
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(4, pos.line);
                    assert_eq!(5, pos.column);
                    assert_eq!(String::from("kernel f mustn't have type parameters without trait T"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_limiter_check_limits_complains_on_printf_takes_first_argument_that_must_be_literal()
{
    let s = "
builtin type Char;
builtin type Int;
builtin type ConstantSlice;
a: Int =
    let s = \"%d\\n\";
    in  printf(s, 1);
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
        Err(errs) => {
            assert_eq!(1, errs.errors().len());
            match &errs.errors()[0] {
                FrontendError::Message(pos, msg) => {
                    assert_eq!(6, pos.line);
                    assert_eq!(9, pos.column);
                    assert_eq!(String::from("printf takes first argument that must be literal"), *msg);
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}
