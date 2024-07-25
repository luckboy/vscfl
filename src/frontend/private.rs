//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::frontend::error::*;
use crate::frontend::tree::*;
use crate::frontend::type_stack::*;

pub(crate) fn pattern_pos(pattern: &Pattern) -> &Pos
{
    match pattern {
        Pattern::Literal(_, _, pos) => pos,
        Pattern::As(_, _, _, _, pos) => pos,
        Pattern::Const(_, _, pos) => pos,
        Pattern::UnnamedFieldCon(_, _, _, _, pos) => pos,
        Pattern::NamedFieldCon(_, _, _, _, pos) => pos,
        Pattern::Var(_, _, _, pos) => pos,
        Pattern::At(_, _, _, _, pos) => pos,
        Pattern::Wildcard(_, pos) => pos,
        Pattern::Alt(_, _, pos) => pos,
    }
}

pub(crate) fn type_name_for_var_ident_and_local_type(ident: &String, local_type: LocalType, tree: &Tree, type_stack: &mut TypeStack, local_types: &LocalTypes) -> FrontendResultWithErrors<Option<TypeName>>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            let (trait_ident, typ) = match &*var_r {
                Var::Builtin(tmp_trait_ident, Some(tmp_type)) => (tmp_trait_ident, tmp_type), 
                Var::Var(_, _, _, _, tmp_trait_ident, _, _, Some(tmp_type), _) => (tmp_trait_ident, tmp_type),
                Var::Fun(_, tmp_trait_ident, Some(tmp_type)) => (tmp_trait_ident, tmp_type),
                _ => return Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_name_for_var_ident_and_local_type: no type"))])),
            };
            match trait_ident {
                Some(trait_ident) => {
                    match type_stack.push_type_entries_for_local_type(local_type, local_types) {
                        Ok(new_local_type) => {
                            match type_stack.type_name_for_local_type_and_type(new_local_type, typ, trait_ident.as_str()) {
                                Ok(type_name) => {
                                    type_stack.pop_type_entries();
                                    Ok(type_name)
                                },
                                Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("{}", err))])),
                            }
                        },
                        Err(err) => Err(FrontendErrors::new(vec![FrontendError::Internal(format!("{}", err))])),
                    }
                },
                None => Ok(None),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_name_for_var_ident_and_local_type: no variable"))])),
    }
}

pub(crate) fn type_for_fun_ident_in<T, F>(ident: &String, tree: &Tree, mut f: F) -> FrontendResultWithErrors<T>
    where F: FnMut(&Type) -> FrontendResultWithErrors<T>
{
    match tree.var(ident) {
        Some(var) => {
            let var_r = var.borrow();
            match &*var_r {
                Var::Fun(_, _, Some(typ)) => f(typ),
                _ => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_fun_ident_in: variable isn't function or no type"))])),
            }
        },
        None => Err(FrontendErrors::new(vec![FrontendError::Internal(String::from("type_for_fun_ident_in: no variable"))])),
    }
}
