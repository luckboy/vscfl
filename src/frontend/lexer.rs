//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::*;
use crate::frontend::error::*;

pub struct Lexer<'a>
{
    path: String,
    pos: Pos,
    reader: &'a mut dyn BufRead,
    pushed_chars: Vec<(char, Pos)>
}

impl<'a> Lexer<'a>
{
    pub fn new(path: String, reader: &'a mut dyn BufRead) -> Self
    { Lexer { path, pos: Pos::new(1, 1), reader, pushed_chars: Vec::new(), } }
    
    fn read_char(&mut self) -> FrontendResult<Option<char>>
    {
        let mut c_buf: Vec<u8> = Vec::new();
        for i in 0..6 {
            let mut buf: [u8; 1] = [0; 1];
            let mut is_eof = false;
            loop {
                match self.reader.read(&mut buf) {
                    Ok(0) => {
                        is_eof = true;
                        break;
                    },
                    Ok(_) => {
                        c_buf.push(buf[0]);
                        break;
                    },
                    Err(err) if err.kind() == ErrorKind::Interrupted => (),
                    Err(err) => return Err(FrontendError::Io(self.path.clone(), err)),
                }
            }
            if is_eof {
                if i == 0 {
                    return Ok(None);
                } else {
                    return Err(FrontendError::Io(self.path.clone(), Error::new(ErrorKind::InvalidData, "stream did not contain valid UTF-8")))
                }
            } else {
                match String::from_utf8(c_buf.clone()) {
                    Ok(s) => return Ok(Some(s.chars().next().unwrap())),
                    Err(_) => (),
                }
            }
        }
        Err(FrontendError::Io(self.path.clone(), Error::new(ErrorKind::InvalidData, "stream did not contain valid UTF-8")))
    }
    
    pub fn next_char(&mut self) -> FrontendResult<(Option<char>, Pos)>
    {
        let res = match self.pushed_chars.pop() {
            Some((c, pos)) => {
                self.pos = pos;
                Ok((Some(c), pos)) 
            },
            None => {
                match self.read_char() {
                    Ok(None) => Ok((None, self.pos)),
                    Ok(Some(c)) => Ok((Some(c), self.pos)),
                    Err(err) => Err(err),
                }
            }
        };
        match res {
            Ok((Some(c), pos)) => {
                if c == '\n' {
                    self.pos.line += 1;
                    self.pos.column = 1;
                } else {
                    self.pos.column += 1;
                }
                Ok((Some(c), pos))
            },
            res => res,
        }
    }
    
    pub fn undo_char(&mut self, c: char, pos: Pos)
    { self.pushed_chars.push((c, pos)); }
}
