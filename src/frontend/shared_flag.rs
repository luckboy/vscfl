//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::tree::*;
use crate::frontend::type_matcher::*;

pub fn shared_flag_for_type_value(type_value: &Rc<TypeValue>, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<SharedFlag>
{
    let type_matcher = TypeMatcher::new();
    type_matcher.shared_flag_for_type_value(type_value, tree, local_types)
}

pub fn shared_flag_for_local_type(local_type: LocalType, tree: &Tree, local_types: &LocalTypes) -> FrontendInternalResult<SharedFlag>
{
    let type_matcher = TypeMatcher::new();
    type_matcher.shared_flag(local_type, tree, local_types)
}
