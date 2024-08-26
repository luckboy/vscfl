//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_pattern_forest_add_node_adds_pattern_nodes()
{
    let mut forest: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(4));
    assert_eq!(true, forest.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(4, PatternForests::Unfilled(Vec::new()))));
    match forest {
        PatternForest::Alt(nodes, Some(4)) => {
            assert_eq!(3, nodes.len());
            let node1_r = nodes[0].borrow();
            assert_eq!(1, *node1_r.id());
            let node2_r = nodes[1].borrow();
            assert_eq!(2, *node2_r.id());
            let node3_r = nodes[2].borrow();
            assert_eq!(4, *node3_r.id());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_pattern_forest_add_node_does_not_adds_pattern_nodes()
{
    let mut forest: PatternForest<i32> = PatternForest::All;
    assert_eq!(false, forest.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(false, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(false, forest.add_node(PatternNode::new(4, PatternForests::Unfilled(Vec::new()))));
    match forest {
        PatternForest::All => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_pattern_forest_normalize_normalizes_pattern_forest()
{
    let mut forest: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(4));
    assert_eq!(true, forest.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(4, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    match forest.normalize() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match forest {
        PatternForest::Alt(nodes, Some(4)) => {
            assert_eq!(3, nodes.len());
            let node1_r = nodes[0].borrow();
            assert_eq!(1, *node1_r.id());
            let node2_r = nodes[1].borrow();
            assert_eq!(2, *node2_r.id());
            let node3_r = nodes[2].borrow();
            assert_eq!(4, *node3_r.id());
        },
        _ => assert!(false),
    }
}

#[test]
fn test_pattern_forest_normalize_normalizes_pattern_forest_to_all()
{
    let mut forest: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(4));
    assert_eq!(true, forest.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(3, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(4, PatternForests::Unfilled(Vec::new()))));
    match forest.normalize() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match forest {
        PatternForest::All => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_pattern_forest_normalize_normalizes_pattern_forest_with_nested_pattern_nodes()
{
    let mut forest: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest3: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest3.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest1, forest2, forest3]))));
    let mut forest4: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest4.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest5: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest5.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest6: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest6.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest4, forest5, forest6]))));
    match forest.normalize() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match forest {
        PatternForest::Alt(nodes, Some(3)) => {
            assert_eq!(2, nodes.len());
            let node1_r = nodes[0].borrow();
            assert_eq!(1, *node1_r.id());
            let node2_r = nodes[1].borrow();
            assert_eq!(2, *node2_r.id());
            match node2_r.forests() {
                PatternForests::Unfilled(forests) => {
                    match &forests[0] {
                        PatternForest::Alt(nodes, Some(2)) => {
                            assert_eq!(1, nodes.len());
                            let node1_r = nodes[0].borrow();
                            assert_eq!(1, *node1_r.id());
                        },
                        _ => assert!(false),
                    }
                    match &forests[1] {
                        PatternForest::All => assert!(true),
                        _ => assert!(false),
                    }
                    match &forests[2] {
                        PatternForest::Alt(nodes, Some(2)) => {
                            assert_eq!(1, nodes.len());
                            let node1_r = nodes[0].borrow();
                            assert_eq!(1, *node1_r.id());
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_pattern_forest_normalize_normalizes_pattern_forest_with_nested_pattern_nodes_without_all()
{
    let mut forest: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest3: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest3.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest1, forest2, forest3]))));
    let mut forest4: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest4.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest5: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest5.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest6: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(2));
    assert_eq!(true, forest6.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest4, forest5, forest6]))));
    match forest.normalize() {
        Ok(()) => assert!(true),
        Err(_) => assert!(false),
    }
    match forest {
        PatternForest::Alt(nodes, Some(3)) => {
            assert_eq!(2, nodes.len());
            let node1_r = nodes[0].borrow();
            assert_eq!(1, *node1_r.id());
            let node2_r = nodes[1].borrow();
            assert_eq!(2, *node2_r.id());
            match node2_r.forests() {
                PatternForests::Unfilled(forests) => {
                    match &forests[0] {
                        PatternForest::Alt(nodes, Some(2)) => {
                            assert_eq!(1, nodes.len());
                            let node1_r = nodes[0].borrow();
                            assert_eq!(1, *node1_r.id());
                        },
                        _ => assert!(false),
                    }
                    match &forests[1] {
                        PatternForest::Alt(nodes, Some(3)) => {
                            assert_eq!(2, nodes.len());
                            let node1_r = nodes[0].borrow();
                            assert_eq!(1, *node1_r.id());
                            let node2_r = nodes[1].borrow();
                            assert_eq!(2, *node2_r.id());
                        },
                        _ => assert!(false),
                    }
                    match &forests[2] {
                        PatternForest::Alt(nodes, Some(2)) => {
                            assert_eq!(1, nodes.len());
                            let node1_r = nodes[0].borrow();
                            assert_eq!(1, *node1_r.id());
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        _ => assert!(false),
    }
}

#[test]
fn test_pattern_forest_union_return_new_forest_for_left_kind()
{
    // forest1
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest11: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest11.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest11.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest12: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest12.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest12.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest13: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest13.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest13.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest11, forest12, forest13]))));
    // forest2
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest21: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest21.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest22: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest22.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest22.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest23: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest23.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest21, forest22, forest23]))));
    match forest1.union(&forest2) {
        Ok((kind3, forest3)) => {
            assert_eq!(PatternKind::Left, kind3);
            match forest3 {
                PatternForest::Alt(nodes, Some(3)) => {
                    assert_eq!(2, nodes.len());
                    let node1_r = nodes[0].borrow();
                    assert_eq!(1, *node1_r.id());
                    let node2_r = nodes[1].borrow();
                    assert_eq!(2, *node2_r.id());
                    match node2_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pattern_forest_union_return_new_forest_for_right_kind()
{
    // forest1
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest11: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest11.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest12: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest12.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest12.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest13: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest13.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest11, forest12, forest13]))));
    // forest2
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest21: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest21.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest21.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest22: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest22.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest22.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest23: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest23.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest23.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest21, forest22, forest23]))));
    match forest1.union(&forest2) {
        Ok((kind3, forest3)) => {
            assert_eq!(PatternKind::Right, kind3);
            match forest3 {
                PatternForest::Alt(nodes, Some(3)) => {
                    assert_eq!(2, nodes.len());
                    let node1_r = nodes[0].borrow();
                    assert_eq!(1, *node1_r.id());
                    let node2_r = nodes[1].borrow();
                    assert_eq!(2, *node2_r.id());
                    match node2_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pattern_forest_union_return_new_forest_for_both_kind()
{
    // forest1
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest11: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest11.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest12: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest12.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest12.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest13: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest13.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest11, forest12, forest13]))));
    // forest2
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest21: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest21.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest22: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest22.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest22.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest23: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest23.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest21, forest22, forest23]))));
    match forest1.union(&forest2) {
        Ok((kind3, forest3)) => {
            assert_eq!(PatternKind::Both, kind3);
            match forest3 {
                PatternForest::Alt(nodes, Some(3)) => {
                    assert_eq!(2, nodes.len());
                    let node1_r = nodes[0].borrow();
                    assert_eq!(1, *node1_r.id());
                    let node2_r = nodes[1].borrow();
                    assert_eq!(2, *node2_r.id());
                    match node2_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(2, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pattern_forest_union_return_new_forest_for_new_kind()
{
    // forest1
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest11: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest11.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest12: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest12.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest13: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest13.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest11, forest12, forest13]))));
    // forest2
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest21: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest21.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest22: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest22.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest23: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest23.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest21, forest22, forest23]))));
    match forest1.union(&forest2) {
        Ok((kind3, forest3)) => {
            assert_eq!(PatternKind::New, kind3);
            match forest3 {
                PatternForest::Alt(nodes, Some(3)) => {
                    assert_eq!(2, nodes.len());
                    let node1_r = nodes[0].borrow();
                    assert_eq!(1, *node1_r.id());
                    let node2_r = nodes[1].borrow();
                    assert_eq!(2, *node2_r.id());
                    match node2_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(2, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pattern_forest_union_return_new_forest_for_left_kind_and_right_kind()
{
    // forest1
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest11: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest11.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest12: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest12.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest12.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest13: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest13.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest11, forest12, forest13]))));
    // forest2
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest21: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest21.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest22: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest22.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest22.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest23: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest23.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest21, forest22, forest23]))));
    match forest1.union(&forest2) {
        Ok((kind3, forest3)) => {
            assert_eq!(PatternKind::New, kind3);
            match forest3 {
                PatternForest::Alt(nodes, Some(3)) => {
                    assert_eq!(3, nodes.len());
                    let node1_r = nodes[0].borrow();
                    assert_eq!(1, *node1_r.id());
                    let node2_r = nodes[1].borrow();
                    assert_eq!(2, *node2_r.id());
                    match node2_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                    let node3_r = nodes[2].borrow();
                    assert_eq!(2, *node3_r.id());
                    match node3_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(2, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(2, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pattern_forest_union_return_new_forest_for_nested_pattern_node()
{
    // forest1
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest11: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest11.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest12: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest12.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest121: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest121.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest121.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest12.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest121]))));
    let mut forest13: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest13.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest11, forest12, forest13]))));
    // forest2
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest21: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest21.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest22: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest22.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest221: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest221.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest221.add_node(PatternNode::new(3, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest22.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest221]))));
    let mut forest23: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest23.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest21, forest22, forest23]))));
    match forest1.union(&forest2) {
        Ok((kind3, forest3)) => {
            assert_eq!(PatternKind::New, kind3);
            match forest3 {
                PatternForest::Alt(nodes, Some(3)) => {
                    assert_eq!(2, nodes.len());
                    let node1_r = nodes[0].borrow();
                    assert_eq!(1, *node1_r.id());
                    let node2_r = nodes[1].borrow();
                    assert_eq!(2, *node2_r.id());
                    match node2_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                    match node2_r.forests() {
                                        PatternForests::Unfilled(forests) => {
                                            match &forests[0] {
                                                PatternForest::All => assert!(true),
                                                _ => assert!(false),
                                            }
                                        },
                                        _ => assert!(false),
                                    }
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(1, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(2, *node1_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}

#[test]
fn test_pattern_forest_union_return_new_mixed_forest()
{
    // forest1
    let mut forest1: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest1.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    // node1
    let mut forest11: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest11.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest11.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest12: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest12.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest13: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest13.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest13.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest11, forest12, forest13]))));
    // node2
    let mut forest14: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest14.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest14.add_node(PatternNode::new(3, PatternForests::Unfilled(Vec::new()))));
    let mut forest15: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest15.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest16: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest16.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest16.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest1.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest14, forest15, forest16]))));
    // forest2
    let mut forest2: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest2.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    // node3
    let mut forest21: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest21.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest21.add_node(PatternNode::new(3, PatternForests::Unfilled(Vec::new()))));
    let mut forest22: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest22.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    let mut forest23: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest23.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest23.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest21, forest22, forest23]))));
    // node4
    let mut forest24: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest24.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest24.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest25: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest25.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    let mut forest26: PatternForest<i32> = PatternForest::Alt(Vec::new(), Some(3));
    assert_eq!(true, forest26.add_node(PatternNode::new(1, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest26.add_node(PatternNode::new(2, PatternForests::Unfilled(Vec::new()))));
    assert_eq!(true, forest2.add_node(PatternNode::new(2, PatternForests::Unfilled(vec![forest24, forest25, forest26]))));
    match forest1.union(&forest2) {
        Ok((kind3, forest3)) => {
            assert_eq!(PatternKind::New, kind3);
            match forest3 {
                PatternForest::Alt(nodes, Some(3)) => {
                    assert_eq!(2, nodes.len());
                    let node1_r = nodes[0].borrow();
                    assert_eq!(1, *node1_r.id());
                    let node2_r = nodes[1].borrow();
                    assert_eq!(2, *node2_r.id());
                    match node2_r.forests() {
                        PatternForests::Unfilled(forests) => {
                            match &forests[0] {
                                PatternForest::All => assert!(true),
                                _ => assert!(false),
                            }
                            match &forests[1] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                            match &forests[2] {
                                PatternForest::Alt(nodes, Some(3)) => {
                                    assert_eq!(2, nodes.len());
                                    let node1_r = nodes[0].borrow();
                                    assert_eq!(1, *node1_r.id());
                                    let node2_r = nodes[1].borrow();
                                    assert_eq!(2, *node2_r.id());
                                },
                                _ => assert!(false),
                            }
                        },
                        _ => assert!(false),
                    }
                },
                _ => assert!(false),
            }
        },
        Err(_) => assert!(false),
    }
}
