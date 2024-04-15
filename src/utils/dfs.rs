//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeSet;

pub fn dfs_with_result<T: Clone + Ord, U, E, F, G>(from: &T, data: &mut U, mut f: F, mut g: G) -> Result<(), E>
    where F: FnMut(&T, &BTreeSet<T>, &mut U) -> Result<Vec<T>, E>,
        G: FnMut(&T, &mut U) -> Result<(), E>
{
    let mut stack: Vec<(T, Vec<T>)> = Vec::new();
    let mut visited: BTreeSet<T> = BTreeSet::new();
    let mut processed: BTreeSet<T> = BTreeSet::new();
    processed.insert(from.clone());
    let mut tmp_neighbors = f(from, &processed, data)?;
    tmp_neighbors.reverse();
    stack.push((from.clone(), tmp_neighbors));
    visited.insert(from.clone());
    loop {
        match stack.pop() {
            Some((u, mut neighbors)) => {
                processed.remove(&u);
                let v = loop {
                    match neighbors.pop() {
                        Some(w) if visited.contains(&w) => (),
                        Some(w) => break Some(w),
                        None => break None,
                    }
                };
                match v {
                    Some(v) => {
                        processed.insert(u.clone());
                        stack.push((u, neighbors));
                        processed.insert(v.clone());
                        let mut tmp_neighbors = f(&v, &processed, data)?;
                        tmp_neighbors.reverse();
                        stack.push((v.clone(), tmp_neighbors));
                        visited.insert(v.clone());
                    },
                    None => g(&u, data)?,
                }
            },
            None => break,
        }
    }
    Ok(())
}

pub fn dfs<T: Clone + Ord, U, F, G>(from: &T, data: &mut U, mut f: F, mut g: G)
    where F: FnMut(&T, &BTreeSet<T>, &mut U) -> Vec<T>,
        G: FnMut(&T, &mut U)
{ let _res: Result<(), ()> = dfs_with_result(from, data, |u, processed, data| Ok(f(u, processed, data)), |u, data| Ok(g(u, data))); }
