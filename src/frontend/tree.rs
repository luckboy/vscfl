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

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum VarModifier
{
    None,
    Private,
    Local,
    Global,
    Constant,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum FunModifier
{
    None,
    Kernel,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum InlineModifier
{
    None,
    Inline,
}

#[derive(Clone, Debug)]
pub struct Tree
{
    defs: Vec<Box<Def>>,
}

impl Tree
{
    pub fn new() -> Self
    { Tree { defs: Vec::new(), } }
    
    pub fn defs(&self) -> &[Box<Def>]
    { self.defs.as_slice() }
    
    pub fn add_def(&mut self, def: Def)
    { self.defs.push(Box::new(def)); }
}

#[derive(Clone, Debug)]
pub enum Def
{
    Type(String, Rc<RefCell<TypeVar>>, Pos),
    Var(String, Rc<RefCell<Var>>, Pos),
    Trait(String, Rc<RefCell<Trait>>, Pos),
    Impl(Rc<RefCell<Impl>>, Pos),
}

#[derive(Clone, Debug)]
pub enum TypeVar
{
    Builtin,
    Data(Vec<Box<TypeArg>>, Vec<Rc<RefCell<Con>>>, Option<SharedFlag>),
    Synonim(Vec<Box<TypeArg>>, Box<TypeExpr>),
}

#[derive(Clone, Debug)]
pub struct TypeArg(pub String, pub Pos);

#[derive(Clone, Debug)]
pub enum Con
{
    UnnamedField(String, Vec<Box<TypeExpr>>, Pos),
    NamedField(String, Vec<NamedFieldPair<TypeExpr>>, Pos),
}

#[derive(Clone, Debug)]
pub struct NamedFieldPair<T>(pub String, pub Box<T>, pub Pos);

#[derive(Clone, Debug)]
pub enum TypeExpr
{
    Tuple(Vec<Box<TypeExpr>>),
    Fun(Vec<Box<TypeExpr>>, Box<TypeExpr>),
    Array(Box<TypeExpr>, usize),
    Param(String, Pos),
    Var(String, Pos),
    App(String, Vec<Box<TypeExpr>>, Pos),
    Uniq(Box<TypeExpr>, Pos),
}

#[derive(Clone, Debug)]
pub enum Var
{
    Var(VarModifier, Box<TypeExpr>, Vec<WherePair>, Option<Box<Expr>>, Option<Box<LocalTypes>>, Option<Box<Type>>),
    Fun(Box<Fun>, Option<Box<Type>>),
}

#[derive(Clone, Debug)]
pub enum Fun
{
    Builtin,
    Fun(FunModifier, InlineModifier, Vec<Arg>, Box<TypeExpr>, Vec<WherePair>, Option<Box<Expr>>, Option<LocalType>, Option<Box<LocalTypes>>),
    Con(Rc<RefCell<Con>>),
}

#[derive(Clone, Debug)]
pub struct WherePair(pub String, pub Vec<Box<TraitExpr>>, pub Pos);

#[derive(Clone, Debug)]
pub enum TraitExpr
{
    Shared(Pos),
    Fun(Vec<Box<TypeExpr>>, Box<TypeExpr>, Pos),
    Trait(String, Vec<Box<TypeExpr>>, Pos),
}

#[derive(Clone, Debug)]
pub struct Arg(pub String, pub Box<TypeExpr>, pub Option<LocalType>, Pos);

#[derive(Clone, Debug)]
pub enum Expr
{
    Literal(Box<Literal<Expr>>, Option<LocalType>, Pos),
    Lambda(Vec<LambdaArg>, Option<Box<TypeExpr>>, Box<Expr>, Option<LocalType>, Pos),
    Var(String, Option<LocalType>, Pos),
    App(Box<Expr>, Vec<Box<Expr>>, Option<LocalType>, Pos),
    NamedFieldConApp(String, Vec<NamedFieldPair<Expr>>, Option<LocalType>, Pos),
    PrintfApp(Vec<Box<Expr>>, Option<LocalType>, Pos),
    Uniq(Box<Expr>, Option<LocalType>, Pos),
    Shared(Box<Expr>, Option<LocalType>, Pos),
    Field(Box<Expr>, Vec<Field>, Option<LocalType>, Pos),
    GetField(Box<Expr>, Vec<Field>, Option<LocalType>, Pos),
    SetField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    UpdateField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    UpdateGetField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    Typed(Box<Expr>, Box<TypeExpr>, Option<LocalType>, Pos),
    As(Box<Expr>, Box<TypeExpr>, Option<LocalType>, Pos),
    Let(Vec<Bind>, Box<Expr>, Option<LocalType>, Pos),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Option<LocalType>, Pos),
    Match(Box<Expr>, Vec<Case>, Option<LocalType>, Pos),
}

#[derive(Clone, Debug)]
pub enum Field
{
    Unnamed(usize),
    Named(String),
}

#[derive(Clone, Debug)]
pub enum Bind
{
    Var(VarModifier, Option<String>, Box<Expr>, Option<LocalType>, Pos),
    Tuple(Vec<BindPair>, Box<Expr>, Pos),
}

#[derive(Clone, Debug)]
pub struct BindPair(pub VarModifier, pub Option<String>, pub Option<LocalType>);

#[derive(Clone, Debug)]
pub struct Case(pub Box<Pattern>, pub Box<Expr>);

#[derive(Clone, Debug)]
pub enum Pattern
{
    Literal(Box<Literal<Pattern>>, Option<LocalType>, Pos),
    As(Box<Literal<Pattern>>, Box<TypeExpr>, Option<LocalType>, Pos),
    Const(String, Option<LocalType>, Pos),
    UnnamedFieldCon(String, Vec<Box<Pattern>>, Option<LocalType>, Pos),
    NamedFieldCon(String, Vec<NamedFieldPair<Pattern>>, Option<LocalType>, Pos),
    Var(Option<String>, Option<LocalType>, Pos),
    At(Option<String>, Box<Pattern>, Option<LocalType>, Pos),
    Alt(Vec<Box<Pattern>>, Option<LocalType>, Pos),
}

#[derive(Clone, Debug)]
pub enum Literal<T>
{
    Bool(bool),
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
    FilledArray(Box<T>, usize),
}

#[derive(Clone, Debug)]
pub struct LambdaArg(pub String, pub Option<Box<TypeExpr>>, pub Option<LocalType>, Pos);

#[derive(Clone, Debug)]
pub struct Trait(pub String, pub Vec<TypeArg>, pub Vec<Box<Def>>, Option<Box<TraitVars>>);

#[derive(Clone, Debug)]
pub enum Impl
{
    Builtin(String, TypeName, Option<Box<ImplVars>>),
    Impl(String, TypeName, Vec<Box<ImplDef>>, Option<Box<ImplVars>>),
}

#[derive(Clone, Debug)]
pub enum TypeName
{
    Tuple(usize),
    Array(Option<usize>),
    Fun(usize),
    Name(String),
}

#[derive(Clone, Debug)]
pub struct ImplDef(pub String, pub Rc<RefCell<ImplVar>>, Pos);

#[derive(Clone, Debug)]
pub enum ImplVar
{
    Var(Box<Expr>, Option<Box<LocalTypes>>, Option<Box<Type>>),
    Fun(Box<ImplFun>, Option<Box<Type>>),
}

#[derive(Clone, Debug)]
pub enum ImplFun
{
    Builtin,
    Fun(Vec<ImplArg>, Box<Expr>, Option<LocalType>, Option<Box<LocalTypes>>),
}

#[derive(Clone, Debug)]
pub struct ImplArg(pub String, pub Option<LocalType>, pub Pos);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SharedFlag
{
    None,
    Shared,
}

#[derive(Clone, Debug)]
pub struct Type;

#[derive(Copy, Clone, Debug)]
pub struct LocalType;

#[derive(Clone, Debug)]
pub struct LocalTypes;

#[derive(Clone, Debug)]
pub struct TraitVars;

#[derive(Clone, Debug)]
pub struct ImplVars;
