//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::error;
use std::fmt;
use std::rc::*;
use disjoint::DisjointSet;
use disjoint::DisjointSetVec;
use disjoint::disjoint_set_vec;
use crate::frontend::error::FrontendResult;
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
 
    pub fn add_def(&mut self, def: Def)
    { self.defs.push(Box::new(def)); }
    
    pub fn type_vars(&self) -> &HashMap<String, Rc<RefCell<TypeVar>>>
    { &self.type_vars }
    
    pub fn type_var(&self, ident: &String) -> Option<&Rc<RefCell<TypeVar>>>
    { self.type_vars.get(ident) }

    pub fn add_type_var(&mut self, ident: String, type_var: Rc<RefCell<TypeVar>>)
    { self.type_vars.insert(ident, type_var); }
    
    pub fn vars(&self) -> &HashMap<String, Rc<RefCell<Var>>>
    { &self.vars }

    pub fn var(&self, ident: &String) -> Option<&Rc<RefCell<Var>>>
    { self.vars.get(ident) }

    pub fn add_var(&mut self, ident: String, var: Rc<RefCell<Var>>)
    { self.vars.insert(ident, var); }
    
    pub fn traits(&self) -> &HashMap<String, Rc<RefCell<Trait>>>
    { &self.traits }
    
    pub fn trait1(&self, ident: &String) -> Option<&Rc<RefCell<Trait>>>
    { self.traits.get(ident) }

    pub fn add_trait(&mut self, ident: String, trait1: Rc<RefCell<Trait>>)
    { self.traits.insert(ident, trait1); }
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
    Builtin(Option<Box<TypeArgs>>, Option<Box<Fields>>, Option<SharedFlag>),
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
    Var(VarModifier, Box<TypeExpr>, Vec<WhereTuple>, Option<Box<Expr>>, Option<String>, Option<LocalType>, Option<Box<LocalTypes>>, Option<Box<Type>>, Option<Value>),
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

impl fmt::Display for TraitName
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            TraitName::Shared => write!(f, "shared"),
            TraitName::Fun => write!(f, "->"),
            TraitName::Name(ident) => write!(f, "{}", ident),
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypeParam(pub String, pub Pos);

#[derive(Clone, Debug)]
pub enum Expr
{
    Literal(Box<Literal<Expr>>, Option<LocalType>, Pos),
    Lambda(Vec<LambdaArg>, Option<Box<TypeExpr>>, Box<Expr>, Option<LocalType>, Option<LocalType>, Option<LocalFun>, Option<Box<Closure>>, Pos),
    Var(String, Option<LocalType>, Pos),
    NamedFieldConApp(String, Vec<NamedFieldPair<Expr>>, Option<LocalType>, Option<LocalType>, Pos),
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
    Unnamed(usize, Option<LocalType>),
    Named(String, Option<LocalType>),
}

impl fmt::Display for Field
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            Field::Unnamed(idx, _) => write!(f, "{}", idx),
            Field::Named(ident, _) => write!(f, "{}", ident),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Bind(pub Box<Pattern>, pub Box<Expr>);

#[derive(Clone, Debug)]
pub struct Case(pub Box<Pattern>, pub Box<Expr>);

#[derive(Clone, Debug)]
pub enum Pattern
{
    Literal(Box<Literal<Pattern>>, Option<LocalType>, Pos),
    As(Box<Literal<Pattern>>, Box<TypeExpr>, Option<LocalType>, Option<LocalType>, Pos),
    Const(String, Option<LocalType>, Pos),
    UnnamedFieldCon(String, Vec<Box<Pattern>>, Option<LocalType>, Option<LocalType>, Pos),
    NamedFieldCon(String, Vec<NamedFieldPair<Pattern>>, Option<LocalType>, Option<LocalType>, Pos),
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

impl TypeName
{
    pub fn to_type_value_name(&self) -> TypeValueName
    {
        match self {
            TypeName::Tuple(_) => TypeValueName::Tuple,
            TypeName::Array(len) => TypeValueName::Array(*len),
            TypeName::Fun(_) => TypeValueName::Fun,
            TypeName::Name(ident) => TypeValueName::Name(ident.clone()),
        }
    }
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
    Var(Box<Expr>, Option<LocalType>, Option<Box<LocalTypes>>, Option<Box<Type>>, Option<Value>),
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
pub struct TypeArgs
{
    type_arg_idents: Vec<String>,
}

impl TypeArgs
{
    pub fn new() -> Self
    { TypeArgs { type_arg_idents: Vec::new(), } }

    pub fn type_arg_idents(&self) -> &[String]
    { self.type_arg_idents.as_slice() }
    
    pub fn add_type_arg_ident(&mut self, ident: String)
    { self.type_arg_idents.push(ident); }
}

#[derive(Clone, Debug)]
pub struct Fields
{
    field_type_values: Vec<Rc<TypeValue>>,
    field_indices: BTreeMap<String, usize>,
}

impl Fields
{
    pub fn new() -> Self
    { Fields { field_type_values: Vec::new(), field_indices: BTreeMap::new(), } }

    pub fn field_type_values(&self) -> &[Rc<TypeValue>]
    { self.field_type_values.as_slice() }

    pub fn add_field_type_value(&mut self, type_value: Rc<TypeValue>)
    { self.field_type_values.push(type_value); }
    
    pub fn field_indices(&self) -> &BTreeMap<String, usize>
    { &self.field_indices }
    
    pub fn field_index(&self, ident: &String) -> Option<usize>
    {
       match self.field_indices.get(ident) {
           Some(i) => Some(*i),
           None => None,
       }
    }
    
    pub fn add_field_index(&mut self, ident: String, field_idx: usize)
    { self.field_indices.insert(ident, field_idx); }
}

#[derive(Clone, Debug)]
pub struct NamedFields
{
    field_indices: BTreeMap<String, usize>,
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
    
    pub fn add_field_index(&mut self, ident: String, field_idx: usize)
    { self.field_indices.insert(ident, field_idx); }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum TypeValueName
{
    Tuple,
    Fun,
    Array(Option<usize>),
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

    pub fn type_name(&self) -> Option<TypeName>
    {
        match self {
            TypeValue::Param(_, _) => None,
            TypeValue::Type(_, TypeValueName::Tuple, args) => Some(TypeName::Tuple(args.len())),
            TypeValue::Type(_, TypeValueName::Fun, args) => {
                if args.len() >= 1 {
                    Some(TypeName::Fun(args.len() - 1))
                } else {
                    None
                }
            },
            TypeValue::Type(_, TypeValueName::Array(len), _) => Some(TypeName::Array(*len)),
            TypeValue::Type(_, TypeValueName::Name(ident), _) => Some(TypeName::Name(ident.clone())),
        }
    }
    
    fn add_to_string<F>(&self, s: &mut String, f: &mut F)
        where F: FnMut(&Self, &mut String) -> Option<Self>
    {
        match f(self, s) {
            Some(TypeValue::Param(uniq_flag, local_type)) => {
                if uniq_flag == UniqFlag::Uniq {
                    s.push_str("uniq ");
                }
                s.push_str(format!("{{local type: {}}}", local_type.index()).as_str());
            },
            Some(TypeValue::Type(uniq_flag, name, args)) => {
                if uniq_flag == UniqFlag::Uniq {
                    s.push_str("uniq ");
                }
                match name {
                    TypeValueName::Tuple => {
                        s.push('(');
                        let mut is_first = true;
                        for arg in &args {
                            if !is_first {
                                s.push_str(", ");
                            }
                            arg.add_to_string(s, f);
                            is_first = false;
                        }
                        s.push(')');
                    },
                    TypeValueName::Array(len) => {
                        s.push('[');
                        args[0].add_to_string(s, f);
                        s.push_str("; ");
                        match len {
                            Some(len) => s.push_str(format!("{}", len).as_str()),
                            None => s.push('_'),
                        }
                        s.push(']');
                    },
                    TypeValueName::Fun => {
                        s.push('(');
                        let mut is_first = true;
                        for arg in &args[0..(args.len() - 1)] {
                            if !is_first {
                                s.push_str(", ");
                            }
                            arg.add_to_string(s, f);
                            is_first = false;
                        }
                        s.push_str(") -> ");
                        args[args.len() - 1].add_to_string(s, f);                        
                    },
                    TypeValueName::Name(ident) => {
                        s.push_str(ident.as_str());
                        if !args.is_empty() {
                            s.push('<');
                            let mut is_first = true;
                            for arg in &args {
                                if !is_first {
                                    s.push_str(", ");
                                }
                                arg.add_to_string(s, f);
                                is_first = false;
                            }
                            s.push('>');
                        }
                    },
                }
            },
            None => (),
        }
    }
    
    pub fn to_string<F>(&self, mut f: F) -> String
        where F: FnMut(&Self, &mut String) -> Option<Self>
    {
        let mut s = String::new();
        self.add_to_string(&mut s, &mut f);
        s
    }
    
    pub fn to_string_without_fun(&self) -> String
    {
        self.to_string(|type_value, s| {
                match type_value {
                    TypeValue::Param(uniq_flag, local_type)  => {
                        if *uniq_flag == UniqFlag::Uniq {
                            s.push_str("uniq ");
                        }
                        s.push_str(format!("t{}", local_type.index() + 1).as_str());
                        None
                    },
                    TypeValue::Type(_, _, _) => Some(type_value.clone())
                }
        })
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
    pub closure_local_types: BTreeSet<LocalType>,
    pub number: Option<usize>,
    pub ident: Option<String>,
    pub pos: Option<Pos>,
}

impl TypeParamEntry
{
    pub fn new() -> Self
    {
        TypeParamEntry {
            trait_names: BTreeSet::new(),
            type_values: Vec::new(),
            closure_local_types: BTreeSet::new(),
            number: None,
            ident: None,
            pos: None,
        }
    }

    pub fn new_with_number(num: usize) -> Self
    {
        TypeParamEntry {
            trait_names: BTreeSet::new(),
            type_values: Vec::new(),
            closure_local_types: BTreeSet::new(),
            number: Some(num),
            ident: None,
            pos: None,
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
            closure_local_types: BTreeSet::new(),
            number: num,
            ident: Some(ident),
            pos: None,
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

    pub fn new_with_type_param_entry_count(type_value: Rc<TypeValue>, count: usize) -> Self
    {
        Type {
            type_value,
            type_param_entries: (0..count).map(|_| Rc::new(RefCell::new(TypeParamEntry::new()))).collect(),
            eq_type_param_set: DisjointSet::with_len(count),
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
    {
        if local_type1.index() < self.eq_type_param_set.len() && local_type2.index() < self.eq_type_param_set.len() {
            self.eq_type_param_set.is_joined(local_type1.index(), local_type2.index())
        } else {
            false
        }
    }

    pub fn set_eq_type_params(&mut self, local_type1: LocalType, local_type2: LocalType) -> Option<LocalType>
    {
        if local_type1.index() < self.eq_type_param_set.len() && local_type2.index() < self.eq_type_param_set.len() {
            self.eq_type_param_set.join(local_type1.index(), local_type2.index());
            Some(LocalType::new(self.eq_type_param_set.root_of(local_type1.index())))
        } else {
            None
        }
    }
    
    pub fn add_type_params(&mut self, type_param_idents: &[String])
    {
        for type_param_ident in type_param_idents {
            self.type_param_entries.push(Rc::new(RefCell::new(TypeParamEntry::new_with_ident(type_param_ident.clone()))));
            self.eq_type_param_set.add_singleton();
        }
    }
    
    pub fn to_string(&self) -> String
    {
        self.type_value.to_string(|type_value, s| {
                match type_value {
                    TypeValue::Param(uniq_flag, local_type) => {
                        match self.type_param_entry(*local_type) {
                            Some(type_param_entry) => {
                                let type_param_entry_r = type_param_entry.borrow();
                                if let Some(ident) = &type_param_entry_r.ident {
                                    if *uniq_flag == UniqFlag::Uniq {
                                        s.push_str("uniq ");
                                    }
                                    s.push_str(ident.as_str());
                                    None
                                } else if let Some(num) = &type_param_entry_r.number {
                                    if *uniq_flag == UniqFlag::Uniq {
                                        s.push_str("uniq ");
                                    }
                                    s.push_str(format!("t{}", *num).as_str());
                                    None
                                } else {
                                    Some(type_value.clone())
                                }
                            },
                            None => Some(type_value.clone()),
                        }
                    },
                    _ => Some(type_value.clone()),
                }
        })
    }
}

impl fmt::Display for Type
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "{}", self.to_string()) }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LocalType
{
    index: usize,
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
    pub is_in_non_uniq_lambda: bool,
    pub is_defined: bool,
    pub local_types: BTreeSet<LocalType>,
}

impl EqTypeParamEntry
{
    pub fn new() -> EqTypeParamEntry
    {
        EqTypeParamEntry {
            type_value_name: None,
            is_in_non_uniq_lambda: false,
            is_defined: false,
            local_types: BTreeSet::new(),
        }
    }

    pub fn defined_new() -> EqTypeParamEntry
    {
        EqTypeParamEntry {
            type_value_name: None,
            is_in_non_uniq_lambda: false,
            is_defined: true,
            local_types: BTreeSet::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct LocalTypes
{
    type_entries: DisjointSetVec<LocalTypeEntry>,
    eq_type_param_entries: DisjointSetVec<EqTypeParamEntry>,
    orig_eq_type_param_set: DisjointSet,
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
            orig_eq_type_param_set: DisjointSet::new(),
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
    {
        if local_type1.index() < self.eq_type_param_entries.len() && local_type2.index() < self.eq_type_param_entries.len() {
            self.eq_type_param_entries.is_joined(local_type1.index(), local_type2.index())
        } else {
            false
        }
    }

    pub fn orig_eq_type_param_set(&self) -> &DisjointSet
    { &self.orig_eq_type_param_set }
    
    pub fn has_orig_eq_type_params(&self, local_type1: LocalType, local_type2: LocalType) -> bool
    {
        if local_type1.index() < self.orig_eq_type_param_set.len() && local_type2.index() < self.orig_eq_type_param_set.len() {
            self.orig_eq_type_param_set.is_joined(local_type1.index(), local_type2.index())
        } else {
            false
        }
    }
    
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
                                Some(LocalTypeEntry::Type(type_value3)) => {
                                    match &*type_value3 {
                                        TypeValue::Param(_, _) => None,
                                        TypeValue::Type(uniq_flag2, type_value_name, type_values) => {
                                            let new_uniq_flag = if *uniq_flag == UniqFlag::Uniq || *uniq_flag2 == UniqFlag::Uniq {
                                                UniqFlag::Uniq
                                            } else {
                                                UniqFlag::None
                                            };
                                            if new_uniq_flag != *uniq_flag2 {
                                                Some(LocalTypeEntry::Type(Rc::new(TypeValue::Type(new_uniq_flag, type_value_name.clone(), type_values.clone()))))
                                            } else {
                                                Some(LocalTypeEntry::Type(type_value3))
                                            }
                                        },
                                    }
                                },
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
        self.eq_type_param_entries = disjoint_set_vec![EqTypeParamEntry::defined_new(); typ.eq_type_param_set.len()];
        for i in 0..typ.eq_type_param_set.len() {
            for j in (i + 1)..typ.eq_type_param_set.len() {
                if typ.eq_type_param_set.is_joined(i, j) {
                    self.eq_type_param_entries.join(i, j);
                    let eq_root_idx = self.eq_type_param_entries.root_of(i);
                    if eq_root_idx == i {
                        self.eq_type_param_entries[j].local_types.clear();
                        self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(j));
                    } else if eq_root_idx == j {
                        self.eq_type_param_entries[i].local_types.clear();
                        self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(i));
                    } else {
                        self.eq_type_param_entries[i].local_types.clear();
                        self.eq_type_param_entries[j].local_types.clear();
                        self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(i));
                        self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(j));
                    }
                }
            }
        }
        self.orig_eq_type_param_set = typ.eq_type_param_set.clone();
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
            let mut new_type_param_entries: Vec<Rc<RefCell<TypeParamEntry>>> = Vec::new();
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
                let tmp_type_param_entry = Rc::new(RefCell::new(new_type_param_entry));
                new_type_param_entries.push(tmp_type_param_entry.clone());
                self.type_entries.push(LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, tmp_type_param_entry, tmp_local_type));
                self.eq_type_param_entries.push(EqTypeParamEntry::new());
                self.type_param_number_counter += 1;
            }
            for i in idx..(idx + typ.eq_type_param_set.len()) {
                for j in (i + 1)..(idx + typ.eq_type_param_set.len()) {
                    if typ.eq_type_param_set.is_joined(i - idx, j - idx) {
                        self.eq_type_param_entries.join(i, j);
                        let eq_root_idx = self.eq_type_param_entries.root_of(i);
                        if eq_root_idx == i {
                            self.eq_type_param_entries[j].local_types.clear();
                            self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(j));
                        } else if eq_root_idx == j {
                            self.eq_type_param_entries[i].local_types.clear();
                            self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(i));
                        } else {
                            self.eq_type_param_entries[i].local_types.clear();
                            self.eq_type_param_entries[j].local_types.clear();
                            self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(i));
                            self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(j));
                        }
                    }
                }
            }
            let root_idx = self.type_entries.root_of(local_type.index());
            match typ.type_value.substitute(type_values.as_slice())? {
                Some(new_type_value) => self.type_entries[root_idx] = LocalTypeEntry::Type(new_type_value),
                None => self.type_entries[root_idx] = LocalTypeEntry::Type(typ.type_value.clone()),
            }
            for new_type_param_entry in &new_type_param_entries {
                let mut new_type_param_entry_r = new_type_param_entry.borrow_mut();
                for type_value in &mut new_type_param_entry_r.type_values {
                    match type_value.substitute(type_values.as_slice())? {
                        Some(new_type_value) => *type_value = new_type_value,
                        None => (),
                    }
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn set_type_param(&mut self, local_type: LocalType, type_param_entry: Rc<RefCell<TypeParamEntry>>) -> bool
    {
        if local_type.index() < self.type_entries.len() {
            let root_idx = self.type_entries.root_of(local_type.index());
            match &self.type_entries[root_idx] {
                LocalTypeEntry::Param(_, _, old_type_param_entry, _) => {
                    {
                        let old_type_param_entry_r = old_type_param_entry.borrow();
                        let mut type_param_entry_r = type_param_entry.borrow_mut();
                        type_param_entry_r.number = old_type_param_entry_r.number;
                        type_param_entry_r.ident = old_type_param_entry_r.ident.clone();
                    }
                    self.type_entries[root_idx] = LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, LocalType::new(root_idx));
                },
                _ => {
                    self.set_new_value_for_type_param_number_counter();
                    {
                        let mut type_param_entry_r = type_param_entry.borrow_mut();
                        type_param_entry_r.number = Some(self.type_param_number_counter);
                        type_param_entry_r.ident = None;
                    }
                    self.type_entries[root_idx] = LocalTypeEntry::Param(DefinedFlag::Undefined, UniqFlag::None, type_param_entry, LocalType::new(root_idx));
                },
            }
            true
        } else {
            false
        }
    }

    pub fn set_type_param_entry(&mut self, local_type: LocalType, type_param_entry: Rc<RefCell<TypeParamEntry>>, defined_flag: DefinedFlag) -> bool
    {
        if local_type.index() < self.type_entries.len() {
            let root_idx = self.type_entries.root_of(local_type.index());
            self.type_entries[root_idx] = LocalTypeEntry::Param(defined_flag, UniqFlag::None, type_param_entry, LocalType::new(root_idx));
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
    
    pub fn set_uniq(&mut self, local_type: LocalType) -> bool
    {
        if local_type.index() < self.type_entries.len() {
            let root_idx = self.type_entries.root_of(local_type.index());
            match &self.type_entries[root_idx] {
                LocalTypeEntry::Param(defined_flag, uniq_flag, type_param_entry, _) => {
                    let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
                    let new_local_type = LocalType::new(self.type_entries.len());
                    self.type_entries.push(LocalTypeEntry::Param(*defined_flag, *uniq_flag, type_param_entry.clone(), new_local_type));
                    self.eq_type_param_entries.push(EqTypeParamEntry::new());
                    let eq_local_types = self.eq_type_param_entries[eq_root_idx].local_types.clone();
                    self.eq_type_param_entries.join(new_local_type.index(), eq_root_idx);
                    for eq_local_type in &eq_local_types {
                        self.eq_type_param_entries.join(new_local_type.index(), eq_local_type.index());
                    }
                    let new_eq_root_idx = self.eq_type_param_entries.root_of(new_local_type.index());
                    if new_eq_root_idx != eq_root_idx {
                        self.eq_type_param_entries[eq_root_idx].local_types.insert(LocalType::new(self.type_entries.root_of(eq_root_idx)));
                        self.eq_type_param_entries[eq_root_idx].local_types.remove(&LocalType::new(root_idx));
                        self.eq_type_param_entries[new_eq_root_idx] = self.eq_type_param_entries[eq_root_idx].clone();
                        self.eq_type_param_entries[eq_root_idx].local_types.clear();
                    } else {
                        if self.type_entries.root_of(eq_root_idx) != root_idx {
                            self.eq_type_param_entries[eq_root_idx].local_types.remove(&LocalType::new(root_idx));
                            self.eq_type_param_entries[eq_root_idx].local_types.insert(new_local_type);
                        }
                    }
                    self.type_entries[root_idx] = LocalTypeEntry::Type(Rc::new(TypeValue::Param(UniqFlag::Uniq, new_local_type)));
                    self.eq_type_param_entries[eq_root_idx] = EqTypeParamEntry::new();
                },
                LocalTypeEntry::Type(type_value) => {
                    let mut new_type_value = (**type_value).clone();
                    new_type_value.set_uniq_flag(UniqFlag::Uniq);
                    self.type_entries[root_idx] = LocalTypeEntry::Type(Rc::new(new_type_value));
                },
            }
            true
        } else {
            false
        }
    }

    pub fn has_in_non_uniq_lambda(&self, local_type: LocalType) -> bool
    {
        if local_type.index() < self.eq_type_param_entries.len() {
            let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
            self.eq_type_param_entries[eq_root_idx].is_in_non_uniq_lambda
        } else {
            false
        }
    }

    pub fn set_in_non_uniq_lambda(&mut self, local_type: LocalType, is_in_non_uniq_lambda: bool) -> bool
    {
        if local_type.index() < self.eq_type_param_entries.len() {
            let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
            self.eq_type_param_entries[eq_root_idx].is_in_non_uniq_lambda = is_in_non_uniq_lambda;
            true
        } else {
            false
        }
    }

    pub fn has_defined_type_param_eq(&self, local_type: LocalType) -> bool
    {
        if local_type.index() < self.eq_type_param_entries.len() {
            let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
            self.eq_type_param_entries[eq_root_idx].is_defined
        } else {
            false
        }
    }

    pub fn set_defined_type_param_eq(&mut self, local_type: LocalType, is_defined: bool) -> bool
    {
        if local_type.index() < self.eq_type_param_entries.len() {
            let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
            self.eq_type_param_entries[eq_root_idx].is_defined = is_defined;
            true
        } else {
            false
        }
    }
    
    pub fn eq_root_local_type_and_eq_local_types(&self, local_type: LocalType) -> Option<(LocalType, &BTreeSet<LocalType>)>
    {
        if local_type.index() < self.eq_type_param_entries.len() {
            let eq_root_idx = self.eq_type_param_entries.root_of(local_type.index());
            Some((LocalType::new(eq_root_idx), &self.eq_type_param_entries[eq_root_idx].local_types))
        } else {
            None
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
        local_type
    }
    
    pub fn join_local_types(&mut self, local_type1: LocalType, local_type2: LocalType) -> Option<(LocalType, LocalType)>
    {
        if local_type1.index() < self.type_entries.len() && local_type1.index() < self.eq_type_param_entries.len() && local_type2.index() < self.type_entries.len() && local_type2.index() < self.eq_type_param_entries.len() {
            let root_idx1 = self.type_entries.root_of(local_type1.index());
            let root_idx2 = self.type_entries.root_of(local_type2.index());
            let eq_root_idx1 = self.eq_type_param_entries.root_of(local_type1.index());
            let eq_root_idx2 = self.eq_type_param_entries.root_of(local_type2.index());
            self.type_entries.join(local_type1.index(), local_type2.index());
            self.eq_type_param_entries.join(local_type1.index(), local_type2.index());
            let root_idx = self.type_entries.root_of(local_type1.index());
            let eq_root_idx = self.eq_type_param_entries.root_of(local_type1.index());
            let mut eq_local_types: BTreeSet<LocalType> = self.eq_type_param_entries[eq_root_idx1].local_types.union(&self.eq_type_param_entries[eq_root_idx2].local_types).map(|e| e.clone()).collect();
            self.eq_type_param_entries[eq_root_idx1].local_types.clear();
            self.eq_type_param_entries[eq_root_idx2].local_types.clear();
            match &self.type_entries[self.type_entries.root_of(eq_root_idx1)] {
                LocalTypeEntry::Param(_, _, _, _) => {
                    eq_local_types.insert(LocalType::new(self.type_entries.root_of(eq_root_idx1)));
                },
                _ => (),
            }
            match &self.type_entries[self.type_entries.root_of(eq_root_idx2)] {
                LocalTypeEntry::Param(_, _, _, _) => {
                    eq_local_types.insert(LocalType::new(self.type_entries.root_of(eq_root_idx2)));
                },
                _ => (),
            }
            eq_local_types.remove(&LocalType::new(root_idx1));
            eq_local_types.remove(&LocalType::new(root_idx2));
            eq_local_types.insert(LocalType::new(root_idx));
            eq_local_types.remove(&LocalType::new(self.type_entries.root_of(eq_root_idx)));
            self.eq_type_param_entries[eq_root_idx].local_types = eq_local_types;
            Some((LocalType::new(root_idx), LocalType::new(eq_root_idx)))
        } else {
            None
        }
    }
    
    pub fn type_value_to_string(&self, type_value: &Rc<TypeValue>) -> String
    {
        type_value.to_string(|type_value, s| {
                match self.type_entry_for_type_value(&Rc::new(type_value.clone())) {
                    Some(LocalTypeEntry::Param(_, uniq_flag, type_param_entry, _)) => {
                        let type_param_entry_r = type_param_entry.borrow();
                        if let Some(ident) = &type_param_entry_r.ident {
                            if uniq_flag == UniqFlag::Uniq {
                                s.push_str("uniq ");
                            }
                            s.push_str(ident.as_str());
                            None
                        } else if let Some(num) = &type_param_entry_r.number {
                            if uniq_flag == UniqFlag::Uniq {
                                s.push_str("uniq ");
                            }
                            s.push_str(format!("t{}", *num).as_str());
                            None
                        } else {
                            Some(type_value.clone())
                        }
                        
                    },
                    Some(LocalTypeEntry::Type(type_value2)) => Some((*type_value2).clone()),
                    None => Some(type_value.clone()),
                }
        })
    }
    
    pub fn local_type_to_string(&self, local_type: LocalType) -> String
    { self.type_value_to_string(&Rc::new(TypeValue::Param(UniqFlag::None, local_type))) }
}

#[derive(Clone, Debug)]
pub struct TypeValueWithLocalTypes<'a>(pub Rc<TypeValue>, pub  &'a LocalTypes);

impl<'a> fmt::Display for TypeValueWithLocalTypes<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "{}", self.1.type_value_to_string(&self.0)) }
}

#[derive(Copy, Clone, Debug)]
pub struct LocalTypeWithLocalTypes<'a>(pub LocalType, pub &'a LocalTypes);

impl<'a> fmt::Display for LocalTypeWithLocalTypes<'a>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "{}", self.1.local_type_to_string(self.0)) }
}

#[derive(Copy, Clone, Debug)]
pub struct LocalFun
{
    index: usize,
}

impl LocalFun
{
    pub fn new(idx: usize) -> Self
    { LocalFun { index: idx, } }
    
    pub fn index(&self) -> usize
    { self.index }
}

#[derive(Clone, Debug)]
pub enum Object
{
    String(Vec<u8>),
    CharN(Vec<i8>),
    ShortN(Vec<i16>),
    IntN(Vec<i32>),
    LongN(Vec<i64>),
    UcharN(Vec<u8>),
    UshortN(Vec<u16>),
    UintN(Vec<i32>),
    UlongN(Vec<i64>),
    FloatN(Vec<f32>),
    DoubleN(Vec<f64>),
    Tuple(Vec<Value>),
    Array(Vec<Value>),
    Data(String, Vec<Value>),
}

#[derive(Clone, Debug)]
pub enum Value
{
    Bool(bool),
    Char(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Uchar(u8),
    Ushort(u16),
    Uint(u32),
    Ulong(u64),
    Float(f32),
    Double(f64),
    SizeT(u64),
    PtrdiffT(i64),
    IntptrT(i64),
    UintptrT(u64),
    Object(SharedFlag, Rc<RefCell<Object>>),
    Builtin(String),
    Fun(String),
    Lambda(String, LocalFun),
    EvalFun(String, fn(&[Value], &Pos) -> FrontendResult<Value>),
}

#[derive(Clone, Debug)]
pub struct Closure
{
    values: BTreeMap<String, Value>,
}

impl Closure
{
    pub fn new() -> Self
    { Closure { values: BTreeMap::new(), } }
    
    pub fn values(&self) -> &BTreeMap<String, Value>
    { &self.values }

    pub fn value(&self, ident: &String) -> Option<&Value>
    { self.values.get(ident) }

    pub fn add_value(&mut self, ident: String, value: Value)
    { self.values.insert(ident, value); }
}

#[derive(Clone, Debug)]
pub struct TraitVars
{
    impls: BTreeMap<TypeName, Rc<RefCell<Impl>>>,
    vars: BTreeMap<String, Rc<RefCell<Var>>>,
}

impl TraitVars
{
    pub fn new() -> Self
    { TraitVars { impls: BTreeMap::new(), vars: BTreeMap::new(), } }

    pub fn impls(&self) -> &BTreeMap<TypeName, Rc<RefCell<Impl>>>
    { &self.impls }
    
    pub fn impl1(&self, type_name: &TypeName) -> Option<&Rc<RefCell<Impl>>>
    { self.impls.get(type_name) }

    pub fn add_impl(&mut self, type_name: TypeName, impl1: Rc<RefCell<Impl>>)
    { self.impls.insert(type_name, impl1); } 
    
    pub fn vars(&self) -> &BTreeMap<String, Rc<RefCell<Var>>>
    { &self.vars }
    
    pub fn var(&self, ident: &String) -> Option<&Rc<RefCell<Var>>>
    { self.vars.get(ident) }

    pub fn add_var(&mut self, ident: String, var: Rc<RefCell<Var>>)
    { self.vars.insert(ident, var); } 
}

#[derive(Clone, Debug)]
pub struct ImplVars
{
    vars: BTreeMap<String, Rc<RefCell<ImplVar>>>,
}

impl ImplVars
{
    pub fn new() -> Self
    { ImplVars { vars: BTreeMap::new(), } }

    pub fn vars(&self) -> &BTreeMap<String, Rc<RefCell<ImplVar>>>
    { &self.vars }
    
    pub fn var(&self, ident: &String) -> Option<&Rc<RefCell<ImplVar>>>
    { self.vars.get(ident) }

    pub fn add_var(&mut self, ident: String, var: Rc<RefCell<ImplVar>>)
    { self.vars.insert(ident, var); }
}

#[cfg(test)]
mod tests;
