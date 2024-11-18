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
use std::rc::*;
use std::result;

#[derive(Clone, Debug)]
pub struct Pos
{
    pub path: Rc<String>,
    pub line: u64,
    pub column: u64,
}

impl Pos
{
    pub fn new(path: String, line: u64, column: u64) -> Self
    { Pos { path: Rc::new(path), line, column, } }
}

#[derive(Debug)]
pub enum FrontendError
{
    Io(String, Error),
    Message(Pos, String),
    Internal(String),
}

impl error::Error for FrontendError
{}

impl fmt::Display for FrontendError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            FrontendError::Io(path, err) => write!(f, "{}: i/o error: {}", path, err),
            FrontendError::Message(pos, msg) => write!(f, "{}: {}.{}: {}", pos.path, pos.line, pos.column, msg),
            FrontendError::Internal(msg) => write!(f, "frontend internal error: {}", msg),
        }
    }
}

#[derive(Debug)]
pub struct FrontendErrors
{
    errors: Vec<FrontendError>,
}

impl FrontendErrors
{
    pub fn new(errors: Vec<FrontendError>) -> Self
    { FrontendErrors { errors, } }
    
    pub fn errors(&self) -> &[FrontendError]
    { &self.errors }
    
    pub(crate) fn append_to(&mut self, errs: &mut Vec<FrontendError>)
    { errs.append(&mut self.errors); }
}

impl error::Error for FrontendErrors
{}

impl fmt::Display for FrontendErrors
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        let mut is_first = true;
        for err in &self.errors {
            if !is_first {
                write!(f, "\n")?;
            }
            write!(f, "{}", err)?;
            is_first = false;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct FrontendInternalError(pub String);

impl error::Error for FrontendInternalError
{}

impl fmt::Display for FrontendInternalError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    { write!(f, "{}", self.0) }
}

pub type FrontendResult<T> = result::Result<T, FrontendError>;

pub type FrontendResultWithErrors<T> = result::Result<T, FrontendErrors>;

pub type FrontendInternalResult<T> = result::Result<T, FrontendInternalError>;
