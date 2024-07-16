//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;
use crate::frontend::error::*;
use crate::frontend::tree::*;

#[derive(Clone, Debug)]
pub struct Evals
{
    funs: HashMap<(String, Option<TypeName>), fn(&[Value], &Pos) -> FrontendResult<Value>>,
}

impl Evals
{
    pub fn new() -> Self
    { Evals { funs: HashMap::new(), } }
    
    pub fn funs(&self) -> &HashMap<(String, Option<TypeName>), fn(&[Value], &Pos) -> FrontendResult<Value>>
    { &self.funs }

    pub fn fun(&self, key: &(String, Option<TypeName>)) -> Option<fn(&[Value], &Pos) -> FrontendResult<Value>>
    {
        match self.funs.get(key) {
            Some(fun) => Some(*fun),
            None => None,
        }
    }
    
    pub fn add_fun(&mut self, key: (String, Option<TypeName>), fun: fn(&[Value], &Pos) -> FrontendResult<Value>)
    { self.funs.insert(key, fun); }

    pub fn remove_fun(&mut self, key: &(String, Option<TypeName>)) -> bool
    { self.funs.remove(key).is_some() }
}
