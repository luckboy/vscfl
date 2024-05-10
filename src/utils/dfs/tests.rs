//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

fn f(u: &usize, processeds: &BTreeSet<usize>, data: &mut (Vec<usize>, Vec<usize>), graph: &[Vec<usize>]) -> Result<Vec<usize>, bool>
{
    match graph.get(*u) {
        Some(neighbors) => {
            data.0.push(*u);
            if !neighbors.iter().any(|v| processeds.contains(v)) {
                Ok(neighbors.clone())
            } else {
                Err(true)
            }
        },
        None => Err(false),
    }
}

fn g(u: &usize, data: &mut (Vec<usize>, Vec<usize>), graph: &[Vec<usize>]) -> Result<(), bool>
{
    if graph.get(*u).is_some() {
        data.1.push(*u);
        Ok(())
    } else {
        Err(false)
    }
}

#[test]
fn test_dfs_with_result_searches_graph()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // 0
        vec![3], // 1
        vec![3], // 2
        Vec::new() // 3
    ];
    let mut visiteds: BTreeSet<usize> = BTreeSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs_with_result(&0, &mut visiteds, &mut data, |u, processeds, data| f(u, processeds, data, graph.as_slice()), |u, data| g(u, data, graph.as_slice()));
    match res {
        Ok(()) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1, 3, 2], data.0);
    assert_eq!(vec![3, 1, 2, 0], data.1);
    let mut expected_visiteds: BTreeSet<usize> = BTreeSet::new();
    expected_visiteds.insert(0);
    expected_visiteds.insert(1);
    expected_visiteds.insert(2);
    expected_visiteds.insert(3);
    assert_eq!(expected_visiteds, visiteds);
}

#[test]
fn test_dfs_with_result_searches_second_graph()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2, 3], // 0
        vec![4, 5], // 1
        vec![5, 6], // 2
        vec![4, 6], // 3
        Vec::new(), // 4
        Vec::new(), // 5
        Vec::new()  // 6
    ];
    let mut visiteds: BTreeSet<usize> = BTreeSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs_with_result(&0, &mut visiteds, &mut data, |u, processeds, data| f(u, processeds, data, graph.as_slice()), |u, data| g(u, data, graph.as_slice()));
    match res {
        Ok(()) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1, 4, 5, 2, 6, 3], data.0);
    assert_eq!(vec![4, 5, 1, 6, 2, 3, 0], data.1);
    let mut expected_visiteds: BTreeSet<usize> = BTreeSet::new();
    expected_visiteds.insert(0);
    expected_visiteds.insert(1);
    expected_visiteds.insert(2);
    expected_visiteds.insert(3);
    expected_visiteds.insert(4);
    expected_visiteds.insert(5);
    expected_visiteds.insert(6);
    assert_eq!(expected_visiteds, visiteds);
}

#[test]
fn test_dfs_with_result_searches_graph_with_little_cycle()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1], // 0
        vec![1] // 1
    ];
    let mut visiteds: BTreeSet<usize> = BTreeSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs_with_result(&0, &mut visiteds, &mut data, |u, processeds, data| f(u, processeds, data, graph.as_slice()), |u, data| g(u, data, graph.as_slice()));
    match res {
        Err(true) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1], data.0);
    assert_eq!(true, data.1.is_empty());
    let mut expected_visiteds: BTreeSet<usize> = BTreeSet::new();
    expected_visiteds.insert(0);
    assert_eq!(expected_visiteds, visiteds);
}

#[test]
fn test_dfs_with_result_searches_graph_with_cycle()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2, 3], // 0
        vec![4, 5], // 1
        vec![5, 0], // 2
        vec![4, 6], // 3
        Vec::new(), // 4
        Vec::new(), // 5
        Vec::new()  // 6
    ];
    let mut visiteds: BTreeSet<usize> = BTreeSet::new();
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs_with_result(&0, &mut visiteds, &mut data, |u, processeds, data| f(u, processeds, data, graph.as_slice()), |u, data| g(u, data, graph.as_slice()));
    match res {
        Err(true) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 1, 4, 5, 2], data.0);
    assert_eq!(vec![4, 5, 1], data.1);
    let mut expected_visiteds: BTreeSet<usize> = BTreeSet::new();
    expected_visiteds.insert(0);
    expected_visiteds.insert(1);
    expected_visiteds.insert(4);
    expected_visiteds.insert(5);
    assert_eq!(expected_visiteds, visiteds);
}

#[test]
fn test_dfs_with_result_searches_graph_from_two_vertices()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![2, 3], // 0
        vec![2, 4], // 1
        vec![5, 6], // 2
        Vec::new(), // 3
        vec![6], // 4
        Vec::new(), // 5
        Vec::new()  // 6
    ];
    let mut visiteds: BTreeSet<usize> = BTreeSet::new();
    let mut data1: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res1 = dfs_with_result(&0, &mut visiteds, &mut data1, |u, processeds, data| f(u, processeds, data, graph.as_slice()), |u, data| g(u, data, graph.as_slice()));
    match res1 {
        Ok(()) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![0, 2, 5, 6, 3], data1.0);
    assert_eq!(vec![5, 6, 2, 3, 0], data1.1);
    let mut expected_visiteds1: BTreeSet<usize> = BTreeSet::new();
    expected_visiteds1.insert(0);
    expected_visiteds1.insert(2);
    expected_visiteds1.insert(3);
    expected_visiteds1.insert(5);
    expected_visiteds1.insert(6);
    assert_eq!(expected_visiteds1, visiteds);
    let mut data2: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res2 = dfs_with_result(&1, &mut visiteds, &mut data2, |u, processeds, data| f(u, processeds, data, graph.as_slice()), |u, data| g(u, data, graph.as_slice()));
    match res2 {
        Ok(()) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(vec![1, 4], data2.0);
    assert_eq!(vec![4, 1], data2.1);
    let mut expected_visiteds2: BTreeSet<usize> = BTreeSet::new();
    expected_visiteds2.insert(0);
    expected_visiteds2.insert(1);
    expected_visiteds2.insert(2);
    expected_visiteds2.insert(3);
    expected_visiteds2.insert(4);
    expected_visiteds2.insert(5);
    expected_visiteds2.insert(6);
    assert_eq!(expected_visiteds2, visiteds);
}

#[test]
fn test_dfs_with_result_does_not_search_from_visited_vertex()
{
    let graph: Vec<Vec<usize>> = vec![
        vec![1, 2], // 0
        vec![3], // 1
        vec![3], // 2
        Vec::new() // 3
    ];
    let mut visiteds: BTreeSet<usize> = BTreeSet::new();
    visiteds.insert(0);
    let mut data: (Vec<usize>, Vec<usize>) = (Vec::new(), Vec::new());
    let res = dfs_with_result(&0, &mut visiteds, &mut data, |u, processeds, data| f(u, processeds, data, graph.as_slice()), |u, data| g(u, data, graph.as_slice()));
    match res {
        Ok(()) => assert!(true),
        _ => assert!(false),
    }
    assert_eq!(true, data.0.is_empty());
    assert_eq!(true, data.1.is_empty());
    let mut expected_visiteds: BTreeSet<usize> = BTreeSet::new();
    expected_visiteds.insert(0);
    assert_eq!(expected_visiteds, visiteds);
}
