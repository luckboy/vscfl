//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::error;
use std::fmt;
use std::result;
use crate::frontend::error::Pos;

#[derive(Debug)]
pub struct BackendErrorMessage(pub Pos, pub String);

impl fmt::Display for BackendErrorMessage
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            BackendErrorMessage(pos, msg) => write!(f, "{}: {}.{}: {}", pos.path, pos.line, pos.column, msg),
        }
    }
}

#[derive(Debug)]
pub enum BackendError
{
    Messages(Vec<BackendErrorMessage>),
    Internal(String),
}

impl error::Error for BackendError
{}

impl fmt::Display for BackendError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        match self {
            BackendError::Messages(msgs) => {
                let mut is_first = true;
                for msg in msgs {
                    if !is_first {
                        write!(f, "\n")?;
                    }
                    write!(f, "{}", msg)?;
                    is_first = false;
                }
                Ok(())
            },
            BackendError::Internal(msg) => write!(f, "backend internal error: {}", msg),
        }
    }
}

pub type BackendResult<T> = result::Result<T, BackendError>;
