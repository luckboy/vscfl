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
use std::rc::*;
use crate::frontend::error::*;
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
    empty_type_param_entry: Rc<RefCell<TypeParamEntry>>,
}

impl TypeStack
{
    pub fn new() -> Self
    {
        TypeStack {
            type_values: Vec::new(),
            type_entries: Vec::new(), 
            empty_type_param_entry: Rc::new(RefCell::new(TypeParamEntry::new())),
        }
    }
    
    pub fn type_value_stack_len(&self) -> usize
    { self.type_values.len() }
    
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
            Some(type_values) => type_values.get(local_type.index()),
            None => None,
        }
    }
    
    pub fn type_entries(&self) -> &[TypeStackEntry]
    { self.type_entries.as_slice() }

    pub fn type_entry(&self, local_type: LocalType) -> Option<&TypeStackEntry>
    { self.type_entries().get(local_type.index()) }
    
    pub fn set_first_type_values_for_type(&mut self, typ: &Type)
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

    fn add_type_entry(&mut self, local_type: LocalType, new_local_types: &mut BTreeMap<LocalType, LocalType>, added_local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>) -> FrontendInternalResult<LocalType>
    {
        let new_local_type = match new_local_types.get(&local_type) {
            Some(tmp_local_type) => *tmp_local_type,
            None => {
                let tmp_local_type = LocalType::new(self.type_entries.len());
                if processed_local_types.contains(&local_type) {
                    return Err(FrontendInternalError(String::from("add_type_entry: cycle of local types")));
                }
                self.type_entries.push(TypeStackEntry::Param(self.empty_type_param_entry.clone()));
                new_local_types.insert(local_type, tmp_local_type);
                added_local_types.push(local_type);
                tmp_local_type
            },
        };
        Ok(new_local_type)
    }
    
    fn real_type_value_from_type_value(&mut self, type_value: &Rc<TypeValue>, local_types: &LocalTypes, new_local_types: &mut BTreeMap<LocalType, LocalType>, added_local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>) -> FrontendInternalResult<Rc<TypeValue>>
    {
        match local_types.type_entry_for_type_value(type_value) {
            Some(LocalTypeEntry::Param(_, uniq_flag, _, local_type)) => {
                match self.type_values.last() {
                    Some((type_values, _)) => {
                        let mut j: Option<usize> = None;
                        for i in 0..type_values.len() {
                            match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, LocalType::new(i)))) {
                                Some(LocalTypeEntry::Param(_, _, _, local_type2)) => {
                                    if local_type == local_type2 {
                                        j = Some(i);
                                        break;
                                    }
                                },
                                _ => return Err(FrontendInternalError(String::from("real_type_value_from_type_value: no local type entry or local type entry is type"))),
                            }
                        }
                        match j {
                            Some(j) => {
                                match type_values.get(j) {
                                    Some(type_value) => Ok(type_value.clone()),
                                    None => Ok(Rc::new(TypeValue::Param(uniq_flag, self.add_type_entry(local_type, new_local_types, added_local_types, processed_local_types)?))),
                                }
                            },
                            None => Ok(Rc::new(TypeValue::Param(uniq_flag, self.add_type_entry(local_type, new_local_types, added_local_types, processed_local_types)?))),
                        }
                    },
                    None => Err(FrontendInternalError(String::from("real_type_value_from_type_value: no type values"))),
                }
            },
            Some(LocalTypeEntry::Type(type_value2)) => {
                match &*type_value2 {
                    TypeValue::Param(_, _) => Err(FrontendInternalError(String::from("real_type_value_from_type_value: type parameter in local type entry"))),
                    TypeValue::Type(uniq_flag, type_value_name, type_values) => {
                        let mut type_values2: Vec<Rc<TypeValue>> = Vec::new(); 
                        for type_value3 in type_values {
                            type_values2.push(self.real_type_value_from_type_value(type_value3, local_types, new_local_types, added_local_types, processed_local_types)?);
                        }
                        Ok(Rc::new(TypeValue::Type(*uniq_flag, type_value_name.clone(), type_values2)))
                    },
                }
            },
            None => Err(FrontendInternalError(String::from("real_type_value_from_type_value: no local type entry"))),
        }
    }
    
    fn local_types_for_local_type(&mut self, local_type: LocalType, local_types: &LocalTypes, new_local_types: &mut BTreeMap<LocalType, LocalType>, processed_local_types: &BTreeSet<LocalType>) -> FrontendInternalResult<Vec<LocalType>>
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
                            None => Err(FrontendInternalError(String::from("local_types_for_local_type: no type stack entry"))),
                        }
                    },
                    None => Err(FrontendInternalError(String::from("local_types_for_local_type: no new local type"))),
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
                            None => Err(FrontendInternalError(String::from("local_types_for_local_type: no type stack entry"))),
                        }
                    },
                    None => Err(FrontendInternalError(String::from("local_types_for_local_type: no new local type"))),
                }
            },
            None => Err(FrontendInternalError(String::from("local_types_for_local_type: no local type entry"))),
        }
    }
    
    pub fn push_type_entries_for_local_type(&mut self, local_type: LocalType, local_types: &LocalTypes) -> FrontendInternalResult<LocalType>
    {
        let mut new_local_types: BTreeMap<LocalType, LocalType> = BTreeMap::new();
        let new_local_type = LocalType::new(self.type_entries.len());
        self.type_entries.push(TypeStackEntry::Param(self.empty_type_param_entry.clone()));
        new_local_types.insert(local_type, new_local_type);
        let mut visited_local_types: BTreeSet<LocalType> = BTreeSet::new();
        dfs_with_result(&local_type, &mut visited_local_types, &mut (), |local_type, processed_local_types, _| {
                self.local_types_for_local_type(*local_type, local_types, &mut new_local_types, processed_local_types)
        }, |_, _| Ok(()))?;
        Ok(new_local_type)
    }
    
    fn type_name_for_type_values(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, trait_ident: &str, typ: &Type) -> FrontendInternalResult<Option<TypeName>>
    {
        match (&**type_value1, &**type_value2) {
            (TypeValue::Param(_, local_type1), TypeValue::Param(_, local_type2)) => {
                match (self.type_entries.get(local_type1.index()), typ.type_param_entry(*local_type2)) {
                    (Some(TypeStackEntry::Param(type_param_entry1)), Some(type_param_entry2)) => {
                        let type_param_entry1_r = type_param_entry1.borrow();
                        let type_param_entry2_r = type_param_entry2.borrow();
                        let mut type_name: Option<TypeName> = None;
                        if !type_param_entry2_r.type_values.is_empty() {
                            for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                                match self.type_name_for_type_values(type_value3, type_value4, trait_ident, typ)? {
                                    Some(tmp_type_name) => {
                                        type_name = Some(tmp_type_name.clone());
                                        break;
                                    }
                                    None => (),
                                }
                            }
                        }
                        Ok(type_name)
                    },
                    (Some(TypeStackEntry::Type(type_value3)), Some(_)) => self.type_name_for_type_values(type_value3, type_value2, trait_ident, typ),
                    _ => Err(FrontendInternalError(String::from("type_name_from_type_values: no type stack entry or type parameter entry"))),
                }
            },
            (TypeValue::Param(_, local_type1), TypeValue::Type(_, _, _)) => {
                match self.type_entries.get(local_type1.index()) {
                    Some(TypeStackEntry::Type(type_value3)) => self.type_name_for_type_values(type_value3, type_value2, trait_ident, typ),
                    Some(TypeStackEntry::Param(_)) => Err(FrontendInternalError(String::from("type_name_from_type_values: can't match type parameter with type"))),
                    None => Err(FrontendInternalError(String::from("type_name_from_type_values: no type stack entry"))),
                }
            },
            (TypeValue::Type(_, _, type_values1), TypeValue::Param(_, local_type2)) => {
                match typ.type_param_entry(*local_type2) {
                    Some(type_param_entry2) => {
                        let type_param_entry2_r = type_param_entry2.borrow();
                        if type_param_entry2_r.trait_names.contains(&TraitName::Name(String::from(trait_ident))) {
                            Ok(type_value1.type_name())
                        } else {
                            let mut type_name: Option<TypeName> = None;
                            if !type_param_entry2_r.type_values.is_empty() {
                                for (type_value3, type_value4) in type_values1.iter().zip(type_param_entry2_r.type_values.iter()) {
                                    match self.type_name_for_type_values(type_value3, type_value4, trait_ident, typ)? {
                                        Some(tmp_type_name) => {
                                            type_name = Some(tmp_type_name.clone());
                                            break;
                                        }
                                        None => (),
                                    }
                                }
                            }
                            Ok(type_name)
                        }
                    },
                    None => Err(FrontendInternalError(String::from("type_name_from_type_values: no type parameter entry"))),
                }
            },
            (TypeValue::Type(_, _, type_values1), TypeValue::Type(_, _, type_values2)) => {
                let mut type_name: Option<TypeName> = None;
                for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                    match self.type_name_for_type_values(type_value3, type_value4, trait_ident, typ)? {
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

    pub fn type_name_for_local_type_and_type(&self, local_type: LocalType, typ: &Type, trait_ident: &str) -> FrontendInternalResult<Option<TypeName>>
    { self.type_name_for_type_values(&Rc::new(TypeValue::Param(UniqFlag::None, local_type)), typ.type_value(), trait_ident, typ) }

    fn set_type_values_for_type_value(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, typ: &Type, type_values: &mut [Rc<TypeValue>]) -> FrontendInternalResult<Rc<TypeValue>>
    {
        match (&**type_value1, &**type_value2) {
            (TypeValue::Param(_, local_type1), TypeValue::Param(uniq_flag2, local_type2)) => {
                match (self.type_entries.get(local_type1.index()), typ.type_param_entry(*local_type2)) {
                    (Some(TypeStackEntry::Param(type_param_entry1)), Some(type_param_entry2)) => {
                        let mut type_param_entry1_r = type_param_entry1.borrow_mut();
                        let type_param_entry2_r = type_param_entry2.borrow();
                        let mut new_type_values: Vec<Rc<TypeValue>> = Vec::new();
                        if !type_param_entry2_r.type_values.is_empty() {
                            for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                                new_type_values.push(self.set_type_values_for_type_value(type_value3, type_value4, typ, type_values)?);
                            }
                        } else {
                            new_type_values.extend_from_slice(type_param_entry1_r.type_values.as_slice());
                        }
                        type_param_entry1_r.type_values = new_type_values;
                        let new_type_value = Rc::new(TypeValue::Param(*uniq_flag2, *local_type1));
                        match type_values.get_mut(local_type2.index()) {
                            Some(type_value) => *type_value = new_type_value.clone(),
                            None => return Err(FrontendInternalError(String::from("set_type_values_for_type_value: no type value"))),
                        }
                        Ok(new_type_value)
                    },
                    (Some(TypeStackEntry::Type(type_value3)), Some(_)) => self.set_type_values_for_type_value(type_value3, type_value2, typ, type_values),
                    _ => Err(FrontendInternalError(String::from("set_type_values_for_type_value: no type stack entry or type parameter entry"))),
                }
            },
            (TypeValue::Param(_, local_type1), TypeValue::Type(_, _, _)) => {
                match self.type_entries.get(local_type1.index()) {
                    Some(TypeStackEntry::Type(type_value3)) => self.set_type_values_for_type_value(type_value3, type_value2, typ, type_values),
                    Some(TypeStackEntry::Param(_)) => Err(FrontendInternalError(String::from("set_type_values_for_type_value: can't match type parameter with type"))),
                    None => Err(FrontendInternalError(String::from("set_type_values_for_type_value: no type stack entry"))),
                }
            },
            (TypeValue::Type(_, type_value_name1, type_values1), TypeValue::Param(uniq_flag2, local_type2)) => {
                match typ.type_param_entry(*local_type2) {
                    Some(type_param_entry2) => {
                        let type_param_entry2_r = type_param_entry2.borrow();
                        let mut new_type_values: Vec<Rc<TypeValue>> = Vec::new();
                        if !type_param_entry2_r.type_values.is_empty() {
                            for (type_value3, type_value4) in type_values1.iter().zip(type_param_entry2_r.type_values.iter()) {
                                new_type_values.push(self.set_type_values_for_type_value(type_value3, type_value4, typ, type_values)?);
                            }
                        } else {
                            new_type_values.extend_from_slice(type_values1.as_slice());
                        }
                        let new_type_value = Rc::new(TypeValue::Type(*uniq_flag2, type_value_name1.clone(), new_type_values));
                        match type_values.get_mut(local_type2.index()) {
                            Some(type_value) => *type_value = new_type_value.clone(),
                            None => return Err(FrontendInternalError(String::from("set_type_values_for_type_value: no type value"))),
                        }
                        Ok(new_type_value)
                    },
                    None => Err(FrontendInternalError(String::from("set_type_values_for_type_value: no type parameter entry"))),
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
    
    pub fn push_type_values_for_local_type_and_type(&mut self, local_type: LocalType, typ: &Type) -> FrontendInternalResult<()>
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

    pub fn push_type_values(&mut self, type_values: Vec<Rc<TypeValue>>)
    { self.type_values.push((type_values, self.type_entries.len())); }
    
    pub fn pop_type_values(&mut self) -> Option<Vec<Rc<TypeValue>>>
    {
        match self.type_values.pop() {
            Some((type_values, _)) => {
                let new_len = self.type_values.last().map(|p| p.1).unwrap_or(0);
                for _ in (new_len..self.type_entries.len()).rev() {
                    self.type_entries.pop();
                }
                Some(type_values)
            },
            None => None,
        }
    }

    pub fn pop_type_entries(&mut self)
    {
        let new_len = self.type_values.last().map(|p| p.1).unwrap_or(0);
        for _ in (new_len..self.type_entries.len()).rev() {
            self.type_entries.pop();
        }
    }

    fn shared_flag_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree) -> FrontendInternalResult<SharedFlag>
    {
        match &**type_value {
            TypeValue::Param(UniqFlag::None, local_type) => {
                match self.type_entries.get(local_type.index()) {
                    Some(TypeStackEntry::Param(type_param_entry)) => {
                        let type_param_entry_r = type_param_entry.borrow();
                        if type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                            Ok(SharedFlag::Shared)
                        } else {
                            Ok(SharedFlag::None)
                        }
                    },
                    Some(TypeStackEntry::Type(type_value2)) => self.shared_flag_for_type_value(type_value2, tree),
                    None => Err(FrontendInternalError(String::from("shared_flag_for_type_value: no type stack entry"))),
                }
            },
            TypeValue::Type(UniqFlag::None, TypeValueName::Fun, _) => Ok(SharedFlag::Shared),
            TypeValue::Type(UniqFlag::None, type_value_name, type_values) => {
                let mut shared_flag = match type_value_name {
                    TypeValueName::Name(ident) => {
                        match tree.type_var(ident) {
                            Some(type_var) => {
                                let type_var_r = type_var.borrow();
                                match &*type_var_r {
                                    TypeVar::Builtin(_, _, Some(tmp_shared_flag)) => *tmp_shared_flag,
                                    TypeVar::Data(_, _, Some(tmp_shared_flag)) => *tmp_shared_flag,
                                    _ => return Err(FrontendInternalError(String::from("shared_flag_for_type_value: type variable isn't type or type hasn't shared flag"))),
                                }
                            },
                            None => return Err(FrontendInternalError(String::from("shared_flag_for_type_value: no type variable"))),
                        }
                    },
                    _ => SharedFlag::Shared,
                };
                if shared_flag == SharedFlag::Shared {
                    for type_value2 in type_values {
                        if self.shared_flag_for_type_value(type_value2, tree)? == SharedFlag::None {
                            shared_flag = SharedFlag::None;
                        }
                    }
                }
                Ok(shared_flag)
            },
            _ => Ok(SharedFlag::None),
        }
    }
    
    pub fn shared_flag_for_local_type(&self, local_type: LocalType, tree: &Tree) -> FrontendInternalResult<SharedFlag>
    { self.shared_flag_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, local_type)), tree) }

    fn add_local_types_for_type_value(&self, type_value: &Rc<TypeValue>, local_types: &mut Vec<LocalType>, processed_local_types: &BTreeSet<LocalType>) -> FrontendInternalResult<()>
    {
        match &**type_value {
            TypeValue::Param(_, local_type) => {
                if processed_local_types.contains(local_type) {
                    return Err(FrontendInternalError(String::from("add_local_types_for_type_value: cycle of local types")));
                }
                local_types.push(*local_type);
                Ok(())
            },
            TypeValue::Type(_, _, type_values) => {
                for type_value2 in type_values {
                    self.add_local_types_for_type_value(type_value2, local_types, processed_local_types)?;
                }
                Ok(())
            },
        }
    }

    fn local_types_for_local_type_and_change(&self, local_type: LocalType, processed_local_types: &BTreeSet<LocalType>) -> FrontendInternalResult<Vec<LocalType>>
    {
        match self.type_entries.get(local_type.index()) {
            Some(TypeStackEntry::Param(type_param_entry)) => {
                let mut local_types: Vec<LocalType> = Vec::new();
                let type_param_entry_r = type_param_entry.borrow();
                for type_value in &type_param_entry_r.type_values {
                    self.add_local_types_for_type_value(type_value, &mut local_types, processed_local_types)?;
                }
                for closure_local_type in &type_param_entry_r.closure_local_types {
                    if processed_local_types.contains(closure_local_type) {
                        return Err(FrontendInternalError(String::from("local_types_for_local_type_and_change: cycle of local types")));
                    }
                    local_types.push(*closure_local_type);
                }
                Ok(local_types)
            },
            Some(TypeStackEntry::Type(type_value)) => {
                let mut local_types: Vec<LocalType> = Vec::new();
                self.add_local_types_for_type_value(type_value, &mut local_types, processed_local_types)?;
                Ok(local_types)
            },
            None => Err(FrontendInternalError(String::from("local_types_for_local_type_and_change: no local type entry"))),
        }
    }
    
    fn set_type_value_for_local_type(&self, local_type: LocalType, tree: &Tree, type_values: &mut [Rc<TypeValue>]) -> FrontendInternalResult<()>
    {
        match self.type_entries.get(local_type.index()) {
            Some(TypeStackEntry::Param(type_param_entry)) => {
                let type_param_entry_r = type_param_entry.borrow();
                if type_param_entry_r.trait_names.contains(&TraitName::Fun) {
                    let mut new_type_values: Vec<Rc<TypeValue>> = Vec::new();
                    for type_value in &type_param_entry_r.type_values {
                        match type_value.substitute(type_values) {
                            Ok(Some(new_type_value)) => new_type_values.push(new_type_value),
                            Ok(None) => new_type_values.push(type_value.clone()),
                            Err(err) => return Err(FrontendInternalError(format!("set_type_value_for_local_type: {}", err))),
                        }
                    }
                    let mut shared_flag = SharedFlag::Shared;
                    for closure_local_type in &type_param_entry_r.closure_local_types {
                        if self.shared_flag_for_local_type(*closure_local_type, tree)? == SharedFlag::None {
                            shared_flag = SharedFlag::None;
                        }
                    }
                    let uniq_flag = if shared_flag == SharedFlag::None {
                        UniqFlag::Uniq
                    } else {
                        UniqFlag::None
                    };
                    match type_values.get_mut(local_type.index()) {
                        Some(type_value) => *type_value = Rc::new(TypeValue::Type(uniq_flag, TypeValueName::Fun, new_type_values)),
                        None => return Err(FrontendInternalError(String::from("set_type_value_for_local_type: no type value"))),
                    }
                } else {
                    match type_values.get_mut(local_type.index()) {
                        Some(type_value) => *type_value = Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, Vec::new())),
                        None => return Err(FrontendInternalError(String::from("set_type_value_for_local_type: no type value"))),
                    }
                }
            },
            Some(TypeStackEntry::Type(type_value)) => {
                let new_type_value = match type_value.substitute(type_values) {
                    Ok(Some(new_type_value)) => new_type_value,
                    Ok(None) => type_value.clone(),
                    Err(err) => return Err(FrontendInternalError(format!("set_type_value_for_local_type: {}", err))),
                };
                match type_values.get_mut(local_type.index()) {
                    Some(type_value2) => *type_value2 = new_type_value,
                    None => return Err(FrontendInternalError(String::from("set_type_value_for_local_type: no type value"))),
                }
            },
            None => return Err(FrontendInternalError(String::from("set_type_value_for_local_type: no local type entry"))),
        }
        Ok(())
    }

    pub fn change_type_params_to_types(&mut self, tree: &Tree) -> FrontendInternalResult<Option<LocalType>>
    {
        let mut visited_local_types: BTreeSet<LocalType> = BTreeSet::new();
        let mut type_values = vec![Rc::new(TypeValue::Type(UniqFlag::None, TypeValueName::Tuple, Vec::new())); self.type_entries.len()];
        for i in 0..self.type_entries.len() {
            let local_type = LocalType::new(i);
            dfs_with_result(&local_type, &mut visited_local_types, &mut (), |local_type, processed_local_types, _| {
                    self.local_types_for_local_type_and_change(*local_type, processed_local_types)
            }, |local_type, _| {
                    self.set_type_value_for_local_type(*local_type, tree, type_values.as_mut_slice())
            })?;
        }
        let last_idx = self.type_values.last().map(|p| p.1).unwrap_or(0);
        let last_type_value = type_values.get(last_idx);
        for (type_values2, idx) in self.type_values.iter_mut().rev() {
            if *idx == 0 {
                break;
            }
            for type_value2 in type_values2 {
                let new_type_value2 = match type_value2.substitute(type_values.as_slice()) {
                    Ok(Some(new_type_value)) => new_type_value,
                    Ok(None) => type_value2.clone(),
                    Err(err) => return Err(FrontendInternalError(format!("change_type_params_to_types: {}", err))),
                };
                *type_value2 = new_type_value2;
            }
            *idx = 0;
        }
        self.type_entries.clear();
        match last_type_value {
            Some(last_type_value) => {
                self.type_entries.push(TypeStackEntry::Type(last_type_value.clone()));
                Ok(Some(LocalType::new(0)))
            },
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests;
