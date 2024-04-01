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
    Backslash,
    Dot,
    DotDot,
    DotDotEq,
    LArrow,
    RArrow,
    DArrow,
    EqGt,
    Comma,
    Colon,
    Semi,
    As,
    Builtin,
    Data,
    Else,
    False,
    For,
    If,
    In,
    Inline,
    Impl,
    Let,
    Match,
    Shared,
    Trait,
    True,
    Type,
    Uniq,
    Where,
    Kernel,
    Private,
    Local,
    Global,
    Constant,
    Char(u8),
    String(Vec<u8>),
    Int(i32),
    Uint(u32),
    Float(f32),
    Long(i64),
    Ulong(u64),
    Double(f64),
    ConIdent(String),
    VarIdent(String),
    Eof,
}

#[derive(Copy, Clone, Debug)]
enum TokenChar
{
    Byte(u8),
    Char(char),
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

    fn read_token_char(&mut self, is_char_token: bool, token_pos: Pos) -> FrontendResult<Option<TokenChar>>
    {
        match self.next_char()? {
            (None, _) => {
                if is_char_token {
                    Err(FrontendError::Message(self.path.clone(), token_pos, String::from("unclosed character")))
                } else {
                    Err(FrontendError::Message(self.path.clone(), token_pos, String::from("unclosed string")))
                }
            },
            (Some('\''), _) if is_char_token => Ok(None),
            (Some('"'), _) if !is_char_token => Ok(None),
            (Some('\\'), pos) => {
                match self.next_char()? {
                    (None, _) => {
                        if is_char_token {
                            Err(FrontendError::Message(self.path.clone(), token_pos, String::from("unclosed character")))
                        } else {
                            Err(FrontendError::Message(self.path.clone(), token_pos, String::from("unclosed string")))
                        }
                    },
                    (Some('X' | 'x'), _) => {
                        let mut s = String::new();
                        for _ in 0..2 {
                            match self.next_char()? {
                                (None, _) => {
                                    if is_char_token {
                                        return Err(FrontendError::Message(self.path.clone(), token_pos, String::from("unclosed character")));
                                    } else {
                                        return Err(FrontendError::Message(self.path.clone(), token_pos, String::from("unclosed string")));
                                    }
                                },
                                (Some(c3), _) if c3.is_digit(16) => s.push(c3),
                                (Some(_), pos3) => return Err(FrontendError::Message(self.path.clone(), pos3, String::from("unexpected character"))),
                            }
                        }
                        match u8::from_str_radix(s.as_str(), 16) {
                            Ok(n) => Ok(Some(TokenChar::Byte(n))),
                            Err(_) => Err(FrontendError::Message(self.path.clone(), pos, String::from("invalud escape")))
                        }
                    },
                    (Some('0'), _) => Ok(Some(TokenChar::Byte(0))),
                    (Some('n'), _) => Ok(Some(TokenChar::Char('\n'))),
                    (Some('r'), _) => Ok(Some(TokenChar::Char('\r'))),
                    (Some('t'), _) => Ok(Some(TokenChar::Char('\t'))),
                    (Some(c2), _) => Ok(Some(TokenChar::Char(c2))),
                }
            },
            (Some(c), _) => Ok(Some(TokenChar::Char(c))),
        }
    }
    
    fn next_char_token(&mut self) -> FrontendResult<Option<(Token, Pos)>>
    {
        match self.next_char()? {
            (None, _) => Ok(None), 
            (Some('\''), pos) => {
                match self.read_token_char(true, pos)? {
                    None => Err(FrontendError::Message(self.path.clone(), pos, String::from("empty character"))),
                    Some(TokenChar::Byte(n)) => Ok(Some((Token::Char(n), pos))),
                    Some(TokenChar::Char(c)) => {
                        let mut s = String::new();
                        s.push(c);
                        let b = s.as_bytes();
                        if b.len() == 1 {
                            match s.as_bytes().first() {
                                Some(n) => Ok(Some((Token::Char(*n), pos))),
                                None => Err(FrontendError::Message(self.path.clone(), pos, String::from("invalid character")))
                            }
                        } else {
                            Err(FrontendError::Message(self.path.clone(), pos, String::from("invalid character")))
                        }
                    },
                }
            },
            (Some(c), pos) => {
                self.undo_char(c, pos);
                Ok(None)
            },
        }
    }

    fn next_string_token(&mut self) -> FrontendResult<Option<(Token, Pos)>>
    {
        match self.next_char()? {
            (None, _) => Ok(None), 
            (Some('"'), pos) => {
                let mut bs = Vec::new();
                loop {
                    match self.read_token_char(false, pos)? {
                        None => break,
                        Some(TokenChar::Byte(n)) => bs.push(n), 
                        Some(TokenChar::Char(c)) => {
                            let mut s = String::new();
                            s.push(c);
                            bs.extend_from_slice(s.as_bytes());
                        },
                    }
                }
                Ok(Some((Token::String(bs), pos)))
            },
            (Some(c), pos) => {
                self.undo_char(c, pos);
                Ok(None)
            },
        }
    }
    
    fn read_token_digits(&mut self, s: &mut String, radix: u32) -> FrontendResult<()>
    {
        loop {
            match self.next_char()? {
                (None, _) => break,
                (Some(c), _) if c.is_digit(radix) => s.push(c),
                (Some(c), pos) => {
                    self.undo_char(c, pos); 
                    break;
                },
            }
        }
        Ok(())
    }

    fn read_one_or_more_token_digits(&mut self, s: &mut String, radix: u32, token_pos: Pos) -> FrontendResult<()>
    {
        match self.next_char()? {
            (None, _) => return Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
            (Some(c), _) if c.is_digit(radix) => {
                s.push(c);
                self.read_token_digits(s, radix)?;
            },
            (Some(c), pos) => self.undo_char(c, pos),
        }
        Ok(())
    }
    
    fn next_number_token(&mut self) -> FrontendResult<Option<(Token, Pos)>>
    {
        let mut s = String::new();
        let mut is_dot_or_exp = false;
        let token_pos: Pos;
        match self.next_char()? {
            (None, _) => return Ok(None),
            (Some('0'), pos) => {
                token_pos = pos;
                match self.next_char()? {
                    (None, _) => (),
                    (Some(c2 @ ('B' | 'b' | 'O' | 'o' | 'X' | 'x')), _) => {
                        let radix = match c2 {
                            'B' | 'b' => 2,
                            'O' | 'o' => 8,
                            _ => 16,
                        };
                        self.read_one_or_more_token_digits(&mut s, radix, token_pos)?;
                        match self.next_char()? {
                            (Some('I'), _) => {
                                match i64::from_str_radix(s.as_str(), radix) {
                                    Ok(n) => return Ok(Some((Token::Long(n), token_pos))),
                                    Err(_) => return Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                                }
                            },
                            (Some('u'), _) => {
                                match u32::from_str_radix(s.as_str(), radix) {
                                    Ok(n) => return Ok(Some((Token::Uint(n), token_pos))),
                                    Err(_) => return Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                                }
                            },
                            (Some('U'), _) => {
                                match u64::from_str_radix(s.as_str(), radix) {
                                    Ok(n) => return Ok(Some((Token::Ulong(n), token_pos))),
                                    Err(_) => return Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                                }
                            },
                            (opt_c3 @ (None | Some(_)), pos3) => {
                                match opt_c3 {
                                    None | Some('i') => (), 
                                    Some(c3) => self.undo_char(c3, pos3),
                                }
                                match i32::from_str_radix(s.as_str(), radix) {
                                    Ok(n) => return Ok(Some((Token::Int(n), token_pos))),
                                    Err(_) => return Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                                }
                            },
                        }
                    },
                    (Some(c2), pos2) => {
                        s.push('0');
                        self.undo_char(c2, pos2);
                    },
                }
            },
            (Some(c), pos) if c.is_digit(10) => {
                token_pos = pos;
                s.push(c);
            }
            (Some(c), pos) => {
                self.undo_char(c, pos);
                return Ok(None);
            },
        }
        self.read_token_digits(&mut s, 10)?;
        match self.next_char()? {
            (None, _) => (),
            (Some('.'), _) => {
                is_dot_or_exp = true;
                s.push('.');
                self.read_one_or_more_token_digits(&mut s, 10, token_pos)?;
            },
            (Some(c), pos) => self.undo_char(c, pos),
        }
        match self.next_char()? {
            (None, _) => (),
            (Some(c @ ('E' | 'e')), _) => {
                is_dot_or_exp = true;
                s.push(c);
                match self.next_char()? {
                    (None, _) => (),
                    (Some(c2 @ ('+' | '-')), _) => s.push(c2),
                    (Some(c2), pos2) => self.undo_char(c2, pos2),
                }
                self.read_one_or_more_token_digits(&mut s, 10, token_pos)?;
            },
            (Some(c), pos) => self.undo_char(c, pos),
        }
        if is_dot_or_exp {
            match self.next_char()? {
                (Some('F'), _) => {
                    match s.parse::<f64>() {
                        Ok(n) => Ok(Some((Token::Double(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
                (opt_c @ (None | Some(_)), pos) => {
                    match opt_c {
                        None | Some('f') => (),
                        Some(c) => self.undo_char(c, pos), 
                    }
                    match s.parse::<f32>() {
                        Ok(n) => Ok(Some((Token::Float(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
            }
        } else {
            match self.next_char()? {
                (Some('f'), _) => {
                    match s.parse::<f32>() {
                        Ok(n) => Ok(Some((Token::Float(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
                (Some('F'), _) => {
                    match s.parse::<f64>() {
                        Ok(n) => Ok(Some((Token::Double(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
                (Some('I'), _) => {
                    match s.parse::<i64>() {
                        Ok(n) => Ok(Some((Token::Long(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
                (Some('u'), _) => {
                    match s.parse::<u32>() {
                        Ok(n) => Ok(Some((Token::Uint(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
                (Some('U'), _) => {
                    match s.parse::<u64>() {
                        Ok(n) => Ok(Some((Token::Ulong(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
                (opt_c @ (None | Some(_)), pos) => {
                    match opt_c {
                        None | Some('i') => (),
                        Some(c) => self.undo_char(c, pos), 
                    }
                    match s.parse::<i32>() {
                        Ok(n) => Ok(Some((Token::Int(n), token_pos))),
                        Err(_) => Err(FrontendError::Message(self.path.clone(), token_pos, String::from("invalid number"))),
                    }
                },
            }
        }
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
                    (Some('*'), pos) => Ok((Token::Star, pos)),
                    (Some('/'), pos) => Ok((Token::Slash, pos)),
                    (Some('%'), pos) => Ok((Token::Perc, pos)),
                    (Some('+'), pos) => Ok((Token::Plus, pos)),
                    (Some('-'), pos) => {
                        match self.next_char()? {
                            (None, _) => Ok((Token::Minus, pos)),
                            (Some('>'), _) => Ok((Token::RArrow, pos)),
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
                            (Some('-'), _) => {
                                match self.next_char()? {
                                    (None, _) => Ok((Token::LArrow, pos)),
                                    (Some('>'), _) => Ok((Token::DArrow, pos)),
                                    (Some(c3), pos3) => {
                                        self.undo_char(c3, pos3);
                                        Ok((Token::LArrow, pos))
                                    },
                                }
                            },
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
                    (Some('\\'), pos) => Ok((Token::Backslash, pos)),
                    (Some(c), pos) => {
                        self.undo_char(c, pos);
                        if let Some((token, pos)) = self.next_char_token()? {
                            Ok((token, pos))
                        } else if let Some((token, pos)) = self.next_string_token()? {
                            Ok((token, pos))
                        } else if let Some((token, pos)) = self.next_number_token()? {
                            Ok((token, pos))
                        } else {
                            Err(FrontendError::Message(self.path.clone(), pos, String::from("unexpected character")))
                        }
                    },
                }
            },
        }
    }
    
    pub fn undo_token(&mut self, token: Token, pos: Pos)
    { self.pushed_tokens.push((token, pos)); }
}
