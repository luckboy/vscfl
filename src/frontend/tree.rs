//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::cell::*;
use std::fmt;
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
    Inline,
}

#[derive(Clone, Debug)]
pub struct Tree
{
    pub(crate) defs: Vec<Box<Def>>,
    pub(crate) type_vars: HashMap<String, Rc<RefCell<TypeVar>>>,
    pub(crate) vars: HashMap<String, Rc<RefCell<Var>>>,
    pub(crate) traits: HashMap<String, Rc<RefCell<Trait>>>,
}

impl Tree
{
    pub fn new() -> Self
    {
        Tree {
            defs: Vec::new(),
            type_vars: HashMap::new(),
            vars: HashMap::new(),
            traits: HashMap::new(),
        }
    }
    
    pub fn defs(&self) -> &[Box<Def>]
    { self.defs.as_slice() }
    
    pub fn type_vars(&self) -> &HashMap<String, Rc<RefCell<TypeVar>>>
    { &self.type_vars }
    
    pub fn type_var(&self, ident: &String) -> Option<&Rc<RefCell<TypeVar>>>
    { self.type_vars.get(ident) }
    
    pub fn vars(&self) -> &HashMap<String, Rc<RefCell<Var>>>
    { &self.vars }

    pub fn var(&self, ident: &String) -> Option<&Rc<RefCell<Var>>>
    { self.vars.get(ident) }
    
    pub fn traits(&self) -> &HashMap<String, Rc<RefCell<Trait>>>
    { &self.traits }
    
    pub fn trait1(&self, ident: &String) -> Option<&Rc<RefCell<Trait>>>
    { self.traits.get(ident) }
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
    Builtin(Option<TypeArgs>, Option<SharedFlag>),
    Data(Vec<TypeArg>, Vec<Rc<RefCell<Con>>>, Option<SharedFlag>),
    Synonym(Vec<TypeArg>, Box<TypeExpr>, Option<TypeValue>),
}

#[derive(Clone, Debug)]
pub struct TypeArg(pub String, pub Pos);

#[derive(Clone, Debug)]
pub enum Con
{
    UnnamedField(String, Vec<Box<TypeExpr>>, String, Pos),
    NamedField(String, Vec<NamedFieldPair<TypeExpr>>, String, Option<Box<NamedFields>>, Pos),
}

#[derive(Clone, Debug)]
pub struct NamedFieldPair<T>(pub String, pub Box<T>, pub Pos);

#[derive(Clone, Debug)]
pub enum TypeExpr
{
    Tuple(Vec<Box<TypeExpr>>, Pos),
    Fun(Vec<Box<TypeExpr>>, Box<TypeExpr>, Pos),
    Array(Box<TypeExpr>, Option<usize>, Pos),
    Param(String, Pos),
    Var(String, Pos),
    App(String, Vec<Box<TypeExpr>>, Pos),
    Uniq(Box<TypeExpr>, Pos),
}

#[derive(Clone, Debug)]
pub enum Var
{
    Builtin(Option<String>, Option<Box<Type>>),
    Var(VarModifier, Box<TypeExpr>, Vec<WhereTuple>, Option<Box<Expr>>, Option<String>, Option<Box<LocalTypes>>, Option<Box<Type>>, Option<Value>),
    Fun(Box<Fun>, Option<String>, Option<Box<Type>>),
}

#[derive(Clone, Debug)]
pub enum Fun
{
    Fun(FunModifier, Vec<Arg>, Box<TypeExpr>, Vec<WhereTuple>, Option<Box<Expr>>, Option<LocalType>, Option<Box<LocalTypes>>),
    Con(Rc<RefCell<Con>>),
}

#[derive(Clone, Debug)]
pub struct Arg(pub String, pub Box<TypeExpr>, pub Option<LocalType>, pub Pos);

#[derive(Clone, Debug)]
pub enum WhereTuple
{
    Traits(String, Vec<TraitName>, Vec<Box<TypeExpr>>, Pos),
    Eq(Vec<TypeParam>),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TraitName
{
    Shared,
    Fun,
    Name(String),
}

#[derive(Clone, Debug)]
pub struct TypeParam(pub String, pub Pos);

#[derive(Clone, Debug)]
pub enum Expr
{
    Literal(Box<Literal<Expr>>, Option<LocalType>, Pos),
    Lambda(Vec<LambdaArg>, Option<Box<TypeExpr>>, Box<Expr>, Option<LocalType>, Pos),
    Var(String, Option<LocalType>, Pos),
    NamedFieldConApp(String, Vec<NamedFieldPair<Expr>>, Option<LocalType>, Pos),
    PrintfApp(Vec<Box<Expr>>, Option<LocalType>, Pos),
    App(Box<Expr>, Vec<Box<Expr>>, Option<LocalType>, Pos),
    GetField(Box<Expr>, Vec<Field>, Option<LocalType>, Pos),
    Get2Field(Box<Expr>, Vec<Field>, Option<LocalType>, Pos),
    SetField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    UpdateField(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    UpdateGet2Field(Box<Expr>, Vec<Field>, Box<Expr>, Option<LocalType>, Pos),
    Uniq(Box<Expr>, Option<LocalType>, Pos),
    Shared(Box<Expr>, Option<LocalType>, Pos),
    Typed(Box<Expr>, Box<TypeExpr>, Option<LocalType>, Pos),
    As(Box<Expr>, Box<TypeExpr>, Option<LocalType>, Pos),
    Let(Vec<Bind>, Box<Expr>, Option<LocalType>, Pos),
    If(Box<Expr>, Box<Expr>, Box<Expr>, Option<LocalType>, Pos),
    Match(Box<Expr>, Vec<Case>, Option<LocalType>, Pos),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Field
{
    Unnamed(usize),
    Named(String),
}

#[derive(Clone, Debug)]
pub struct Bind(pub Box<Pattern>, pub Box<Expr>);

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
    Var(VarModifier, String, Option<LocalType>, Pos),
    At(VarModifier, String, Box<Pattern>, Option<LocalType>, Pos),
    Wildcard(Option<LocalType>, Pos),
    Alt(Vec<Box<Pattern>>, Option<LocalType>, Pos),
}

#[derive(Clone, Debug)]
pub enum Literal<T>
{
    Bool(bool),
    Char(i8),
    Int(i32),
    Long(i64),
    Uint(u32),
    Ulong(u64),
    Float(f32),
    Double(f64),
    String(Vec<u8>),
    Tuple(Vec<Box<T>>),
    Array(Vec<Box<T>>),
    FilledArray(Box<T>, usize),
}

#[derive(Clone, Debug)]
pub struct LambdaArg(pub String, pub Option<Box<TypeExpr>>, pub Option<LocalType>, pub Pos);

#[derive(Clone, Debug)]
pub struct Trait(pub Vec<TypeArg>, pub Vec<Box<TraitDef>>, pub Option<Box<TraitVars>>);

#[derive(Clone, Debug)]
pub struct TraitDef(pub String, pub Rc<RefCell<Var>>, pub Pos);

#[derive(Clone, Debug)]
pub enum Impl
{
    Builtin(String, TypeName, Option<Box<ImplVars>>),
    Impl(String, TypeName, Vec<Box<ImplDef>>, Option<Box<ImplVars>>),
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TypeName
{
    Tuple(usize),
    Array(Option<usize>),
    Fun(usize),
    Name(String),
}

impl fmt::Display for TypeName
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            TypeName::Tuple(count) => {
                write!(f, "(")?;
                let mut is_first = true;
                for _ in 0..*count {
                    if !is_first {
                        write!(f, ", _")?;
                    } else {
                        write!(f, "_")?;
                    }
                    is_first = false;
                }
                write!(f, ")")
            },
            TypeName::Fun(count) => {
                write!(f, "(")?;
                let mut is_first = true;
                for _ in 0..*count {
                    if !is_first {
                        write!(f, ", _")?;
                    } else {
                        write!(f, "_")?;
                    }
                    is_first = false;
                }
                write!(f, ") -> _")
            },
            TypeName::Array(Some(len)) => write!(f, "[_; {}]", len),
            TypeName::Array(None) => write!(f, "[_; _]"),
            TypeName::Name(ident) => write!(f, "{}", ident),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImplDef(pub String, pub Rc<RefCell<ImplVar>>, pub Pos);

#[derive(Clone, Debug)]
pub enum ImplVar
{
    Builtin(Option<Box<Type>>),
    Var(Box<Expr>, Option<Box<LocalTypes>>, Option<Box<Type>>, Option<Value>),
    Fun(Box<ImplFun>, Option<Box<Type>>),
}

#[derive(Clone, Debug)]
pub struct ImplFun(pub Vec<ImplArg>, pub Box<Expr>, pub Option<LocalType>, pub Option<Box<LocalTypes>>);

#[derive(Clone, Debug)]
pub struct ImplArg(pub String, pub Option<LocalType>, pub Pos);

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SharedFlag
{
    None,
    Shared,
}

#[derive(Clone, Debug)]
pub struct TypeArgs;

#[derive(Clone, Debug)]
pub struct NamedFields
{
    pub(crate) field_indices: BTreeMap<String, usize>,
}

impl NamedFields
{
    pub fn new() -> Self
    { NamedFields { field_indices: BTreeMap::new(), } }
    
    pub fn field_indices(&self) -> &BTreeMap<String, usize>
    { &self.field_indices }
    
    pub fn field_index(&self, ident: &String) -> Option<usize>
    {
       match self.field_indices.get(ident) {
           Some(i) => Some(*i),
           None => None,
       }
    }
}

#[derive(Clone, Debug)]
pub struct TypeValue;

#[derive(Clone, Debug)]
pub struct Type;

#[derive(Copy, Clone, Debug)]
pub struct LocalType;

#[derive(Clone, Debug)]
pub struct LocalTypes;

#[derive(Clone, Debug)]
pub struct Value;

#[derive(Clone, Debug)]
pub struct TraitVars
{
    pub(crate) impls: BTreeMap<TypeName, Rc<RefCell<Impl>>>,
    pub(crate) vars: BTreeMap<String, Rc<RefCell<Var>>>,
}

impl TraitVars
{
    pub fn new() -> Self
    { TraitVars { impls: BTreeMap::new(), vars: BTreeMap::new(), } }

    pub fn impls(&self) -> &BTreeMap<TypeName, Rc<RefCell<Impl>>>
    { &self.impls }
    
    pub fn impl1(&self, type_name: &TypeName) -> Option<&Rc<RefCell<Impl>>>
    { self.impls.get(type_name) }
    
    pub fn vars(&self) -> &BTreeMap<String, Rc<RefCell<Var>>>
    { &self.vars }
    
    pub fn var(&self, ident: &String) -> Option<&Rc<RefCell<Var>>>
    { self.vars.get(ident) }
}

#[derive(Clone, Debug)]
pub struct ImplVars
{
    pub(crate) vars: BTreeMap<String, Rc<RefCell<ImplVar>>>,
}

impl ImplVars
{
    pub fn new() -> Self
    { ImplVars { vars: BTreeMap::new(), } }

    pub fn vars(&self) -> &BTreeMap<String, Rc<RefCell<ImplVar>>>
    { &self.vars }
    
    pub fn var(&self, ident: &String) -> Option<&Rc<RefCell<ImplVar>>>
    { self.vars.get(ident) }
}
