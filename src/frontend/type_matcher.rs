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
use crate::frontend::builtins::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;

#[derive(Clone, Debug)]
pub enum MismatchedTypeInfo
{
    Param(LocalType, TraitName, LocalType),
    Type(TypeName, TraitName, LocalType),
    Eq(LocalType, LocalType, LocalType),
    SharedParam(LocalType),
    SharedClosure(LocalType),
    NoClosure(LocalType, LocalType),
    UniqParam(LocalType),
    InNonUniqLambda,
    DefinedTypeParamEq,
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
            MismatchedTypeInfo::SharedParam(local_type) => {
                write!(f, "type parameter {} mustn't shared", LocalTypeWithLocalTypes(*local_type, self.1))
            },
            MismatchedTypeInfo::SharedClosure(local_type) => {
                write!(f, "closure variable type {} mustn't shared", LocalTypeWithLocalTypes(*local_type, self.1))
            },
            MismatchedTypeInfo::NoClosure(local_type1, local_type2) => {
                write!(f, "closure variable of type {} isn't in function of type parameter {}", LocalTypeWithLocalTypes(*local_type1, self.1), LocalTypeWithLocalTypes(*local_type2, self.1))
            },
            MismatchedTypeInfo::UniqParam(local_type) => {
                write!(f, "type parameter {} mustn't unique", LocalTypeWithLocalTypes(*local_type, self.1))
            },
            MismatchedTypeInfo::InNonUniqLambda => {
                write!(f, "closure variable type parameter mustn't be unique type in non-unique lambda")
            },
            MismatchedTypeInfo::DefinedTypeParamEq => {
                write!(f, "type parameter mustn't be type in defined type parameter equation")
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum TypeMatcherResult
{
    Matched,
    Mismatched(Vec<MismatchedTypeInfo>),
}

pub struct TypeMatcher
{
    empty_type_param_entry: Rc<RefCell<TypeParamEntry>>,
}

impl TypeMatcher
{
    pub fn new() -> Self
    { TypeMatcher { empty_type_param_entry: Rc::new(RefCell::new(TypeParamEntry::new())), } }
    
    fn uniq_flag_and_shared_flag_for_type_value2(&self, type_value: &Rc<TypeValue>, type_arg_shared_flag: Option<SharedFlag>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<(UniqFlag, SharedFlag)>
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
            Some(LocalTypeEntry::Param(_, UniqFlag::Uniq, _, _)) => Ok((UniqFlag::Uniq, SharedFlag::None)),
            Some(LocalTypeEntry::Type(type_value)) => {
                match &*type_value {
                    TypeValue::Param(_, _) => Err(FrontendInternalError(String::from("uniq_flag_and_shared_flag_for_type_value2: type parameter in local type entry"))),
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
                                            _ => return Err(FrontendInternalError(String::from("uniq_flag_and_shared_flag_for_type_value2: type variable isn't type or type hasn't shared flag"))),
                                        }
                                    },
                                    None => return Err(FrontendInternalError(String::from("uniq_flag_and_shared_flag_for_type_value2: no type variable"))),
                                }
                            },
                            _ => SharedFlag::Shared,
                        };
                        match type_arg_shared_flag {
                            Some(SharedFlag::None) => shared_flag = SharedFlag::None,
                            Some(SharedFlag::Shared) => (), 
                            None => {
                                if shared_flag == SharedFlag::Shared {
                                    for type_value2 in type_values {
                                        if self.shared_flag_for_type_value2(type_value2, None, tree, local_types)? == SharedFlag::None {
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
            None => Err(FrontendInternalError(String::from("uniq_flag_and_shared_flag_for_type_value2: no local type entry"))),
        }
    }

    fn uniq_flag_for_type_value2(&self, type_value: &Rc<TypeValue>, type_arg_shared_flag: Option<SharedFlag>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<UniqFlag>
    {
        match self.uniq_flag_and_shared_flag_for_type_value2(type_value, type_arg_shared_flag, tree, local_types) {
            Ok((uniq_flag, _)) => Ok(uniq_flag),
            Err(err) => Err(err),
        }
    }
    
    fn shared_flag_for_type_value2(&self, type_value: &Rc<TypeValue>, type_arg_shared_flag: Option<SharedFlag>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<SharedFlag>
    {
        match self.uniq_flag_and_shared_flag_for_type_value2(type_value, type_arg_shared_flag, tree, local_types) {
            Ok((_, shared_flag)) => Ok(shared_flag),
            Err(err) => Err(err),
        }
    }

    pub fn real_uniq_flag_for_type_value(&self, type_value: &Rc<TypeValue>, local_types: &LocalTypes) -> FrontendInternalResult<UniqFlag>
    {
        match local_types.type_entry_for_type_value(type_value) {
            Some(LocalTypeEntry::Param(_, uniq_flag, _, _)) => Ok(uniq_flag),
            Some(LocalTypeEntry::Type(type_value)) => {
                match &*type_value {
                    TypeValue::Param(_, _) => Err(FrontendInternalError(String::from("real_uniq_flag_for_type_value: type parameter in local type entry"))),
                    TypeValue::Type(uniq_flag, _, _) => Ok(*uniq_flag),
                }
            },
            None => Err(FrontendInternalError(String::from("real_uniq_flag_for_type_value: no local type entry"))),
        }
    }
    
    pub fn real_uniq_flag(&self, local_type: LocalType, local_types: &LocalTypes) -> FrontendInternalResult<UniqFlag>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        self.real_uniq_flag_for_type_value(&type_value, local_types)
    }
    
    pub fn set_shared_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<bool>
    {
        match local_types.type_entry_for_type_value(&type_value) {
            Some(LocalTypeEntry::Param(defined_flag, UniqFlag::None, type_param_entry, _)) => {
                let mut type_param_entry_r = type_param_entry.borrow_mut();
                if !type_param_entry_r.trait_names.contains(&TraitName::Shared) {
                    if defined_flag == DefinedFlag::Undefined { 
                        if !type_param_entry_r.trait_names.contains(&TraitName::Fun) {
                            let mut type_arg_shared_flag = SharedFlag::Shared;
                            for type_value2 in &type_param_entry_r.type_values {
                                if self.shared_flag_for_type_value2(type_value2, None, tree, local_types)? == SharedFlag::None {
                                    type_arg_shared_flag = SharedFlag::None;
                                }
                            }
                            if type_arg_shared_flag == SharedFlag::None {
                                return Ok(false);
                            }
                        }
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
            None=> Err(FrontendInternalError(String::from("set_shared_for_type_value: no local type entry"))),
        }
    }

    pub fn set_shared(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<bool>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        self.set_shared_for_type_value(&type_value, tree, local_types)
    }

    fn set_trait_names_for_local_types(&self, root_local_type1: LocalType, root_local_type2: LocalType, root_local_type: LocalType, trait_names: &BTreeSet<TraitName>, local_types: &mut LocalTypes) -> FrontendInternalResult<()>
    {
        match local_types.eq_root_local_type_and_eq_local_types(root_local_type) {
            Some((eq_root_local_type, eq_local_types)) => {
                let eq_local_type = LocalType::new(local_types.type_entries().root_of(eq_root_local_type.index()));
                match local_types.type_entry_for_type_value(&Rc::new(TypeValue::Param(UniqFlag::None, eq_local_type))) {
                    Some(LocalTypeEntry::Param(_, _, type_param_entry, local_type)) => {
                        if local_type != root_local_type1 && local_type != root_local_type2 {
                            let mut type_param_entry_r = type_param_entry.borrow_mut();
                            type_param_entry_r.trait_names = trait_names.clone();
                        }
                    },
                    Some(_) => return Err(FrontendInternalError(String::from("set_trait_names_for_local_types: no type parameter entry"))),
                    None => return Err(FrontendInternalError(String::from("set_trait_names_for_local_types: no local type entry"))),
                }
                for eq_local_type2 in eq_local_types {
                    match local_types.type_entry(*eq_local_type2) {
                        Some(LocalTypeEntry::Param(_, _, type_param_entry, local_type)) => {
                            if *local_type != root_local_type1 && *local_type != root_local_type2 {
                                let mut type_param_entry_r = type_param_entry.borrow_mut();
                                type_param_entry_r.trait_names = trait_names.clone();
                            }
                        },
                        Some(_) => return Err(FrontendInternalError(String::from("set_trait_names_for_local_types: no type parameter entry"))),
                        None => return Err(FrontendInternalError(String::from("set_trait_names_for_local_types: no local type entry"))),
                    }
                }
                Ok(())
            },
            None => Err(FrontendInternalError(String::from("set_trait_names_for_local_types: no equation root local type and no equation local types"))),
        }
    }
    
    fn match_local_type_entries_with_infos(&self, local_type_entry1: &LocalTypeEntry, local_type_entry2: &LocalTypeEntry, tree: &Tree, local_types: &mut LocalTypes, infos: &mut Vec<MismatchedTypeInfo>) -> FrontendInternalResult<Option<SharedFlag>>
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
                let mut are_type_values1 = true;
                let mut are_type_values2 = true;
                let mut is_success = true;
                let (type_values1, type_values2) = {
                    let type_param_entry1_r = type_param_entry1.borrow();
                    let type_param_entry2_r = type_param_entry2.borrow();
                    if (type_param_entry1_r.trait_names.is_empty() || (type_param_entry1_r.trait_names.len() == 1 && type_param_entry1_r.trait_names.contains(&TraitName::Shared))) && type_param_entry1_r.type_values.is_empty() {
                        are_type_values1 = false;
                    }
                    if (type_param_entry2_r.trait_names.is_empty() || (type_param_entry2_r.trait_names.len() == 1 && type_param_entry2_r.trait_names.contains(&TraitName::Shared))) && type_param_entry2_r.type_values.is_empty() {
                        are_type_values2 = false;
                    }
                    (type_param_entry1_r.type_values.clone(), type_param_entry2_r.type_values.clone())
                };
                if are_type_values1 && are_type_values2 {
                    if type_values1.len() != type_values2.len() {
                        return Ok(None);
                    }
                    for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                        if self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)?.is_none() {
                            is_success = false;
                        }
                    }
                }
                {
                    let type_param_entry1_r = type_param_entry1.borrow();
                    let type_param_entry2_r = type_param_entry2.borrow();
                    if are_type_values1 && !are_type_values2 {
                        if type_param_entry2_r.trait_names.contains(&TraitName::Shared) && !type_param_entry1_r.trait_names.contains(&TraitName::Shared) && !type_param_entry1_r.trait_names.contains(&TraitName::Fun) {
                            let mut type_arg_shared_flag1 = SharedFlag::Shared; 
                            for type_value3 in &type_param_entry1_r.type_values {
                                if self.shared_flag_for_type_value2(type_value3, None, tree, local_types)? == SharedFlag::None {
                                    type_arg_shared_flag1 = SharedFlag::None;
                                }
                            }
                            if type_arg_shared_flag1 == SharedFlag::None {
                                infos.push(MismatchedTypeInfo::SharedParam(*local_type1));
                                is_success = false;
                            }
                        }
                    } else if !are_type_values1 && are_type_values2 {
                        if type_param_entry1_r.trait_names.contains(&TraitName::Shared) && !type_param_entry2_r.trait_names.contains(&TraitName::Shared) && !type_param_entry2_r.trait_names.contains(&TraitName::Fun) {
                            let mut type_arg_shared_flag2 = SharedFlag::Shared; 
                            for type_value4 in &type_param_entry2_r.type_values {
                                if self.shared_flag_for_type_value2(type_value4, None, tree, local_types)? == SharedFlag::None {
                                    type_arg_shared_flag2 = SharedFlag::None;
                                }
                            }
                            if type_arg_shared_flag2 == SharedFlag::None {
                                infos.push(MismatchedTypeInfo::SharedParam(*local_type2));
                                is_success = false;
                            }
                        }
                    }
                    if !type_param_entry1_r.trait_names.contains(&TraitName::Shared) && type_param_entry2_r.trait_names.contains(&TraitName::Shared) {
                        for closure_local_type in &type_param_entry1_r.closure_local_types {
                            if !self.set_shared(*closure_local_type, tree, local_types)? {
                                infos.push(MismatchedTypeInfo::SharedClosure(*closure_local_type));
                                is_success = false;
                            }
                        }
                    } else if type_param_entry1_r.trait_names.contains(&TraitName::Shared) && !type_param_entry2_r.trait_names.contains(&TraitName::Shared) {
                        for closure_local_type in &type_param_entry2_r.closure_local_types {
                            if !self.set_shared(*closure_local_type, tree, local_types)? {
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
                    let new_closure_local_types: BTreeSet<LocalType> = type_param_entry1_r.closure_local_types.union(&type_param_entry2_r.closure_local_types).map(|e| *e).collect();
                    let new_number = match (type_param_entry1_r.number, type_param_entry2_r.number) {
                        (Some(num1), Some(num2)) => Some(min(num1, num2)),
                        (Some(num1), None) => Some(num1),
                        (None, Some(num2)) => Some(num2),
                        (None, None) => None,
                    };
                    let mut new_type_param_entry = TypeParamEntry::new();
                    new_type_param_entry.trait_names = new_trait_names.clone();
                    new_type_param_entry.type_values = new_type_values;
                    new_type_param_entry.closure_local_types = new_closure_local_types;
                    new_type_param_entry.number = new_number;
                    let is_in_non_uniq_lambda = local_types.has_in_non_uniq_lambda(*local_type1) | local_types.has_in_non_uniq_lambda(*local_type2);
                    let is_defined_type_param_eq = local_types.has_defined_type_param_eq(*local_type1) | local_types.has_defined_type_param_eq(*local_type2);
                    local_types.set_type_param_entry(*local_type1, self.empty_type_param_entry.clone(), DefinedFlag::Undefined);
                    local_types.set_type_param_entry(*local_type2, self.empty_type_param_entry.clone(), DefinedFlag::Undefined);
                    match local_types.join_local_types(*local_type1, *local_type2) {
                        Some((root_local_type, eq_root_local_type)) => {
                            local_types.set_type_param_entry(root_local_type, Rc::new(RefCell::new(new_type_param_entry)), DefinedFlag::Undefined);
                            local_types.set_in_non_uniq_lambda(eq_root_local_type, is_in_non_uniq_lambda);
                            local_types.set_defined_type_param_eq(eq_root_local_type, is_defined_type_param_eq);
                            self.set_trait_names_for_local_types(*local_type1, *local_type2, eq_root_local_type, &new_trait_names, local_types)?;
                            let shared_flag = self.shared_flag_for_type_value2(&Rc::new(TypeValue::Param(uniq_flag, root_local_type)), None, tree, local_types)?;
                            Ok(Some(shared_flag))
                        },
                        None => Err(FrontendInternalError(String::from("match_local_type_entries_with_infos: can't join local types"))),
                    }
                }
            },
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Param(DefinedFlag::Defined, uniq_flag2, type_param_entry2, local_type2)) => {
                if *uniq_flag1 != *uniq_flag2 {
                    return Ok(None);
                }
                let uniq_flag = *uniq_flag1;
                let mut are_type_values1 = true;
                let mut is_success = true;
                let (type_values1, type_values2) = {
                    let type_param_entry1_r = type_param_entry1.borrow();
                    let type_param_entry2_r = type_param_entry2.borrow();
                    if (type_param_entry1_r.trait_names.is_empty() || (type_param_entry1_r.trait_names.len() == 1 && type_param_entry1_r.trait_names.contains(&TraitName::Shared))) && type_param_entry1_r.type_values.is_empty() {
                        are_type_values1 = false;
                    }
                    (type_param_entry1_r.type_values.clone(), type_param_entry2_r.type_values.clone())
                };
                if are_type_values1 {
                    if type_values1.len() != type_values2.len() {
                        return Ok(None);
                    }
                    for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                        if self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)?.is_none() {
                            is_success = false;
                        }
                    }
                }
                {
                    let type_param_entry1_r = type_param_entry1.borrow();
                    let type_param_entry2_r = type_param_entry2.borrow();
                    for trait_name in &type_param_entry1_r.trait_names {
                        if !type_param_entry2_r.trait_names.contains(trait_name) {
                            infos.push(MismatchedTypeInfo::Param(*local_type2, trait_name.clone(), *local_type1)); 
                            is_success = false;
                        }
                    }
                    for i in 0..local_types.orig_eq_type_param_set().len() {
                        let local_type = LocalType::new(i);
                        if !local_types.has_eq_type_params(*local_type2, local_type) {
                            if local_types.has_eq_type_params(*local_type1, local_type) {
                                infos.push(MismatchedTypeInfo::Eq(*local_type1, local_type, *local_type2));
                                is_success = false;
                            }
                        }
                    }
                    for closure_local_type in &type_param_entry1_r.closure_local_types {
                        if !type_param_entry2_r.closure_local_types.contains(closure_local_type) {
                            infos.push(MismatchedTypeInfo::NoClosure(*closure_local_type, *local_type2));
                            is_success = false;
                        }
                    }
                    if !is_success {
                        return Ok(None);
                    }
                    let is_in_non_uniq_lambda = local_types.has_in_non_uniq_lambda(*local_type1) | local_types.has_in_non_uniq_lambda(*local_type2);
                    let is_defined_type_param_eq = local_types.has_defined_type_param_eq(*local_type1) | local_types.has_defined_type_param_eq(*local_type2);
                    local_types.set_type_param_entry(*local_type1, self.empty_type_param_entry.clone(), DefinedFlag::Undefined);
                    local_types.set_type_param_entry(*local_type2, self.empty_type_param_entry.clone(), DefinedFlag::Undefined);
                    match local_types.join_local_types(*local_type1, *local_type2) {
                        Some((root_local_type, eq_root_local_type)) => {
                            local_types.set_type_param_entry(root_local_type, type_param_entry2.clone(), DefinedFlag::Defined);
                            local_types.set_in_non_uniq_lambda(eq_root_local_type, is_in_non_uniq_lambda);
                            local_types.set_defined_type_param_eq(eq_root_local_type, is_defined_type_param_eq);
                            self.set_trait_names_for_local_types(*local_type1, *local_type2, eq_root_local_type, &type_param_entry2_r.trait_names, local_types)?;
                            let shared_flag = self.shared_flag_for_type_value2(&Rc::new(TypeValue::Param(uniq_flag, root_local_type)), None, tree, local_types)?;
                            Ok(Some(shared_flag))
                        },
                        None => Err(FrontendInternalError(String::from("match_local_type_entries_with_infos: can't join local types"))),
                    }
                }
            },
            (LocalTypeEntry::Param(DefinedFlag::Undefined, uniq_flag1, type_param_entry1, local_type1), LocalTypeEntry::Type(type_value2)) => {
                match &**type_value2 {
                    TypeValue::Param(_, _) => Err(FrontendInternalError(String::from("match_local_type_entries_with_infos: type parameter in local type entry"))),
                    TypeValue::Type(uniq_flag2, type_value_name2, type_values2) => {
                        if *uniq_flag1 == UniqFlag::Uniq && *uniq_flag2 == UniqFlag::None {
                            return Ok(None);
                        }
                        let mut are_type_values1 = true;
                        let mut type_arg_shared_flag = SharedFlag::Shared;
                        let mut is_success = true;
                        let type_values1 = {
                            let type_param_entry1_r = type_param_entry1.borrow();
                            if (type_param_entry1_r.trait_names.is_empty() || (type_param_entry1_r.trait_names.len() == 1 && type_param_entry1_r.trait_names.contains(&TraitName::Shared))) && type_param_entry1_r.type_values.is_empty() {
                                are_type_values1 = false;
                            }
                            type_param_entry1_r.type_values.clone()
                        };
                        if are_type_values1 {
                            if type_values1.len() != type_values2.len() {
                                return Ok(None);
                            }
                            for (type_value3, type_value4) in type_values1.iter().zip(type_values2.iter()) {
                                match self.match_type_values_with_infos(type_value3, type_value4, tree, local_types, infos)? {
                                    Some(tmp_shared_flag) => {
                                        if tmp_shared_flag == SharedFlag::None {
                                            type_arg_shared_flag = SharedFlag::None;
                                        }
                                    },
                                    None => is_success = false,
                                }
                            }
                        }
                        {
                            let type_param_entry1_r = type_param_entry1.borrow();
                            let shared_flag = if are_type_values1 {
                                self.shared_flag_for_type_value2(type_value2, Some(type_arg_shared_flag), tree, local_types)?
                            } else {
                                self.shared_flag_for_type_value2(type_value2, None, tree, local_types)?
                            };
                            for trait_name in &type_param_entry1_r.trait_names {
                                let type_name = match type_value2.type_name() {
                                    Some(tmp_type_name) => tmp_type_name,
                                    None => return Err(FrontendInternalError(String::from("no type name"))),
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
                                                    _ => return Err(FrontendInternalError(String::from("no trait variables")))
                                                }
                                            },
                                            _ => return Err(FrontendInternalError(String::from("no trait"))),
                                        }
                                    },
                                }
                            }
                            if !type_param_entry1_r.trait_names.contains(&TraitName::Shared) && shared_flag == SharedFlag::Shared {
                                for closure_local_type in &type_param_entry1_r.closure_local_types {
                                    if !self.set_shared(*closure_local_type, tree, local_types)? {
                                        infos.push(MismatchedTypeInfo::SharedClosure(*closure_local_type));
                                        is_success = false;
                                    }
                                }
                            }
                            if local_types.has_in_non_uniq_lambda(*local_type1) && shared_flag == SharedFlag::None {
                                infos.push(MismatchedTypeInfo::InNonUniqLambda);
                                is_success = false;
                            }
                            if local_types.has_defined_type_param_eq(*local_type1) {
                                infos.push(MismatchedTypeInfo::DefinedTypeParamEq);
                                is_success = false;
                            }
                            if !is_success {
                                return Ok(None);
                            }
                            local_types.set_type_value(*local_type1, type_value2.clone());
                            Ok(Some(shared_flag))
                        }
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
                    _ => Err(FrontendInternalError(String::from("match_local_type_entries_with_infos: type parameter in local type entry"))),
                }
            }
            _ => Ok(None),
        }
    }

    fn match_type_values_with_infos(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &mut LocalTypes, infos: &mut Vec<MismatchedTypeInfo>) -> FrontendInternalResult<Option<SharedFlag>>
    {
        let local_type_entry1 = local_types.type_entry_for_type_value(type_value1);
        let local_type_entry2 = local_types.type_entry_for_type_value(type_value2);
        match (local_type_entry1, local_type_entry2) {
            (Some(local_type_entry1), Some(local_type_entry2)) => self.match_local_type_entries_with_infos(&local_type_entry1, &local_type_entry2, tree, local_types, infos),
            (_, _) => Err(FrontendInternalError(String::from("match_type_values_with_infos: no local type entry")))
        }
    }

    pub fn uniq_flag_and_shared_flag_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<(UniqFlag, SharedFlag)>
    { self.uniq_flag_and_shared_flag_for_type_value2(type_value, None, tree, local_types) }

    pub fn uniq_flag_and_shared_flag(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<(UniqFlag, SharedFlag)>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        self.uniq_flag_and_shared_flag_for_type_value(&type_value, tree, local_types)
    }

    pub fn uniq_flag_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<UniqFlag>
    { self.uniq_flag_for_type_value2(type_value, None, tree, local_types) }
    
    pub fn uniq_flag(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<UniqFlag>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        self.uniq_flag_for_type_value(&type_value, tree, local_types)
    }

    pub fn shared_flag_for_type_value(&self, type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<SharedFlag>
    { self.shared_flag_for_type_value2(type_value, None, tree, local_types) }
    
    pub fn shared_flag(&self, local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<SharedFlag>
    {
        let type_value = Rc::new(TypeValue::Param(UniqFlag::None, local_type));
        self.shared_flag_for_type_value(&type_value, tree, local_types)
    }
    
    pub fn match_type_values(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &mut LocalTypes) -> FrontendInternalResult<TypeMatcherResult>
    {
        let mut infos: Vec<MismatchedTypeInfo> = Vec::new();
        match self.match_type_values_with_infos(type_value1, type_value2, tree, local_types, &mut infos) {
            Ok(Some(_)) => Ok(TypeMatcherResult::Matched),
            Ok(None) => Ok(TypeMatcherResult::Mismatched(infos)),
            Err(err) => Err(err),
        }
    }
    
    pub fn matches(&self, local_type1: LocalType, local_type2: LocalType, tree: &Tree, local_types: &mut LocalTypes) -> FrontendInternalResult<TypeMatcherResult>
    {
        let type_value1 = Rc::new(TypeValue::Param(UniqFlag::None, local_type1));
        let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, local_type2));
        self.match_type_values(&type_value1, &type_value2, tree, local_types)
    }

    pub fn match_for_first_pattern_type(&self, local_type1: LocalType, is_var1: bool, local_type2: LocalType, tree: &Tree, local_types: &mut LocalTypes) -> FrontendInternalResult<TypeMatcherResult>
    {
        let type_value1 = Rc::new(TypeValue::Param(UniqFlag::None, local_type1));
        let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, local_type2));
        let uniq_flag = self.real_uniq_flag_for_type_value(&type_value2, local_types)?;
        if uniq_flag == UniqFlag::Uniq {
            if is_var1 && self.shared_flag(local_type1, tree, local_types)? == SharedFlag::Shared {
                return Ok(TypeMatcherResult::Mismatched(vec![MismatchedTypeInfo::UniqParam(local_type1)]));
            }
            local_types.set_uniq(local_type1);
        }
        self.match_type_values(&type_value1, &type_value2, tree, local_types)
    }

    pub fn match_for_second_pattern_type(&self, local_type1: LocalType, local_type2: LocalType, is_var2: bool, tree: &Tree, local_types: &mut LocalTypes) -> FrontendInternalResult<TypeMatcherResult>
    {
        let type_value1 = Rc::new(TypeValue::Param(UniqFlag::None, local_type1));
        let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, local_type2));
        let uniq_flag = self.real_uniq_flag_for_type_value(&type_value1, local_types)?;
        if uniq_flag == UniqFlag::Uniq {
            if is_var2 && self.shared_flag(local_type2, tree, local_types)? == SharedFlag::Shared {
                return Ok(TypeMatcherResult::Mismatched(vec![MismatchedTypeInfo::UniqParam(local_type2)]));
            }
            local_types.set_uniq(local_type2);
        }
        self.match_type_values(&type_value1, &type_value2, tree, local_types)
    }

    fn has_primitive_for_type_ident(&self, ident: &String, tree: &Tree, builtins: &Builtins) -> FrontendInternalResult<bool>
    {
        match tree.type_var(ident) {
            Some(type_var) => {
                let mut type_var_r = type_var.borrow_mut();
                match &mut *type_var_r {
                    TypeVar::Builtin(_, _, _) => {
                        match builtins.type_var(ident) {
                            Some(builtin_type_var) => Ok(builtin_type_var.is_primitive),
                            None => Ok(false),
                        }
                    },
                    TypeVar::Data(_, _, _) => Ok(false),
                    _ => Err(FrontendInternalError(String::from("has_primitive_for_type_ident: type variable is type synonym"))),
                }
            },
            None => Err(FrontendInternalError(String::from("has_primitive_for_type_ident: no type variable"))),
        }
    }
    
    fn match_local_type_entries_for_casting(&self, local_type_entry1: &LocalTypeEntry, local_type_entry2: &LocalTypeEntry, tree: &Tree, local_types: &LocalTypes, builtins: &Builtins) -> FrontendInternalResult<Option<SharedFlag>>
    {
        match (local_type_entry1, local_type_entry2) {
            (LocalTypeEntry::Type(type_value1), LocalTypeEntry::Type(type_value2)) => {
                match (&**type_value1, &**type_value2) {
                    (TypeValue::Type(_, type_value_name1, type_values1), TypeValue::Type(_, type_value_name2, type_values2)) => {
                        let mut is_success = true;
                        match (type_value_name1, type_value_name2) {
                            (TypeValueName::Name(ident1), TypeValueName::Name(ident2)) => {
                                if !self.has_primitive_for_type_ident(ident1, tree, builtins)? || !self.has_primitive_for_type_ident(ident2, tree, builtins)? {
                                    is_success = false;
                                }
                            },
                            (TypeValueName::Tuple, TypeValueName::Tuple) => (),
                            (TypeValueName::Array(Some(len1)), TypeValueName::Array(Some(len2))) if len1 == len2 => (),
                            _ => is_success = false,
                        }
                        if !is_success {
                            return Ok(None);
                        }
                        if type_values1.len() != type_values2.len() {
                            return Ok(None);
                        }
                        let mut type_arg_shared_flag = SharedFlag::Shared;
                        for (type_value1, type_value2) in type_values1.iter().zip(type_values2.iter()) {
                            match self.match_type_values_for_casting2(type_value1, type_value2, tree, local_types, builtins)? {
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
                    _ => Err(FrontendInternalError(String::from("match_local_type_entries_for_casting: no variable"))),
                }
            },
            _ => Ok(None),
        }
    }

    fn match_type_values_for_casting2(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes, builtins: &Builtins) -> FrontendInternalResult<Option<SharedFlag>>
    {
        let local_type_entry1 = local_types.type_entry_for_type_value(type_value1);
        let local_type_entry2 = local_types.type_entry_for_type_value(type_value2);
        match (local_type_entry1, local_type_entry2) {
            (Some(local_type_entry1), Some(local_type_entry2)) => self.match_local_type_entries_for_casting(&local_type_entry1, &local_type_entry2, tree, local_types, builtins),
            (_, _) => Err(FrontendInternalError(String::from("match_type_values_for_casting2: no local type entry"))),
        }
    }

    pub fn match_type_values_for_casting(&self, type_value1: &Rc<TypeValue>, type_value2: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes, builtins: &Builtins) -> FrontendInternalResult<bool>
    {
        match self.match_type_values_for_casting2(type_value1, type_value2, tree, local_types, builtins) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(err) => Err(err),
        }
    }

    pub fn match_for_casting(&self, local_type1: LocalType, local_type2: LocalType, tree: &Tree, local_types: &LocalTypes, builtins: &Builtins) -> FrontendInternalResult<bool>
    {
        let type_value1 = Rc::new(TypeValue::Param(UniqFlag::None, local_type1));
        let type_value2 = Rc::new(TypeValue::Param(UniqFlag::None, local_type2));
        self.match_type_values_for_casting(&type_value1, &type_value2, tree, local_types, builtins)
    }
}

#[cfg(test)]
mod tests;
