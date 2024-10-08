//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::stdlib::*;

pub mod builtins;
pub mod error;
pub mod evals;
pub mod evaluator;
pub mod instancer;
pub mod lexer;
pub mod limiter;
pub mod namer;
pub mod parser;
pub(crate) mod private;
pub mod recurser;
pub mod source;
pub mod shared_flag;
pub mod tree;
pub mod type_matcher;
pub mod type_stack;
pub mod typer;

pub use source::Source;
pub use tree::Tree;
pub use error::FrontendResultWithErrors;

use error::FrontendErrors;

pub fn do_frontend_phases_with_sources_without_stdlib(srcs: &[Source]) -> FrontendResultWithErrors<Tree>
{
    let mut tree = Tree::new();
    for src in srcs {
        match src {
            Source::String(path, s) => {
                match parser::parse_with_path(path.as_str(), s.as_str(), &mut tree) {
                    Ok(()) => (),
                    Err(err) => return Err(FrontendErrors::new(vec![err])),
                }
            },
            Source::File(path) => {
                match parser::parse_from_file(path.as_str(), &mut tree) {
                    Ok(()) => (),
                    Err(err) => return Err(FrontendErrors::new(vec![err])),
                }
            },
        }
    }
    namer::check_idents(&mut tree)?;
    typer::check_types(&tree)?;
    instancer::check_insts(&tree)?;
    limiter::check_limits(&tree)?;
    evaluator::evaluate_values(&tree)?;
    recurser::check_recursions(&tree)?;
    Ok(tree)
}

pub fn do_frontend_phases_with_sources(srcs: &[Source]) -> FrontendResultWithErrors<Tree>
{
    let mut srcs_with_stdlib = stdlib_sources();
    srcs_with_stdlib.extend_from_slice(srcs);
    do_frontend_phases_with_sources_without_stdlib(srcs_with_stdlib.as_slice())
}

pub fn do_frontend_phases_with_path(path: &str, src: &str) -> FrontendResultWithErrors<Tree>
{
    let srcs = vec![Source::String(String::from(path), String::from(src))];
    do_frontend_phases_with_sources(srcs.as_slice())
}

pub fn do_frontend_phases(src: &str) -> FrontendResultWithErrors<Tree>
{ do_frontend_phases_with_path("(string)", src) }

pub fn do_frontend_phases_with_file(path: &str) -> FrontendResultWithErrors<Tree>
{
    let srcs = vec![Source::File(String::from(path))];
    do_frontend_phases_with_sources(srcs.as_slice())
}

#[cfg(test)]
mod tests;
