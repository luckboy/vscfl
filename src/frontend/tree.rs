//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::rc::*;
use crate::frontend::error::Pos;

#[derive(Clone, Debug)]
pub enum VarQualifier
{
    None,
    Private,
    Local,
    Global,
    Constant,
}

#[derive(Clone, Debug)]
pub enum FunQualifier
{
    None,
    Kernel,
}

#[derive(Clone, Debug)]
pub enum InlineQualifier
{
    None,
    Inline,
}

#[derive(Clone, Debug)]
pub enum Def
{
    Type(String, Rc<RefCell<TypeVar>>, Pos),
    Var(String, Rc<RefCell<Var>>, Pos),
    Trait(String, Rc<RefCell<Trait>>, Pos),
    Impl(String, Rc<RefCell<Impl>>, Pos),
}

#[derive(Clone, Debug)]
pub enum TypeVar
{
    Builtin,
    Data(Vec<Box<TypeArg>>, Vec<Rc<RefCell<Con>>>),
    Synonim(Vec<Box<TypeArg>>, Box<TypeExpr>),
}

#[derive(Clone, Debug)]
pub struct TypeArg(String, Pos);

#[derive(Clone, Debug)]
pub enum Con
{
    UnnamedField(Vec<Box<TypeExpr>>),
    NamedField(Vec<NamedFieldPair<TypeExpr>>),
}

#[derive(Clone, Debug)]
pub struct NamedFieldPair<T>(String, Box<T>, Pos);

#[derive(Clone, Debug)]
pub enum TypeExpr
{
    Tuple(Vec<Box<TypeExpr>>),
    Fun(Vec<Box<TypeExpr>>, Box<TypeExpr>),
    Array(Box<TypeExpr>, i32),
    Var(String, Pos),
    Uniq(Box<TypeExpr>, Pos),
    App(String, Vec<Box<TypeExpr>>, Pos),
}

#[derive(Clone, Debug)]
pub enum Var
{
    Var(VarQualifier, Box<TypeExpr>, Option<Box<Expr>>, Option<LocalTypes>, Option<Type>),
    Fun(Fun, Option<Type>),
}

#[derive(Clone, Debug)]
pub enum Fun
{
    Builtin,
    Fun(FunQualifier, InlineQualifier, Vec<Arg>, Box<TypeExpr>, Option<Box<Expr>>, Option<LocalType>, Option<LocalTypes>),
    Con(Rc<RefCell<Con>>),
}

#[derive(Clone, Debug)]
pub struct Arg(String, Box<TypeExpr>, Option<LocalType>, Pos);

#[derive(Clone, Debug)]
pub enum Expr
{
    Literal(Box<Literal<Expr>>, Option<LocalType>, Pos),
    Typed(Box<Expr>, Box<TypeExpr>, Option<LocalType>, Pos),
    Lambda(Vec<LambdaArg>, Option<Box<TypeExpr>>, Box<Expr>, Option<LocalType>, Pos),
    Var(String, Option<LocalType>, Pos),
    App(Box<Expr>, Vec<Box<Expr>>, Option<LocalType>, Pos),
    NamedFieldConApp(String, Vec<NamedFieldPair<Expr>>, Option<LocalType>, Pos),
    Field(Box<Expr>, Vec<Field>, Option<LocalType>, Pos),
    GetField(Box<Expr>, Vec<Field>, Option<LocalType>, Pos),
    SetField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    UpdateField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    UpdateGetField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    Let(Vec<Bind>, Box<Expr>, Option<LocalType>, Pos),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Option<LocalType>, Pos),
    Match(Box<Expr>, Vec<Case>, Option<LocalType>, Pos),
}

#[derive(Clone, Debug)]
pub enum Field
{
    Unnamed(i32),
    Named(String),
}

#[derive(Clone, Debug)]
pub enum Bind
{
    Var(VarQualifier, Option<String>, Box<Expr>, Option<LocalType>, Pos),
    Tuple(Vec<BindPair>, Box<Expr>, Option<LocalType>, Pos),
}

#[derive(Clone, Debug)]
pub struct BindPair(VarQualifier, Option<String>);

#[derive(Clone, Debug)]
pub struct Case(Box<Pattern>, Box<Expr>);

#[derive(Clone, Debug)]
pub enum Pattern
{
    Literal(Literal<Pattern>, Pos),
    Const(String, Pos),
    UnnamedFieldCon(String, Vec<Box<Pattern>>, Pos),
    NamedFieldCon(String, Vec<NamedFieldPair<Pattern>>, Pos),
    Var(Option<String>, Option<LocalType>, Pos),
    At(Option<String>, Box<Pattern>, Option<LocalType>, Pos),
    Alt(Vec<Box<Pattern>>, Pos),
}

#[derive(Clone, Debug)]
pub enum Literal<T>
{
    Char(u8),
    Int(i32),
    Long(i64),
    Uint(i32),
    Ulong(i64),
    Float(f32),
    Double(f64),
    String(Vec<u8>),
    Tuple(Vec<Box<T>>),
    Array(Vec<Box<T>>),
}

#[derive(Clone, Debug)]
pub struct LambdaArg(String, Option<Box<TypeExpr>>, Option<LocalType>, Pos);

#[derive(Clone, Debug)]
pub struct Trait;

#[derive(Clone, Debug)]
pub enum Impl
{
    Builtin,
    Impl,
}

#[derive(Clone, Debug)]
pub struct Type;

#[derive(Clone, Debug)]
pub struct LocalType;

#[derive(Clone, Debug)]
pub struct LocalTypes;
