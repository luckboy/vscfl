//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::HashMap;
use std::collections::HashSet;
use crate::frontend::tree::*;

#[derive(Clone, Debug)]
pub struct BuiltinTypeVar
{
    pub type_arg_source: String,
    pub field_type_sources: Vec<String>,
    pub filed_indices: Vec<(String, usize)>,
}

#[derive(Clone, Debug)]
pub struct BuiltinVar
{
    pub type_source: String,
    pub where_source: String,
}

#[derive(Clone, Debug)]
pub struct Builtins
{
    type_vars: HashMap<String, BuiltinTypeVar>,
    vars: HashMap<String, BuiltinVar>,
    impl_pairs: HashSet<(String, TypeName)>,
}

impl Builtins
{
    pub fn new() -> Self
    { Builtins { type_vars: HashMap::new(), vars: HashMap::new(), impl_pairs: HashSet::new(), } }
    
    pub fn type_vars(&self) -> &HashMap<String, BuiltinTypeVar>
    { &self.type_vars }

    pub fn type_var(&self, ident: &String) -> Option<&BuiltinTypeVar>
    { self.type_vars.get(ident) }

    pub fn add_type_var(&mut self, ident: String, type_var: BuiltinTypeVar)
    { self.type_vars.insert(ident, type_var); }

    pub fn remove_type_var(&mut self, ident: &String) -> bool
    { self.type_vars.remove(ident).is_some() }

    pub fn vars(&self) -> &HashMap<String, BuiltinVar>
    { &self.vars }

    pub fn var(&self, ident: &String) -> Option<&BuiltinVar>
    { self.vars.get(ident) }

    pub fn add_var(&mut self, ident: String, var: BuiltinVar)
    { self.vars.insert(ident, var); }

    pub fn remove_var(&mut self, ident: &String) -> bool
    { self.vars.remove(ident).is_some() }
    
    pub fn impl_pairs(&self) -> &HashSet<(String, TypeName)>
    { &self.impl_pairs }
    
    pub fn has_impl_pair(&self, impl_pair: &(String, TypeName)) -> bool
    { self.impl_pairs.contains(impl_pair) }

    pub fn add_impl_pair(&mut self, impl_pair: (String, TypeName))
    { self.impl_pairs.insert(impl_pair); }

    pub fn remove_impl_pair(&mut self, impl_pair: &(String, TypeName))
    { self.impl_pairs.remove(impl_pair); }
}
