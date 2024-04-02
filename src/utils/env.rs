//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;

pub struct Environment<T>
{
    stack: Vec<BTreeMap<String, T>>, 
}

impl<T> Environment<T>
{
    pub fn new() -> Self
    { Environment { stack: Vec::new(), } }
    
    pub fn push_new_vars(&mut self)
    { self.stack.push(BTreeMap::new()); }
    
    pub fn pop_vars(&mut self)
    { self.stack.pop(); }
    
    pub fn var(&self, ident: &String) -> Option<&T>
    {
        for vars in self.stack.iter().rev() {
            match vars.get(ident) {
                Some(value) => return Some(value),
                None => (),
            }
        }
        None
    }

    pub fn var_mut(&mut self, ident: &String) -> Option<&mut T>
    {
        for vars in self.stack.iter_mut().rev() {
            match vars.get_mut(ident) {
                Some(value) => return Some(value),
                None => (),
            }
        }
        None
    }
    
    pub fn add_var(&mut self, ident: String, value: T) -> bool
    {
        match self.stack.last_mut() {
            Some(vars) => {
                vars.insert(ident, value);
                true
            },
            None => false,
        }
    }

    pub fn remove_var(&mut self, ident: &String) -> bool
    {
        match self.stack.last_mut() {
            Some(vars) => {
                match vars.remove(ident) {
                    Some(_) => true,
                    None => false,
                }
            },
            None => false,
        }
    }
}
