//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::error;
use std::fmt;
use std::io::*;
use std::result;

#[derive(Copy, Clone, Debug)]
pub struct Pos
{
    pub line: u64,
    pub column: u64,
}

impl Pos
{
    pub fn new(line: u64, column: u64) -> Self
    { Pos { line, column, } }
}

#[derive(Debug)]
pub enum FrontendError
{
    Io(String, Error),
    Message(String, Pos, String),
}

impl error::Error for FrontendError
{}

impl fmt::Display for FrontendError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            FrontendError::Io(path, err) => write!(f, "{}: I/O: {}", path, err),
            FrontendError::Message(path, pos, msg) => write!(f, "{}: {}.{}: {}", pos.line, pos.column, path, msg),
        }
    }
}

pub type FrontendResult<T> = result::Result<T, FrontendError>;
