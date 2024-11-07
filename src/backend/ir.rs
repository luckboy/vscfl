//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::rc::*;
use crate::frontend::error::Pos;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrGlobalVarModifier
{
    None,
    Global,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrLocalVarModifier
{
    None,
    Private,
    Local,
    Global,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrPtrModifier
{
    None,
    Private,
    Local,
    Global,
    Constant,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrFunModifier
{
    None,
    Kernel,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrAccessModifier
{
    None,
    Const,
}

#[derive(Clone, Debug)]
pub struct IrTree
{
    defs: Vec<IrDef>,
    structs: HashMap<String, Rc<RefCell<IrStruct>>>,
    unions: HashMap<String, Rc<RefCell<IrUnion>>>,
    vars: HashMap<String, Rc<RefCell<IrVar>>>,
}

impl IrTree
{
    pub fn new() -> Self
    {
        IrTree {
            defs: Vec::new(),
            structs: HashMap::new(),
            unions: HashMap::new(),
            vars: HashMap::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum IrDef
{
    Struct(String, Rc<RefCell<IrStruct>>),
    Union(String, Rc<RefCell<IrUnion>>),
    Var(String, Rc<RefCell<IrVar>>),
}

#[derive(Clone, Debug)]
pub enum IrStruct
{
    Struct(Vec<Box<IrType>>),
    Closure(BTreeMap<usize, Box<IrType>>),
}
    
#[derive(Clone, Debug)]
pub struct IrUnion(pub Vec<Box<IrType>>);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrType
{
    Void,
    Char,
    Short,
    Int,
    Long,
    Uchar,
    Ushort,
    Uint,
    Ulong,
    Float,
    Double,
    SizeT,
    PtrdiffT,
    IntptrT,
    UintptrT,
    CharN(usize),
    ShortN(usize),
    IntN(usize),
    LongN(usize),
    UcharN(usize),
    UshortN(usize),
    UintN(usize),
    UlongN(usize),
    FloatN(usize),
    DoubleN(usize),
    Struct(String),
    Union(String),
    Array(Box<IrType>, usize),
    Ptr(IrPtrModifier, Box<IrType>),
}

#[derive(Clone, Debug)]
pub enum IrVar
{
    Const(Box<IrType>, IrValue<IrValueVar>),
    Var(IrGlobalVarModifier, IrAccessModifier, Box<IrType>, IrValue<IrValueVar>),
    Fun(Box<IrFun>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum IrObject<T>
{
    String(Vec<u8>),
    BuiltinVar(String, Option<Box<IrType>>, Option<Box<IrType>>),
    Var(T, Option<Box<IrType>>),
    Vector(Vec<IrValue<T>>, Box<IrType>),
    Array(Vec<IrValue<T>>, Option<Box<IrType>>),
    Struct(Vec<IrValue<T>>, Vec<IrFieldPair<T>>, Option<Box<IrType>>),
    Union(usize, IrValue<T>, Option<Box<IrType>>),
    Closure(Vec<IrFieldPair<T>>, Option<Box<IrType>>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum IrValue<T>
{
    Char(i8, Option<Box<IrType>>),
    Short(i16, Option<Box<IrType>>),
    Int(i32, Option<Box<IrType>>),
    Long(i64, Option<Box<IrType>>),
    Uchar(u8, Option<Box<IrType>>),
    Ushort(u16, Option<Box<IrType>>),
    Uint(u32, Option<Box<IrType>>),
    Ulong(u64, Option<Box<IrType>>),
    Float(f32, Option<Box<IrType>>),
    Double(f64, Option<Box<IrType>>),
    SizeT(u64, Option<Box<IrType>>),
    PtrdiffT(i64, Option<Box<IrType>>),
    IntptrT(i64, Option<Box<IrType>>),
    UintptrT(u64, Option<Box<IrType>>),
    Object(Box<IrObject<T>>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct IrFieldPair<T>(pub usize, pub IrValue<T>);

#[derive(Clone, PartialEq, Debug)]
pub struct IrValueVar(pub String, pub Vec<IrValueOp>);

#[derive(Clone, PartialEq, Debug)]
pub enum IrValueOp
{
    Deref(Option<Box<IrType>>),
    Dot(Option<Box<IrType>>, usize),
    Arrow(Option<Box<IrType>>, usize),
    Index(Option<Box<IrType>>, u64),
}

#[derive(Clone, PartialEq, Debug)]
pub enum IrArgVar
{
    Global(String, Vec<IrArgOp>),
    Local(usize, Vec<IrArgOp>),
    CallerFunArg(usize, Vec<IrArgOp>),
    PrivateClosure(usize, Vec<IrArgOp>),
    LocalClosure(usize, Vec<IrArgOp>),
    GlobalClosure(usize, Vec<IrArgOp>),
    RefGlobal(String, Vec<IrArgOp>),
    RefLocal(usize, Vec<IrArgOp>),
    RefCallerFunArg(usize, Vec<IrArgOp>),
    RefPrivateClosure(usize, Vec<IrArgOp>),
    RefLocalClosure(usize, Vec<IrArgOp>),
    RefGlobalClosure(usize, Vec<IrArgOp>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum IrArgOp
{
    Deref(Option<Box<IrType>>),
    Dot(Option<Box<IrType>>, usize),
    Arrow(Option<Box<IrType>>, usize),
    Index(Option<Box<IrType>>, u64),
    GlobalIndex(Option<Box<IrType>>, String),
    LocalIndex(Option<Box<IrType>>, usize),
    CallerFunArgIndex(Option<Box<IrType>>, usize),
    PrivateClosureIndex(Option<Box<IrType>>, usize),
    LocalClosureIndex(Option<Box<IrType>>, usize),
    GlobalClosureIndex(Option<Box<IrType>>, usize),
}

#[derive(Clone, Debug)]
pub enum IrFun
{
    Fun(IrFunModifier, Vec<Box<IrType>>, Box<IrType>, Box<IrBlock>, IrPrivateHeapFlag, IrLocalHeapFlag, IrGlobalHeapFlag, IrPanicFlag),
    Caller(Vec<Box<IrType>>, Box<IrType>, BTreeMap<usize, IrCallerFun>, IrPrivateHeapFlag, IrLocalHeapFlag, IrGlobalHeapFlag, IrPanicFlag),
}

#[derive(Clone, Debug)]
pub struct IrCallerFun(pub Option<Box<IrType>>, pub Option<Box<IrType>>, pub Option<Box<IrType>>, pub usize, pub Box<IrBlock>);

#[derive(Clone, Debug)]
pub struct IrBlock(pub Vec<IrLocalVarPair>, Vec<IrInstr>);

#[derive(Clone, Debug)]
pub struct IrLocalVarPair(pub IrLocalVarModifier, pub Box<IrType>);

#[derive(Clone, Debug)]
pub enum IrInstr
{
    Op(IrOp),
    Assign(IrInstrVar, IrOp),
    Return(Option<IrOp>),
    Break,
    Continue,
    Block(Box<IrBlock>),
    If(IrOp, Box<IrBlock>, Box<IrBlock>),
    Switch(IrOp, Vec<IrCase>),
    Loop(Box<IrBlock>),
    Panic(String, Vec<Pos>),
}

#[derive(Clone, Debug)]
pub enum IrOp
{
    Load(IrValue<IrArgVar>),
    Neg(IrValue<IrArgVar>),
    Not(IrValue<IrArgVar>),
    Mul(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Div(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Rem(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Add(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Sub(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Shl(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Shr(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Eq(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Ne(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Lt(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Ge(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Gt(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Le(IrValue<IrArgVar>, IrValue<IrArgVar>),
    And(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Xor(IrValue<IrArgVar>, IrValue<IrArgVar>),
    Or(IrValue<IrArgVar>, IrValue<IrArgVar>),
    CallBuiltinFun(String, Vec<IrValue<IrArgVar>>),
    CallFun(String, Option<Box<IrType>>, Vec<IrValue<IrArgVar>>, Pos),
    CallFunWithoutPanic(String, Vec<IrValue<IrArgVar>>, Pos),
}

#[derive(Clone, Debug)]
pub enum IrInstrVar
{
    Global(String, Vec<IrArgOp>),
    Local(usize, Vec<IrArgOp>),
    CallerFunArg(usize, Vec<IrArgOp>),
    PrivateClosure(usize, Vec<IrArgOp>),
    LocalClosure(usize, Vec<IrArgOp>),
    GlobalClosure(usize, Vec<IrArgOp>),
}

#[derive(Clone, Debug)]
pub struct IrCase(pub IrCaseValue, pub Box<IrBlock>);

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrCaseValue
{
    Char(i8, Option<Box<IrType>>),
    Short(i16, Option<Box<IrType>>),
    Int(i32, Option<Box<IrType>>),
    Long(i64, Option<Box<IrType>>),
    Uchar(u8, Option<Box<IrType>>),
    Ushort(u16, Option<Box<IrType>>),
    Uint(u32, Option<Box<IrType>>),
    Ulong(u64, Option<Box<IrType>>),
    SizeT(u64, Option<Box<IrType>>),
    PtrdiffT(i64, Option<Box<IrType>>),
    IntptrT(i64, Option<Box<IrType>>),
    UintptrT(u64, Option<Box<IrType>>),
    BuiltinVar(String, Option<Box<IrType>>, Option<Box<IrType>>),
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrPrivateHeapFlag
{
    None,
    Heap,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrLocalHeapFlag
{
    None,
    Heap,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrGlobalHeapFlag
{
    None,
    Heap,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrPanicFlag
{
    None,
    Panic,
}
