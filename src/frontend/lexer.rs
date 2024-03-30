//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::io::*;
use crate::frontend::error::*;

#[derive(Clone, Debug)]
pub enum Token
{
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    Ex,
    Tylda,
    Star,
    Slash,
    Perc,
    Plus,
    Minus,
    LtLt,
    GtGt,
    Lt,
    GtEq,
    Gt,
    LtEq,
    EqEq,
    ExEq,
    Amp,
    Caret,
    Bar,
    Eq,
    Dot,
    DotDot,
    DotDotEq,
    RArrow,
    EqGt,
    Comma,
    Colon,
    Semi,
    Data,
    As,
    Else,
    If,
    In,
    Inst,
    Let,
    Match,
    Type,
    Kernel,
    Private,
    Local,
    Global,
    Char(u8),
    String(String),
    Int(i64),
    Uint(u64),
    Double(f64),
    ConstrIdent(String),
    VarIdent(String),
    Eof,
}

pub struct Lexer<'a>
{
    path: String,
    pos: Pos,
    reader: &'a mut dyn BufRead,
    pushed_chars: Vec<(char, Pos)>,
    pushed_tokens: Vec<(Token, Pos)>,
    has_single_greater: bool,
}

impl<'a> Lexer<'a>
{
    pub fn new(path: String, reader: &'a mut dyn BufRead) -> Self
    {
        Lexer {
            path,
            pos: Pos::new(1, 1),
            reader,
            pushed_chars: Vec::new(),
            pushed_tokens: Vec::new(),
            has_single_greater: false,
        }
    }
    
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
    
    fn skip_comment(&mut self) -> FrontendResult<()>
    {
        match self.next_char()? {
            (None, _) => (),
            (Some('/'), pos) => {
                match self.next_char()? {
                    (None, pos2) => self.undo_char('/', pos2),
                    (Some('/'), _) => {
                        loop {
                            match self.next_char()? {
                                (None, _) => (),
                                (Some('\n'), pos3) => {
                                    self.undo_char('\n', pos3);
                                    break;
                                },
                                _ => (),
                            }
                        }
                    },
                    (Some('*'), _) => {
                        loop {
                            match self.next_char()? {
                                (None, pos3) => return Err(FrontendError::Message(self.path.clone(), pos3, String::from("unclosed comment"))),
                                (Some('*'), _) => {
                                    match self.next_char()? {
                                        (None, pos3) => return Err(FrontendError::Message(self.path.clone(), pos3, String::from("unclosed comment"))),
                                        (Some('/'), _) => break,
                                        (Some(c3), pos3) => self.undo_char(c3, pos3),
                                    }
                                },
                                _ => (),
                            }
                        }
                    },
                    (Some(c2), pos2) => {
                        self.undo_char(c2, pos2);
                        self.undo_char('/', pos);
                    },
                }
            },
            (Some(c), pos) => self.undo_char(c, pos), 
        }
        Ok(())
    }
    
    fn skip_comments_and_whitespaces(&mut self) -> FrontendResult<()>
    {
        loop {
            self.skip_comment()?;
            match self.next_char()? {
                (None, _) => break,
                (Some(c), _) if c.is_whitespace() => (),
                (Some(c), pos) => {
                    self.undo_char(c, pos);
                    break;
                },
            }
        }
        Ok(())
    }
    
    pub fn next_token(&mut self) -> FrontendResult<(Token, Pos)>
    {
        match self.pushed_tokens.pop() {
            Some((token, pos)) => Ok((token, pos)),
            None => {
                self.skip_comments_and_whitespaces()?;
                match self.next_char()? {
                    (None, pos) => Ok((Token::Eof, pos)),
                    (Some('('), pos) => Ok((Token::LParen, pos)),
                    (Some(')'), pos) => Ok((Token::RParen, pos)),
                    (Some('['), pos) => Ok((Token::LBracket, pos)),
                    (Some(']'), pos) => Ok((Token::RBracket, pos)),
                    (Some('{'), pos) => Ok((Token::LBrace, pos)),
                    (Some('}'), pos) => Ok((Token::RBrace, pos)),
                    (Some('!'), pos) => {
                        match self.next_char()? {
                            (None, _) => Ok((Token::Ex, pos)),
                            (Some('='), _) => Ok((Token::ExEq, pos)),
                            (Some(c2), pos2) => {
                                self.undo_char(c2, pos2);
                                Ok((Token::Ex, pos))
                            },
                        }
                    },
                    (Some('~'), pos) => Ok((Token::Tylda, pos)),
                    (Some('*'), pos) => Ok((Token::Star, pos)),
                    (Some('/'), pos) => Ok((Token::Slash, pos)),
                    (Some('%'), pos) => Ok((Token::Perc, pos)),
                    (Some('+'), pos) => Ok((Token::Plus, pos)),
                    (Some('-'), pos) => {
                        match self.next_char()? {
                            (None, _) => Ok((Token::Minus, pos)),
                            (Some('='), _) => Ok((Token::RArrow, pos)),
                            (Some(c2), pos2) => {
                                self.undo_char(c2, pos2);
                                Ok((Token::Minus, pos))
                            },
                        }
                    },
                    (Some('<'), pos) => {
                        match self.next_char()? {
                            (None, _) => Ok((Token::Lt, pos)),
                            (Some('<'), _) => Ok((Token::LtLt, pos)),
                            (Some('='), _) => Ok((Token::LtEq, pos)),
                            (Some(c2), pos2) => {
                                self.undo_char(c2, pos2);
                                Ok((Token::Lt, pos))
                            },
                        }
                    },
                    (Some('>'), pos) => {
                        match self.next_char()? {
                            (None, _) => Ok((Token::Gt, pos)),
                            (Some('>'), _) if !self.has_single_greater => Ok((Token::GtGt, pos)),
                            (Some('='), _) => Ok((Token::GtEq, pos)),
                            (Some(c2), pos2) => {
                                self.undo_char(c2, pos2);
                                Ok((Token::Gt, pos))
                            },
                        }
                    },
                    (Some('&'), pos) => Ok((Token::Amp, pos)),
                    (Some('^'), pos) => Ok((Token::Caret, pos)),
                    (Some('|'), pos) => Ok((Token::Bar, pos)),
                    (Some('='), pos) => {
                        match self.next_char()? {
                            (None, _) => Ok((Token::Eq, pos)),
                            (Some('='), _) => Ok((Token::EqEq, pos)),
                            (Some('>'), _) => Ok((Token::EqGt, pos)),
                            (Some(c2), pos2) => {
                                self.undo_char(c2, pos2);
                                Ok((Token::Eq, pos))
                            },
                        }
                    },
                    (Some('.'), pos) => {
                        match self.next_char()? {
                            (None, _) => Ok((Token::Dot, pos)),
                            (Some('.'), _) => {
                                match self.next_char()? {
                                    (None, _) => Ok((Token::DotDot, pos)),
                                    (Some('='), _) => Ok((Token::DotDotEq, pos)),
                                    (Some(c3), pos3) => {
                                        self.undo_char(c3, pos3);
                                        Ok((Token::DotDot, pos))
                                    },
                                }
                            },
                            (Some(c2), pos2) => {
                                self.undo_char(c2, pos2);
                                Ok((Token::Dot, pos))
                            },
                        }
                    },
                    (Some(','), pos) => Ok((Token::Comma, pos)),
                    (Some(':'), pos) => Ok((Token::Colon, pos)),
                    (Some(';'), pos) => Ok((Token::Semi, pos)),
                    (_, pos) => Err(FrontendError::Message(self.path.clone(), pos, String::from("unexpected character"))),
                }
            },
        }
    }
    
    pub fn undo_token(&mut self, token: Token, pos: Pos)
    { self.pushed_tokens.push((token, pos)); }
}
