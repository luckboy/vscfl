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
use std::error;
use std::fmt;
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
    CallerFunIndex(String, usize, Option<Box<IrType>>),
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
    Caller(Box<IrType>, Vec<Box<IrType>>, Box<IrType>, Box<IrCallerFuns>, IrPrivateHeapFlag, IrLocalHeapFlag, IrGlobalHeapFlag, IrPanicFlag),
}

#[derive(Clone, Debug)]
pub struct IrCallerFuns
{
    funs: BTreeMap<usize, IrCallerFun>,
    fun_pairs: BTreeMap<String, (usize, usize)>,
    index_counter: usize,
}

impl IrCallerFuns
{
    pub fn new() -> Self
    {
        IrCallerFuns {
            funs: BTreeMap::new(),
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
            IrCallerFun::InlineFun(ident, _) => {
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
            Some(IrCallerFun::InlineFun(ident, _)) => {
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
    Fun(String),
    InlineFun(String, Box<IrBlock>),
    Lambda(Option<Box<IrType>>, Option<Box<IrType>>, Option<Box<IrType>>, usize, Box<IrBlock>)
}

#[derive(Clone, Debug)]
pub struct VarSubstitution
{
    arg_substitutions: Vec<ArgSubstitution>,
    has_var: bool,
}

impl VarSubstitution
{
    pub fn new(arg_substitutions: Vec<ArgSubstitution>, has_var: bool) -> Self
    { VarSubstitution { arg_substitutions, has_var, } }

    pub fn arg_substitutions(&self) -> &[ArgSubstitution]
    { self.arg_substitutions.as_slice() }
    
    pub fn has_var(&self) -> bool
    { self.has_var }
}

#[derive(Clone, Debug)]
pub enum ArgSubstitution
{
    Value(IrValue<IrArgVar>),
    Fun(String),
    Lambda(usize, Vec<Box<IrType>>, Box<IrType>, Box<IrBlock>),
}

#[derive(Clone, Debug)]
struct VarTuple
{
    typ: Box<IrType>,
    old_block_index: Option<usize>,
    assign_index: Option<usize>,
    value: Option<IrValue<IrArgVar>>,
}

impl VarTuple
{
    fn new(typ: Box<IrType>, old_block_idx: Option<usize>) -> Self
    {
        VarTuple {
            typ,
            old_block_index: old_block_idx,
            assign_index: None,
            value: None,
        }
    }

    fn new_with_value(typ: Box<IrType>, old_block_idx: Option<usize>, value: IrValue<IrArgVar>) -> Self
    {
        VarTuple {
            typ,
            old_block_index: old_block_idx,
            assign_index: None,
            value: Some(value),
        }
    }
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

    pub fn add_block(&mut self, block: IrBlock)
    {
        if !block.local_var_pairs.is_empty() {
            self.add_instr(IrInstr::Block(Box::new(block)));
        } else {
            for instr in &block.instrs {
                self.add_instr(instr.clone());
            }
        }
    }
    
    pub fn block_count(&self) -> usize
    { self.block_count }

    fn var_arg_substitution_tuple(&self, var_idx: usize, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>) -> Result<(Option<ArgSubstitution>, Box<IrType>, usize), IrBlockError>
    {
        match var_idxs.get(&var_idx) {
            Some(new_var_idx) => {
                match var_tuples.get(new_var_idx - new_start_var_idx) {
                    Some(var_tuple) => {
                        match var_tuple.old_block_index {
                            Some(old_block_idx) => {
                                match substitutions.get(&(var_idx, old_block_idx)) {
                                    Some(substitution) => {
                                        match var_tuple.assign_index {
                                            Some(assign_index) => {
                                                if assign_index < substitution.arg_substitutions.len() {
                                                    Ok((Some(substitution.arg_substitutions[assign_index].clone()), var_tuple.typ.clone(), *new_var_idx))
                                                } else {
                                                    Ok((None, var_tuple.typ.clone(), *new_var_idx))
                                                }
                                            },
                                            None => Ok((None, var_tuple.typ.clone(), *new_var_idx)),
                                        }
                                    },
                                    None => Ok((None, var_tuple.typ.clone(), *new_var_idx)),
                                }
                            },
                            None => Err(IrBlockError::NoOldBlockIndex),
                        }
                    },
                    None => Err(IrBlockError::NoVarTuple),
                }
            },
            None => Err(IrBlockError::NoVarIndex),
        }
    }

    fn var_value_tuple(&self, var_idx: usize, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>) -> Result<(Option<IrValue<IrArgVar>>, Box<IrType>, usize), IrBlockError>
    {
        match self.var_arg_substitution_tuple(var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
            (Some(ArgSubstitution::Value(new_value)), typ, new_var_idx) => Ok((Some(new_value.clone()), typ, new_var_idx)),
            (Some(_), _, _) => Err(IrBlockError::InvalidArgSubstitution),
            (None, typ, new_var_idx) => Ok((None, typ, new_var_idx)),
        }
    }
    
    fn new_var_value(&self, typ: &Option<Box<IrType>>, var_idx: usize, ops: &Vec<IrArgOp>, vector_elem_ptr_type: Option<&Option<Box<IrType>>>, value3: &IrValue<IrArgVar>, type2: &Box<IrType>, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrValue<IrArgVar>, IrBlockError>
    {
        if !ops.is_empty() {
            match new_var_idxs.get(&var_idx) {
                Some(new_var_idx) => {
                    if new_var_idx - new_start_var_idx - var_tuples.len() < new_var_tuples.len() {
                        match vector_elem_ptr_type {
                            Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(*new_var_idx, ops.clone(), vector_elem_ptr_type.clone()), typ.clone())))),
                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(*new_var_idx, ops.clone()), typ.clone())))),
                        }
                    } else {
                        Err(IrBlockError::NoVarTuple)
                    }
                },
                None => {
                    let value4 = self.substitute_arg_ops_for_value(&value3, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                    let new_var_idx = new_start_var_idx + var_tuples.len() + new_var_tuples.len();
                    new_var_tuples.push(VarTuple::new_with_value(type2.clone(), None, value4));
                    new_var_idxs.insert(var_idx, new_var_idx);
                    match vector_elem_ptr_type {
                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx, ops.clone(), vector_elem_ptr_type.clone()), typ.clone())))),
                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx, ops.clone()), typ.clone())))),
                    }
                },
            }
        } else {
            Ok(value3.clone())
        }
    }
    
    fn add_arg_ops(&self, ops: &mut Vec<IrArgOp>, typ: &Option<Box<IrType>>, ops2: &[IrArgOp])
    {
        match ops2.first() {
            Some(op) => {
                match op {
                    IrArgOp::Deref(type2) => ops.push(IrArgOp::Deref(type2.clone().or(typ.clone()))),
                    IrArgOp::Dot(type2, field_idx) => ops.push(IrArgOp::Dot(type2.clone().or(typ.clone()), *field_idx)),
                    IrArgOp::Arrow(type2, field_idx) => ops.push(IrArgOp::Arrow(type2.clone().or(typ.clone()), *field_idx)),
                    IrArgOp::Index(type2, idx) => ops.push(IrArgOp::Index(type2.clone().or(typ.clone()), *idx)),
                    IrArgOp::GlobalIndex(type2, ident) => ops.push(IrArgOp::GlobalIndex(type2.clone().or(typ.clone()), ident.clone())),
                    IrArgOp::LocalIndex(type2, var_idx) => ops.push(IrArgOp::LocalIndex(type2.clone().or(typ.clone()), *var_idx)),
                    IrArgOp::CallerFunArgIndex(type2, var_idx) => ops.push(IrArgOp::CallerFunArgIndex(type2.clone().or(typ.clone()), *var_idx)),
                    IrArgOp::PrivateClosureIndex(type2, var_idx) => ops.push(IrArgOp::PrivateClosureIndex(type2.clone().or(typ.clone()), *var_idx)),
                    IrArgOp::LocalClosureIndex(type2, var_idx) => ops.push(IrArgOp::LocalClosureIndex(type2.clone().or(typ.clone()), *var_idx)),
                    IrArgOp::GlobalClosureIndex(type2, var_idx) => ops.push(IrArgOp::GlobalClosureIndex(type2.clone().or(typ.clone()), *var_idx)),
                }
                ops.extend_from_slice(&ops2[1..]);
            }
            None => (),
        }
    }
    
    fn substitute_value_without_arg_ops(&self, value: &IrValue<IrArgVar>, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrValue<IrArgVar>, IrBlockError>
    {
        match value {
            IrValue::Object(object) => {
                match &**object {
                    IrObject::Var(var, typ) => {
                        let (var_idx, ops, vector_elem_ptr_type, value2, type2, new_var_idx) = match var {
                            IrArgVar::Local(tmp_var_idx, tmp_ops) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::CallerFunArg(tmp_var_idx, tmp_ops) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::PrivateClosure(tmp_var_idx, tmp_ops) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::LocalClosure(tmp_var_idx, tmp_ops) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::GlobalClosure(tmp_var_idx, tmp_ops) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::RefLocal(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::RefCallerFunArg(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::RefPrivateClosure(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::RefLocalClosure(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            IrArgVar::RefGlobalClosure(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_tuple(*tmp_var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value_without_arg_ops(&tmp_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type, tmp_new_var_idx),
                                    (None, tmp_type, tmp_new_var_idx) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type, tmp_new_var_idx),
                                }
                            },
                            _ => return Ok(value.clone()),
                        };
                        match value2.as_ref().map(|v| (v, None, true)).unwrap_or((value, Some(new_var_idx), false)) {
                            (value3 @ IrValue::Object(object2), new_var_idx2, are_ops) => {
                                match &**object2 {
                                    IrObject::Var(var2, type3) => {
                                        match var2 {
                                            IrArgVar::Global(ident, ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                match vector_elem_ptr_type {
                                                    Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobal(ident.clone(), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                    None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Global(ident.clone(), ops3), typ.clone())))),
                                                }
                                            },
                                            IrArgVar::Local(new_var_idx3, ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                match vector_elem_ptr_type {
                                                    Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                    None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                }
                                            },
                                            IrArgVar::CallerFunArg(new_var_idx3, ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                if is_caller_fun_arg_change {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                } else {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefCallerFunArg(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::CallerFunArg(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                }
                                            },
                                            IrArgVar::PrivateClosure(new_var_idx3, ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                if is_closure_var_change {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                } else {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefPrivateClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::PrivateClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                }
                                            },
                                            IrArgVar::LocalClosure(new_var_idx3, ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                if is_closure_var_change {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                } else {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocalClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::LocalClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                }
                                            },
                                            IrArgVar::GlobalClosure(new_var_idx3, ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                if is_closure_var_change {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                } else {
                                                    match vector_elem_ptr_type {
                                                        Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobalClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::GlobalClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops3), typ.clone())))),
                                                    }
                                                }
                                            },
                                            IrArgVar::PrivateHeap(ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                match vector_elem_ptr_type {
                                                    Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefPrivateHeap(ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                    None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::PrivateHeap(ops3), typ.clone())))),
                                                }
                                            },
                                            IrArgVar::LocalHeap(ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                match vector_elem_ptr_type {
                                                    Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocalHeap(ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                    None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::LocalHeap(ops3), typ.clone())))),
                                                }
                                            },
                                            IrArgVar::GlobalHeap(ops2) => {
                                                let mut ops3 = ops2.clone();
                                                if are_ops {
                                                    self.add_arg_ops(&mut ops3, &type3, ops.as_slice());
                                                }
                                                match vector_elem_ptr_type {
                                                    Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobalHeap(ops3, vector_elem_ptr_type.clone()), typ.clone())))),
                                                    None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::GlobalHeap(ops3), typ.clone())))),
                                                }
                                            },
                                            IrArgVar::RefGlobal(ident, ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    match vector_elem_ptr_type {
                                                        Some(_) => Err(IrBlockError::InvalidArgVar),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobal(ident.clone(), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefLocal(new_var_idx3, ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    match vector_elem_ptr_type {
                                                        Some(_) => Err(IrBlockError::InvalidArgVar),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefCallerFunArg(new_var_idx3, ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    if is_caller_fun_arg_change {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    } else {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefCallerFunArg(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefPrivateClosure(new_var_idx3, ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    if is_closure_var_change {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    } else {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefPrivateClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefLocalClosure(new_var_idx3, ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    if is_closure_var_change {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    } else {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocalClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefGlobalClosure(new_var_idx3, ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    if is_closure_var_change {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    } else {
                                                        match vector_elem_ptr_type {
                                                            Some(_) => Err(IrBlockError::InvalidArgVar),
                                                            None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobalClosure(new_var_idx2.unwrap_or(*new_var_idx3), ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                        }
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefPrivateHeap(ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    match vector_elem_ptr_type {
                                                        Some(_) => Err(IrBlockError::InvalidArgVar),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefPrivateHeap(ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefLocalHeap(ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    match vector_elem_ptr_type {
                                                        Some(_) => Err(IrBlockError::InvalidArgVar),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocalHeap(ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                            IrArgVar::RefGlobalHeap(ops2, vector_elem_ptr_type2) => {
                                                if !are_ops || ops.is_empty() {
                                                    match vector_elem_ptr_type {
                                                        Some(_) => Err(IrBlockError::InvalidArgVar),
                                                        None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobalHeap(ops2.clone(), vector_elem_ptr_type2.clone()), typ.clone())))),
                                                    }
                                                } else {
                                                    Err(IrBlockError::InvalidArgVar)
                                                }
                                            },
                                        }
                                    },
                                    _ => self.new_var_value(typ, var_idx, &ops, vector_elem_ptr_type, value3, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs),
                                }
                            },
                            (value3, _, _) => self.new_var_value(typ, var_idx, &ops, vector_elem_ptr_type, value3, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs),
                        }
                    },
                    IrObject::Vector(values, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_value_without_arg_ops(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Vector(new_values, typ.clone()))))
                    },
                    IrObject::Array(values, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_value_without_arg_ops(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Array(new_values, typ.clone()))))
                    },
                    IrObject::Struct(values, field_pairs, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_value_without_arg_ops(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        let mut new_field_pairs: Vec<IrFieldPair<IrArgVar>> = Vec::new();
                        for field_pair in field_pairs {
                            match field_pair {
                                IrFieldPair(var_idx, value2) => new_field_pairs.push(IrFieldPair(*var_idx, self.substitute_value_without_arg_ops(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                            }
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Struct(new_values, new_field_pairs, typ.clone()))))
                    },
                    IrObject::Union(var_idx, value2, typ) => {
                        let new_value = self.substitute_value_without_arg_ops(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                        Ok(IrValue::Object(Box::new(IrObject::Union(*var_idx, new_value, typ.clone()))))
                    },
                    IrObject::Closure(field_pairs, typ) => {
                        let mut new_field_pairs: Vec<IrFieldPair<IrArgVar>> = Vec::new();
                        for field_pair in field_pairs {
                            match field_pair {
                                IrFieldPair(var_idx, value2) => new_field_pairs.push(IrFieldPair(*var_idx, self.substitute_value_without_arg_ops(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                            }
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Closure(new_field_pairs, typ.clone()))))
                    },
                    _ => Ok(value.clone()),
                }
            },
            _ => Ok(value.clone()),
        }
    }

    fn new_var_arg_op_tuple(&self, typ: &Option<Box<IrType>>, var_idx: usize, value: &IrValue<IrArgVar>, type2: &Box<IrType>, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<(Option<IrArgOp>, usize, bool), IrBlockError>
    {
        match new_var_idxs.get(&var_idx) {
            Some(new_var_idx) => {
                if new_var_idx - new_start_var_idx - var_tuples.len() < new_var_tuples.len() {
                    Ok((Some(IrArgOp::LocalIndex(typ.clone(), *new_var_idx)), *new_var_idx, false))
                } else {
                    Err(IrBlockError::NoVarTuple)
                }
            },
            None => {
                let value2 = self.substitute_value(&value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_var_idx = new_start_var_idx + var_tuples.len() + new_var_tuples.len();
                new_var_tuples.push(VarTuple::new_with_value(type2.clone(), None, value2));
                new_var_idxs.insert(var_idx, new_var_idx);
                Ok((Some(IrArgOp::LocalIndex(typ.clone(), new_var_idx)), new_var_idx, false))
            },
        }
    }    

    fn var_arg_op_tuple(&self, typ: &Option<Box<IrType>>, var_idx: usize, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<(Option<IrArgOp>, usize, bool), IrBlockError>
    {
        match self.var_value_tuple(var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
            (Some(value), type2, new_var_idx) => {
                let (idx, type3) = match &value {
                    IrValue::Char(c, tmp_type) => (*c as u64, tmp_type),
                    IrValue::Short(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::Int(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::Long(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::Uchar(c, tmp_type) => (*c as u64, tmp_type),
                    IrValue::Ushort(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::Uint(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::Ulong(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::SizeT(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::PtrdiffT(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::IntptrT(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::UintptrT(n, tmp_type) => (*n as u64, tmp_type),
                    IrValue::Object(object) => {
                        match &**object {
                            IrObject::Var(var, None) => {
                                match var {
                                    IrArgVar::Local(var_idx2, ops) => {
                                        if ops.is_empty() {
                                            return Ok((Some(IrArgOp::LocalIndex(typ.clone(), *var_idx2)), new_var_idx, true));
                                        }
                                        return self.new_var_arg_op_tuple(typ, var_idx, &value, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                    },
                                    IrArgVar::CallerFunArg(var_idx2, ops) => {
                                        if ops.is_empty() {
                                            return Ok((Some(IrArgOp::CallerFunArgIndex(typ.clone(), *var_idx2)), new_var_idx, true));
                                        }
                                        return self.new_var_arg_op_tuple(typ, var_idx, &value, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                    },
                                    IrArgVar::PrivateClosure(var_idx2, ops) => {
                                        if ops.is_empty() {
                                            return Ok((Some(IrArgOp::PrivateClosureIndex(typ.clone(), *var_idx2)), new_var_idx, true));
                                        }
                                        return self.new_var_arg_op_tuple(typ, var_idx, &value, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                    },
                                    IrArgVar::LocalClosure(var_idx2, ops) => {
                                        if ops.is_empty() {
                                            return Ok((Some(IrArgOp::LocalClosureIndex(typ.clone(), *var_idx2)), new_var_idx, true));
                                        }
                                        return self.new_var_arg_op_tuple(typ, var_idx, &value, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                    },
                                    IrArgVar::GlobalClosure(var_idx2, ops) => {
                                        if ops.is_empty() {
                                            return Ok((Some(IrArgOp::GlobalClosureIndex(typ.clone(), *var_idx2)), new_var_idx, true));
                                        }
                                        return self.new_var_arg_op_tuple(typ, var_idx, &value, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                    },
                                    _ => {
                                        return self.new_var_arg_op_tuple(typ, var_idx, &value, &type2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                    },
                                }
                            },
                            _ => return Err(IrBlockError::InvalidObject),
                        }
                    }
                    _ => return Err(IrBlockError::InvalidValue),
                };
                match type3 {
                    Some(type3) => {
                        match &**type3 {
                            IrType::Char => Ok((Some(IrArgOp::Index(typ.clone(), idx & (u8::MAX as u64))), new_var_idx, true)),
                            IrType::Short => Ok((Some(IrArgOp::Index(typ.clone(), idx & (u16::MAX as u64))), new_var_idx, true)),
                            IrType::Int => Ok((Some(IrArgOp::Index(typ.clone(), idx & (u32::MAX as u64))), new_var_idx, true)),
                            IrType::Long => Ok((Some(IrArgOp::Index(typ.clone(), idx)), new_var_idx, true)),
                            IrType::Uchar => Ok((Some(IrArgOp::Index(typ.clone(), idx & (u8::MAX as u64))), new_var_idx, true)),
                            IrType::Ushort => Ok((Some(IrArgOp::Index(typ.clone(), idx & (u16::MAX as u64))), new_var_idx, true)),
                            IrType::Uint => Ok((Some(IrArgOp::Index(typ.clone(), idx & (u32::MAX as u64))), new_var_idx, true)),
                            IrType::Ulong => Ok((Some(IrArgOp::Index(typ.clone(), idx)), new_var_idx, true)),
                            IrType::SizeT => Ok((Some(IrArgOp::Index(typ.clone(), idx)), new_var_idx, true)),
                            IrType::PtrdiffT => Ok((Some(IrArgOp::Index(typ.clone(), idx)), new_var_idx, true)),
                            IrType::IntptrT => Ok((Some(IrArgOp::Index(typ.clone(), idx)), new_var_idx, true)),
                            IrType::UintptrT => Ok((Some(IrArgOp::Index(typ.clone(), idx)), new_var_idx, true)),
                            _ => Err(IrBlockError::InvalidType),
                        }
                    },
                    None => Ok((Some(IrArgOp::Index(typ.clone(), idx)), new_var_idx, true)),
                }
            }
            (None, _, new_var_idx) => Ok((None, new_var_idx, false)), 
        }
    }
    
    fn substitute_arg_op(&self, op: &IrArgOp, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrArgOp, IrBlockError>
    {
        let (op2, new_var_idx) = match op {
            IrArgOp::LocalIndex(tmp_type, tmp_var_idx) => {
                match self.var_arg_op_tuple(tmp_type, *tmp_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)? {
                    (Some(tmp_op), tmp_new_var_idx, is_substitution) => {
                        if is_substitution {
                            (Some(self.substitute_arg_op(&tmp_op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_new_var_idx)
                        } else {
                            (Some(tmp_op.clone()), tmp_new_var_idx)
                        }
                    },
                    (None, tmp_new_var_idx, _) => (None, tmp_new_var_idx),
                }
            },
            IrArgOp::CallerFunArgIndex(tmp_type, tmp_var_idx) => {
                match self.var_arg_op_tuple(tmp_type, *tmp_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)? {
                    (Some(tmp_op), tmp_new_var_idx, is_substitution) => {
                        if is_substitution {
                            (Some(self.substitute_arg_op(&tmp_op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_new_var_idx)
                        } else {
                            (Some(tmp_op.clone()), tmp_new_var_idx)
                        }
                    },
                    (None, tmp_new_var_idx, _) => (None, tmp_new_var_idx),
                }
            },
            IrArgOp::PrivateClosureIndex(tmp_type, tmp_var_idx) => {
                match self.var_arg_op_tuple(tmp_type, *tmp_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)? {
                    (Some(tmp_op), tmp_new_var_idx, is_substitution) => {
                        if is_substitution {
                            (Some(self.substitute_arg_op(&tmp_op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_new_var_idx)
                        } else {
                            (Some(tmp_op.clone()), tmp_new_var_idx)
                        }
                    },
                    (None, tmp_new_var_idx, _) => (None, tmp_new_var_idx),
                }
            },
            IrArgOp::LocalClosureIndex(tmp_type, tmp_var_idx) => {
                match self.var_arg_op_tuple(tmp_type, *tmp_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)? {
                    (Some(tmp_op), tmp_new_var_idx, is_substitution) => {
                        if is_substitution {
                            (Some(self.substitute_arg_op(&tmp_op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_new_var_idx)
                        } else {
                            (Some(tmp_op.clone()), tmp_new_var_idx)
                        }
                    },
                    (None, tmp_new_var_idx, _) => (None, tmp_new_var_idx),
                }
            },
            IrArgOp::GlobalClosureIndex(tmp_type, tmp_var_idx) => {
                match self.var_arg_op_tuple(tmp_type, *tmp_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)? {
                    (Some(tmp_op), tmp_new_var_idx, is_substitution) => {
                        if is_substitution {
                            (Some(self.substitute_arg_op(&tmp_op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_new_var_idx)
                        } else {
                            (Some(tmp_op.clone()), tmp_new_var_idx)
                        }
                    },
                    (None, tmp_new_var_idx, _) => (None, tmp_new_var_idx),
                }
            },
            _ => return Ok(op.clone()),
        };
        match op2.as_ref().map(|o| (o, None)).unwrap_or((op, Some(new_var_idx))) {
            (IrArgOp::LocalIndex(type2, new_var_idx3), new_var_idx2) => Ok(IrArgOp::LocalIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3))),
            (IrArgOp::CallerFunArgIndex(type2, new_var_idx3), new_var_idx2) => {
                if is_caller_fun_arg_change {
                    Ok(IrArgOp::LocalIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                } else {
                    Ok(IrArgOp::CallerFunArgIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                }
            },
            (IrArgOp::PrivateClosureIndex(type2, new_var_idx3), new_var_idx2) => {
                if is_caller_fun_arg_change {
                    Ok(IrArgOp::LocalIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                } else {
                    Ok(IrArgOp::PrivateClosureIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                }
            },
            (IrArgOp::LocalClosureIndex(type2, new_var_idx3), new_var_idx2) => {
                if is_caller_fun_arg_change {
                    Ok(IrArgOp::LocalIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                } else {
                    Ok(IrArgOp::LocalClosureIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                }
            },
            (IrArgOp::GlobalClosureIndex(type2, new_var_idx3), new_var_idx2) => {
                if is_caller_fun_arg_change {
                    Ok(IrArgOp::LocalIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                } else {
                    Ok(IrArgOp::GlobalClosureIndex(type2.clone(), new_var_idx2.unwrap_or(*new_var_idx3)))
                }
            },
            (op3, _) => Ok(op3.clone()),
        }
    }
    
    fn substitute_arg_ops_for_value(&self, value: &IrValue<IrArgVar>, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrValue<IrArgVar>, IrBlockError>
    {
        match value {
            IrValue::Object(object) => {
                match &**object {
                    IrObject::Var(var, typ) => {
                        match var {
                            IrArgVar::Global(ident, ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Global(ident.clone(), new_ops), typ.clone()))))
                            },
                            IrArgVar::Local(var_idx, ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(*var_idx, new_ops), typ.clone()))))
                            },
                            IrArgVar::CallerFunArg(var_idx, ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::CallerFunArg(*var_idx, new_ops), typ.clone()))))
                            },
                            IrArgVar::PrivateClosure(var_idx, ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::PrivateClosure(*var_idx, new_ops), typ.clone()))))
                            },
                            IrArgVar::LocalClosure(var_idx, ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::LocalClosure(*var_idx, new_ops), typ.clone()))))
                            },
                            IrArgVar::GlobalClosure(var_idx, ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::GlobalClosure(*var_idx, new_ops), typ.clone()))))
                            },
                            IrArgVar::PrivateHeap(ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::PrivateHeap(new_ops), typ.clone()))))
                            },
                            IrArgVar::LocalHeap(ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::LocalHeap(new_ops), typ.clone()))))
                            },
                            IrArgVar::GlobalHeap(ops) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::GlobalHeap(new_ops), typ.clone()))))
                            },
                            IrArgVar::RefGlobal(ident, ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobal(ident.clone(), new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefLocal(var_idx, ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(*var_idx, new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefCallerFunArg(var_idx, ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefCallerFunArg(*var_idx, new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefPrivateClosure(var_idx, ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefPrivateClosure(*var_idx, new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefLocalClosure(var_idx, ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocalClosure(*var_idx, new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefGlobalClosure(var_idx, ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobalClosure(*var_idx, new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefPrivateHeap(ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefPrivateHeap(new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefLocalHeap(ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocalHeap(new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                            IrArgVar::RefGlobalHeap(ops, vector_elem_ptr_type) => {
                                let mut new_ops: Vec<IrArgOp> = Vec::new();
                                for op in ops {
                                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefGlobalHeap(new_ops, vector_elem_ptr_type.clone()), typ.clone()))))
                            },
                        }
                    },
                    IrObject::Vector(values, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_arg_ops_for_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Vector(new_values, typ.clone()))))
                    },
                    IrObject::Array(values, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_arg_ops_for_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Array(new_values, typ.clone()))))
                    },
                    IrObject::Struct(values, field_pairs, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_arg_ops_for_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        let mut new_field_pairs: Vec<IrFieldPair<IrArgVar>> = Vec::new();
                        for field_pair in field_pairs {
                            match field_pair {
                                IrFieldPair(var_idx, value2) => new_field_pairs.push(IrFieldPair(*var_idx, self.substitute_arg_ops_for_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                            }
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Struct(new_values, new_field_pairs, typ.clone()))))
                    },
                    IrObject::Union(var_idx, value2, typ) => {
                        let new_value = self.substitute_arg_ops_for_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                        Ok(IrValue::Object(Box::new(IrObject::Union(*var_idx, new_value, typ.clone()))))
                    },
                    IrObject::Closure(field_pairs, typ) => {
                        let mut new_field_pairs: Vec<IrFieldPair<IrArgVar>> = Vec::new();
                        for field_pair in field_pairs {
                            match field_pair {
                                IrFieldPair(var_idx, value2) => new_field_pairs.push(IrFieldPair(*var_idx, self.substitute_arg_ops_for_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                            }
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Closure(new_field_pairs, typ.clone()))))
                    },
                    _ => Ok(value.clone()),
                }
            },
            _ => Ok(value.clone()),
        }
    }

    fn substitute_value(&self, value: &IrValue<IrArgVar>, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrValue<IrArgVar>, IrBlockError>
    {
        let value2 = self.substitute_value_without_arg_ops(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
        self.substitute_arg_ops_for_value(&value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)
    }
    
    fn arg_substitution(&self, value: &IrValue<IrArgVar>, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<ArgSubstitution, IrBlockError>
    {
        match value {
            IrValue::Object(object) => {
                match &**object {
                    IrObject::Var(var, None) => {
                        match var {
                            IrArgVar::Local(var_idx, ops) => {
                                if ops.is_empty() {
                                    match self.var_arg_substitution_tuple(*var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                        (Some(ArgSubstitution::Value(value2)), _, _) => self.arg_substitution(&value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs),
                                        (Some(substitution), _, _) => Ok(substitution),
                                        (None, _, _) => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                                    }
                                } else {
                                    Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?))
                                }
                            },
                            IrArgVar::CallerFunArg(var_idx, ops) => {
                                if ops.is_empty() {
                                    match self.var_arg_substitution_tuple(*var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                        (Some(ArgSubstitution::Value(value2)), _, _) => self.arg_substitution(&value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs),
                                        (Some(substitution), _, _) => Ok(substitution),
                                        (None, _, _) => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                                    }
                                } else {
                                    Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?))
                                }
                            },
                            IrArgVar::PrivateClosure(var_idx, ops) => {
                                if ops.is_empty() {
                                    match self.var_arg_substitution_tuple(*var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                        (Some(ArgSubstitution::Value(value2)), _, _) => self.arg_substitution(&value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs),
                                        (Some(substitution), _, _) => Ok(substitution),
                                        (None, _, _) => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                                    }
                                } else {
                                    Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?))
                                }
                            },
                            IrArgVar::LocalClosure(var_idx, ops) => {
                                if ops.is_empty() {
                                    match self.var_arg_substitution_tuple(*var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                        (Some(ArgSubstitution::Value(value2)), _, _) => self.arg_substitution(&value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs),
                                        (Some(substitution), _, _) => Ok(substitution),
                                        (None, _, _) => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                                    }
                                } else {
                                    Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?))
                                }
                            },
                            IrArgVar::GlobalClosure(var_idx, ops) => {
                                if ops.is_empty() {
                                    match self.var_arg_substitution_tuple(*var_idx, new_start_var_idx, substitutions, var_tuples, var_idxs)? {
                                        (Some(ArgSubstitution::Value(value2)), _, _) => self.arg_substitution(&value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs),
                                        (Some(substitution), _, _) => Ok(substitution),
                                        (None, _, _) => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                                    }
                                } else {
                                    Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?))
                                }
                            },
                            _ => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                        }
                    },
                    _ => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                }
            },
            _ => Ok(ArgSubstitution::Value(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
        }
    }

    fn new_var_index(&self, var_idx: usize, var_idxs: &BTreeMap<usize, usize>) -> Result<usize, IrBlockError>
    {
        match var_idxs.get(&var_idx) {
            Some(new_var_idx) => Ok(*new_var_idx),
            None => Err(IrBlockError::NoVarIndex),
        }
    }
    
    fn substitute_instr_var(&self, var: &Box<IrInstrVar>, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrInstrVar, IrBlockError>
    {
        match &**var {
            IrInstrVar::Global(ident, ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                Ok(IrInstrVar::Global(ident.clone(), new_ops))
            },
            IrInstrVar::Local(var_idx, ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                Ok(IrInstrVar::Local(self.new_var_index(*var_idx, var_idxs)?, new_ops))
            },
            IrInstrVar::CallerFunArg(var_idx, ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                if is_caller_fun_arg_change {
                    Ok(IrInstrVar::Local(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                } else {
                    Ok(IrInstrVar::CallerFunArg(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                }
            },
            IrInstrVar::PrivateClosure(var_idx, ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                if is_closure_var_change {
                    Ok(IrInstrVar::Local(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                } else {
                    Ok(IrInstrVar::PrivateClosure(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                }
            },
            IrInstrVar::LocalClosure(var_idx, ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                if is_closure_var_change {
                    Ok(IrInstrVar::Local(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                } else {
                    Ok(IrInstrVar::LocalClosure(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                }
            },
            IrInstrVar::GlobalClosure(var_idx, ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                if is_closure_var_change {
                    Ok(IrInstrVar::Local(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                } else {
                    Ok(IrInstrVar::GlobalClosure(self.new_var_index(*var_idx, var_idxs)?, new_ops))
                }
            },
            IrInstrVar::PrivateHeap(ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                Ok(IrInstrVar::PrivateHeap(new_ops))
            },
            IrInstrVar::LocalHeap(ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                Ok(IrInstrVar::LocalHeap(new_ops))
            },
            IrInstrVar::GlobalHeap(ops) => {
                let mut new_ops: Vec<IrArgOp> = Vec::new();
                for op in ops {
                    new_ops.push(self.substitute_arg_op(op, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                Ok(IrInstrVar::GlobalHeap(new_ops))
            },
        }
    }
    
    fn fun_block_and_fun_op(&self, fun_old_start_var_idx: usize, fun_arg_types: &[Box<IrType>], fun_ret_type: &Box<IrType>, fun_block: &Box<IrBlock>, arg_values: &[IrValue<IrArgVar>], pos: &Pos, panic_poses: &[Pos], new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, ret_var: Option<Option<&Box<IrInstrVar>>>, poses: &[Pos], tree: &IrTree, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<(Option<IrBlock>, Option<IrOp>), IrBlockError>
    {
        let mut new_arg_values: Vec<IrValue<IrArgVar>> = Vec::new();
        for arg_value in arg_values {
            new_arg_values.push(self.substitute_value(arg_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
        }
        let (new_op, new_ret_var) = match ret_var {
            Some(Some(ret_var)) => {
                let new_ret_var = Box::new(self.substitute_instr_var(ret_var, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                (None, Some(new_ret_var))
            },
            Some(None) => (None, None),
            None => {
                let new_var_idx = new_start_var_idx + var_tuples.len() + new_var_tuples.len();
                new_var_tuples.push(VarTuple::new(fun_ret_type.clone(), None));
                (Some(IrOp::Load(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx, Vec::new()), None))))), Some(Box::new(IrInstrVar::Local(new_var_idx, Vec::new()))))
            },
        };
        let new_ret_var2 = match &new_ret_var {
            Some(new_ret_var) => Some(Some(new_ret_var)),
            None => Some(None),
        };
        let fun_new_start_var_idx = new_start_var_idx + var_tuples.len() + new_var_tuples.len();
        let mut new_poses = vec![pos.clone()];
        new_poses.extend_from_slice(panic_poses);
        new_poses.extend_from_slice(poses);
        let new_fun_block = fun_block.substitute(fun_old_start_var_idx, fun_arg_types, fun_new_start_var_idx, &BTreeMap::new(), new_ret_var2, new_poses.as_slice(), tree, true, true)?;
        if !arg_values.is_empty() {
            let mut new_fun_block2 = IrBlock::new();
            for fun_arg_type in fun_arg_types {
                new_fun_block2.add_local_var_pair(IrLocalVarPair(IrLocalVarModifier::None, fun_arg_type.clone()));
            }
            for (i, new_arg_value) in new_arg_values.iter().enumerate() {
                new_fun_block2.add_instr(IrInstr::Assign(Box::new(IrInstrVar::Local(fun_new_start_var_idx + i, Vec::new())), IrOp::Load(new_arg_value.clone())));
            }
            new_fun_block2.add_block(new_fun_block);
            Ok((Some(new_fun_block2), new_op))
        } else {
            Ok((Some(new_fun_block), new_op))
        }
    }
    
    fn substitute_fun_op<F>(&self, ident: &String, values: &[IrValue<IrArgVar>], pos: &Pos, panic_poses: &[Pos], new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, ret_var: Option<Option<&Box<IrInstrVar>>>, poses: &[Pos], tree: &IrTree, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>, mut f: F) -> Result<(Option<IrBlock>, Option<IrOp>), IrBlockError>
        where F: FnMut(String, Vec<IrValue<IrArgVar>>, Pos, Vec<Pos>, &mut Vec<VarTuple>, &mut BTreeMap<usize, usize>) -> Result<IrOp, IrBlockError>
    {
        match tree.var(ident) {
            Some(var) => {
                let var_r = var.borrow();
                match &*var_r {
                    IrVar::Fun(fun) => {
                        match &**fun {
                            IrFun::Caller(_, _, _, _, _, _, _, _) => {
                                match values.first() {
                                    Some(value) => {
                                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                                        match self.arg_substitution(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)? {
                                            ArgSubstitution::Value(new_value) => new_values.push(new_value),
                                            ArgSubstitution::Fun(ident2) => {
                                                match tree.var(&ident2) {
                                                    Some(var2) => {
                                                        let var2_r = var2.borrow();
                                                        match &*var2_r {
                                                            IrVar::Fun(fun2) => {
                                                                match &**fun2 {
                                                                    IrFun::Fun(IrFunModifier::Inline, fun_arg_types, fun_ret_type, fun_block, _, _, _, _) => {
                                                                        return self.fun_block_and_fun_op(0, fun_arg_types, fun_ret_type, fun_block, &values[1..], pos, panic_poses, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                                                    },
                                                                    _ => (),
                                                                }
                                                            },
                                                            _ => return Err(IrBlockError::ConstOrVar),
                                                        }
                                                    },
                                                    None => return Err(IrBlockError::NoFun),
                                                }
                                                new_values.push(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                            },
                                            ArgSubstitution::Lambda(fun_old_start_var_idx, fun_arg_types, fun_ret_type, fun_block) => {
                                                return self.fun_block_and_fun_op(fun_old_start_var_idx, fun_arg_types.as_slice(), &fun_ret_type, &fun_block, &values[1..], pos, panic_poses, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs);
                                            },
                                        }
                                        for value2 in &values[1..] {
                                            new_values.push(self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                        }
                                        let mut new_panic_poses = panic_poses.to_vec();
                                        new_panic_poses.extend_from_slice(poses);
                                        Ok((None, Some(f(ident.clone(), new_values, pos.clone(), new_panic_poses, new_var_tuples, new_var_idxs)?)))
                                    },
                                    None => Err(IrBlockError::NoFirstValue),
                                }
                            },
                            _ => {
                                let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                                for value in values {
                                    new_values.push(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                                }
                                let mut new_panic_poses = panic_poses.to_vec();
                                new_panic_poses.extend_from_slice(poses);
                                Ok((None, Some(f(ident.clone(), new_values, pos.clone(), new_panic_poses, new_var_tuples, new_var_idxs)?)))
                            },
                        }
                    },
                    _ => Err(IrBlockError::ConstOrVar),
                }
            },
            None => Err(IrBlockError::NoFun),
        }
    }
    
    fn substitute_op(&self, op: &IrOp, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, ret_var: Option<Option<&Box<IrInstrVar>>>, poses: &[Pos], tree: &IrTree, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<(Option<IrBlock>, Option<IrOp>), IrBlockError>
    {
        match op {
            IrOp::Load(value) => {
                let new_value = self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Load(new_value))))
            },
            IrOp::Neg(value) => {
                let new_value = self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Neg(new_value))))
            },
            IrOp::Not(value) => {
                let new_value = self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Not(new_value))))
            },
            IrOp::Mul(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Mul(new_value1, new_value2))))
            },
            IrOp::Div(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Div(new_value1, new_value2))))
            },
            IrOp::Rem(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Rem(new_value1, new_value2))))
            },
            IrOp::Add(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Add(new_value1, new_value2))))
            },
            IrOp::Sub(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Sub(new_value1, new_value2))))
            },
            IrOp::Shl(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Shl(new_value1, new_value2))))
            },
            IrOp::Shr(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Shr(new_value1, new_value2))))
            },
            IrOp::Eq(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Eq(new_value1, new_value2))))
            },
            IrOp::Ne(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Ne(new_value1, new_value2))))
            },
            IrOp::Lt(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Lt(new_value1, new_value2))))
            },
            IrOp::Ge(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Ge(new_value1, new_value2))))
            },
            IrOp::Gt(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Gt(new_value1, new_value2))))
            },
            IrOp::Le(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Le(new_value1, new_value2))))
            },
            IrOp::And(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::And(new_value1, new_value2))))
            },
            IrOp::Xor(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Xor(new_value1, new_value2))))
            },
            IrOp::Or(value1, value2) => {
                let new_value1 = self.substitute_value(value1, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                let new_value2 = self.substitute_value(value2, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                Ok((None, Some(IrOp::Or(new_value1, new_value2))))
            },
            IrOp::CallBuiltinFun(ident, typ, values) => {
                let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                for value in values {
                    new_values.push(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                }
                Ok((None, Some(IrOp::CallBuiltinFun(ident.clone(), typ.clone(), new_values))))
            },
            IrOp::CallFun(ident, values, pos, panic_poses, panic_value) => {
                self.substitute_fun_op(ident, values.as_slice(), pos, panic_poses.as_slice(), new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs, |ident, new_values, pos, new_panic_poses, new_var_tuples, new_var_idxs| {
                        let new_panic_value = match panic_value {
                            Some(panic_value) => Some(self.substitute_value(panic_value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?),
                            None => None,
                        };
                        Ok(IrOp::CallFun(ident, new_values, pos, new_panic_poses, new_panic_value.clone()))
                })
            },
            IrOp::CallFunWithoutPanic(ident, values, pos) => {
                self.substitute_fun_op(ident, values.as_slice(), pos, &[], new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs, |ident, new_values, pos, _, _, _| {
                        Ok(IrOp::CallFunWithoutPanic(ident, new_values, pos))
                })
            },
        }
    }
    
    fn push_new_tuples(&self, var_tuples: &mut Vec<VarTuple>, new_var_idx: usize, new_tuples: &[VarTuple]) -> usize
    {
        var_tuples.extend_from_slice(new_tuples);
        new_var_idx + new_tuples.len()
    }

    fn pop_tuples(&self, var_tuples: &mut Vec<VarTuple>, new_var_idx: usize, new_tuple_count: usize) -> usize
    {
        for _ in (0..new_tuple_count).rev() {
            var_tuples.pop();
        }
        new_var_idx - new_tuple_count
    }
    
    fn substitute_from(&self, old_start_var_idx: usize, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, ret_var: Option<Option<&Box<IrInstrVar>>>, poses: &[Pos], tree: &IrTree, is_caller_fun_arg_change: bool, is_closure_var_change: bool, old_var_idx: usize, new_var_idx: usize, block_idx: &mut usize, var_tuples: &mut Vec<VarTuple>, var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrBlock, IrBlockError>
    {
        let mut new_block = IrBlock::new();
        let current_block_idx = *block_idx;
        *block_idx += 1;
        let mut old_var_idx2 = old_var_idx;
        let mut new_var_idx2 = new_var_idx;
        for local_var_pair in &self.local_var_pairs {
            let is_var = match  substitutions.get(&(old_var_idx2, current_block_idx)) {
                Some(substitution) => substitution.has_var(),
                None => true,
            };
            if is_var {
                var_tuples.push(VarTuple::new(local_var_pair.1.clone(), Some(current_block_idx)));
                var_idxs.insert(old_var_idx2, new_var_idx2);
                new_block.add_local_var_pair(local_var_pair.clone());
                new_var_idx2 += 1;
            } else {
                var_tuples.push(VarTuple::new(local_var_pair.1.clone(), None));
            }
            old_var_idx2 += 1;
        }
        for instr in &self.instrs {
            let mut new_var_tuples: Vec<VarTuple> = Vec::new();
            let mut new_var_idxs: BTreeMap<usize, usize> = BTreeMap::new();
            let (new_block2, new_instr) = match instr {
                IrInstr::Op(op) => {
                    let (tmp_new_block, new_op) = self.substitute_op(op, new_start_var_idx, substitutions, Some(None), poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                    match new_op {
                        Some(new_op) => (tmp_new_block, Some(IrInstr::Op(new_op))),
                        None => (tmp_new_block, None),
                    }
                },
                IrInstr::Assign(var, op) => {
                    let var_idx = match &**var {
                        IrInstrVar::Local(tmp_var_idx, _) => Some(tmp_var_idx),
                        IrInstrVar::CallerFunArg(tmp_var_idx, _) => Some(tmp_var_idx),
                        IrInstrVar::PrivateClosure(tmp_var_idx, _) => Some(tmp_var_idx),
                        IrInstrVar::LocalClosure(tmp_var_idx, _) => Some(tmp_var_idx),
                        IrInstrVar::GlobalClosure(tmp_var_idx, _) => Some(tmp_var_idx),
                        _ => None,
                    };
                    let is_assign = match var_idx {
                        Some(var_idx) => {
                            match var_idxs.get(var_idx) {
                                Some(new_var_idx) => {
                                    match var_tuples.get_mut(new_var_idx - new_start_var_idx) {
                                        Some(var_tuple) => {
                                            match var_tuple.old_block_index {
                                                Some(old_block_idx) => {
                                                    match substitutions.get(&(*var_idx, old_block_idx)) {
                                                        Some(substitution) => {
                                                            match &mut var_tuple.assign_index {
                                                                Some(assign_index) => {
                                                                    if *assign_index < substitution.arg_substitutions.len() {
                                                                        *assign_index += 1;
                                                                        false
                                                                    } else {
                                                                        true
                                                                    }
                                                                },
                                                                None => {
                                                                    var_tuple.assign_index = Some(0);
                                                                    false
                                                                }
                                                            }
                                                        },
                                                        None => true,
                                                    }
                                                },
                                                None => return Err(IrBlockError::NoOldBlockIndex),
                                            }
                                        },
                                        None => return Err(IrBlockError::NoVarTuple),
                                    }
                                },
                                None => return Err(IrBlockError::NoVarIndex),
                            }
                        },
                        _ => true,
                    };
                    if is_assign {
                        let (tmp_new_block, new_op) = self.substitute_op(op, new_start_var_idx, substitutions, Some(None), poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                        let new_var = self.substitute_instr_var(var, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                        match new_op {
                            Some(new_op) => (tmp_new_block, Some(IrInstr::Assign(Box::new(new_var), new_op))),
                            None => (tmp_new_block, None),
                        }
                    } else {
                        (None, None)
                    }
                },
                IrInstr::Return(op) => {
                    match ret_var {
                        Some(Some(ret_var)) => {
                            match op {
                                Some(op) => {
                                    let (tmp_new_block, new_op) = self.substitute_op(op, new_start_var_idx, substitutions, None, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                                    match new_op {
                                        Some(new_op) => (tmp_new_block, Some(IrInstr::Assign(ret_var.clone(), new_op))),
                                        None => return Err(IrBlockError::NoOp),
                                    }
                                },
                                None => (None, None),
                            }
                        }
                        Some(None) => {
                            match op {
                                Some(op) => {
                                    let (tmp_new_block, new_op) = self.substitute_op(op, new_start_var_idx, substitutions, Some(None), poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                                    match new_op {
                                        Some(_) => (tmp_new_block, None),
                                        None => return Err(IrBlockError::NoOp),
                                    }
                                },
                                None => (None, None),
                            }
                        },
                        None => {
                            match op {
                                Some(op) => {
                                    let (tmp_new_block, new_op) = self.substitute_op(op, new_start_var_idx, substitutions, None, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                                    match new_op {
                                        Some(new_op) => (tmp_new_block, Some(IrInstr::Return(Some(new_op)))),
                                        None => return Err(IrBlockError::NoOp),
                                    }
                                },
                                None => (None, Some(IrInstr::Return(None))),
                            }
                        },
                    }
                },
                IrInstr::Block(block) => {
                    new_block.add_block(block.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_var_idx2, new_var_idx2, block_idx, var_tuples, var_idxs)?);
                    (None, None)
                },
                IrInstr::If(op, block1, block2) => {
                    let (tmp_new_block, new_op) = self.substitute_op(op, new_start_var_idx, substitutions, None, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                    match new_op {
                        Some(new_op) => {
                            new_var_idx2 = self.push_new_tuples(var_tuples, new_var_idx2, new_var_tuples.as_slice());
                            let new_block1 = block1.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_var_idx2, new_var_idx2, block_idx, var_tuples, var_idxs)?;
                            let new_block2 = block2.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_var_idx2, new_var_idx2, block_idx, var_tuples, var_idxs)?;
                            new_var_idx2 = self.pop_tuples(var_tuples, new_var_idx2, new_var_tuples.len());
                            (tmp_new_block, Some(IrInstr::If(new_op, Box::new(new_block1), Box::new(new_block2))))
                        },
                        None => return Err(IrBlockError::NoOp),
                    }
                },
                IrInstr::Switch(op, cases) => {
                    let (tmp_new_block, new_op) = self.substitute_op(op, new_start_var_idx, substitutions, None, poses, tree, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?;
                    match new_op {
                        Some(new_op) => {
                            let mut new_cases: Vec<IrCase> = Vec::new();
                            new_var_idx2 = self.push_new_tuples(var_tuples, new_var_idx2, new_var_tuples.as_slice());
                            for case in cases {
                                match case {
                                    IrCase::Case(value, block) => {
                                        new_cases.push(IrCase::Case(value.clone(), Box::new(block.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_var_idx2, new_var_idx2, block_idx, var_tuples, var_idxs)?)));
                                    },
                                    IrCase::Default(block) => {
                                        new_cases.push(IrCase::Default(Box::new(block.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_var_idx2, new_var_idx2, block_idx, var_tuples, var_idxs)?)));
                                    },
                                }
                            }
                            new_var_idx2 = self.pop_tuples(var_tuples, new_var_idx2, new_var_tuples.len());
                            (tmp_new_block, Some(IrInstr::Switch(new_op, new_cases)))
                        },
                        None => return Err(IrBlockError::NoOp),
                    }
                },
                IrInstr::Loop(block) => {
                    (None, Some(IrInstr::Loop(Box::new(block.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_var_idx2, new_var_idx2, block_idx, var_tuples, var_idxs)?))))
                },
                IrInstr::Panic(msg, pos, panic_poses, value) => {
                    let new_value = match value {
                        Some(value) => Some(self.substitute_value(value, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples.as_slice(), var_idxs, &mut new_var_tuples, &mut new_var_idxs)?),
                        None => None,
                    };
                    let mut new_panic_poses = panic_poses.clone();
                    new_panic_poses.extend_from_slice(poses);
                    (None, Some(IrInstr::Panic(msg.clone(), pos.clone(), new_panic_poses, new_value)))
                },
                _ => (None, Some(instr.clone())),
            };
            if new_block2.is_some() || new_instr.is_some() {
                if new_var_tuples.is_empty() {
                    let mut new_block3 = IrBlock::new();
                    for new_var_tuple in &new_var_tuples {
                        new_block3.add_local_var_pair(IrLocalVarPair(IrLocalVarModifier::None, new_var_tuple.typ.clone()));
                    }
                    let mut new_var_idx3 = new_var_idx2;
                    for new_var_tuple in &new_var_tuples {
                        match &new_var_tuple.value {
                            Some(value) => new_block3.add_instr(IrInstr::Assign(Box::new(IrInstrVar::Local(new_var_idx3, Vec::new())), IrOp::Load(value.clone()))),
                            None => (), 
                        }
                        new_var_idx3 += 1;
                    }
                    match new_block2 {
                        Some(new_block2) => new_block3.add_block(new_block2),
                        None => (),
                    }
                    match new_instr {
                        Some(new_instr) => new_block3.add_instr(new_instr),
                        None => (),
                    }
                    new_block.add_block(new_block3);
                } else {
                    match new_block2 {
                        Some(new_block2) => new_block.add_block(new_block2),
                        None => (),
                    }
                    match new_instr {
                        Some(new_instr) => new_block.add_instr(new_instr),
                        None => (),
                    }
                }
            }
        }
        for _ in self.local_var_pairs.iter().rev() {
            old_var_idx2 -= 1;
            let is_var = match  substitutions.get(&(old_var_idx2, current_block_idx)) {
                Some(substitution) => substitution.has_var(),
                None => true,
            };
            if is_var {
                var_tuples.pop();
                var_idxs.remove(&old_var_idx2);
            }
        }
        Ok(new_block)
    }

    pub fn substitute(&self, old_start_var_idx: usize, var_types: &[Box<IrType>], new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, ret_var: Option<Option<&Box<IrInstrVar>>>, poses: &[Pos], tree: &IrTree, is_caller_fun_arg_change: bool, is_closure_var_change: bool) -> Result<IrBlock, IrBlockError>
    {
        let mut var_tuples: Vec<VarTuple> = Vec::new();
        let mut var_idxs: BTreeMap<usize, usize> = BTreeMap::new();
        let mut old_var_idx = old_start_var_idx;
        let mut new_var_idx = new_start_var_idx;
        for var_type in var_types {
            let is_var = match  substitutions.get(&(old_var_idx, 0)) {
                Some(substitution) => substitution.has_var(),
                None => true,
            };
            if is_var {
                let mut var_tuple = VarTuple::new(var_type.clone(), Some(0));
                var_tuple.assign_index = Some(0);
                var_tuples.push(var_tuple);
                var_idxs.insert(old_var_idx, new_var_idx);
                new_var_idx += 1;
            }
            old_var_idx += 1;
        }
        let mut block_idx = 1usize;
        self.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_start_var_idx + var_types.len(), new_start_var_idx + var_tuples.len(), &mut block_idx, &mut var_tuples, &mut var_idxs)
    }
}

#[derive(Debug)]
pub enum IrBlockError
{
    InvalidArgSubstitution,
    InvalidArgVar,
    InvalidValue,
    InvalidObject,
    InvalidType,
    NoVarIndex,
    NoVarTuple,
    NoOldBlockIndex,
    NoFun,
    ConstOrVar,
    NoFirstValue,
    NoOp,
}

impl error::Error for IrBlockError
{}

impl fmt::Display for IrBlockError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { 
        match self {
          IrBlockError::InvalidArgSubstitution => write!(f, "invalid argument substitution"),
          IrBlockError::InvalidArgVar => write!(f, "invalid argument variable"),
          IrBlockError::InvalidValue => write!(f, "invalid value"),
          IrBlockError::InvalidObject => write!(f, "invalid object"),
          IrBlockError::InvalidType => write!(f, "invalid type"),
          IrBlockError::NoVarIndex => write!(f, "no variable index"),
          IrBlockError::NoVarTuple => write!(f, "no variable tuple"),
          IrBlockError::NoOldBlockIndex => write!(f, "no old block index"),
          IrBlockError::NoFun => write!(f, "no function"),
          IrBlockError::ConstOrVar => write!(f, "variable isn't function"),
          IrBlockError::NoFirstValue => write!(f, "no first value"),
          IrBlockError::NoOp => write!(f, "no operation"),
        }
    }
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
            IrInstr::Switch(_, cases) => {
                cases.iter().fold(0usize, |n, c| {
                        match c {
                            IrCase::Case(_, block) => n + block.block_count() + 1,
                            IrCase::Default(block) => n + block.block_count() + 1,
                        }
                })
            },
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
    PrivateHeap(Vec<IrArgOp>),
    LocalHeap(Vec<IrArgOp>),
    GlobalHeap(Vec<IrArgOp>),
}

#[derive(Clone, Debug)]
pub enum IrCase
{
    Case(IrCaseValue, Box<IrBlock>),
    Default(Box<IrBlock>),
}

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
