//
// Copyright (c) 2025 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::rc::*;
use crate::frontend::tree::*;
use crate::backend::error::*;

fn add_mangled_ident_to_string(s: &mut String, ident: &str)
{ s.push_str(format!("{}{}", ident.len(), ident).as_str()); }

fn add_mangled_usize_to_string(s: &mut String, n: usize)
{ s.push_str(format!("I{}I", n).as_str()); }

fn add_mangled_type_value_to_string(s: &mut String, type_value: &TypeValue) -> BackendResult<()>
{
    match type_value {
        TypeValue::Param(_, _) => return Err(BackendError::Internal(String::from("add_mangled_type_value_to_string: type value is type parameter"))),
        TypeValue::Type(uniq_flag, type_value_name, args) => {
            if *uniq_flag == UniqFlag::Uniq {
                s.push('X');
                match type_value_name {
                    TypeValueName::Tuple => {
                        s.push('L');
                        let mut is_first = true;
                        for arg in args {
                            if !is_first {
                                s.push('E');
                            }
                            add_mangled_type_value_to_string(s, arg)?;
                            is_first = false;
                        }
                        s.push('R');
                    },
                    TypeValueName::Array(len) => {
                        s.push('M');
                        add_mangled_type_value_to_string(s, &args[0])?;
                        s.push('T');
                        match len {
                            Some(len) => add_mangled_usize_to_string(s, *len),
                            None => s.push('_'),
                        }
                        s.push('Q');
                    },
                    TypeValueName::Fun => {
                        s.push('L');
                        let mut is_first = true;
                        for arg in &args[0..(args.len() - 1)] {
                            if !is_first {
                                s.push('E');
                            }
                            add_mangled_type_value_to_string(s, arg)?;
                            is_first = false;
                        }
                        s.push('R');
                        s.push('A');
                        add_mangled_type_value_to_string(s, &args[args.len() - 1])?;
                    },
                    TypeValueName::Name(ident) => {
                        add_mangled_ident_to_string(s, ident.as_str());
                        s.push('N');
                        let mut is_first = true;
                        for arg in args {
                            if !is_first {
                                s.push('E');
                            }
                            add_mangled_type_value_to_string(s, arg)?;
                            is_first = false;
                        }
                        s.push('P');
                    },
                }
            }
        },        
    }
    Ok(())
}

fn add_mangled_type_name_to_string(s: &mut String, type_name: &TypeName)
{
    match type_name {
        TypeName::Tuple(field_count) => {
            s.push('L');
            let mut is_first = true;
            for _ in 0..*field_count {
                if !is_first {
                    s.push('E');
                }
                s.push('_');
                is_first = false;
            }
            s.push('R');
        },
        TypeName::Fun(arg_count) => {
            s.push('L');
            let mut is_first = true;
            for _ in 0..*arg_count {
                if !is_first {
                    s.push('E');
                }
                s.push('_');
                is_first = false;
            }
            s.push('R');
            s.push('A');
            s.push('_');
        },
        TypeName::Array(len) => {
            s.push('M');
            s.push('_');
            s.push('T');
            match len {
                Some(len) => add_mangled_usize_to_string(s, *len),
                None => s.push('_'),
            }
            s.push('Q');
        },
        TypeName::Name(ident) => add_mangled_ident_to_string(s, ident.as_str()),
    }
}

fn add_mangled_type_params_to_string(s: &mut String, type_values: &[Rc<TypeValue>], typ: &Type) -> BackendResult<()>
{
    if !type_values.is_empty() && !typ.type_param_entries().is_empty() {
        s.push('N');
        let mut is_first = true;
        for (type_value, type_param_entry) in type_values.iter().zip(typ.type_param_entries().iter()) {
            if !is_first {
                s.push('E');
            }
            let type_param_entry_r = type_param_entry.borrow();
            if (type_param_entry_r.trait_names.is_empty() || (type_param_entry_r.trait_names.len() == 1 && type_param_entry_r.trait_names.contains(&TraitName::Shared))) && type_param_entry_r.type_values.is_empty() {
                add_mangled_type_value_to_string(s, &**type_value)?;
            } else {
                match type_value.type_name() {
                    Some(type_name) => add_mangled_type_name_to_string(s, &type_name),
                    None => return Err(BackendError::Internal(String::from("add_mangled_type_params_to_string: type value hasn't type name"))),
                }
            }
            is_first = false;
        }
        s.push('P');
    }
    Ok(())
}

pub fn mangle_struct_name(type_value: &TypeValue) -> BackendResult<String>
{
    let mut s = String::from("_VS");
    add_mangled_type_value_to_string(&mut s, type_value)?;
    Ok(s)
}

pub fn mangle_private_closure_name(type_value: &TypeValue, idx: usize) -> BackendResult<String>
{
    let mut s = String::from("_VDO");
    add_mangled_type_value_to_string(&mut s, type_value)?;
    add_mangled_usize_to_string(&mut s, idx);
    Ok(s)
}

pub fn mangle_local_closure_name(type_value: &TypeValue, idx: usize) -> BackendResult<String>
{
    let mut s = String::from("_VDK");
    add_mangled_type_value_to_string(&mut s, type_value)?;
    add_mangled_usize_to_string(&mut s, idx);
    Ok(s)
}

pub fn mangle_global_closure_name(type_value: &TypeValue, idx: usize) -> BackendResult<String>
{
    let mut s = String::from("_VDG");
    add_mangled_type_value_to_string(&mut s, type_value)?;
    add_mangled_usize_to_string(&mut s, idx);
    Ok(s)
}

pub fn mangle_union_name(type_value: &TypeValue) -> BackendResult<String>
{
    let mut s = String::from("_VU");
    add_mangled_type_value_to_string(&mut s, type_value)?;
    Ok(s)
}

pub fn mangle_var_name(ident: &str, type_values: &[Rc<TypeValue>], typ: &Type) -> BackendResult<String>
{
    let mut s = String::from("_VV");
    add_mangled_ident_to_string(&mut s, ident);
    add_mangled_type_params_to_string(&mut s, type_values, typ)?;
    Ok(s)
}

pub fn mangle_ref_value_name(idx: usize) -> String
{
    let mut s = String::from("_VW");
    add_mangled_usize_to_string(&mut s, idx);
    s
}

pub fn mangle_lambda_name(ident: &str, type_values: &[Rc<TypeValue>], typ: &Type, local_fun: LocalFun) -> BackendResult<String>
{
    let mut s = String::from("_VB");
    add_mangled_ident_to_string(&mut s, ident);
    add_mangled_type_params_to_string(&mut s, type_values, typ)?;
    add_mangled_usize_to_string(&mut s, local_fun.index());
    Ok(s)
}

pub fn mangle_fun_name(ident: &str, type_values: &[Rc<TypeValue>], typ: &Type) -> BackendResult<String>
{
    let mut s = String::from("_VF");
    add_mangled_ident_to_string(&mut s, ident);
    add_mangled_type_params_to_string(&mut s, type_values, typ)?;
    Ok(s)
}

pub fn mangle_private_alloc_fun_name() -> String
{ String::from("_VHO") }

pub fn mangle_local_alloc_fun_name() -> String
{ String::from("_VHK") }

pub fn mangle_global_alloc_fun_name() -> String
{ String::from("_VHG") }

pub fn mangle_caller_name(type_value: &TypeValue) -> BackendResult<String>
{
    let mut s = String::from("_VC");
    add_mangled_type_value_to_string(&mut s, type_value)?;
    Ok(s)
}
