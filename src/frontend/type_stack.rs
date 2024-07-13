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
use std::error;
use std::fmt;
use std::rc::*;
use crate::frontend::tree::*;
use crate::utils::dfs::*;

#[derive(Clone, Debug)]
pub enum TypeStackEntry
{
    Param(Rc<RefCell<TypeParamEntry>>),
    Type(Rc<TypeValue>),
}

#[derive(Clone, Debug)]
pub struct TypeStack
{
    type_values: Vec<(Vec<Rc<TypeValue>>, usize)>,
    type_entries: Vec<TypeStackEntry>,
    type_param_entry: Rc<RefCell<TypeParamEntry>>,
}

impl TypeStack
{
    pub fn new() -> Self
    {
        TypeStack {
            type_values: Vec::new(),
            type_entries: Vec::new(), 
            type_param_entry: Rc::new(RefCell::new(TypeParamEntry::new())),
        }
    }
    
    pub fn type_values_and_type_entry_index(&self) -> Option<(&[Rc<TypeValue>], usize)>
    {
        match self.type_values.last() {
            Some((type_values, idx)) => Some((type_values.as_slice(), *idx)),
            None => None,
        }
    }

    pub fn type_values(&self) -> Option<&[Rc<TypeValue>]>
    { self.type_values_and_type_entry_index().map(|p| p.0) }

    pub fn type_entry_index(&self) -> Option<usize>
    { self.type_values_and_type_entry_index().map(|p| p.1) }

    pub fn type_value(&self, local_type: LocalType) -> Option<&Rc<TypeValue>>
    {
        match self.type_values() {
            Some(type_values) => {
                match type_values.get(local_type.index()) {
                    Some(type_value) => Some(type_value),
                    None => None,
                }
            },
            None => None,
        }
    }
    
    pub fn type_entries(&self) -> &[TypeStackEntry]
    { self.type_entries.as_slice() }

    pub fn type_entry(&self, local_type: LocalType) -> Option<&TypeStackEntry>
    {
        match self.type_entries().get(local_type.index()) {
            Some(type_entry) => Some(type_entry),
            None => None,
        }
    }
    
    pub fn set_first_type_values(&mut self, typ: &Type)
    {
        self.type_values.clear();
        self.type_entries.clear();
        let mut type_values: Vec<Rc<TypeValue>> = Vec::new();
        for (i, type_param_entry) in typ.type_param_entries().iter().enumerate() {
            type_values.push(Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(i))));
            self.type_entries.push(TypeStackEntry::Param(type_param_entry.clone()));
        }
        self.type_values.push((type_values, self.type_entries.len()));
    }

    fn add_type_entry(&mut self, local_type: LocalType, new_local_types: &mut BTreeMap<LocalType, LocalType>, added_local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>) -> Result<LocalType, TypeStackInternalError>
    {
        let new_local_type = match new_local_types.get(&local_type) {
            Some(tmp_local_type) => *tmp_local_type,
            None => {
                let tmp_local_type = LocalType::new(self.type_entries.len());
                if processed_local_types.contains(&local_type) {
                    return Err(TypeStackInternalError(String::from("add_type_entry: cycle of local types")));
                }
                self.type_entries.push(TypeStackEntry::Param(self.type_param_entry.clone()));
                new_local_types.insert(local_type, tmp_local_type);
                added_local_types.push(local_type);
                tmp_local_type
            },
        };
        Ok(new_local_type)
    }
    
    fn real_type_value_from_type_value(&mut self, type_value: &Rc<TypeValue>, local_types: &LocalTypes, new_local_types: &mut BTreeMap<LocalType, LocalType>, added_local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>) -> Result<Rc<TypeValue>, TypeStackInternalError>
    {
        match local_types.type_entry_for_type_value(type_value) {
            Some(LocalTypeEntry::Param(_, uniq_flag, _, local_type)) => {
                match self.type_values.last() {
                    Some((type_values, _)) => {
                        match type_values.get(local_type.index()) {
                            Some(type_value) => Ok(type_value.clone()),
                            None => Ok(Rc::new(TypeValue::Param(uniq_flag, self.add_type_entry(local_type, new_local_types, added_local_types, processed_local_types)?))),
                        }
                    },
                    None => Err(TypeStackInternalError(String::from("real_type_value_from_type_value: no type values"))),
                }
            },
            Some(LocalTypeEntry::Type(type_value2)) => {
                match &*type_value2 {
                    TypeValue::Param(_, _) => Err(TypeStackInternalError(String::from("real_type_value_from_type_value: no type values"))),
                    TypeValue::Type(uniq_flag, type_value_name, type_values) => {
                        let mut type_values2: Vec<Rc<TypeValue>> = Vec::new(); 
                        for type_value3 in type_values {
                            type_values2.push(self.real_type_value_from_type_value(type_value3, local_types, new_local_types, added_local_types, processed_local_types)?);
                        }
                        Ok(Rc::new(TypeValue::Type(*uniq_flag, type_value_name.clone(), type_values2)))
                    },
                }
            },
            None => Err(TypeStackInternalError(String::from("real_type_value_from_type_value: no local type entry"))),
        }
    }
    
    fn local_types_for_local_type(&mut self, local_type: LocalType, local_types: &LocalTypes, new_local_types: &mut BTreeMap<LocalType, LocalType>, processed_local_types: &BTreeSet<LocalType>) -> Result<Vec<LocalType>, TypeStackInternalError>
    {
        match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type))) {
            Some(LocalTypeEntry::Param(_, _, type_param_entry, _)) => {
                let mut added_local_types: Vec<LocalType> = Vec::new();
                let type_param_entry_r = type_param_entry.borrow();
                let mut new_type_param_entry = TypeParamEntry::new();
                for type_value in &type_param_entry_r.type_values {
                    new_type_param_entry.type_values.push(self.real_type_value_from_type_value(type_value, local_types, new_local_types, &mut added_local_types, processed_local_types)?);
                }
                new_type_param_entry.trait_names = type_param_entry_r.trait_names.clone();
                for closure_local_type in &type_param_entry_r.closure_local_types {
                    new_type_param_entry.closure_local_types.insert(self.add_type_entry(*closure_local_type, new_local_types, &mut added_local_types, processed_local_types)?);
                }
                new_type_param_entry.number = type_param_entry_r.number;
                new_type_param_entry.ident = type_param_entry_r.ident.clone();
                new_type_param_entry.pos = type_param_entry_r.pos.clone();
                match new_local_types.get(&local_type) {
                    Some(new_local_type) => {
                        match self.type_entries.get_mut(new_local_type.index()) {
                            Some(type_entry) => {
                                *type_entry = TypeStackEntry::Param(Rc::new(RefCell::new(new_type_param_entry)));
                                Ok(added_local_types)
                            },
                            None => Err(TypeStackInternalError(String::from("local_types_for_local_type: no type stack entry"))),
                        }
                    },
                    None => Err(TypeStackInternalError(String::from("local_types_for_local_type: no new local type"))),
                }
            },
            Some(LocalTypeEntry::Type(type_value)) => {
                let mut added_local_types: Vec<LocalType> = Vec::new();
                let new_type_value = self.real_type_value_from_type_value(&type_value, local_types, new_local_types, &mut added_local_types, processed_local_types)?;
                match new_local_types.get(&local_type) {
                    Some(new_local_type) => {
                        match self.type_entries.get_mut(new_local_type.index()) {
                            Some(type_entry) => {
                                *type_entry = TypeStackEntry::Type(new_type_value);
                                Ok(added_local_types)
                            },
                            None => Err(TypeStackInternalError(String::from("local_types_for_local_type: no type stack entry"))),
                        }
                    },
                    None => Err(TypeStackInternalError(String::from("local_types_for_local_type: no new local type"))),
                }
            },
            None => Err(TypeStackInternalError(String::from("local_types_for_local_type: no local type entry"))),
        }
    }
    
    pub fn push_type_entries(&mut self, local_type: LocalType, local_types: &LocalTypes) -> Result<LocalType, TypeStackInternalError>
    {
        let mut new_local_types: BTreeMap<LocalType, LocalType> = BTreeMap::new();
        let new_local_type = LocalType::new(self.type_entries.len());
        self.type_entries.push(TypeStackEntry::Param(self.type_param_entry.clone()));
        new_local_types.insert(local_type, new_local_type);
        let mut visited_local_types: BTreeSet<LocalType> = BTreeSet::new();
        dfs_with_result(&local_type, &mut visited_local_types, &mut (), |local_type, processed_local_types, _| {
                self.local_types_for_local_type(*local_type, local_types, &mut new_local_types, processed_local_types)
        }, |_, _| Ok(()))?;
        Ok(new_local_type)
    }
    
    fn type_name_from_type_values(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, trait_ident: &str, typ: &Type) -> Result<Option<TypeName>, TypeStackInternalError>
    {
        match (&**type_value1, &**type_value2) {
            (TypeValue::Param(_, local_type1), TypeValue::Param(_, local_type2)) => {
                match (self.type_entries.get(local_type1.index()), typ.type_param_entry(*local_type2)) {
                    (Some(TypeStackEntry::Param(type_param_entry1)), Some(type_param_entry2)) => {
                        let type_param_entry1_r = type_param_entry1.borrow();
                        let type_param_entry2_r = type_param_entry2.borrow();
                        let mut type_name: Option<TypeName> = None;
                        for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                            match self.type_name_from_type_values(type_value3, type_value4, trait_ident, typ)? {
                                Some(tmp_type_name) => {
                                    type_name = Some(tmp_type_name.clone());
                                    break;
                                }
                                None => (),
                            }
                        }
                        Ok(type_name)
                    },
                    (Some(TypeStackEntry::Type(type_value3)), Some(_)) => self.type_name_from_type_values(type_value3, type_value2, trait_ident, typ),
                    _ => Err(TypeStackInternalError(String::from("type_name_from_type_values: no type stack entry or type parameter entry"))),
                }
            },
            (TypeValue::Param(_, _), TypeValue::Type(_, _, _)) => Err(TypeStackInternalError(String::from("type_name_from_type_values: can't match type parameter with type"))),
            (TypeValue::Type(_, _, type_values1), TypeValue::Param(_, local_type2)) => {
                match typ.type_param_entry(*local_type2) {
                    Some(type_param_entry2) => {
                        let type_param_entry2_r = type_param_entry2.borrow();
                        if type_param_entry2_r.trait_names.contains(&TraitName::Name(String::from(trait_ident))) {
                            Ok(type_value1.type_name())
                        } else {
                            let mut type_name: Option<TypeName> = None;
                            for (type_value3, type_value4) in type_values1.iter().zip(type_param_entry2_r.type_values.iter()) {
                                match self.type_name_from_type_values(type_value3, type_value4, trait_ident, typ)? {
                                    Some(tmp_type_name) => {
                                        type_name = Some(tmp_type_name.clone());
                                        break;
                                    }
                                    None => (),
                                }
                            }
                            Ok(type_name)
                        }
                    },
                    None => Err(TypeStackInternalError(String::from("type_name_from_type_values: no type parameter entry"))),
                }
            },
            (TypeValue::Type(_, _, type_values1), TypeValue::Type(_, _, type_values2)) => {
                let mut type_name: Option<TypeName> = None;
                for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                    match self.type_name_from_type_values(type_value3, type_value4, trait_ident, typ)? {
                        Some(tmp_type_name) => {
                            type_name = Some(tmp_type_name.clone());
                            break;
                        }
                        None => (),
                    }
                }
                Ok(type_name)
            },
        }
    }

    pub fn type_name(&self, local_type: LocalType, typ: &Type, trait_ident: &str) -> Result<Option<TypeName>, TypeStackInternalError>
    { self.type_name_from_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, local_type)), typ.type_value(), trait_ident, typ) }

    fn set_type_values_for_type_value(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, typ: &Type, type_values: &mut [Rc<TypeValue>]) -> Result<Rc<TypeValue>, TypeStackInternalError>
    {
        match (&**type_value1, &**type_value2) {
            (TypeValue::Param(_, local_type1), TypeValue::Param(uniq_flag2, local_type2)) => {
                match (self.type_entries.get(local_type1.index()), typ.type_param_entry(*local_type2)) {
                    (Some(TypeStackEntry::Param(type_param_entry1)), Some(type_param_entry2)) => {
                        let mut type_param_entry1_r = type_param_entry1.borrow_mut();
                        let type_param_entry2_r = type_param_entry2.borrow();
                        let mut new_type_values: Vec<Rc<TypeValue>> = Vec::new();
                        for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                            new_type_values.push(self.set_type_values_for_type_value(type_value3, type_value4, typ, type_values)?);
                        }
                        type_param_entry1_r.type_values = new_type_values;
                        let new_type_value = Rc::new(TypeValue::Param(*uniq_flag2, *local_type1));
                        match type_values.get_mut(local_type2.index()) {
                            Some(type_value) => *type_value = new_type_value.clone(),
                            None => return Err(TypeStackInternalError(String::from("set_type_values_for_type_value: no type value"))),
                        }
                        Ok(new_type_value)
                    },
                    (Some(TypeStackEntry::Type(type_value3)), Some(_)) => self.set_type_values_for_type_value(type_value3, type_value2, typ, type_values),
                    _ => Err(TypeStackInternalError(String::from("set_type_values_for_type_value: no type stack entry or type parameter entry"))),
                }
            },
            (TypeValue::Param(_, _), TypeValue::Type(_, _, _)) => Err(TypeStackInternalError(String::from("set_type_values_for_type_value: can't match type parameter with type"))),
            (TypeValue::Type(_, type_value_name1, type_values1), TypeValue::Param(uniq_flag2, local_type2)) => {
                match typ.type_param_entry(*local_type2) {
                    Some(type_param_entry2) => {
                        let type_param_entry2_r = type_param_entry2.borrow();
                        let mut new_type_values: Vec<Rc<TypeValue>> = Vec::new();
                        for (type_value3, type_value4) in type_values1.iter().zip(type_param_entry2_r.type_values.iter()) {
                            new_type_values.push(self.set_type_values_for_type_value(type_value3, type_value4, typ, type_values)?);
                        }
                        let new_type_value = Rc::new(TypeValue::Type(*uniq_flag2, type_value_name1.clone(), new_type_values));
                        match type_values.get_mut(local_type2.index()) {
                            Some(type_value) => *type_value = new_type_value.clone(),
                            None => return Err(TypeStackInternalError(String::from("set_type_values_for_type_value: no type value"))),
                        }
                        Ok(new_type_value)
                    },
                    None => Err(TypeStackInternalError(String::from("set_type_values_for_type_value: no type parameter entry"))),
                }
            },
            (TypeValue::Type(_, type_value_name1, type_values1), TypeValue::Type(uniq_flag2, _, type_values2)) => {
                let mut new_type_values: Vec<Rc<TypeValue>> = Vec::new();
                for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                    new_type_values.push(self.set_type_values_for_type_value(type_value3, type_value4, typ, type_values)?);
                }
                Ok(Rc::new(TypeValue::Type(*uniq_flag2, type_value_name1.clone(), new_type_values)))
            },
        }
    }
    
    pub fn push_type_values(&mut self, local_type: LocalType, typ: &Type) -> Result<(), TypeStackInternalError>
    {
        let mut type_values = vec![Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, Vec::new())); typ.type_param_entries().len()];
        let new_type_value = self.set_type_values_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type)), typ.type_value(), typ, type_values.as_mut_slice())?;
        match &self.type_entries[local_type.index()] {
            TypeStackEntry::Type(_) => self.type_entries[local_type.index()] = TypeStackEntry::Type(new_type_value),
            _ => (),
        }
        self.type_values.push((type_values, self.type_entries.len()));
        Ok(())
    }
    
    pub fn pop_type_values(&mut self) -> bool
    {
        match self.type_values.pop() {
            Some((_, new_len)) => {
                for _ in (new_len..self.type_entries.len()).rev() {
                    self.type_entries.pop();
                }
                true
            },
            None => false,
        }
    }
}

#[derive(Debug)]
pub struct TypeStackInternalError(String);

impl error::Error for TypeStackInternalError
{}

impl fmt::Display for TypeStackInternalError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "{}",self.0) }
}
