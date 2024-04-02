//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use crate::frontend::error::*;
use crate::frontend::lexer::*;
use crate::frontend::tree::*;

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

    fn parse_usize(&mut self) -> FrontendResult<usize>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::Int(n), _) => Ok(n as usize),
            (Token::Uint(n), _) => Ok(n as usize),
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_type_expr1(&mut self) -> FrontendResult<Box<TypeExpr>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::LParen, pos) => {
                let mut type_exprs = self.parse_type_exprs(&[Token::RParen])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RParen, _) => {
                        match self.lexer.next_token()? {
                            (Token::RArrow, _) => {
                                let type_expr = self.parse_type_expr1()?;
                                Ok(Box::new(TypeExpr::Fun(type_exprs, type_expr, pos)))
                            },
                            (token3, pos3) => {
                                self.lexer.undo_token(token3, pos3);
                                if type_exprs.len() == 1 {
                                    Ok(type_exprs.remove(0))
                                } else {
                                    Ok(Box::new(TypeExpr::Tuple(type_exprs, pos)))
                                }
                            },
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                }
            },
            (Token::LBracket, pos) => {
                let type_expr = self.parse_type_expr1()?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Semi, _) => {
                        let len = self.parse_usize()?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RBracket, _) => Ok(Box::new(TypeExpr::Array(type_expr, len, pos))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed bracket"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (Token::VarIdent(ident), pos) => Ok(Box::new(TypeExpr::Param(ident, pos))),
            (Token::ConIdent(ident), pos) => {
                match self.lexer.next_token()? {
                    (Token::Lt, _) => {
                        let type_exprs = self.parse_one_or_more_type_exprs(&[Token::Gt])?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::Gt, _) => Ok(Box::new(TypeExpr::App(ident, type_exprs, pos))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed angle bracket"))),
                        }
                    }
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        Ok(Box::new(TypeExpr::Var(ident, pos)))
                    },
                }
            },
            (Token::Uniq, pos) => Ok(Box::new(TypeExpr::Uniq(self.parse_type_expr1()?, pos))),
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_type_expr(&mut self) -> FrontendResult<Box<TypeExpr>>
    {
        let saved_single_greater_flag = self.lexer.has_single_greater();
        self.lexer.set_single_greater(true);
        let res = self.parse_type_expr1();
        self.lexer.set_single_greater(saved_single_greater_flag);
        res
    }

    fn parse_type_exprs(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Box<TypeExpr>>>
    { self.parse_zero_or_more(&Token::Comma, end_tokens, Self::parse_type_expr) }

    fn parse_one_or_more_type_exprs(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Box<TypeExpr>>>
    { self.parse_one_or_more(&Token::Comma, end_tokens, Self::parse_type_expr) }
}
