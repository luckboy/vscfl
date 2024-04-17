//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::cmp::min;
use std::collections::BTreeSet;
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;

#[derive(Clone)]
pub enum MismatchedTypeInfo
{
    Param(LocalType, TraitName, LocalType),
    Type(TypeName, TraitName, LocalType),
    Eq(LocalType, LocalType, LocalType),
}

#[derive(Clone)]
pub enum TypeMatcherResult
{
    Matched,
    Mismatched(Vec<MismatchedTypeInfo>),
}

pub struct TypeMatcher
{}

impl TypeMatcher
{
    pub fn new() -> Self
    { TypeMatcher {} }
    
    fn shared_flag_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<SharedFlag>
    {
        match local_types.type_entry_for_type_value(type_value) {
            Some(LocalTypeEntry::Param(_, UniqFlag::None, type_param_entry, _)) => {
                let type_param_entry_r = type_param_entry.borrow();
                if type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                    Ok(SharedFlag::Shared)
                } else {
                    Ok(SharedFlag::None)
                }
            },
            Some(LocalTypeEntry::Param(_, UniqFlag::Uniq, _, _)) => Ok(SharedFlag::None),
            Some(LocalTypeEntry::Type(type_value)) => {
                match &*type_value {
                    TypeValue::Param(_, _) => Err(FrontendError::Internal(String::from("type parameter in local type entry"))),
                    TypeValue::Type(UniqFlag::None, TypeValueName::Fun, _) => Ok(SharedFlag::Shared),
                    TypeValue::Type(UniqFlag::None, type_value_name, type_values) => {
                        let mut shared_flag = match type_value_name {
                            TypeValueName::Name(ident) => {
                                match tree.type_var(ident) {
                                    Some(type_var) => {
                                        let type_var_r = type_var.borrow();
                                        match &*type_var_r {
                                            TypeVar::Builtin(_, Some(tmp_shared_flag)) => *tmp_shared_flag,
                                            TypeVar::Data(_, _, Some(tmp_shared_flag)) => *tmp_shared_flag,
                                            _ => return Err(FrontendError::Internal(String::from("type variable isn't type or type hasn't shared flag"))),
                                        }
                                    },
                                    None => return Err(FrontendError::Internal(String::from("no type variable"))),
                                }
                            },
                            _ => SharedFlag::Shared,
                        };
                        for type_value in type_values {
                            if self.shared_flag_for_type_value(type_value, tree, local_types)? == SharedFlag::None {
                                shared_flag = SharedFlag::None;
                            }
                        }
                        Ok(shared_flag)
                    },
                    _ => Ok(SharedFlag::None),
                }
                
            },
            _ => Err(FrontendError::Internal(String::from("no local type entry"))),
        }
    }
    
    fn match_local_type_entries_with_infos(&self, local_type_entry1: &LocalTypeEntry, local_type_entry2: &LocalTypeEntry, tree: &Tree, local_types: &mut LocalTypes, infos: &mut Vec<MismatchedTypeInfo>) -> FrontendResult<bool>
    {
        match (local_type_entry1, local_type_entry2) {
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag2, type_param_entry2, local_type2)) => {
                if *uniq_flag1 != *uniq_flag2 {
                    return Ok(false);
                }
                let type_param_entry1_r = type_param_entry1.borrow();
                let type_param_entry2_r = type_param_entry2.borrow();
                let mut are_type_values = true;
                if (type_param_entry1_r.trait_names.is_empty() || (type_param_entry1_r.trait_names.len() == 1 && type_param_entry1_r.trait_names.contains(&TraitName::Shared))) && type_param_entry1_r.type_values.is_empty() {
                    are_type_values = false;
                }
                if (type_param_entry1_r.trait_names.is_empty() || (type_param_entry1_r.trait_names.len() == 1 && type_param_entry1_r.trait_names.contains(&TraitName::Shared))) && type_param_entry2_r.type_values.is_empty() {
                    are_type_values = false;
                }
                if are_type_values {
                    if type_param_entry1_r.trait_names.len() != type_param_entry2_r.trait_names.len() {
                        return Ok(false);
                    }
                    let mut is_success = true;
                    for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                        if !self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)? {
                            is_success = false;
                        }
                    }
                    if !is_success {
                        return Ok(false);
                    }
                }
                let new_trait_names: BTreeSet<TraitName> = type_param_entry1_r.trait_names.union(&type_param_entry2_r.trait_names).map(|e| e.clone()).collect();
                let new_type_values = if type_param_entry1_r.type_values.len() > type_param_entry2_r.type_values.len() {
                    type_param_entry1_r.type_values.clone()
                } else {
                    type_param_entry2_r.type_values.clone()
                };
                let new_number = match (type_param_entry1_r.number, type_param_entry2_r.number) {
                    (Some(num1), Some(num2)) => Some(min(num1, num2)),
                    (Some(num1), None) => Some(num1),
                    (None, Some(num2)) => Some(num2),
                    (None, None) => None,
                };
                let mut new_type_param_entry = TypeParamEntry::new();
                new_type_param_entry.trait_names = new_trait_names;
                new_type_param_entry.type_values = new_type_values;
                new_type_param_entry.number = new_number;
                let root_local_type = local_types.join_local_types(*local_type1, *local_type2).0;
                local_types.set_type_param_entry(root_local_type, Rc::new(RefCell::new(new_type_param_entry)), DefinedFlag::Undefined);
                Ok(true)
            },
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Param(DefinedFlag::Defined, uniq_flag2, type_param_entry2, local_type2)) => {
                if *uniq_flag1 != *uniq_flag2 {
                    return Ok(false);
                }
                let type_param_entry1_r = type_param_entry1.borrow();
                let type_param_entry2_r = type_param_entry2.borrow();
                if type_param_entry1_r.type_values.len() != type_param_entry2_r.type_values.len() {
                    return Ok(false);
                }
                let mut is_success = true;
                for trait_name in &type_param_entry1_r.trait_names {
                    if !type_param_entry2_r.trait_names.contains(trait_name) {
                        infos.push(MismatchedTypeInfo::Param(*local_type2, trait_name.clone(), *local_type1)); 
                        is_success = false;
                    }
                }
                for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                    if !self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)? {
                        is_success = false;
                    }
                }
                match type_param_entry2_r.orig_local_type {
                    Some(orig_local_type) => {
                        for i in 0..local_types.orig_eq_type_param_set().len() {
                            let local_type = LocalType::new(i);
                            if !local_types.has_orig_eq_type_params(orig_local_type, local_type) {
                                if local_types.has_eq_type_params(*local_type1, local_type) {
                                    infos.push(MismatchedTypeInfo::Eq(*local_type1, local_type, orig_local_type));
                                    is_success = false;
                                }
                            }
                        }
                    },
                    None => return Err(FrontendError::Internal(String::from("no original local type"))),
                }
                if !is_success {
                    return Ok(false);
                }
                let root_local_type = local_types.join_local_types(*local_type1, *local_type2).0;
                local_types.set_type_param_entry(root_local_type, type_param_entry2.clone(), DefinedFlag::Undefined);
                Ok(true)
            },
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Type(type_value2)) => {
                match &**type_value2 {
                    TypeValue::Param(_, _) => Err(FrontendError::Internal(String::from("type parameter in local type entry"))),
                    TypeValue::Type(uniq_flag2, type_value_name2, type_values2) => {
                        if *uniq_flag1 == UniqFlag::Uniq && *uniq_flag2 == UniqFlag::None {
                            return Ok(false)
                        }
                        let type_param_entry1_r = type_param_entry1.borrow();
                        if type_param_entry1_r.type_values.len() != type_values2.len() {
                            return Ok(false);
                        }
                        let mut is_success = true;
                        for trait_name in &type_param_entry1_r.trait_names {
                            let type_name = match type_value2.type_name() {
                                Some(tmp_type_name) => tmp_type_name,
                                None => return Err(FrontendError::Internal(String::from("no type name"))),
                            };
                            match trait_name {
                                TraitName::Shared => {
                                    let shared_flag = self.shared_flag_for_type_value(type_value2, tree, local_types)?;
                                    if shared_flag == SharedFlag::None {
                                        infos.push(MismatchedTypeInfo::Type(type_name, trait_name.clone(), *local_type1));
                                        is_success = false;
                                    }
                                },
                                TraitName::Fun => {
                                    match type_value_name2 {
                                        TypeValueName::Fun => (),
                                        _ => {
                                            infos.push(MismatchedTypeInfo::Type(type_name, trait_name.clone(), *local_type1));
                                            is_success = false;
                                        },
                                    }
                                },
                                TraitName::Name(ident) => {
                                    match tree.trait1(ident) {
                                        Some(trait1) => {
                                            let trait_r = trait1.borrow();
                                            match &*trait_r {
                                                Trait(_, _, Some(trait_vars)) => {
                                                    if trait_vars.impl1(&type_name).is_none() {
                                                        match type_name {
                                                            TypeName::Array(Some(_)) => {
                                                                if trait_vars.impl1(&TypeName::Array(None)).is_none() {
                                                                    infos.push(MismatchedTypeInfo::Type(type_name, trait_name.clone(), *local_type1));
                                                                    is_success = false;
                                                                }
                                                            },
                                                            _ => {
                                                                infos.push(MismatchedTypeInfo::Type(type_name, trait_name.clone(), *local_type1));
                                                                is_success = false;
                                                            },
                                                        }
                                                    }
                                                },
                                                _ => return Err(FrontendError::Internal(String::from("no trait variables")))
                                            }
                                        },
                                        _ => return Err(FrontendError::Internal(String::from("no trait"))),
                                    }
                                },
                            }
                        }
                        for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_values2.iter()) {
                            if !self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)? {
                                is_success = false;
                            }
                        }
                        if !is_success {
                            return Ok(false);
                        }
                        local_types.set_type_value(*local_type1, type_value2.clone());
                        Ok(true)
                    },
                }
            },
            (LocalTypeEntry::Param(DefinedFlag::Defined, _, _, _), LocalTypeEntry::Param(DefinedFlag::Undefined, _, _, _)) => {
                self.match_local_type_entries_with_infos(local_type_entry2, local_type_entry1, tree, local_types, infos)
            },
            (LocalTypeEntry::Param(DefinedFlag::Defined, uniq_flag1, _, local_type1), LocalTypeEntry::Param(DefinedFlag::Defined, uniq_flag2, _, local_type2)) => {
                if *uniq_flag1 != *uniq_flag2 || *local_type1 != *local_type2 {
                    return Ok(false);
                }
                Ok(true)
            },
            (LocalTypeEntry::Type(_), LocalTypeEntry::Param(DefinedFlag::Undefined, _, _, _)) => {
                self.match_local_type_entries_with_infos(local_type_entry2, local_type_entry1, tree, local_types, infos)
            },
            (LocalTypeEntry::Type(type_value1), LocalTypeEntry::Type(type_value2)) => {
                match (&**type_value1, &**type_value2) {
                    (TypeValue::Type(_, type_value_name1, type_values1), TypeValue::Type(_, type_value_name2, type_values2)) => {
                        if type_values1.len() != type_values2.len() {
                            return Ok(false);
                        }
                        let shared_flag1 = self.shared_flag_for_type_value(type_value1, tree, local_types)?;
                        let shared_flag2 = self.shared_flag_for_type_value(type_value2, tree, local_types)?;
                        if shared_flag1 != shared_flag2 {
                            return Ok(false);
                        }
                        if type_value_name1 != type_value_name2 {
                            return Ok(false);
                        }
                        let mut is_success = true;
                        for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                            if !self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)? {
                                is_success = false;
                            }
                        }
                        if !is_success {
                            return Ok(false);
                        }
                        Ok(true)
                    },
                    _ => Err(FrontendError::Internal(String::from("type parameter in local type entry"))),
                }
            }
            _ => Ok(false),
        }
    }

    fn match_type_values_with_infos(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &mut LocalTypes, infos: &mut Vec<MismatchedTypeInfo>) -> FrontendResult<bool>
    {
        let local_type_entry1 = local_types.type_entry_for_type_value(type_value1);
        let local_type_entry2 = local_types.type_entry_for_type_value(type_value2);
        match (local_type_entry1, local_type_entry2) {
            (Some(local_type_entry1), Some(local_type_entry2)) => self.match_local_type_entries_with_infos(&local_type_entry1, &local_type_entry2, tree, local_types, infos),
            (_, _) => Err(FrontendError::Internal(String::from("no local type entry")))
        }
    }
    
    pub fn match_type_values(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &mut LocalTypes) -> FrontendResult<TypeMatcherResult>
    {
        let mut infos: Vec<MismatchedTypeInfo> = Vec::new();
        match self.match_type_values_with_infos(type_value1, type_value2, tree, local_types, &mut infos) {
            Ok(true) => Ok(TypeMatcherResult::Matched),
            Ok(false) => Ok(TypeMatcherResult::Mismatched(infos)),
            Err(err) => Err(err),
        }
    }
    
    pub fn matches(&self, local_type1: LocalType, local_type2: LocalType, tree: &Tree, local_types: &mut LocalTypes) -> FrontendResult<TypeMatcherResult>
    {
        let type_value1 = Rc::new(TypeValue::Param(UniqFlag::None, local_type1));
        let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, local_type2));
        self.match_type_values(&type_value1, &type_value2, tree, local_types)
    }
}
