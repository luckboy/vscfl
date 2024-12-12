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
    Lambda(usize, Vec<Box<IrType>>, Box<IrBlock>),
}

#[derive(Clone, Debug)]
struct VarTuple
{
    typ: Box<IrType>,
    old_block_index: Option<usize>,
    new_var_index: Option<usize>,
    assign_index: Option<usize>,
    value: Option<IrValue<IrArgVar>>,
}

impl VarTuple
{
    pub fn new(typ: Box<IrType>, old_block_idx: Option<usize>, new_var_idx: Option<usize>) -> Self
    {
        VarTuple {
            typ,
            old_block_index: old_block_idx,
            new_var_index: new_var_idx,
            assign_index: None,
            value: None,
        }
    }

    pub fn new_with_value(typ: Box<IrType>, old_block_idx: Option<usize>, new_var_idx: Option<usize>, value: IrValue<IrArgVar>) -> Self
    {
        VarTuple {
            typ,
            old_block_index: old_block_idx,
            new_var_index: new_var_idx,
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

    pub fn block_count(&self) -> usize
    { self.block_count }

    fn var_value_and_var_type(&self, var_idx: usize, old_start_var_idx: usize, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>) -> Result<(Option<IrValue<IrArgVar>>, IrType), IrBlockError>
    {
        match var_idxs.get(&(var_idx)) {
            Some(new_var_idx) => {
                match var_tuples.get(new_var_idx - new_start_var_idx) {
                    Some(var_tuple) => {
                        match &var_tuple.value {
                            Some(new_value) => Ok((Some(new_value.clone()), (*var_tuple.typ).clone())),
                            None => {
                                match var_tuple.old_block_index {
                                    Some(old_block_idx) => {
                                        match substitutions.get(&(var_idx, old_block_idx)) {
                                            Some(substitution) => {
                                                match var_tuple.assign_index {
                                                    Some(assign_index) => {
                                                        if assign_index < substitution.arg_substitutions.len() {
                                                            match &substitution.arg_substitutions[assign_index] {
                                                                ArgSubstitution::Value(new_value) => Ok((Some(new_value.clone()), (*var_tuple.typ).clone())),
                                                                _ => Err(IrBlockError::InvalidArgSubstitution),
                                                            }
                                                        } else {
                                                            Ok((None, (*var_tuple.typ).clone()))
                                                        }
                                                    },
                                                    None => Ok((None, (*var_tuple.typ).clone())),
                                                }
                                            },
                                            None => Ok((None, (*var_tuple.typ).clone())),
                                        }
                                    },
                                    None => Err(IrBlockError::NoOldBlockIndex),
                                }
                            },
                        }
                    },
                    None => Err(IrBlockError::NoVarTuple),
                }
            },
            None => Err(IrBlockError::NoVarIndex),
        }
    }

    fn substitute_value(&self, value: &IrValue<IrArgVar>, old_start_var_idx: usize, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, is_caller_fun_arg_change: bool, is_closure_var_change: bool, var_tuples: &[VarTuple], var_idxs: &BTreeMap<usize, usize>, new_var_tuples: &mut Vec<VarTuple>, new_var_idxs: &mut BTreeMap<usize, usize>) -> Result<IrValue<IrArgVar>, IrBlockError>
    {
        match value {
            IrValue::Object(object) => {
                match &**object {
                    IrObject::Var(var, typ) => {
                        let (var_idx, ops, vector_elem_ptr_type, value2, typ2) = match var {
                            IrArgVar::Local(tmp_var_idx, tmp_ops) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type),
                                }
                            },
                            IrArgVar::CallerFunArg(tmp_var_idx, tmp_ops) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type),
                                }
                            },
                            IrArgVar::PrivateClosure(tmp_var_idx, tmp_ops) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type),
                                }
                            },
                            IrArgVar::LocalClosure(tmp_var_idx, tmp_ops) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type),
                                }
                            },
                            IrArgVar::GlobalClosure(tmp_var_idx, tmp_ops) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), None, None, tmp_type),
                                }
                            },
                            IrArgVar::RefLocal(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type),
                                }
                            },
                            IrArgVar::RefCallerFunArg(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type),
                                }
                            },
                            IrArgVar::RefPrivateClosure(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type),
                                }
                            },
                            IrArgVar::RefLocalClosure(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type),
                                }
                            },
                            IrArgVar::RefGlobalClosure(tmp_var_idx, tmp_ops, tmp_vector_elem_ptr_type) => {
                                match self.var_value_and_var_type(*tmp_var_idx, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs)? {
                                    (Some(tmp_value), tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), Some(self.substitute_value(&tmp_value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?), tmp_type),
                                    (None, tmp_type) => (*tmp_var_idx, tmp_ops.clone(), Some(tmp_vector_elem_ptr_type), None, tmp_type),
                                }
                            },
                            _ => return Ok(value.clone()),
                        };
                        match value2.as_ref().unwrap_or(value) {
                            IrValue::Object(object) => Err(IrBlockError::NoFun),
                            value3 => {
                                if !ops.is_empty() {
                                    match new_var_idxs.get(&(var_idx)) {
                                        Some(new_var_idx) => {
                                            if new_var_idx - new_start_var_idx - var_tuples.len() < new_var_tuples.len() {
                                                match vector_elem_ptr_type {
                                                    Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(*new_var_idx, ops.clone(), vector_elem_ptr_type.clone()), None)))),
                                                    None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(*new_var_idx, ops.clone()), None)))),
                                                }
                                            } else {
                                                Err(IrBlockError::NoVarTuple)
                                            }
                                        },
                                        None => {
                                            let new_var_idx = new_start_var_idx + var_tuples.len() + new_var_tuples.len();
                                            new_var_tuples.push(VarTuple::new_with_value(Box::new(typ2.clone()), None, Some(new_var_idx), value3.clone()));
                                            new_var_idxs.insert(var_idx, new_var_idx);
                                            match vector_elem_ptr_type {
                                                Some(vector_elem_ptr_type) => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::RefLocal(new_var_idx, ops.clone(), vector_elem_ptr_type.clone()), None)))),
                                                None => Ok(IrValue::Object(Box::new(IrObject::Var(IrArgVar::Local(new_var_idx, ops.clone()), None)))),
                                            }
                                        },
                                    }
                                } else {
                                    Ok(value3.clone())
                                }
                            },
                        }
                    },
                    IrObject::Vector(values, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_value(value2, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Vector(new_values, typ.clone()))))
                    },
                    IrObject::Array(values, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_value(value2, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Array(new_values, typ.clone()))))
                    },
                    IrObject::Struct(values, field_pairs, typ) => {
                        let mut new_values: Vec<IrValue<IrArgVar>> = Vec::new();
                        for value2 in values {
                            new_values.push(self.substitute_value(value2, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?);
                        }
                        let mut new_field_pairs: Vec<IrFieldPair<IrArgVar>> = Vec::new();
                        for field_pair in field_pairs {
                            match field_pair {
                                IrFieldPair(var_idx, value) => new_field_pairs.push(IrFieldPair(*var_idx, self.substitute_value(value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
                            }
                        }
                        Ok(IrValue::Object(Box::new(IrObject::Struct(new_values, new_field_pairs, typ.clone()))))
                    },
                    IrObject::Union(var_idx, value2, typ) => {
                        let new_value = self.substitute_value(value2, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?;
                        Ok(IrValue::Object(Box::new(IrObject::Union(*var_idx, new_value, typ.clone()))))
                    },
                    IrObject::Closure(field_pairs, typ) => {
                        let mut new_field_pairs: Vec<IrFieldPair<IrArgVar>> = Vec::new();
                        for field_pair in field_pairs {
                            match field_pair {
                                IrFieldPair(var_idx, value) => new_field_pairs.push(IrFieldPair(*var_idx, self.substitute_value(value, old_start_var_idx, new_start_var_idx, substitutions, is_caller_fun_arg_change, is_closure_var_change, var_tuples, var_idxs, new_var_tuples, new_var_idxs)?)),
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
    
    fn substitute_from(&self, old_start_var_idx: usize, new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, ret_var: Option<&IrInstrVar>, poses: &[Pos], tree: &IrTree, is_caller_fun_arg_change: bool, is_closure_var_change: bool, old_var_idx: usize, new_var_idx: usize, block_idx: &mut usize, var_tuples: &mut Vec<VarTuple>, var_idxs: &mut BTreeMap<(usize, usize), usize>) -> Result<IrBlock, IrBlockError>
    {
        let mut new_block = IrBlock::new();
        let current_block_idx = *block_idx;
        let mut old_var_idx2 = old_var_idx;
        let mut new_var_idx2 = new_var_idx;
        for local_var_pair in &self.local_var_pairs {
            let is_var = match  substitutions.get(&(old_var_idx2, current_block_idx)) {
                Some(substitution) => substitution.has_var(),
                None => true,
            };
            if is_var {
                var_tuples.push(VarTuple::new(local_var_pair.1.clone(), Some(current_block_idx), Some(new_var_idx2)));
                var_idxs.insert((old_var_idx2, 0), new_var_idx2);
                new_block.add_local_var_pair(local_var_pair.clone());
                new_var_idx2 += 1;
            } else {
                var_tuples.push(VarTuple::new(local_var_pair.1.clone(), None, None));
            }
            old_var_idx2 += 1;
        }
        for instr in &self.instrs {
        }
        for _ in 0..(new_var_idx2 - new_var_idx) {
            var_tuples.pop();
        }
        Ok(new_block)
    }

    pub fn substitute(&self, old_start_var_idx: usize, var_types: &[Box<IrType>], new_start_var_idx: usize, substitutions: &BTreeMap<(usize, usize), VarSubstitution>, ret_var: Option<&IrInstrVar>, poses: &[Pos], tree: &IrTree, is_caller_fun_arg_change: bool, is_closure_var_change: bool) -> Result<IrBlock, IrBlockError>
    {
        let mut var_tuples: Vec<VarTuple> = Vec::new();
        let mut var_idxs: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        for i in 0..var_types.len() {
            var_tuples.push(VarTuple::new(var_types[i].clone(), Some(0), Some(i + new_start_var_idx)));
            var_idxs.insert((i + old_start_var_idx, 0), i + new_start_var_idx);
        }
        let mut block_idx = 1usize;
        self.substitute_from(old_start_var_idx, new_start_var_idx, substitutions, ret_var, poses, tree, is_caller_fun_arg_change, is_closure_var_change, old_start_var_idx + var_types.len(), new_start_var_idx + var_types.len(), &mut block_idx, &mut var_tuples, &mut var_idxs)
    }
}

#[derive(Debug)]
pub enum IrBlockError
{
    InvalidArgSubstitution,
    NoVarIndex,
    NoVarTuple,
    NoOldBlockIndex,
    NoFun,
}

impl error::Error for IrBlockError
{}

impl fmt::Display for IrBlockError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { 
        match self {
          IrBlockError::InvalidArgSubstitution => write!(f, "invalid argument substitution"),
          IrBlockError::NoVarIndex => write!(f, "no variable index"),
          IrBlockError::NoVarTuple => write!(f, "no variable tuple"),
          IrBlockError::NoOldBlockIndex => write!(f, "no old block index"),
          IrBlockError::NoFun => write!(f, "no function"),
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
