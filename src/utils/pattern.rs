//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::collections::BTreeMap;
use std::error;
use std::fmt;
use std::rc::*;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum PatternKind
{
    Left,
    Right,
    Both,
    New,
}

#[derive(Clone, Debug)]
pub enum PatternForests<T>
{
    Unfilled(Vec<PatternForest<T>>),
    Filled(PatternForest<T>, usize),
}

impl<T> PatternForests<T>
{
    pub fn len(&self) -> usize
    {
        match self {
            PatternForests::Unfilled(forests) => forests.len(),
            PatternForests::Filled(_, len) => *len,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PatternNode<T>
{
    id: T,
    forests: PatternForests<T>,
    is_normalized: bool,
}

impl<T: Clone + Eq + Ord> PatternNode<T>
{
    pub fn new(id: T, forests: PatternForests<T>) -> Self
    { PatternNode { id, forests, is_normalized: false, } }
    
    pub fn id(&self) -> &T
    { &self.id }

    pub fn set_id(&mut self, id: T)
    { self.id = id; }
    
    pub fn forests(&self) -> &PatternForests<T>
    { &self.forests }

    pub fn forests_mut(&mut self) -> &mut PatternForests<T>
    { &mut self.forests }
    
    pub fn is_normalized(&self) -> bool
    { self.is_normalized }
    
    pub fn normalize(&mut self) -> Result<(), PatternError>
    {
        if !self.is_normalized {
            match &mut self.forests {
                PatternForests::Unfilled(forests) => {
                    for forest in forests {
                        forest.normalize()?;
                    }
                },
                PatternForests::Filled(forest, _) => forest.normalize()?,
            }
            self.is_normalized = true;
        }
        Ok(())
    }
    
    pub fn is_empty(&self) -> bool
    {
        match &self.forests {
            PatternForests::Unfilled(forests) => forests.iter().any(|f| f.is_empty()),
            PatternForests::Filled(forest, _) => forest.is_empty(),
        }
    }
}

fn union_pattern_nodes_without_normalization<T: Clone + Eq + Ord>(node1: &Rc<RefCell<PatternNode<T>>>, node2: &Rc<RefCell<PatternNode<T>>>) -> Result<Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)>, PatternError>
{
    {
        let node1_r = node1.borrow();
        let node2_r = node2.borrow();
        let id = node1_r.id.clone();
        match (&node1_r.forests, &node2_r.forests) {
            (PatternForests::Unfilled(forests1), PatternForests::Unfilled(forests2)) => {
                let mut new_forest_pairs: Vec<(PatternKind, PatternForest<T>)> = Vec::new();
                let mut left_count = 0usize;
                let mut right_count = 0usize;
                let mut new_count = 0usize;
                for (forest1, forest2) in forests1.iter().zip(forests2.iter()) {
                    let (kind, new_forest) = forest1.union_without_normalization(forest2)?;
                    match kind {
                        PatternKind::Left => left_count += 1,
                        PatternKind::Right => right_count += 1,
                        PatternKind::Both => (),
                        PatternKind::New => new_count += 1,
                    }
                    new_forest_pairs.push((kind, new_forest));
                }
                if left_count > 0 && right_count == 0 && new_count == 0 {
                    return Ok(vec![(PatternKind::Left, node1.clone())]);
                } else if left_count == 0 && right_count > 0 && new_count == 0 {
                    return Ok(vec![(PatternKind::Right, node2.clone())]);
                } else if left_count == 0 && right_count == 0 && new_count == 0 {
                    return Ok(vec![(PatternKind::Both, node1.clone())]);
                } else {
                    let mut new_pairs = vec![(PatternKind::Left, node1.clone()), (PatternKind::Right, node2.clone())];
                    if new_count > 0 {
                        for (i, (kind, new_forest)) in new_forest_pairs.iter().enumerate() {
                            if *kind == PatternKind::New {
                                let mut new_forests: Vec<PatternForest<T>> = Vec::new();
                                let mut is_empty = false;
                                for (forest1, forest2) in (&forests1[0..i]).iter().zip((&forests2[0..i]).iter()) {
                                    let mut new_forest2 = forest1.intersection_without_normalization(forest2)?;
                                    new_forest2.normalize()?;
                                    if new_forest2.is_empty() {
                                        is_empty = true;
                                    }
                                    new_forests.push(new_forest2);
                                }
                                new_forests.push(new_forest.clone());
                                for (forest1, forest2) in (&forests1[(i + 1)..(forests1.len())]).iter().zip((&forests2[(i + 1)..(forests2.len())]).iter()) {
                                    let mut new_forest2 = forest1.intersection_without_normalization(forest2)?;
                                    new_forest2.normalize()?;
                                    if new_forest2.is_empty() {
                                        is_empty = true;
                                    }
                                    new_forests.push(new_forest2);
                                }
                                if is_empty {
                                    continue;
                                }
                                let mut new_node = PatternNode::new(id.clone(), PatternForests::Unfilled(new_forests));
                                new_node.is_normalized = true;
                                new_pairs.push((PatternKind::New, Rc::new(RefCell::new(new_node))));
                            }
                        }
                    }
                    return Ok(new_pairs);
                }
            },
            (PatternForests::Unfilled(forests1), PatternForests::Filled(forest2, _)) => {
                let mut left_or_both_count = 0usize;
                for forest1 in forests1 {
                    match forest1.union_without_normalization(forest2)?.0 {
                        PatternKind::Left | PatternKind::Both => left_or_both_count += 1,
                        _ => (),
                    }
                }
                if left_or_both_count == forests1.len() {
                    return Ok(vec![(PatternKind::Left, node1.clone())]);
                } else {
                    return Ok(vec![(PatternKind::Left, node1.clone()), (PatternKind::Right, node2.clone())]);
                }
            },
            (PatternForests::Filled(_, _), PatternForests::Unfilled(_)) => (),
            (PatternForests::Filled(forest1, len1), PatternForests::Filled(forest2, _)) => {
                let len = *len1;
                let (kind, new_forest) = forest1.union_without_normalization(forest2)?;
                match kind {
                    PatternKind::Left => return Ok(vec![(PatternKind::Left, node1.clone())]),
                    PatternKind::Right => return Ok(vec![(PatternKind::Right, node2.clone())]),
                    PatternKind::Both => return Ok(vec![(PatternKind::Both, node1.clone())]),
                    PatternKind::New => {
                        let mut new_node = PatternNode::new(id, PatternForests::Filled(new_forest, len));
                        new_node.is_normalized = true;
                        return Ok(vec![(PatternKind::New, Rc::new(RefCell::new(new_node)))]);
                    },
                }
            },
        }
    }
    let res_pairs = union_pattern_nodes_without_normalization(node2, node1)?;
    Ok(res_pairs.iter().map(|p| {
            match p.0 {
                PatternKind::Left => (PatternKind::Right, p.1.clone()),
                PatternKind::Right => (PatternKind::Left, p.1.clone()),
                kind => (kind, p.1.clone()),
            }
    }).collect())
}

pub fn union_pattern_nodes<T: Clone + Eq + Ord>(node1: &Rc<RefCell<PatternNode<T>>>, node2: &Rc<RefCell<PatternNode<T>>>) -> Result<Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)>, PatternError>
{
    {
        if !Rc::ptr_eq(node1, node2) {
            let mut node1_r = node1.borrow_mut();
            let mut node2_r = node2.borrow_mut();
            node1_r.normalize()?;
            node2_r.normalize()?;
            if node1_r.id != node2_r.id {
                return Ok(vec![(PatternKind::Left, node1.clone()), (PatternKind::Right, node2.clone())]);
            }
            if node1_r.forests.len() != node2_r.forests.len() {
                return Err(PatternError::Count);
            }
        } else {
            let mut node1_r = node1.borrow_mut();
            node1_r.normalize()?;
            return Ok(vec![(PatternKind::Both, node1.clone())]);
        }
    }
    union_pattern_nodes_without_normalization(node1, node2)
}

fn intersection_pattern_nodes_without_normalization<T: Clone + Eq + Ord>(node1: &Rc<RefCell<PatternNode<T>>>, node2: &Rc<RefCell<PatternNode<T>>>) -> Result<Option<Rc<RefCell<PatternNode<T>>>>, PatternError>
{
    {
        let node1_r = node1.borrow();
        let node2_r = node2.borrow();
        let id = node1_r.id.clone();
        match (&node1_r.forests, &node2_r.forests) {
            (PatternForests::Unfilled(forests1), PatternForests::Unfilled(forests2)) => {
                let mut new_forests: Vec<PatternForest<T>> = Vec::new();
                let mut is_empty = false;
                for (forest1, forest2) in forests1.iter().zip(forests2.iter()) {
                    let new_forest = forest1.intersection(forest2)?;
                    if new_forest.is_empty() {
                        is_empty = true;
                        break;
                    }
                    new_forests.push(new_forest);
                }
                if is_empty {
                    return Ok(None);
                }
                let mut new_node = PatternNode::new(id, PatternForests::Unfilled(new_forests));
                new_node.is_normalized = true;
                return Ok(Some(Rc::new(RefCell::new(new_node))));
            },
            (PatternForests::Unfilled(forests1), PatternForests::Filled(forest2, len2)) => {
                let len = *len2;
                let mut new_forest = forest2.clone();
                let mut is_empty = false;
                for forest1 in forests1 {
                    new_forest = forest1.intersection(&new_forest)?;
                    if new_forest.is_empty() {
                        is_empty = true;
                        break;
                    }
                }
                if is_empty {
                    return Ok(None);
                }
                let mut new_node = PatternNode::new(id, PatternForests::Filled(new_forest, len));
                new_node.is_normalized = true;
                return Ok(Some(Rc::new(RefCell::new(new_node))));
            },
            (PatternForests::Filled(_, _), PatternForests::Unfilled(_)) => (),
            (PatternForests::Filled(forest1, len1), PatternForests::Filled(forest2, _)) => {
                let len = *len1;
                let new_forest = forest1.intersection_without_normalization(forest2)?;
                let mut new_node = PatternNode::new(id, PatternForests::Filled(new_forest, len));
                new_node.is_normalized = true;
                return Ok(Some(Rc::new(RefCell::new(new_node))));
            },
        }
    }
    intersection_pattern_nodes_without_normalization(node2, node1)
}

pub fn intersection_pattern_nodes<T: Clone + Eq + Ord>(node1: &Rc<RefCell<PatternNode<T>>>, node2: &Rc<RefCell<PatternNode<T>>>) -> Result<Option<Rc<RefCell<PatternNode<T>>>>, PatternError>
{
    {
        if !Rc::ptr_eq(node1, node2) {
            let mut node1_r = node1.borrow_mut();
            let mut node2_r = node2.borrow_mut();
            node1_r.normalize()?;
            node2_r.normalize()?;
            if node1_r.id != node2_r.id {
                return Ok(None);
            }
            if node1_r.forests.len() != node2_r.forests.len() {
                return Err(PatternError::Count);
            }
        } else {
            let mut node1_r = node1.borrow_mut();
            node1_r.normalize()?;
            return Ok(Some(node1.clone()));
        }
    }
    intersection_pattern_nodes_without_normalization(node1, node2)
}

#[derive(Clone, Debug)]
pub enum PatternForest<T>
{
    Alt(Vec<Rc<RefCell<PatternNode<T>>>>, Option<usize>),
    All,
}

impl<T: Clone + Eq + Ord> PatternForest<T>
{
    pub fn add_node(&mut self, node: PatternNode<T>) -> bool
    {
        match self {
            PatternForest::Alt(nodes, _) => {
                nodes.push(Rc::new(RefCell::new(node)));
                true
            },
            PatternForest::All => false,
        }
    }
    
    pub fn set_all(&mut self)
    { *self = PatternForest::All; }
    
    pub fn set_max(&mut self, max: Option<usize>) -> bool
    {
        match self {
            PatternForest::Alt(_, max2) => {
                *max2 = max;
                true
            },
            PatternForest::All => false,
        }
    }
    
    fn union_without_normalization(&self, forest: &PatternForest<T>) -> Result<(PatternKind, PatternForest<T>), PatternError>
    {
        match (self, forest) {
            (PatternForest::Alt(nodes1, max1), PatternForest::Alt(nodes2, max2)) => {
                if max1 != max2 {
                    return Err(PatternError::Max);
                }
                let max = max1;
                let mut pair_vec_map1: BTreeMap<T, Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)>> = BTreeMap::new();
                for node1 in nodes1 {
                    let node1_r = node1.borrow();
                    match pair_vec_map1.get_mut(&node1_r.id) {
                        Some(pairs1) => pairs1.push((PatternKind::Left, node1.clone())),
                        None => {
                            pair_vec_map1.insert(node1_r.id.clone(), vec![(PatternKind::Left, node1.clone())]);
                        },
                    }
                }
                let mut pairs2: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = nodes2.iter().map(|n| (PatternKind::Right, n.clone())).collect();
                pairs2.reverse();
                loop {
                    match pairs2.pop() {
                        Some((kind2, node2)) => {
                            let id = {
                                let node2_r = node2.borrow();
                                node2_r.id.clone()
                            };
                            match pair_vec_map1.get_mut(&id) {
                                Some(pairs1) => {
                                    let mut new_pairs1: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = Vec::new();
                                    let mut new_pairs3: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = Vec::new();
                                    let mut is_last_node2 = true;
                                    for (kind1, node1) in &*pairs1 {
                                        let mut is_node2 = false;
                                        let res_pairs = union_pattern_nodes(node1, &node2)?;
                                        for res_pair in res_pairs {
                                            match (*kind1, kind2, res_pair) {
                                                (tmp_kind1, _, (PatternKind::Left, new_node)) => new_pairs1.push((tmp_kind1, new_node)),
                                                (_, _, (PatternKind::Right, _)) => is_node2 = true,
                                                (tmp_kind1, PatternKind::New, (PatternKind::Both, new_node)) => new_pairs1.push((tmp_kind1, new_node)),
                                                (PatternKind::New, _, (PatternKind::Both, _)) => is_node2 = true,
                                                (_, _, (PatternKind::Both, new_node)) => new_pairs1.push((PatternKind::Both, new_node)),
                                                (_, _, (PatternKind::New, new_node)) => new_pairs3.push((PatternKind::New, new_node)),
                                            }
                                        }
                                        if !is_node2 {
                                            is_last_node2 = false;
                                        }
                                    }
                                    if is_last_node2 {
                                        new_pairs1.push((kind2, node2.clone()));
                                    }
                                    *pairs1 = new_pairs1;
                                    new_pairs3.reverse();
                                    pairs2.append(&mut new_pairs3);
                                },
                                None => {
                                    let mut node2_r = node2.borrow_mut();
                                    node2_r.normalize()?;
                                    pair_vec_map1.insert(id, vec![(kind2, node2.clone())]);
                                },
                            }
                        },
                        None => break,
                    }
                }
                let mut left_count = 0usize;
                let mut right_count = 0usize;
                let mut new_count = 0usize;
                for pairs1 in pair_vec_map1.values() {
                    for (kind1, _) in pairs1 {
                        match kind1 {
                            PatternKind::Left => left_count += 1,
                            PatternKind::Right => right_count += 1,
                            PatternKind::Both => (),
                            PatternKind::New => new_count += 1,
                        }
                    }
                }
                let new_kind = if left_count > 0 && right_count == 0 && new_count == 0 {
                    PatternKind::Left
                } else if left_count == 0 && right_count > 0 && new_count == 0 {
                    PatternKind::Right
                } else if left_count == 0 && right_count == 0 && new_count == 0 {
                    PatternKind::Both
                } else {
                    PatternKind::New
                };
                let are_all = max.map(|m| m == pair_vec_map1.len()).unwrap_or(false) && pair_vec_map1.values().all(|ps| {
                        if ps.len() == 1 {
                            let r = ps[0].1.borrow();
                            match &r.forests {
                                PatternForests::Unfilled(forests) => {
                                    forests.iter().all(|f| {
                                            match f {
                                                PatternForest::All => true,
                                                _ => false,
                                            }
                                    })
                                },
                                PatternForests::Filled(_, _) => false,
                            }
                        } else {
                            false
                        }
                });
                if are_all {
                    Ok((new_kind, PatternForest::All))
                } else {
                    Ok((new_kind, PatternForest::Alt(pair_vec_map1.values().flatten().map(|p| p.1.clone()).collect(), *max)))
                }
            },
            (PatternForest::Alt(_, _), PatternForest::All) => Ok((PatternKind::Right, PatternForest::All)),
            (PatternForest::All, PatternForest::Alt(_, _)) => Ok((PatternKind::Left, PatternForest::All)),
            (PatternForest::All, PatternForest::All) => Ok((PatternKind::Both, PatternForest::All)),
        }
    }
    
    pub fn union(&self, forest: &PatternForest<T>) -> Result<(PatternKind, PatternForest<T>), PatternError>
    {
        let mut forest1 = self.clone();
        let mut forest2 = forest.clone();
        forest1.normalize()?;
        forest2.normalize()?;
        forest1.union_without_normalization(&forest2)
    }

    fn intersection_without_normalization(&self, forest: &PatternForest<T>) -> Result<PatternForest<T>, PatternError>
    {
        match (self, forest) {
            (PatternForest::Alt(nodes1, max1), PatternForest::Alt(nodes2, max2)) => {
                if max1 != max2 {
                    return Err(PatternError::Max);
                }
                let max = max1;
                let mut node_vec_map1: BTreeMap<T, Vec<Rc<RefCell<PatternNode<T>>>>> = BTreeMap::new();
                for node1 in nodes1 {
                    let node1_r = node1.borrow();
                    match node_vec_map1.get_mut(&node1_r.id) {
                        Some(tmp_nodes1) => tmp_nodes1.push(node1.clone()),
                        None => {
                            node_vec_map1.insert(node1_r.id.clone(), vec![node1.clone()]);
                        },
                    }
                }
                let mut new_nodes: Vec<Rc<RefCell<PatternNode<T>>>> = Vec::new();
                for node2 in nodes2 {
                    let id = {
                        let node2_r = node2.borrow();
                        node2_r.id.clone()
                    };
                    match node_vec_map1.get(&id) {
                        Some(tmp_nodes1) => {
                            for node1 in tmp_nodes1 {
                                match intersection_pattern_nodes(node1, node2)? {
                                    Some(new_node) => new_nodes.push(new_node),
                                    None => (),
                                }
                            }
                        },
                        None => (),
                    }
                }
                Ok(PatternForest::Alt(new_nodes, *max))
            },
            (forest1 @ PatternForest::Alt(_, _), PatternForest::All) => Ok(forest1.clone()),
            (PatternForest::All, forest2 @ PatternForest::Alt(_, _)) => Ok(forest2.clone()),
            (PatternForest::All, PatternForest::All) => Ok(PatternForest::All),
        }
    }

    pub fn intersection(&self, forest: &PatternForest<T>) -> Result<PatternForest<T>, PatternError>
    {
        let mut forest1 = self.clone();
        let mut forest2 = forest.clone();
        forest1.normalize()?;
        forest2.normalize()?;
        let mut new_forest = forest1.intersection_without_normalization(&forest2)?;
        new_forest.normalize()?;
        Ok(new_forest)
    }
    
    pub fn normalize(&mut self) -> Result<(), PatternError>
    {
        match self {
            PatternForest::Alt(nodes, max) => {
                let new_nodes: Vec<Rc<RefCell<PatternNode<T>>>> = nodes.iter().filter(|n| {
                        let r = n.borrow();
                        !r.is_empty()
                }).map(|n| n.clone()).collect();
                *self = PatternForest::Alt(Vec::new(), *max).union_without_normalization(&PatternForest::Alt(new_nodes, *max))?.1
            },
            PatternForest::All => (),
        }
        Ok(())
    }
    
    pub fn is_empty(&self) -> bool
    {
        match self {
            PatternForest::Alt(nodes, _) => {
                nodes.is_empty() || nodes.iter().all(|n| {
                        let r = n.borrow();
                        r.is_empty()
                })
            },
            PatternForest::All => false,
        }
    }
}

#[derive(Debug)]
pub enum PatternError
{
    Max,
    Count,
}

impl error::Error for PatternError
{}

impl fmt::Display for PatternError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            PatternError::Max => write!(f, "maximal numbers of nodes aren't equal"),
            PatternError::Count => write!(f, "numbers of forests aren't equal"),
        }
    }
}

#[cfg(test)]
mod tests;
