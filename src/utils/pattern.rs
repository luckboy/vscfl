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

    pub fn forests(&self) -> &PatternForests<T>
    { &self.forests }
    
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
                PatternForests::Filled(forest, len) => {
                    forest.normalize()?;
                    if forest.has_one() {
                        self.forests = PatternForests::Unfilled((0..*len).map(|_| forest.clone()).collect());
                    }
                },
            }
            self.is_normalized = true;
        }
        Ok(())
    }
    
    pub fn is_one(&self) -> bool
    {
        match &self.forests {
            PatternForests::Unfilled(forests) => forests.iter().all(|f| f.has_one()),
            PatternForests::Filled(forest, _) => forest.has_one(),
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
                let mut new_forests: Vec<PatternForest<T>> = Vec::new();
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
                    new_forests.push(new_forest);
                }
                if left_count > 0 && right_count == 0 && new_count == 0 {
                    return Ok(vec![(PatternKind::Left, node1.clone())]);
                } else if left_count == 0 && right_count > 0 && new_count == 0 {
                    return Ok(vec![(PatternKind::Right, node2.clone())]);
                } else if left_count == 0 && right_count == 0 && new_count == 0 {
                    return Ok(vec![(PatternKind::Both, node1.clone())]);
                } else if left_count == 0 && right_count == 0 && new_count == 1 {
                    let mut new_node = PatternNode::new(id, PatternForests::Unfilled(new_forests));
                    new_node.is_normalized = true;
                    return Ok(vec![(PatternKind::Right, Rc::new(RefCell::new(new_node)))]);
                } else {
                    return Ok(vec![(PatternKind::Left, node1.clone()), (PatternKind::Right, node2.clone())]);
                }
            },
            (PatternForests::Unfilled(forests1), PatternForests::Filled(forest2, _)) => {
                if !forests1.is_empty() {
                    if forests1.len() == 1 {
                        let (kind, new_forest) = forests1[0].union_without_normalization(forest2)?;
                        match kind {
                            PatternKind::Left => return Ok(vec![(PatternKind::Left, node1.clone())]),
                            PatternKind::Right => {
                                let mut new_node = PatternNode::new(id, PatternForests::Unfilled(vec![forest2.clone()]));
                                new_node.is_normalized = true;
                                return Ok(vec![(PatternKind::New, Rc::new(RefCell::new(new_node)))]);
                            },
                            PatternKind::Both => return Ok(vec![(PatternKind::Left, node1.clone())]),
                            PatternKind::New => {
                                let mut new_node = PatternNode::new(id, PatternForests::Unfilled(vec![new_forest]));
                                new_node.is_normalized = true;
                                return Ok(vec![(PatternKind::New, Rc::new(RefCell::new(new_node)))]);
                            },
                        }
                    } else {
                        let one_count = forests1.iter().fold(0usize, |n, f| if f.has_one() { n + 1 } else { n });
                        if one_count >= forests1.len() - 1 {
                            let mut new_pairs: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = Vec::new();
                            let mut is_new_node = false;
                            let mut is_left = true;
                            if forests1[0].has_one() {
                                match forests1[0].union_without_normalization(forest2)?.0 {
                                    PatternKind::Right | PatternKind::Both => {
                                        let mut both_count_for_first = 0usize;
                                        let mut new_count_for_first = 0usize;
                                        let mut new_forests: Vec<PatternForest<T>> = Vec::new();
                                        for forest1 in forests1 {
                                            let (kind, new_forest) = forest1.union_without_normalization(&forests1[0])?;
                                            match kind {
                                                PatternKind::Both => both_count_for_first += 1,
                                                PatternKind::New => new_count_for_first += 1,
                                               _ => (),
                                            }
                                            new_forests.push(new_forest);
                                        }
                                        if both_count_for_first == forests1.len() - 1 && new_count_for_first == 1 {
                                            let mut new_node = PatternNode::new(id.clone(), PatternForests::Unfilled(new_forests));
                                            new_node.is_normalized = true;
                                            new_pairs.push((PatternKind::New, Rc::new(RefCell::new(new_node))));
                                            is_new_node = true;
                                        } else if both_count_for_first == forests1.len() {
                                            is_left = false;
                                        }
                                    },
                                    _ => (),
                                }
                            }
                            let is_second = match forests1[0].union_without_normalization(&forests1[1])?.0 {
                                PatternKind::Both => false,
                                _ => true,
                            };
                            if is_second && forests1[1].has_one() {
                                match forests1[1].union_without_normalization(forest2)?.0 {
                                    PatternKind::Right | PatternKind::Both => {
                                        let mut both_count_for_second = 0usize;
                                        let mut new_count_for_first = 0usize;
                                        let mut new_forests: Vec<PatternForest<T>> = Vec::new();
                                        for forest1 in forests1 {
                                            let (kind, new_forest) = forest1.union_without_normalization(&forests1[1])?;
                                            match kind {
                                                PatternKind::Both => both_count_for_second += 1,
                                                PatternKind::New => new_count_for_first += 1,
                                                _ => (),
                                            }
                                            new_forests.push(new_forest);
                                        }
                                        if both_count_for_second == forests1.len() - 1 && new_count_for_first == 1 {
                                            let mut new_node = PatternNode::new(id.clone(), PatternForests::Unfilled(new_forests));
                                            new_node.is_normalized = true;
                                            new_pairs.push((PatternKind::New, Rc::new(RefCell::new(new_node))));
                                            is_new_node = true;
                                        } else if both_count_for_second == forests1.len() {
                                            is_left = false;
                                        }
                                    },
                                    _ => (),
                                }
                            }
                            if !is_new_node && is_left {
                                new_pairs.push((PatternKind::Left, node1.clone()));
                            }
                            new_pairs.push((PatternKind::Right, node2.clone()));
                            return Ok(new_pairs);
                        } else {
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
                        }
                    }
                } else {
                    return Ok(vec![(PatternKind::Left, node1.clone())]);
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
                        return Ok(vec![(PatternKind::Right, Rc::new(RefCell::new(new_node)))]);
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
        let mut node1_r = node1.borrow_mut();
        let mut node2_r = node2.borrow_mut();
        if node1_r.id != node2_r.id {
            return Ok(vec![(PatternKind::Left, node1.clone()), (PatternKind::Right, node2.clone())]);
        }
        if node1_r.forests.len() != node2_r.forests.len() {
            return Err(PatternError::Count);
        }
        node1_r.normalize()?;
        node2_r.normalize()?;
    }
    union_pattern_nodes_without_normalization(node1, node2)
}

#[derive(Clone, Debug)]
pub enum PatternForest<T>
{
    Alt(Vec<Rc<RefCell<PatternNode<T>>>>, Option<usize>),
    All(bool),
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
            PatternForest::All(_) => false,
        }
    }
    
    pub fn set_all(&mut self, is_one: bool)
    { *self = PatternForest::All(is_one); }
    
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
                            let node2_r = node2.borrow();
                            match pair_vec_map1.get_mut(&node2_r.id) {
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
                                                (PatternKind::New, PatternKind::New, (PatternKind::Both, new_node)) => new_pairs3.push((PatternKind::New, new_node)),
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
                                    pair_vec_map1.insert(node2_r.id.clone(), vec![(PatternKind::Left, node2.clone())]);
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
                                                PatternForest::All(_) => true,
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
                    let is_one = pair_vec_map1.len() == 1 && pair_vec_map1.values().next().map(|ps| {
                            if ps.len() == 1 {
                                let r = ps[0].1.borrow();
                                r.is_one()
                            } else {
                                false
                            }
                    }).unwrap_or(false);
                    Ok((new_kind, PatternForest::All(is_one)))
                } else {
                    Ok((new_kind, PatternForest::Alt(pair_vec_map1.values().flatten().map(|p| p.1.clone()).collect(), *max)))
                }
            },
            (PatternForest::Alt(_, _), PatternForest::All(is_one1)) => Ok((PatternKind::Right, PatternForest::All(*is_one1))),
            (PatternForest::All(is_one1), PatternForest::Alt(_, _)) => Ok((PatternKind::Left, PatternForest::All(*is_one1))),
            (PatternForest::All(is_one1), PatternForest::All(is_one2)) => {
                if is_one1 != is_one2 {
                    return Err(PatternError::Flag);
                } 
                let is_one = *is_one1;
                Ok((PatternKind::Both, PatternForest::All(is_one)))
            },
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

    pub fn normalize(&mut self) -> Result<(), PatternError>
    {
        match self {
            PatternForest::Alt(nodes, max) => *self = PatternForest::Alt(Vec::new(), *max).union_without_normalization(&PatternForest::Alt(nodes.clone(), *max))?.1,
            PatternForest::All(_) => (),
        }
        Ok(())
    }
    
    pub fn has_one(&self) -> bool
    {
        match self {
            PatternForest::Alt(nodes, _) => {
                if nodes.len() == 1 {
                    let node_r = nodes[0].borrow();
                    node_r.is_one()
                } else {
                    false
                }
            },
            PatternForest::All(is_one) => *is_one,
        }
    }
}

#[derive(Debug)]
pub enum PatternError
{
    Max,
    Count,
    Flag,
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
            PatternError::Flag => write!(f, "one flags aren't equal"),
        }
    }
}
