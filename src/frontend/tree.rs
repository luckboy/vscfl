//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::cell::*;
use std::error;
use std::fmt;
use std::rc::*;
use disjoint::DisjointSet;
use disjoint::DisjointSetVec;
use disjoint::disjoint_set_vec;
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
    Synonym(Vec<TypeArg>, Box<TypeExpr>, Option<Rc<TypeValue>>),
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
pub enum UniqFlag
{
    None,
    Uniq,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum SharedFlag
{
    None,
    Shared,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum DefinedFlag
{
    Undefined,
    Defined,
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

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TypeValueName
{
    Tuple,
    Array(Option<usize>),
    Fun,
    Name(String),
}

#[derive(Clone, Debug)]
pub enum TypeValue
{
    Param(UniqFlag, LocalType),
    Type(UniqFlag, TypeValueName, Vec<Rc<TypeValue>>),
}

impl TypeValue
{
    pub fn uniq_flag(&self) -> UniqFlag
    {
        match self {
            TypeValue::Param(uniq_flag, _) => *uniq_flag,
            TypeValue::Type(uniq_flag, _, _) => *uniq_flag,
        }
    }
    
    pub fn set_uniq_flag(&mut self, uniq_flag: UniqFlag)
    {
        match self {
            TypeValue::Param(uniq_flag2, _) => *uniq_flag2 = uniq_flag,
            TypeValue::Type(uniq_flag2, _, _) => *uniq_flag2 = uniq_flag,
        }
    }
    
    pub fn substitute(&self, type_values: &[Rc<TypeValue>]) -> Result<Option<Rc<TypeValue>>, TypeValueError>
    {
        match self {
            TypeValue::Param(UniqFlag::None, local_type) => {
                match type_values.get(local_type.index()) {
                    Some(type_value) => Ok(Some(type_value.clone())),
                    None => Err(TypeValueError),
                }
            },
            TypeValue::Param(UniqFlag::Uniq, local_type) => {
                match type_values.get(local_type.index()) {
                    Some(type_value) => {
                        match &**type_value {
                            TypeValue::Param(UniqFlag::None, local_type2) => Ok(Some(Rc::new(TypeValue::Param(UniqFlag::Uniq, *local_type2)))),
                            TypeValue::Type(UniqFlag::None, name, args) => Ok(Some(Rc::new(TypeValue::Type(UniqFlag::Uniq, name.clone(), args.clone())))),
                            _ => Ok(Some(type_value.clone())),
                        }
                    },
                    None => Err(TypeValueError),
                }
            },
            TypeValue::Type(uniq_flag, name, args) => {
                let mut new_args: Vec<Rc<TypeValue>> = Vec::new();
                let mut is_changed = false;
                for arg in args {
                    match arg.substitute(type_values)? {
                        Some(new_arg) => {
                            new_args.push(new_arg);
                            is_changed = true;
                        },
                        None => new_args.push(arg.clone()),
                    }
                }
                if is_changed {
                    Ok(Some(Rc::new(TypeValue::Type(*uniq_flag, name.clone(), new_args))))
                } else {
                    Ok(None)
                }
            },
        }
    }
}

#[derive(Debug)]
pub struct TypeValueError;

impl error::Error for TypeValueError
{}

impl fmt::Display for TypeValueError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "no type value for type parameter") }
}

#[derive(Clone, Debug)]
pub struct TypeParamEntry
{
    pub trait_names: BTreeSet<TraitName>,
    pub type_values: Vec<Rc<TypeValue>>,
    pub number: Option<usize>,
    pub ident: Option<Rc<String>>,
}

impl TypeParamEntry
{
    pub fn new_with_number(num: usize) -> Self
    {
        TypeParamEntry {
            trait_names: BTreeSet::new(),
            type_values: Vec::new(),
            number: Some(num),
            ident: None,
        }
    }
    
    pub fn new_with_ident(ident: String) -> Self
    {
        let num = if ident.starts_with("t") {
            match (&ident[1..]).parse::<usize>() {
                Ok(n) => Some(n),
                Err(_) => None
            }
        } else {
            None
        };
        TypeParamEntry {
            trait_names: BTreeSet::new(),
            type_values: Vec::new(),
            number: num,
            ident: Some(Rc::new(ident)),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Type
{
    type_value: Rc<TypeValue>,
    type_param_entries: Vec<Rc<RefCell<TypeParamEntry>>>,
    eq_type_param_set: DisjointSet,
}

impl Type
{
    pub fn new(type_value: Rc<TypeValue>, type_param_idents: &[String]) -> Self
    {
        Type {
            type_value,
            type_param_entries: type_param_idents.iter().map(|s| Rc::new(RefCell::new(TypeParamEntry::new_with_ident(s.clone())))).collect(),
            eq_type_param_set: DisjointSet::with_len(type_param_idents.len()),
        }
    }
    
    pub fn type_value(&self) -> &Rc<TypeValue>
    { &self.type_value }
    
    pub fn type_param_entries(&self) -> &[Rc<RefCell<TypeParamEntry>>]
    { self.type_param_entries.as_slice() }
    
    pub fn type_param_entry(&self, local_type: LocalType) -> Option<&Rc<RefCell<TypeParamEntry>>>
    { self.type_param_entries.get(local_type.index()) }

    pub fn eq_type_param_set(&self) -> &DisjointSet
    { &self.eq_type_param_set }
    
    pub fn has_eq_type_params(&self, local_type1: LocalType, local_type2: LocalType) -> bool
    { self.eq_type_param_set.is_joined(local_type1.index(), local_type2.index()) }

    pub fn set_eq_type_params(&mut self, local_type1: LocalType, local_type2: LocalType) -> LocalType
    {
        self.eq_type_param_set.join(local_type1.index(), local_type2.index());
        LocalType::new(self.eq_type_param_set.root_of(local_type1.index()))
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LocalType
{
    pub(crate) index: usize,
}

impl LocalType
{
    pub fn new(idx: usize) -> Self
    { LocalType { index: idx, } }
    
    pub fn index(&self) -> usize
    { self.index }
}

#[derive(Clone, Debug)]
pub enum LocalTypeEntry
{
    Param(DefinedFlag, UniqFlag, Rc<RefCell<TypeParamEntry>>, LocalType),
    Type(Rc<TypeValue>),
}

#[derive(Clone, Debug)]
pub struct EqTypeParamEntry
{
    pub type_value_name: Option<TypeValueName>,
}

impl EqTypeParamEntry
{
    pub fn new() -> EqTypeParamEntry
    { EqTypeParamEntry { type_value_name: None, } }
}

#[derive(Clone, Debug)]
pub struct LocalTypes
{
    type_entries: DisjointSetVec<LocalTypeEntry>,
    eq_type_param_entries: DisjointSetVec<EqTypeParamEntry>,
    type_param_numbers: BTreeSet<usize>,
    type_param_number_counter: usize,
}

impl LocalTypes
{
    pub fn new() -> Self
    {
        LocalTypes {
            type_entries: DisjointSetVec::new(),
            eq_type_param_entries: DisjointSetVec::new(),
            type_param_numbers: BTreeSet::new(),
            type_param_number_counter: 1,
        }
    }
    
    pub fn type_entries(&self) -> &DisjointSetVec<LocalTypeEntry>
    { &self.type_entries }
    
    pub fn type_entry(&self, local_type: LocalType) -> Option<&LocalTypeEntry>
    {
        if local_type.index() < self.type_entries.len() {
            let root_idx = self.type_entries.root_of(local_type.index());
            Some(&self.type_entries[root_idx])
        } else {
            None
        }
    }
        
    pub fn eq_type_param_entries(&self) -> &DisjointSetVec<EqTypeParamEntry>
    { &self.eq_type_param_entries }

    pub fn eq_type_param_entry(&self, local_type: LocalType) -> Option<&EqTypeParamEntry>
    {
        if local_type.index() < self.eq_type_param_entries.len() {
            let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
            Some(&self.eq_type_param_entries[eq_root_idx])
        } else {
            None
        }
    }
    
    pub fn has_eq_type_params(&self, local_type1: LocalType, local_type2: LocalType) -> bool
    { self.eq_type_param_entries.is_joined(local_type1.index(), local_type2.index()) }
    
    pub fn type_entry_for_type_value(&self, type_value: &Rc<TypeValue>) -> Option<LocalTypeEntry>
    {
        match &**type_value {
            TypeValue::Param(uniq_flag, local_type) => {
                if local_type.index() < self.type_entries.len() {
                    let root_idx = self.type_entries.root_of(local_type.index());
                    match &self.type_entries[root_idx] {
                        LocalTypeEntry::Param(defined_flag, uniq_flag2, type_param_entry, local_type) => {
                            let new_uniq_flag = if *uniq_flag == UniqFlag::Uniq || *uniq_flag2 == UniqFlag::Uniq {
                                UniqFlag::Uniq
                            } else {
                                UniqFlag::None
                            };
                            let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
                            match &self.eq_type_param_entries[eq_root_idx].type_value_name {
                                Some(type_value_name) => {
                                    let type_param_entry_r = type_param_entry.borrow();
                                    Some(LocalTypeEntry::Type(Rc::new(TypeValue::Type(*uniq_flag, type_value_name.clone(), type_param_entry_r.type_values.clone()))))
                                },
                                None => Some(LocalTypeEntry::Param(*defined_flag, new_uniq_flag, type_param_entry.clone(), *local_type))
                            }
                        },
                        LocalTypeEntry::Type(type_value2) => {
                            match self.type_entry_for_type_value(type_value2) {
                                Some(LocalTypeEntry::Param(defined_flag, uniq_flag2, type_param_entry, local_type)) => {
                                    let new_uniq_flag = if *uniq_flag == UniqFlag::Uniq || uniq_flag2 == UniqFlag::Uniq {
                                        UniqFlag::Uniq
                                    } else {
                                        UniqFlag::None
                                    };
                                    Some(LocalTypeEntry::Param(defined_flag, new_uniq_flag, type_param_entry, local_type))
                                },
                                Some(type_entry) => Some(type_entry),
                                None => None,
                            }
                        },
                    }
                } else {
                    None
                }
            },
            TypeValue::Type(_, _, _) => Some(LocalTypeEntry::Type(type_value.clone())),
        }
    }    
    
    fn set_defined_type_params_for_type(&mut self, typ: &Type)
    {
        self.type_entries = DisjointSetVec::new();
        self.type_param_numbers = BTreeSet::new();
        for type_param_entry in &typ.type_param_entries {
            let tmp_local_type = LocalType::new(self.type_entries.len());
            self.type_entries.push(LocalTypeEntry::Param(DefinedFlag::Defined, UniqFlag::None, type_param_entry.clone(), tmp_local_type));
            let type_param_entry_r = type_param_entry.borrow();
            match type_param_entry_r.number {
                Some(num) => {
                    self.type_param_numbers.insert(num);
                },
                None => (),
            }
        }
        self.eq_type_param_entries = disjoint_set_vec![EqTypeParamEntry::new(); typ.eq_type_param_set.len()];
        for i in 0..typ.eq_type_param_set.len() {
            for j in (i + 1)..typ.eq_type_param_set.len() {
                if typ.eq_type_param_set.is_joined(i, j) {
                    self.eq_type_param_entries.join(i, j);
                }
            }
        }
    }
    
    pub fn set_defined_type(&mut self, typ: &Type) -> LocalType
    {
        self.set_defined_type_params_for_type(typ);
        let local_type = LocalType::new(self.type_entries.len());
        self.type_entries.push(LocalTypeEntry::Type(typ.type_value.clone()));
        self.eq_type_param_entries.push(EqTypeParamEntry::new());
        local_type
    }
    
    pub fn set_defined_fun_types(&mut self, typ: &Type) -> Option<Vec<LocalType>>
    {
        match &*typ.type_value {
            TypeValue::Type(_, TypeValueName::Fun, type_values) => {
                self.set_defined_type_params_for_type(typ);
                let mut local_types: Vec<LocalType> = Vec::new();
                for type_value in type_values {
                    let local_type = LocalType::new(self.type_entries.len());
                    self.type_entries.push(LocalTypeEntry::Type(type_value.clone()));
                    self.eq_type_param_entries.push(EqTypeParamEntry::new());
                    local_types.push(local_type);
                }
                Some(local_types)
            },
            _ => None,
        }
    }
    
    fn set_new_value_for_type_param_number_counter(&mut self)
    {
        while self.type_param_numbers.contains(&self.type_param_number_counter) {
            self.type_param_number_counter += 1;
        }
    }    
    
    pub fn set_type(&mut self, local_type: LocalType, typ: &Type) -> Result<bool, TypeValueError>
    {
        if local_type.index() < self.type_entries.len() {
            let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
            let idx = self.type_entries.len();
            for type_param_entry in &typ.type_param_entries {
                self.set_new_value_for_type_param_number_counter();
                let tmp_local_type = LocalType::new(self.type_entries.len());
                type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, tmp_local_type)));
                let type_param_entry_r = type_param_entry.borrow();
                let mut new_type_param_entry = type_param_entry_r.clone();
                new_type_param_entry.number = Some(self.type_param_number_counter);
                new_type_param_entry.ident = None;
                self.type_entries.push(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, Rc::new(RefCell::new(new_type_param_entry)), tmp_local_type));
                self.eq_type_param_entries.push(EqTypeParamEntry::new());
                self.type_param_number_counter += 1;
            }
            for i in idx..(idx + typ.eq_type_param_set.len()) {
                for j in (i + 1)..(idx + typ.eq_type_param_set.len()) {
                    if typ.eq_type_param_set.is_joined(i, j) {
                        self.eq_type_param_entries.join(i, j);
                    }
                }
            }
            let root_idx = self.type_entries.root_of(local_type.index());
            match typ.type_value.substitute(type_values.as_slice())? {
                Some(new_type_value) => self.type_entries[root_idx] = LocalTypeEntry::Type(new_type_value),
                None => self.type_entries[root_idx] = LocalTypeEntry::Type(typ.type_value.clone()),
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn set_type_param_entry(&mut self, local_type: LocalType, type_param_entry: Rc<RefCell<TypeParamEntry>>) -> bool
    {
        if local_type.index() < self.type_entries.len() {
            let root_idx = self.type_entries.root_of(local_type.index());
            self.type_entries[root_idx] = LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, LocalType::new(root_idx));
            true
        } else {
            false
        }
    }
    
    pub fn set_type_value(&mut self, local_type: LocalType, type_value: Rc<TypeValue>) -> bool
    {
        if local_type.index() < self.type_entries.len() {
            let root_idx = self.type_entries.root_of(local_type.index());
            match &*type_value {
                TypeValue::Type(_, type_value_name, _) => {
                    let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
                    self.eq_type_param_entries[eq_root_idx].type_value_name = Some(type_value_name.clone());
                },
                _ => (),
            }
            self.type_entries[root_idx] = LocalTypeEntry::Type(type_value);
            true
        } else {
            false
        }
    }
    
    pub fn set_shared_for_type_param(&mut self, local_type: LocalType) -> bool
    {
        if local_type.index() < self.type_entries.len() {
            let root_idx = self.type_entries.root_of(local_type.index());
            match &self.type_entries[root_idx] {
                LocalTypeEntry::Param(_, _, type_param_entry, _) => {
                    let mut type_param_entry_r = type_param_entry.borrow_mut();
                    type_param_entry_r.trait_names.insert(TraitName::Shared);
                    true
                },
                _ => false,
            }
        } else {
            false
        }
    }
    
    pub fn add_type_param(&mut self, type_param_entry: Rc<RefCell<TypeParamEntry>>) -> LocalType
    {
        self.set_new_value_for_type_param_number_counter();
        let local_type = LocalType::new(self.type_entries.len());
        {
            let mut type_param_entry_r = type_param_entry.borrow_mut();
            type_param_entry_r.number = Some(self.type_param_number_counter);
            type_param_entry_r.ident = None;
        }
        self.type_entries.push(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, local_type));
        self.eq_type_param_entries.push(EqTypeParamEntry::new());
        self.type_param_number_counter += 1;
        local_type
    }
    
    pub fn add_type_value(&mut self, type_value: Rc<TypeValue>) -> LocalType
    {
        self.set_new_value_for_type_param_number_counter();
        let local_type = LocalType::new(self.type_entries.len());
        self.type_entries.push(LocalTypeEntry::Type(type_value));
        self.eq_type_param_entries.push(EqTypeParamEntry::new());
        self.type_param_number_counter += 1;
        local_type
    }
    
    pub fn join_local_types(&mut self, local_type1: LocalType, local_type2: LocalType) -> (LocalType, LocalType)
    {
        self.type_entries.join(local_type1.index(), local_type2.index());
        self.eq_type_param_entries.join(local_type1.index(), local_type2.index());
        (LocalType::new(self.type_entries.root_of(local_type1.index())), LocalType::new(self.eq_type_param_entries.root_of(local_type2.index())))
    }
}

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
