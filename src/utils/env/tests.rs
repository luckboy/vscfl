//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use super::*;

#[test]
fn test_environment_add_var_adds_variables()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(1, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 0)) => assert_eq!(2, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 0)) => assert_eq!(3, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_add_var_replace_variable()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("b"), 3));
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(1, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 0)) => assert_eq!(3, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_remove_var_removes_variables()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    assert_eq!(true, env.add_var(String::from("d"), 3));
    assert_eq!(true, env.remove_var(&String::from("b")));
    assert_eq!(true, env.remove_var(&String::from("d")));
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(1, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 0)) => assert_eq!(3, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_push_new_vars_pushes_new_variables()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(1, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 1)) => assert_eq!(4, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 1)) => assert_eq!(5, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("d")) {
        Some((x, 1)) => assert_eq!(6, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_pop_vars_pops_variables()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    env.pop_vars();
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(1, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 0)) => assert_eq!(2, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 0)) => assert_eq!(3, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("d")) {
        None => assert!(true),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_push_saved_vars_pushes_saved_variables()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    let saved_var_stack_idx = env.saved_var_stack_len();
    env.push_saved_vars();
    match env.var_mut_and_stack_index(&String::from("a")) {
        Some((x, 0)) => *x = 2,
        _ => assert!(false),
    }
    match env.var_mut_and_stack_index(&String::from("b")) {
        Some((x, 1)) => *x = 3,
        _ => assert!(false),
    }
    env.swap_saved_vars();
    env.merge_and_pop_saved_var_vars(saved_var_stack_idx, |x, y| x + y);
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(3, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 1)) => assert_eq!(7, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 1)) => assert_eq!(5, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("d")) {
        Some((x, 1)) => assert_eq!(6, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_push_saved_vars_pushes_second_saved_variables()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    let saved_var_stack_idx = env.saved_var_stack_len();
    env.push_saved_vars();
    match env.var_mut_and_stack_index(&String::from("a")) {
        Some((x, 0)) => *x = 2,
        _ => assert!(false),
    }
    match env.var_mut_and_stack_index(&String::from("b")) {
        Some((x, 1)) => *x = 3,
        _ => assert!(false),
    }
    env.swap_saved_vars();
    env.push_saved_vars();
    match env.var_mut_and_stack_index(&String::from("b")) {
        Some((x, 1)) => *x = 4,
        _ => assert!(false),
    }
    match env.var_mut_and_stack_index(&String::from("c")) {
        Some((x, 1)) => *x = 5,
        _ => assert!(false),
    }
    env.swap_saved_vars();
    env.merge_and_pop_saved_var_vars(saved_var_stack_idx, |x, y| x + y);
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(3, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 1)) => assert_eq!(11, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 1)) => assert_eq!(10, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("d")) {
        Some((x, 1)) => assert_eq!(6, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_push_saved_vars_pushes_nested_saved_variables()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    env.push_saved_vars();
    match env.var_mut_and_stack_index(&String::from("a")) {
        Some((x, 0)) => *x = 2,
        _ => assert!(false),
    }
    match env.var_mut_and_stack_index(&String::from("b")) {
        Some((x, 1)) => *x = 3,
        _ => assert!(false),
    }
    let saved_var_stack_idx = env.saved_var_stack_len();
    env.push_saved_vars();
    match env.var_mut_and_stack_index(&String::from("b")) {
        Some((x, 1)) => *x = 4,
        _ => assert!(false),
    }
    match env.var_mut_and_stack_index(&String::from("c")) {
        Some((x, 1)) => *x = 5,
        _ => assert!(false),
    }
    env.swap_saved_vars();
    env.merge_and_pop_saved_var_vars(saved_var_stack_idx, |x, y| x + y);
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(2, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 1)) => assert_eq!(7, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 1)) => assert_eq!(10, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("d")) {
        Some((x, 1)) => assert_eq!(6, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_swap_saved_vars_removes_saved_variable_that_is_removed()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    let saved_var_stack_idx = env.saved_var_stack_len();
    env.push_saved_vars();
    match env.var_mut_and_stack_index(&String::from("a")) {
        Some((x, 0)) => *x = 2,
        _ => assert!(false),
    }
    assert_eq!(true, env.remove_var(&String::from("b")));
    env.swap_saved_vars();
    env.merge_and_pop_saved_var_vars(saved_var_stack_idx, |x, y| x + y);
    match env.var_and_stack_index(&String::from("a")) {
        Some((x, 0)) => assert_eq!(3, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("b")) {
        Some((x, 1)) => assert_eq!(4, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("c")) {
        Some((x, 1)) => assert_eq!(5, *x),
        _ => assert!(false),
    }
    match env.var_and_stack_index(&String::from("d")) {
        Some((x, 1)) => assert_eq!(6, *x),
        _ => assert!(false),
    }
}

#[test]
fn test_environment_foreach_with_result_calls_function_for_each_variable()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    let mut vars: Vec<(String, i32)> = Vec::new();
    let res: Result<(), ()> = env.foreach_with_result(|id, v| {
            vars.push((id.clone(), *v));
            Ok(())
    });
    match res {
        Ok(()) => assert!(true),
        Err(()) => assert!(false),
    }
    let expected_vars = vec![
        (String::from("b"), 4),
        (String::from("c"), 5),
        (String::from("d"), 6)
    ];
    assert_eq!(expected_vars, vars);
}

#[test]
fn test_environment_foreach_with_result_stops_for_error()
{
    let mut env: Environment<i32> = Environment::new();
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("a"), 1));
    assert_eq!(true, env.add_var(String::from("b"), 2));
    assert_eq!(true, env.add_var(String::from("c"), 3));
    env.push_new_vars();
    assert_eq!(true, env.add_var(String::from("b"), 4));
    assert_eq!(true, env.add_var(String::from("c"), 5));
    assert_eq!(true, env.add_var(String::from("d"), 6));
    let mut vars: Vec<(String, i32)> = Vec::new();
    let res: Result<(), ()> = env.foreach_with_result(|id, v| {
            if *v <= 5 {
                vars.push((id.clone(), *v));
                Ok(())
            } else {
                Err(())
            }
    });
    match res {
        Err(()) => assert!(true),
        Ok(()) => assert!(false),
    }
    let expected_vars = vec![
        (String::from("b"), 4),
        (String::from("c"), 5)
    ];
    assert_eq!(expected_vars, vars);
}
