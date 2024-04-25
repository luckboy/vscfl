//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::collections::BTreeSet;

pub fn dfs_with_result<T: Clone + Ord, U, E, F, G>(from: &T, visiteds: &mut BTreeSet<T>, data: &mut U, mut f: F, mut g: G) -> Result<(), E>
    where F: FnMut(&T, &BTreeSet<T>, &mut U) -> Result<Vec<T>, E>,
        G: FnMut(&T, &mut U) -> Result<(), E>
{
    let mut stack: Vec<(T, Vec<T>)> = Vec::new();
    let mut processeds: BTreeSet<T> = BTreeSet::new();
    if visiteds.contains(from) {
        return Ok(());
    }
    processeds.insert(from.clone());
    let mut tmp_neighbors = f(from, &processeds, data)?;
    tmp_neighbors.reverse();
    stack.push((from.clone(), tmp_neighbors));
    visiteds.insert(from.clone());
    loop {
        match stack.pop() {
            Some((u, mut neighbors)) => {
                processeds.remove(&u);
                let v = loop {
                    match neighbors.pop() {
                        Some(w) if visiteds.contains(&w) => (),
                        Some(w) => break Some(w),
                        None => break None,
                    }
                };
                match v {
                    Some(v) => {
                        processeds.insert(u.clone());
                        stack.push((u, neighbors));
                        processeds.insert(v.clone());
                        let mut tmp_neighbors = f(&v, &processeds, data)?;
                        tmp_neighbors.reverse();
                        stack.push((v.clone(), tmp_neighbors));
                        visiteds.insert(v.clone());
                    },
                    None => g(&u, data)?,
                }
            },
            None => break,
        }
    }
    Ok(())
}

pub fn dfs<T: Clone + Ord, U, F, G>(from: &T, visiteds: &mut BTreeSet<T>, data: &mut U, mut f: F, mut g: G)
    where F: FnMut(&T, &BTreeSet<T>, &mut U) -> Vec<T>,
        G: FnMut(&T, &mut U)
{ let _res: Result<(), ()> = dfs_with_result(from, visiteds, data, |u, processeds, data| Ok(f(u, processeds, data)), |u, data| Ok(g(u, data))); }
