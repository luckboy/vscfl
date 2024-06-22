//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::error;
use std::fmt;
use std::rc::*;

#[derive(Copy, Clone, Debug)]
pub enum PatternKind
{
    Left,
    Right,
    Both,
    New,
}

#[derive(Clone, Debug)]
pub struct PatternNode<T>
{
    id: T,
    forests: Vec<PatternForest<T>>,
    is_normalized: bool,
}

impl<T: Clone + Eq> PatternNode<T>
{
    pub fn new(id: T, forests: Vec<PatternForest<T>>) -> Self
    { PatternNode { id, forests, is_normalized: false, } }
    
    pub fn id(&self) -> &T
    { &self.id }

    pub fn forests(&self) -> &Vec<PatternForest<T>>
    { &self.forests }
    
    pub fn is_normalized(&self) -> bool
    { self.is_normalized }
    
    pub fn normalize(&mut self) -> Result<(), PatternError>
    {
        if !self.is_normalized {
            for forest in &mut self.forests {
                forest.normalize()?;
            }
            self.is_normalized = true;
        }
        Ok(())
    }
}

pub fn union_pattern_nodes<T: Clone + Eq>(node1: &Rc<RefCell<PatternNode<T>>>, node2: &Rc<RefCell<PatternNode<T>>>) -> Result<Option<(PatternKind, Rc<RefCell<PatternNode<T>>>)>, PatternError>
{
    let mut node1_r = node1.borrow_mut();
    let mut node2_r = node2.borrow_mut();
    if node1_r.id != node2_r.id {
        return Ok(None)
    }
    let id = node1_r.id.clone();
    if node1_r.forests.len() != node2_r.forests.len() {
        return Err(PatternError::Max);
    }
    node1_r.normalize()?;
    node2_r.normalize()?;
    let mut new_forests: Vec<PatternForest<T>> = Vec::new();
    let mut left_count = 0usize;
    let mut right_count = 0usize;
    let mut new_count = 0usize;
    for (forest1, forest2) in node1_r.forests.iter().zip(node2_r.forests.iter()) {
        match forest1.union_without_normalization(forest2)? {
            (kind, new_forest) => {
                match kind {
                    PatternKind::Left => left_count += 1,
                    PatternKind::Right => right_count += 1,
                    PatternKind::Both => (),
                    PatternKind::New => new_count += 1,
                }
                new_forests.push(new_forest);
            },
        }
    }
    if left_count > 0 && right_count == 0 && new_count == 0 {
        Ok(Some((PatternKind::Left, node1.clone())))
    } else if left_count == 0 && right_count > 0 && new_count == 0 {
        Ok(Some((PatternKind::Right, node2.clone())))
    } else if left_count == 0 && right_count == 0 && new_count == 0 {
        Ok(Some((PatternKind::Both, node1.clone())))
    } else if left_count == 0 && right_count == 0 && new_count == 1 {
        let mut new_node = PatternNode::new(id, new_forests);
        new_node.is_normalized = true;
        Ok(Some((PatternKind::Right, Rc::new(RefCell::new(new_node)))))
    } else {
        Ok(None)
    }
}

#[derive(Clone, Debug)]
pub enum PatternForest<T>
{
    Alt(Vec<Rc<RefCell<PatternNode<T>>>>, Option<usize>),
    All,
}

impl<T: Clone + Eq> PatternForest<T>
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
    
    fn union_without_normalization(&self, forest: &PatternForest<T>) -> Result<(PatternKind, PatternForest<T>), PatternError>
    {
        match (self, forest) {
            (PatternForest::Alt(nodes1, max1), PatternForest::Alt(nodes2, max2)) => {
                if max1 != max2 {
                    return Err(PatternError::Max);
                }
                let max = max1;
                let mut pairs1: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = nodes1.iter().map(|n| (PatternKind::Left, n.clone())).collect();
                let mut pairs2: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = nodes2.iter().map(|n| (PatternKind::Right, n.clone())).collect();
                pairs2.reverse();
                loop {
                    match pairs2.pop() {
                        Some((kind2, node2)) => {
                            let mut new_pairs1: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = Vec::new();
                            let mut new_pairs3: Vec<(PatternKind, Rc<RefCell<PatternNode<T>>>)> = Vec::new();
                            let mut is_last_node2 = true;
                            for (kind1, node1) in &pairs1 {
                                let mut is_node2 = false;
                                match (*kind1, kind2, union_pattern_nodes(node1, &node2)?) {
                                    (tmp_kind1, _, Some((PatternKind::Left, new_node))) => new_pairs1.push((tmp_kind1, new_node)),
                                    (_, _, Some((PatternKind::Right, _))) => is_node2 = true,
                                    (PatternKind::New, PatternKind::New, Some((PatternKind::Both, new_node))) => new_pairs3.push((PatternKind::New, new_node)),
                                    (tmp_kind1, PatternKind::New, Some((PatternKind::Both, new_node))) => new_pairs1.push((tmp_kind1, new_node)),
                                    (PatternKind::New, _, Some((PatternKind::Both, _))) => is_node2 = false,
                                    (_, _, Some((PatternKind::Both, new_node))) => new_pairs1.push((PatternKind::Both, new_node)),
                                    (_, _, Some((PatternKind::New, new_node))) => new_pairs3.push((PatternKind::New, new_node)),
                                    (_, _, None) => {
                                        new_pairs1.push((*kind1, node1.clone()));
                                        is_node2 = true;
                                    },
                                }
                                if !is_node2 {
                                    is_last_node2 = false;
                                }
                            }
                            if is_last_node2 {
                                new_pairs1.push((kind2, node2.clone()));
                            }
                            pairs1 = new_pairs1;
                            new_pairs3.reverse();
                            pairs2.append(&mut new_pairs3);
                        },
                        None => break,
                    }
                }
                let mut left_count = 0usize;
                let mut right_count = 0usize;
                let mut new_count = 0usize;
                for (kind1, _) in &pairs1 {
                    match kind1 {
                        PatternKind::Left => left_count += 1,
                        PatternKind::Right => right_count += 1,
                        PatternKind::Both => (),
                        PatternKind::New => new_count += 1,
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
                let are_all = max.map(|m| m == pairs1.len()).unwrap_or(false) && pairs1.iter().all(|p| {
                        let r = p.1.borrow();
                        r.forests.iter().all(|f| {
                                match f {
                                    PatternForest::All => true,
                                    _ => false,
                                }
                        })
                });
                if are_all {
                    Ok((new_kind, PatternForest::All))
                } else {
                    Ok((new_kind, PatternForest::Alt(pairs1.iter().map(|p| p.1.clone()).collect(), *max)))
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

    pub fn normalize(&mut self) -> Result<(), PatternError>
    {
        match self {
            PatternForest::Alt(nodes, max) => *self = PatternForest::Alt(Vec::new(), *max).union_without_normalization(&PatternForest::Alt(nodes.clone(), *max))?.1,
            PatternForest::All => (),
        }
        Ok(())
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
