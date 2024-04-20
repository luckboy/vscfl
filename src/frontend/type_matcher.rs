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
use std::fmt;
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;

#[derive(Clone)]
pub enum MismatchedTypeInfo
{
    Param(LocalType, TraitName, LocalType),
    Type(TypeName, TraitName, LocalType),
    Eq(LocalType, LocalType, LocalType),
    SharedClosure(LocalType),
    NoClosure(LocalType, LocalType),
    InNonUniqLambda,
}

#[derive(Clone)]
pub struct MismatchedTypeInfoWidthLocalTypes<'a, 'b>(pub &'a MismatchedTypeInfo, pub &'b LocalTypes);

impl<'a, 'b> fmt::Display for MismatchedTypeInfoWidthLocalTypes<'a, 'b>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self.0 {
            MismatchedTypeInfo::Param(local_type1, trait_name, local_type2) => {
                write!(f, "type parameter {} hasn't trait {} that is required by type parameter {}", LocalTypeWithLocalTypes(*local_type1, self.1), trait_name, LocalTypeWithLocalTypes(*local_type2, self.1))
            },
            MismatchedTypeInfo::Type(type_name, trait_name, local_type) => {
                write!(f, "type {} hasn't implemented trait {} that is required by type parameter {}", type_name, trait_name, LocalTypeWithLocalTypes(*local_type, self.1))
            },
            MismatchedTypeInfo::Eq(local_type1, local_type2, local_type3) => {
                write!(f, "type parameter {} is equal to type parameter {} that mustn't be equal to type parameter {}", LocalTypeWithLocalTypes(*local_type1, self.1), LocalTypeWithLocalTypes(*local_type2, self.1), LocalTypeWithLocalTypes(*local_type3, self.1))
            },
            MismatchedTypeInfo::SharedClosure(local_type) => {
                write!(f, "closure variable type {} mustn't shared", LocalTypeWithLocalTypes(*local_type, self.1))
            },
            MismatchedTypeInfo::NoClosure(local_type1, local_type2) => {
                write!(f, "closure variable of type {} isn't in function of type parameter {}", LocalTypeWithLocalTypes(*local_type1, self.1), LocalTypeWithLocalTypes(*local_type2, self.1))
            },
            MismatchedTypeInfo::InNonUniqLambda => {
                write!(f, "closure variable type parameter mustn't be unique type in non-unique lambda")
            },
        }
    }
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
    
    fn uniq_flag_and_shared_flag_for_type_value2(&self, type_value: &Rc<TypeValue>, type_arg_shared_flag: Option<SharedFlag>, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<(UniqFlag, SharedFlag)>
    {
        match local_types.type_entry_for_type_value(type_value) {
            Some(LocalTypeEntry::Param(_, UniqFlag::None, type_param_entry, _)) => {
                let type_param_entry_r = type_param_entry.borrow();
                if type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                    Ok((UniqFlag::None, SharedFlag::Shared))
                } else {
                    Ok((UniqFlag::None, SharedFlag::None))
                }
            },
            Some(LocalTypeEntry::Param(_, UniqFlag::Uniq, _, _)) => Ok((UniqFlag::None, SharedFlag::None)),
            Some(LocalTypeEntry::Type(type_value)) => {
                match &*type_value {
                    TypeValue::Param(_, _) => Err(FrontendError::Internal(String::from("type parameter in local type entry"))),
                    TypeValue::Type(UniqFlag::None, TypeValueName::Fun, _) => Ok((UniqFlag::None, SharedFlag::Shared)),
                    TypeValue::Type(UniqFlag::None, type_value_name, type_values) => {
                        let mut shared_flag = match type_value_name {
                            TypeValueName::Name(ident) => {
                                match tree.type_var(ident) {
                                    Some(type_var) => {
                                        let type_var_r = type_var.borrow();
                                        match &*type_var_r {
                                            TypeVar::Builtin(_, _, Some(tmp_shared_flag)) => *tmp_shared_flag,
                                            TypeVar::Data(_, _, Some(tmp_shared_flag)) => *tmp_shared_flag,
                                            _ => return Err(FrontendError::Internal(String::from("type variable isn't type or type hasn't shared flag"))),
                                        }
                                    },
                                    None => return Err(FrontendError::Internal(String::from("no type variable"))),
                                }
                            },
                            _ => SharedFlag::Shared,
                        };
                        match type_arg_shared_flag {
                            Some(SharedFlag::None) => shared_flag = SharedFlag::None,
                            Some(SharedFlag::Shared) => (), 
                            None => {
                                if shared_flag == SharedFlag::Shared {
                                    for type_value in type_values {
                                        if self.shared_flag_for_type_value2(type_value, None, tree, local_types)? == SharedFlag::None {
                                            shared_flag = SharedFlag::None;
                                        }
                                    }
                                }
                            },
                        }
                        let uniq_flag = if shared_flag == SharedFlag::None {
                            UniqFlag::Uniq
                        } else {
                            UniqFlag::None
                        };
                        Ok((uniq_flag, shared_flag))
                    },
                    _ => Ok((UniqFlag::Uniq, SharedFlag::None)),
                }
            },
            None => Err(FrontendError::Internal(String::from("no local type entry"))),
        }
    }

    fn shared_flag_for_type_value2(&self, type_value: &Rc<TypeValue>, type_arg_shared_flag: Option<SharedFlag>, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<SharedFlag>
    {
        match self.uniq_flag_and_shared_flag_for_type_value2(type_value, type_arg_shared_flag, tree, local_types) {
            Ok((_, shared_flag)) => Ok(shared_flag),
            Err(err) => Err(err),
        }
    }
    
    fn set_shared_for_local_type(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<bool>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        match local_types.type_entry_for_type_value(&type_value) {
            Some(LocalTypeEntry::Param(defined_flag, UniqFlag::None, type_param_entry, _)) => {
                let mut type_param_entry_r = type_param_entry.borrow_mut();
                if !type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                    if defined_flag == DefinedFlag::Undefined { 
                        type_param_entry_r.trait_names.insert(TraitName::Shared);
                    } else {
                        return Ok(false);
                    }
                }
                Ok(true)
            },
            Some(LocalTypeEntry::Param(_, UniqFlag::Uniq, _, _)) => {
                Ok(false)
            },
            Some(LocalTypeEntry::Type(type_value)) => {
                let shared_flag = self.shared_flag_for_type_value2(&type_value, None, tree, local_types)?;
                if shared_flag == SharedFlag::None {
                    return Ok(false);
                }
                Ok(true)
            },
            None=> Err(FrontendError::Internal(String::from("no local type entry"))),
        }
    }
    
    fn match_local_type_entries_with_infos(&self, local_type_entry1: &LocalTypeEntry, local_type_entry2: &LocalTypeEntry, tree: &Tree, local_types: &mut LocalTypes, infos: &mut Vec<MismatchedTypeInfo>) -> FrontendResult<Option<SharedFlag>>
    {
        match (local_type_entry1, local_type_entry2) {
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag2, type_param_entry2, local_type2)) => {
                if *uniq_flag1 != *uniq_flag2 {
                    return Ok(None);
                }
                let uniq_flag = *uniq_flag1;
                if *local_type1 == *local_type2 {
                    let shared_flag = self.shared_flag_for_type_value2(&Rc::new(TypeValue::Param(uniq_flag, *local_type1)), None, tree, local_types)?;
                    return Ok(Some(shared_flag));
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
                let mut is_success = true;
                if are_type_values {
                    if type_param_entry1_r.trait_names.len() != type_param_entry2_r.trait_names.len() {
                        return Ok(None);
                    }
                    for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                        if self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)?.is_none() {
                            is_success = false;
                        }
                    }
                }
                if !type_param_entry1_r.trait_names.contains(&TraitName::Shared) && type_param_entry2_r.trait_names.contains(&TraitName::Shared) {
                    for closure_local_type in &type_param_entry1_r.closure_local_types {
                        if !self.set_shared_for_local_type(*closure_local_type, tree, local_types)? {
                            infos.push(MismatchedTypeInfo::SharedClosure(*closure_local_type));
                            is_success = false;
                        }
                    }
                } else if type_param_entry1_r.trait_names.contains(&TraitName::Shared) && !type_param_entry2_r.trait_names.contains(&TraitName::Shared) {
                    for closure_local_type in &type_param_entry2_r.closure_local_types {
                        if !self.set_shared_for_local_type(*closure_local_type, tree, local_types)? {
                            infos.push(MismatchedTypeInfo::SharedClosure(*closure_local_type));
                            is_success = false;
                        }
                    }
                }  
                if !is_success {
                    return Ok(None);
                }
                let new_trait_names: BTreeSet<TraitName> = type_param_entry1_r.trait_names.union(&type_param_entry2_r.trait_names).map(|e| e.clone()).collect();
                let new_type_values = if type_param_entry1_r.type_values.len() > type_param_entry2_r.type_values.len() {
                    type_param_entry1_r.type_values.clone()
                } else {
                    type_param_entry2_r.type_values.clone()
                };
                let new_closure_local_types: BTreeSet<LocalType> = type_param_entry1_r.closure_local_types.union(&type_param_entry2_r.closure_local_types).map(|e| e.clone()).collect();
                let new_number = match (type_param_entry1_r.number, type_param_entry2_r.number) {
                    (Some(num1), Some(num2)) => Some(min(num1, num2)),
                    (Some(num1), None) => Some(num1),
                    (None, Some(num2)) => Some(num2),
                    (None, None) => None,
                };
                let mut new_type_param_entry = TypeParamEntry::new();
                new_type_param_entry.trait_names = new_trait_names;
                new_type_param_entry.type_values = new_type_values;
                new_type_param_entry.closure_local_types = new_closure_local_types;
                new_type_param_entry.number = new_number;
                let is_in_non_uniq_lambda = local_types.has_in_non_uniq_lambda(*local_type1) | local_types.has_in_non_uniq_lambda(*local_type2);
                let (root_local_type, eq_root_local_type) = local_types.join_local_types(*local_type1, *local_type2);
                local_types.set_type_param_entry(root_local_type, Rc::new(RefCell::new(new_type_param_entry)), DefinedFlag::Undefined);
                local_types.set_in_non_uniq_lambda(eq_root_local_type, is_in_non_uniq_lambda);
                let shared_flag = self.shared_flag_for_type_value2(&Rc::new(TypeValue::Param(uniq_flag, root_local_type)), None, tree, local_types)?;
                Ok(Some(shared_flag))
            },
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Param(DefinedFlag::Defined, uniq_flag2, type_param_entry2, local_type2)) => {
                if *uniq_flag1 != *uniq_flag2 {
                    return Ok(None);
                }
                let uniq_flag = *uniq_flag1;
                let type_param_entry1_r = type_param_entry1.borrow();
                let type_param_entry2_r = type_param_entry2.borrow();
                let mut are_type_values = true;
                if (type_param_entry1_r.trait_names.is_empty() || (type_param_entry1_r.trait_names.len() == 1 && type_param_entry1_r.trait_names.contains(&TraitName::Shared))) && type_param_entry1_r.type_values.is_empty() {
                    are_type_values = false;
                }
                let mut is_success = true;
                if are_type_values {
                    if type_param_entry1_r.type_values.len() != type_param_entry2_r.type_values.len() {
                        return Ok(None);
                    }
                    for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_param_entry2_r.type_values.iter()) {
                        if self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)?.is_none() {
                            is_success = false;
                        }
                    }
                }
                for trait_name in &type_param_entry1_r.trait_names {
                    if !type_param_entry2_r.trait_names.contains(trait_name) {
                        infos.push(MismatchedTypeInfo::Param(*local_type2, trait_name.clone(), *local_type1)); 
                        is_success = false;
                    }
                }
                for i in 0..local_types.orig_eq_type_param_set().len() {
                    let local_type = LocalType::new(i);
                    if !local_types.has_orig_eq_type_params(*local_type2, local_type) {
                        if local_types.has_eq_type_params(*local_type1, local_type) {
                            infos.push(MismatchedTypeInfo::Eq(*local_type1, local_type, *local_type2));
                            is_success = false;
                        }
                    }
                }
                for closure_local_type in &type_param_entry1_r.closure_local_types {
                    if !type_param_entry2_r.closure_local_types.contains(closure_local_type) {
                        infos.push(MismatchedTypeInfo::NoClosure(*closure_local_type, *local_type2));
                    }
                }
                if !is_success {
                    return Ok(None);
                }
                let is_in_non_uniq_lambda = local_types.has_in_non_uniq_lambda(*local_type1) | local_types.has_in_non_uniq_lambda(*local_type2);
                let (root_local_type, eq_root_local_type) = local_types.join_local_types(*local_type1, *local_type2);
                local_types.set_type_param_entry(root_local_type, type_param_entry2.clone(), DefinedFlag::Undefined);
                local_types.set_in_non_uniq_lambda(eq_root_local_type, is_in_non_uniq_lambda);
                let shared_flag = self.shared_flag_for_type_value2(&Rc::new(TypeValue::Param(uniq_flag, root_local_type)), None, tree, local_types)?;
                Ok(Some(shared_flag))
            },
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Type(type_value2)) => {
                match &**type_value2 {
                    TypeValue::Param(_, _) => Err(FrontendError::Internal(String::from("type parameter in local type entry"))),
                    TypeValue::Type(uniq_flag2, type_value_name2, type_values2) => {
                        if *uniq_flag1 == UniqFlag::Uniq && *uniq_flag2 == UniqFlag::None {
                            return Ok(None);
                        }
                        let type_param_entry1_r = type_param_entry1.borrow();
                        if type_param_entry1_r.type_values.len() != type_values2.len() {
                            return Ok(None);
                        }
                        let mut is_success = true;
                        let mut type_arg_shared_flag = SharedFlag::Shared;
                        for (type_value3, type_value4) in type_param_entry1_r.type_values.iter().zip(type_values2.iter()) {
                            match self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)? {
                                Some(tmp_shared_flag) => {
                                    if tmp_shared_flag == SharedFlag::None {
                                        type_arg_shared_flag = SharedFlag::None;
                                    }
                                },
                                None => is_success = false,
                            }
                        }
                        let shared_flag = self.shared_flag_for_type_value2(type_value2, Some(type_arg_shared_flag), tree, local_types)?;
                        for trait_name in &type_param_entry1_r.trait_names {
                            let type_name = match type_value2.type_name() {
                                Some(tmp_type_name) => tmp_type_name,
                                None => return Err(FrontendError::Internal(String::from("no type name"))),
                            };
                            match trait_name {
                                TraitName::Shared => {
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
                        if !type_param_entry1_r.trait_names.contains(&TraitName::Shared) && shared_flag == SharedFlag::Shared {
                            for closure_local_type in &type_param_entry1_r.closure_local_types {
                                if !self.set_shared_for_local_type(*closure_local_type, tree, local_types)? {
                                    infos.push(MismatchedTypeInfo::SharedClosure(*closure_local_type));
                                    is_success = false;
                                }
                            }
                        }
                        if local_types.has_in_non_uniq_lambda(*local_type1) && shared_flag == SharedFlag::None {
                            infos.push(MismatchedTypeInfo::InNonUniqLambda);
                            is_success = false;
                        }
                        if !is_success {
                            return Ok(None);
                        }
                        local_types.set_type_value(*local_type1, type_value2.clone());
                        Ok(Some(shared_flag))
                    },
                }
            },
            (LocalTypeEntry::Param(DefinedFlag::Defined, _, _, _), LocalTypeEntry::Param(DefinedFlag::Undefined, _, _, _)) => {
                self.match_local_type_entries_with_infos(local_type_entry2, local_type_entry1, tree, local_types, infos)
            },
            (LocalTypeEntry::Param(DefinedFlag::Defined, uniq_flag1, _, local_type1), LocalTypeEntry::Param(DefinedFlag::Defined, uniq_flag2, _, local_type2)) => {
                if *uniq_flag1 != *uniq_flag2 || *local_type1 != *local_type2 {
                    return Ok(None);
                }
                let shared_flag = self.shared_flag_for_type_value2(&Rc::new(TypeValue::Param(*uniq_flag1, *local_type1)), None, tree, local_types)?;
                Ok(Some(shared_flag))
            },
            (LocalTypeEntry::Type(_), LocalTypeEntry::Param(DefinedFlag::Undefined, _, _, _)) => {
                self.match_local_type_entries_with_infos(local_type_entry2, local_type_entry1, tree, local_types, infos)
            },
            (LocalTypeEntry::Type(type_value1), LocalTypeEntry::Type(type_value2)) => {
                match (&**type_value1, &**type_value2) {
                    (TypeValue::Type(_, type_value_name1, type_values1), TypeValue::Type(_, type_value_name2, type_values2)) => {
                        if type_values1.len() != type_values2.len() {
                            return Ok(None);
                        }
                        if type_value_name1 != type_value_name2 {
                            return Ok(None);
                        }
                        let mut is_success = true;
                        let mut type_arg_shared_flag = SharedFlag::Shared;
                        for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                            match  self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)? {
                                Some(tmp_shared_flag) => {
                                    if tmp_shared_flag == SharedFlag::None {
                                        type_arg_shared_flag = SharedFlag::None;
                                    }
                                },
                                None => is_success = false,
                            }
                        }
                        if !is_success {
                            return Ok(None);
                        }
                        let shared_flag1 = self.shared_flag_for_type_value2(type_value1, Some(type_arg_shared_flag), tree, local_types)?;
                        let shared_flag2 = self.shared_flag_for_type_value2(type_value2, Some(type_arg_shared_flag), tree, local_types)?;
                        if shared_flag1 != shared_flag2 {
                            return Ok(None);
                        }
                        let shared_flag = shared_flag1;
                        Ok(Some(shared_flag))
                    },
                    _ => Err(FrontendError::Internal(String::from("type parameter in local type entry"))),
                }
            }
            _ => Ok(None),
        }
    }

    fn match_type_values_with_infos(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &mut LocalTypes, infos: &mut Vec<MismatchedTypeInfo>) -> FrontendResult<Option<SharedFlag>>
    {
        let local_type_entry1 = local_types.type_entry_for_type_value(type_value1);
        let local_type_entry2 = local_types.type_entry_for_type_value(type_value2);
        match (local_type_entry1, local_type_entry2) {
            (Some(local_type_entry1), Some(local_type_entry2)) => self.match_local_type_entries_with_infos(&local_type_entry1, &local_type_entry2, tree, local_types, infos),
            (_, _) => Err(FrontendError::Internal(String::from("no local type entry")))
        }
    }

    pub fn uniq_flag_and_shared_flag_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<(UniqFlag, SharedFlag)>
    { self.uniq_flag_and_shared_flag_for_type_value2(type_value, None, tree, local_types) }

    pub fn uniq_flag_and_shared_flag(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<(UniqFlag, SharedFlag)>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        self.uniq_flag_and_shared_flag_for_type_value(&type_value, tree, local_types)
    }

    pub fn shared_flag_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<SharedFlag>
    { self.shared_flag_for_type_value2(type_value, None, tree, local_types) }
    
    pub fn shared_flag(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendResult<SharedFlag>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        self.shared_flag_for_type_value(&type_value, tree, local_types)
    }
    
    pub fn match_type_values(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &mut LocalTypes) -> FrontendResult<TypeMatcherResult>
    {
        let mut infos: Vec<MismatchedTypeInfo> = Vec::new();
        match self.match_type_values_with_infos(type_value1, type_value2, tree, local_types, &mut infos) {
            Ok(Some(_)) => Ok(TypeMatcherResult::Matched),
            Ok(None) => Ok(TypeMatcherResult::Mismatched(infos)),
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
