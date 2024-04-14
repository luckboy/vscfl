//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct Environment<T>
{
    stack: Vec<BTreeMap<String, T>>,
    saved_var_stack: Vec<BTreeMap<(String, usize), T>>,
}

impl<T: Clone> Environment<T>
{
    pub fn new() -> Self
    { Environment { stack: Vec::new(), saved_var_stack: Vec::new(), } }
    
    pub fn stack_len(&self) -> usize
    { self.stack.len() }

    pub fn push_new_vars(&mut self)
    { self.stack.push(BTreeMap::new()); }
    
    pub fn pop_vars(&mut self)
    { self.stack.pop(); }
    
    pub fn saved_var_stack_len(&self) -> usize
    { self.saved_var_stack.len() }
    
    pub fn push_saved_vars(&mut self)
    { self.saved_var_stack.push(BTreeMap::new()); }

    pub fn merge_and_pop_saved_var_vars<F>(&mut self, saved_var_stack_idx: usize, mut f: F)
        where F: FnMut(&T, &T) -> T
    {
        let mut values: BTreeMap<(String, usize), T> = BTreeMap::new();
        for i in saved_var_stack_idx..self.saved_var_stack.len() {
            for (key, value) in &self.saved_var_stack[i] {
                match values.get(key) {
                    Some(value2) => {
                        values.insert(key.clone(), f(value2, value));
                    },
                    None => {
                        values.insert(key.clone(), value.clone());
                    },
                }
            }
        }
        for ((ident, i), value) in &values {
            match self.stack.get_mut(*i) {
                Some(vars) => {
                    match vars.get_mut(ident) {
                        Some(value2) => *value2 = f(value, value2),
                        None => {
                            vars.insert(ident.clone(), value.clone());
                        },
                    }
                },
                None => (),
            }
        }
        for _ in (saved_var_stack_idx..self.saved_var_stack.len()).rev() {
            self.saved_var_stack.pop();
        }
    }
    
    pub fn restore_vars(&mut self, saved_var_stack_idx: usize) -> bool
    {
        match self.saved_var_stack.get(saved_var_stack_idx) {
            Some(saved_vars) => {
                let mut is_success = true;
                for ((ident, i), value) in saved_vars {
                    match self.stack.get_mut(*i) {
                        Some(vars) => {
                            vars.insert(ident.clone(), value.clone());
                        },
                        None => is_success = false,
                    }
                }
                is_success
            },
            None => false,
        }
    }
    
    pub fn var_and_stack_index(&self, ident: &String) -> Option<(&T, usize)>
    {
        for (i, vars) in self.stack.iter().enumerate().rev() {
            match vars.get(ident) {
                Some(value) => return Some((value, i)),
                None => (),
            }
        }
        None
    }

    pub fn var_mut_and_stack_index(&mut self, ident: &String) -> Option<(&mut T, usize)>
    {
        for (i, vars) in self.stack.iter_mut().enumerate().rev() {
            match vars.get_mut(ident) {
                Some(value) => {
                    match self.saved_var_stack.last_mut() {
                        Some(saved_vars) => {
                            if !saved_vars.contains_key(&(ident.clone(), i)) {
                                saved_vars.insert((ident.clone(), i), value.clone());
                            }
                        },
                        None => (),
                    }
                    return Some((value, i))
                },
                None => (),
            }
        }
        None
    }

    pub fn stack_index(&self, ident: &String) -> Option<usize>
    { 
        match self.var_and_stack_index(ident) {
            Some((_, i)) => Some(i),
            None => None,
        }
    }
    
    pub fn var(&self, ident: &String) -> Option<&T>
    { 
        match self.var_and_stack_index(ident) {
            Some((value, _)) => Some(value),
            None => None,
        }
    }
    pub fn var_mut(&mut self, ident: &String) -> Option<&mut T>
    { 
        match self.var_mut_and_stack_index(ident) {
            Some((value, _)) => Some(value),
            None => None,
        }
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
                    Some(value) => {
                        match self.saved_var_stack.last_mut() {
                            Some(saved_vars) => {
                                if !saved_vars.contains_key(&(ident.clone(), self.stack.len() - 1)) {
                                    saved_vars.insert((ident.clone(), self.stack.len() - 1), value);
                                }
                            },
                            None => (),
                        }
                        true
                    },
                    None => false,
                }
            },
            None => false,
        }
    }
}
