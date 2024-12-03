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
    Inline,
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
    defs: Vec<Box<IrDef>>,
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

    pub fn defs(&self) -> &[Box<IrDef>]
    { self.defs.as_slice() }
    
    pub fn add_def(&mut self, def: IrDef)
    { self.defs.push(Box::new(def)); }

    pub fn structs(&self) -> &HashMap<String, Rc<RefCell<IrStruct>>>
    { &self.structs }
    
    pub fn struct1(&self, ident: &String) -> Option<&Rc<RefCell<IrStruct>>>
    { self.structs.get(ident) }

    pub fn add_struct(&mut self, ident: String, struct1: Rc<RefCell<IrStruct>>)
    { self.structs.insert(ident, struct1); }

    pub fn unions(&self) -> &HashMap<String, Rc<RefCell<IrUnion>>>
    { &self.unions }
    
    pub fn union(&self, ident: &String) -> Option<&Rc<RefCell<IrUnion>>>
    { self.unions.get(ident) }

    pub fn add_union(&mut self, ident: String, union: Rc<RefCell<IrUnion>>)
    { self.unions.insert(ident, union); }

    pub fn vars(&self) -> &HashMap<String, Rc<RefCell<IrVar>>>
    { &self.vars }
    
    pub fn var(&self, ident: &String) -> Option<&Rc<RefCell<IrVar>>>
    { self.vars.get(ident) }

    pub fn add_var(&mut self, ident: String, var: Rc<RefCell<IrVar>>)
    { self.vars.insert(ident, var); }
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
    Sizeof(Box<IrType>, Option<Box<IrType>>),
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
    PrivateHeap(Vec<IrArgOp>),
    LocalHeap(Vec<IrArgOp>),
    GlobalHeap(Vec<IrArgOp>),
    RefGlobal(String, Vec<IrArgOp>, Option<Box<IrType>>),
    RefLocal(usize, Vec<IrArgOp>, Option<Box<IrType>>),
    RefCallerFunArg(usize, Vec<IrArgOp>, Option<Box<IrType>>),
    RefPrivateClosure(usize, Vec<IrArgOp>, Option<Box<IrType>>),
    RefLocalClosure(usize, Vec<IrArgOp>, Option<Box<IrType>>),
    RefGlobalClosure(usize, Vec<IrArgOp>, Option<Box<IrType>>),
    RefPrivateHeap(Vec<IrArgOp>, Option<Box<IrType>>),
    RefLocalHeap(Vec<IrArgOp>, Option<Box<IrType>>),
    RefGlobalHeap(Vec<IrArgOp>, Option<Box<IrType>>),
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
    Caller(Vec<Box<IrType>>, Box<IrType>, Box<IrCallerFuns>, IrPrivateHeapFlag, IrLocalHeapFlag, IrGlobalHeapFlag, IrPanicFlag),
}

#[derive(Clone, Debug)]
pub struct IrCallerFuns
{
    funs: BTreeMap<usize, IrCallerFun>,
    op_pairs: BTreeMap<IrCallerFunOp, (usize, usize)>,
    builtin_fun_pairs: BTreeMap<String, (usize, usize)>,
    fun_pairs: BTreeMap<String, (usize, usize)>,
    index_counter: usize,
}

impl IrCallerFuns
{
    pub fn new() -> Self
    {
        IrCallerFuns {
            funs: BTreeMap::new(),
            op_pairs: BTreeMap::new(),
            builtin_fun_pairs: BTreeMap::new(),
            fun_pairs: BTreeMap::new(),
            index_counter: 1,
        }
    }
    
    pub fn funs(&self) -> &BTreeMap<usize, IrCallerFun>
    { &self.funs }

    pub fn fun(&self, idx: usize) -> Option<&IrCallerFun>
    { self.funs.get(&idx) }

    pub fn add_fun(&mut self, fun: IrCallerFun) -> usize
    {
        match &fun {
            IrCallerFun::Op(op) => {
                match self.op_pairs.get_mut(op) {
                    Some((idx, ref_count)) => {
                        *ref_count += 1;
                        return *idx;
                    },
                    None => {
                        self.op_pairs.insert(*op, (self.index_counter, 1));
                    },
                }
            },
            IrCallerFun::BuiltinFun(ident) =>  {
                match self.builtin_fun_pairs.get_mut(ident) {
                    Some((idx, ref_count)) => {
                        *ref_count += 1;
                        return *idx;
                    },
                    None => {
                        self.builtin_fun_pairs.insert(ident.clone(), (self.index_counter, 1));
                    },
                }
            },
            IrCallerFun::Fun(ident) => {
                match self.fun_pairs.get_mut(ident) {
                    Some((idx, ref_count)) => {
                        *ref_count += 1;
                        return *idx;
                    },
                    None => {
                        self.fun_pairs.insert(ident.clone(), (self.index_counter, 1));
                    },
                }
            },
            _ => (),
        }
        let new_idx = self.index_counter;
        self.funs.insert(new_idx, fun);
        self.index_counter += 1;
        new_idx
    }

    pub fn remove_fun(&mut self, idx: usize) -> bool
    {
        match self.funs.get(&idx) {
            Some(IrCallerFun::Op(op)) => {
                match self.op_pairs.get_mut(op) {
                    Some((_, ref_count)) => {
                        *ref_count -= 1;
                        if *ref_count > 0 {
                            return false;
                        }
                        self.op_pairs.remove(op);
                    },
                    None => (),
                }
            },
            Some(IrCallerFun::BuiltinFun(ident)) =>  {
                match self.builtin_fun_pairs.get_mut(ident) {
                    Some((_, ref_count)) => {
                        *ref_count -= 1;
                        if *ref_count > 0 {
                            return false;
                        }
                        self.builtin_fun_pairs.remove(ident);
                    },
                    None => (),
                }
            },
            Some(IrCallerFun::Fun(ident)) => {
                match self.fun_pairs.get_mut(ident) {
                    Some((_, ref_count)) => {
                        *ref_count -= 1;
                        if *ref_count > 0 {
                            return false;
                        }
                        self.fun_pairs.remove(ident);
                    },
                    None => (),
                }
            },
            Some(_) => (),
            None => return false,
        }
        self.funs.remove(&idx).is_some()
    }
}

#[derive(Clone, Debug)]
pub enum IrCallerFun
{
    Op(IrCallerFunOp),
    BuiltinFun(String),
    Fun(String),
    Lambda(Option<Box<IrType>>, Option<Box<IrType>>, Option<Box<IrType>>, usize, Box<IrBlock>)
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum IrCallerFunOp
{
    Neg,
    Not,
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Shl,
    Shr,
    Eq,
    Ne,
    Lt,
    Ge,
    Gt,
    Le,
    And,
    Xor,
    Or,
}

#[derive(Clone, Debug)]
pub struct IrBlock
{
    local_var_pairs: Vec<IrLocalVarPair>,
    instrs: Vec<IrInstr>,
    block_count: usize,
}

impl IrBlock
{
    pub fn new() -> Self
    { IrBlock { local_var_pairs: Vec::new(), instrs: Vec::new(), block_count: 0, } }
    
    pub fn local_var_pairs(&self) -> &[IrLocalVarPair]
    { self.local_var_pairs.as_slice() }
    
    pub fn add_local_var_pair(&mut self, local_var_pair: IrLocalVarPair)
    { self.local_var_pairs.push(local_var_pair); }

    pub fn instrs(&self) -> &[IrInstr]
    { self.instrs.as_slice() }
    
    pub fn add_instr(&mut self, instr: IrInstr)
    {
        self.block_count += instr.block_count();
        self.instrs.push(instr);
    }

    pub fn block_count(&self) -> usize
    { self.block_count }
}

#[derive(Clone, Debug)]
pub struct IrLocalVarPair(pub IrLocalVarModifier, pub Box<IrType>);

#[derive(Clone, Debug)]
pub enum IrInstr
{
    Op(IrOp),
    Assign(Box<IrInstrVar>, IrOp),
    Return(Option<IrOp>),
    Break,
    Continue,
    Block(Box<IrBlock>),
    If(IrOp, Box<IrBlock>, Box<IrBlock>),
    Switch(IrOp, Vec<IrCase>),
    Loop(Box<IrBlock>),
    Panic(String, Pos, Vec<Pos>, Option<IrValue<IrArgVar>>),
}

impl IrInstr
{
    pub fn block_count(&self) -> usize
    {
        match self {
            IrInstr::Block(block) => block.block_count() + 1,
            IrInstr::If(_, block1, block2) => block1.block_count() + block2.block_count() + 2,
            IrInstr::Switch(_, cases) => cases.iter().fold(0usize, |n, c| n + c.1.block_count() + 1),
            IrInstr::Loop(block) => block.block_count() + 1,
            _ => 0,
        }
    }
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
    CallBuiltinFun(String, Option<Box<IrType>>, Vec<IrValue<IrArgVar>>),
    CallFun(String, Vec<IrValue<IrArgVar>>, Pos, Vec<Pos>, Option<IrValue<IrArgVar>>),
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
