//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::frontend::error::*;
use crate::frontend::lexer::*;

pub struct Parser<'a>
{
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a>
{
    pub fn new(lexer: Lexer<'a>) -> Self
    { Parser { lexer, } }

    fn parse_zero_or_more_with_fn_ref<T, F>(&mut self, xs: &mut Vec<T>, sep_token: &Token, end_tokens: &[Token], f: &mut F) -> FrontendResult<()>
        where F: FnMut(&mut Self) -> FrontendResult<T>
    {
        loop {
            match self.lexer.next_token()? {
                (token, _) if end_tokens.iter().any(|t| t == &token) => break,
                (Token::Eof, pos) => return Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    xs.push(f(self)?);
                    match self.lexer.next_token()? {
                        (token2, _) if &token2 == sep_token => (),
                        (token2, pos2) => {
                            self.lexer.undo_token(token2, pos2);
                            break;
                        },
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_zero_or_more<T, F>(&mut self, sep_token: &Token, end_tokens: &[Token], mut f: F) -> FrontendResult<Vec<T>>
        where F: FnMut(&mut Self) -> FrontendResult<T>
    {
        let mut xs: Vec<T> = Vec::new();
        self.parse_zero_or_more_with_fn_ref(&mut xs, sep_token, end_tokens, &mut f)?; 
        Ok(xs)
    }

    fn parse_one_or_more<T, F>(&mut self, sep_token: &Token, end_tokens: &[Token], mut f: F) -> FrontendResult<Vec<T>>
        where F: FnMut(&mut Self) -> FrontendResult<T>
    {
        let mut xs: Vec<T> = Vec::new();
        xs.push(f(self)?);
        match self.lexer.next_token()? {
            (token, _) if &token == sep_token => self.parse_zero_or_more_with_fn_ref(&mut xs, sep_token, end_tokens, &mut f)?,
            (token, pos) => self.lexer.undo_token(token, pos),
        }
        Ok(xs)
    }
}
