//
// Copyright (c) 2024 Łukasz Szpakowski
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.
//
use std::cell::*;
use std::rc::*;
use crate::frontend::error::*;
use crate::frontend::lexer::*;
use crate::frontend::tree::*;

#[derive(Clone)]
enum AccessFun
{
    Get,
    Get2,
    Set(Box<Expr>),
    Update(Box<Expr>),
    UpdateGet2(Box<Expr>),
}

#[derive(Clone)]
enum SimpleLiteral
{
    Bool(bool),
    Char(i8),
    Int(i32),
    Long(i64),
    Uint(u32),
    Ulong(u64),
    Float(f32),
    Double(f64),    
}

#[derive(Clone)]
struct Modifiers
{
    var_modifier_pair: Option<(VarModifier, Pos)>,
    fun_modifier_pair: Option<(FunModifier, Pos)>,
    inline_modifier_pair: Option<(InlineModifier, Pos)>,
}

impl Modifiers
{
    fn new() -> Self
    { Modifiers { var_modifier_pair: None, fun_modifier_pair: None, inline_modifier_pair: None, } }
}

#[derive(Clone)]
enum LiteralEither<T>
{
    Literal(Box<Literal<T>>),
    Other(Box<T>),
}

pub struct Parser<'a>
{
    lexer: Lexer<'a>,
}

impl<'a> Parser<'a>
{
    pub fn new(lexer: Lexer<'a>) -> Self
    { Parser { lexer, } }

    pub fn parse(&mut self, tree: &mut Tree) -> FrontendResult<()>
    {
        let mut defs = self.parse_defs(&[Token::Eof])?;
        match self.lexer.next_token()? {
            (Token::Eof, _) => {
                tree.append_defs(&mut defs);
                Ok(())
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    pub fn parse_type_args(&mut self) -> FrontendResult<Vec<TypeArg>>
    {
        let type_args = self.parse_zero_or_more(&Token::Comma, &[Token::Eof], Self::parse_type_arg)?;
        match self.lexer.next_token()? {
            (Token::Eof, _) => Ok(type_args),
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }

    pub fn parse_type(&mut self) -> FrontendResult<TypeExpr>
    {
        let type_expr = self.parse_type_expr()?;
        match self.lexer.next_token()? {
            (Token::Eof, _) => Ok((*type_expr).clone()),
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }

    pub fn parse_where(&mut self) -> FrontendResult<Vec<WherePair>>
    {
        let where_pairs = self.parse_one_or_more_where_pairs(&[Token::Eof])?;
        match self.lexer.next_token()? {
            (Token::Eof, _) => Ok(where_pairs),
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
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

    fn parse_one_or_more_without_end_sep<T, F>(&mut self, sep_token: &Token, mut f: F) -> FrontendResult<Vec<T>>
        where F: FnMut(&mut Self) -> FrontendResult<T>
    {
        let mut xs: Vec<T> = Vec::new();
        xs.push(f(self)?);
        loop {
            match self.lexer.next_token()? {
                (token, _) if &token == sep_token => xs.push(f(self)?),
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
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

    fn parse_modifiers(&mut self) -> FrontendResult<Modifiers>
    {
        let mut modifiers = Modifiers::new();
        loop {
            match self.lexer.next_token()? {
                (Token::Private, pos) => {
                    if modifiers.var_modifier_pair.is_none() {
                        modifiers.var_modifier_pair = Some((VarModifier::Private, pos));
                    } else {
                        return Err(FrontendError::Message(pos, String::from("already used variable modifier")));
                    }
                },
                (Token::Local, pos) => {
                    if modifiers.var_modifier_pair.is_none() {
                        modifiers.var_modifier_pair = Some((VarModifier::Local, pos));
                    } else {
                        return Err(FrontendError::Message(pos, String::from("already used variable modifier")));
                    }
                },
                (Token::Global, pos) => {
                    if modifiers.var_modifier_pair.is_none() {
                        modifiers.var_modifier_pair = Some((VarModifier::Global, pos));
                    } else {
                        return Err(FrontendError::Message(pos, String::from("already used variable modifier")));
                    }
                },
                (Token::Constant, pos) => {
                    if modifiers.var_modifier_pair.is_none() {
                        modifiers.var_modifier_pair = Some((VarModifier::Constant, pos));
                    } else {
                        return Err(FrontendError::Message(pos, String::from("already used variable modifier")));
                    }
                },
                (Token::Kernel, pos) => {
                    if modifiers.fun_modifier_pair.is_none() {
                        modifiers.fun_modifier_pair = Some((FunModifier::Kernel, pos));
                    } else {
                        return Err(FrontendError::Message(pos, String::from("already used function modifier")));
                    }
                },
                (Token::Inline, pos) => {
                    if modifiers.inline_modifier_pair.is_none() {
                        modifiers.inline_modifier_pair = Some((InlineModifier::Inline, pos));
                    } else {
                        return Err(FrontendError::Message(pos, String::from("already used inline modifier")));
                    }
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(modifiers)
    }
 
    fn parse_var(&mut self, modifiers: &Modifiers, is_in_trait: bool) -> FrontendResult<Rc<RefCell<Var>>>
    {
        match self.lexer.next_token()? {
            (Token::LParen, _) => {
                // "(", args, ")", "->", type_expr, [ "where", where_pairs ], [ "=", expr ]
                match &modifiers.var_modifier_pair {
                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos.clone(), String::from("function mustn't have variable modifier"))),
                    None => (),
                }
                let fun_modifier = match &modifiers.fun_modifier_pair {
                    Some((tmp_fun_modifier, _)) => *tmp_fun_modifier,
                    None => FunModifier::None,
                };
                let inline_modifier = match &modifiers.inline_modifier_pair {
                    Some((tmp_inline_modifier, _)) => *tmp_inline_modifier,
                    None => InlineModifier::None,
                };
                let args = self.parse_args(&[Token::RParen])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RParen, _) => {
                        let ret_type_expr = match self.lexer.next_token()? {
                            (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RArrow, _) => self.parse_type_expr()?,
                            (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        };
                        let where_pairs = match self.lexer.next_token()? {
                            (Token::Where, _) => self.parse_one_or_more_where_pairs(&[Token::RBrace, Token::Semi, Token::Eq])?,
                            (token3, pos3) => {
                                self.lexer.undo_token(token3, pos3);
                                Vec::new()
                            }
                        };
                        let body = match self.lexer.next_token()? {
                            (Token::Eof, pos3) if !is_in_trait => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::Eq, _) => Some(self.parse_expr()?),
                            (_, pos3) if !is_in_trait => return Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                            (token3, pos3) => {
                                self.lexer.undo_token(token3, pos3);
                                None
                            },
                        };
                        Ok(Rc::new(RefCell::new(Var::Fun(Box::new(Fun::Fun(fun_modifier, inline_modifier, args, ret_type_expr, where_pairs, body, None, None)), None))))
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                }
            },
            (Token::Colon, _) => {
                // ":", type_expr, [ "where", where_pairs ], [ "=", expr ]
                let var_modifier = match &modifiers.var_modifier_pair {
                    Some((tmp_var_modifier, _)) => *tmp_var_modifier,
                    None => VarModifier::None,
                };
                match &modifiers.fun_modifier_pair {
                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos.clone(), String::from("variable mustn't have function modifier"))),
                    None => (),
                }
                match &modifiers.inline_modifier_pair {
                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos.clone(), String::from("variable mustn't have inline modifier"))),
                    None => (),
                }
                let type_expr = self.parse_type_expr()?;
                let where_pairs = match self.lexer.next_token()? {
                    (Token::Where, _) => self.parse_one_or_more_where_pairs(&[Token::RBrace, Token::Semi, Token::Eq])?,
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        Vec::new()
                    }
                };
                let expr = match self.lexer.next_token()? {
                    (Token::Eof, pos2) if !is_in_trait => return Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Eq, _) => Some(self.parse_expr()?),
                    (_, pos2) if !is_in_trait => return Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        None
                    },
                };
                Ok(Rc::new(RefCell::new(Var::Var(var_modifier, type_expr, where_pairs, expr, None, None))))
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_def(&mut self) -> FrontendResult<Box<Def>>
    {
        let first_pos = self.lexer.pos().clone();
        let modifiers = self.parse_modifiers()?;
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::Builtin, _) => {
                // "builtin", "type", con_ident
                // "builtin", ( con_ident | var_ident )
                // "builtin", "impl", con_ident, "for", type_name
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Type, _) => {
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::ConIdent(ident), _) => {
                                match modifiers.var_modifier_pair {
                                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("built-in type mustn't have variable modifier"))),
                                    None => (),
                                }
                                match modifiers.fun_modifier_pair {
                                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("built-in type mustn't have function modifier"))),
                                    None => (),
                                }
                                match modifiers.inline_modifier_pair {
                                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("built-in type mustn't have inline modifier"))),
                                    None => (),
                                }
                                Ok(Box::new(Def::Type(ident, Rc::new(RefCell::new(TypeVar::Builtin)), first_pos)))
                            },
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (token2 @ (Token::ConIdent(_) | Token::VarIdent(_)), _) => {
                        let ident = match token2 {
                            Token::ConIdent(tmp_ident) => tmp_ident,
                            Token::VarIdent(tmp_ident) => tmp_ident,
                            _ => return Err(FrontendError::Interal(String::from("no identifier"))),
                        };
                        match modifiers.var_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("built-in variable mustn't have variable modifier"))),
                            None => (),
                        }
                        match modifiers.fun_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("built-in variable mustn't have function modifier"))),
                            None => (),
                        }
                        match modifiers.inline_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("built-in variable mustn't have inline modifier"))),
                            None => (),
                        }
                        Ok(Box::new(Def::Var(ident, Rc::new(RefCell::new(Var::Builtin(None))), first_pos)))
                    },
                    (Token::Impl, _) => {
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::ConIdent(ident), _) => {
                                match modifiers.var_modifier_pair {
                                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait variable mustn't have variable modifier"))),
                                    None => (),
                                }
                                match modifiers.fun_modifier_pair {
                                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait mustn't have function modifier"))),
                                    None => (),
                                }
                                match modifiers.inline_modifier_pair {
                                    Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait mustn't have inline modifier"))),
                                    None => (),
                                }
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected end of file"))),
                                    (Token::For, _) => {
                                        let type_name = self.parse_type_name()?;
                                        Ok(Box::new(Def::Impl(Rc::new(RefCell::new(Impl::Builtin(ident, type_name, None))), first_pos)))
                                    },
                                    (_, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected token"))),
                                }
                            },
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (Token::Data, _) => {
                // "data", con_ident, [ "<", one_or_more_type_args, ">" ], "=", cons
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::ConIdent(ident), _) => { 
                        match modifiers.var_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("type variable mustn't have variable modifier"))),
                            None => (),
                        }
                        match modifiers.fun_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("type mustn't have function modifier"))),
                            None => (),
                        }
                        match modifiers.inline_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("type mustn't have inline modifier"))),
                            None => (),
                        }
                        let saved_single_greater_flag = self.lexer.has_single_greater();
                        self.lexer.set_single_greater(true);
                        let type_args = match self.lexer.next_token()? {
                            (Token::Lt, _) => {
                                let tmp_type_args = self.parse_one_or_more_type_args(&[Token::Gt])?;
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                    (Token::Gt, _) => tmp_type_args,
                                    (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unclosed angle bracket"))),
                                }
                            },
                            (token2, pos2) => {
                                self.lexer.undo_token(token2, pos2);
                                Vec::new()
                            },
                        };
                        self.lexer.set_single_greater(saved_single_greater_flag);
                        match self.lexer.next_token()? {
                            (Token::Eof, pos2) => return Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                            (Token::Eq, _) => {
                                let cons = self.parse_one_or_more_cons(ident.as_str())?;
                                Ok(Box::new(Def::Type(ident, Rc::new(RefCell::new(TypeVar::Data(type_args, cons, None))), first_pos)))
                            },
                            (_, pos2) => return Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (Token::Type, _) => {
                // "type", con_ident, [ "<", one_or_more_type_args, ">" ], "=", type_expr
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::ConIdent(ident), _) => {
                        match modifiers.var_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("type synonym variable mustn't have variable modifier"))),
                            None => (),
                        }
                        match modifiers.fun_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("type synonym mustn't have function modifier"))),
                            None => (),
                        }
                        match modifiers.inline_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("type synonym mustn't have inline modifier"))),
                            None => (),
                        }
                        let saved_single_greater_flag = self.lexer.has_single_greater();
                        self.lexer.set_single_greater(true);
                        let type_args = match self.lexer.next_token()? {
                            (Token::Lt, _) => {
                                let tmp_type_args = self.parse_one_or_more_type_args(&[Token::Gt])?;
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                    (Token::Gt, _) => tmp_type_args,
                                    (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unclosed angle bracket"))),
                                }
                            },
                            (token2, pos2) => {
                                self.lexer.undo_token(token2, pos2);
                                Vec::new()
                            },
                        };
                        self.lexer.set_single_greater(saved_single_greater_flag);
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::Eq, _) => {
                                let type_expr = self.parse_type_expr()?;
                                Ok(Box::new(Def::Type(ident, Rc::new(RefCell::new(TypeVar::Synonym(type_args, type_expr))), first_pos)))
                            },
                            (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (token @ (Token::ConIdent(_) | Token::VarIdent(_)), _) => {
                // ( con_ident | var_ident ), var
                let ident = match token {
                    Token::ConIdent(tmp_ident) => tmp_ident,
                    Token::VarIdent(tmp_ident) => tmp_ident,
                    _ => return Err(FrontendError::Interal(String::from("no identifier"))),
                };
                Ok(Box::new(Def::Var(ident, self.parse_var(&modifiers, false)?, first_pos)))
            },
            (Token::Trait, _) => {
                // "trait", con_ident, [ "<," one_or_more_type_args, ">" ]
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::ConIdent(ident), _) => {
                        match modifiers.var_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait variable mustn't have variable modifier"))),
                            None => (),
                        }
                        match modifiers.fun_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait mustn't have function modifier"))),
                            None => (),
                        }
                        match modifiers.inline_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait mustn't have inline modifier"))),
                            None => (),
                        }
                        let saved_single_greater_flag = self.lexer.has_single_greater();
                        self.lexer.set_single_greater(true);
                        let type_args = match self.lexer.next_token()? {
                            (Token::Lt, _) => {
                                let tmp_type_args = self.parse_one_or_more_type_args(&[Token::Gt])?;
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                    (Token::Gt, _) => tmp_type_args,
                                    (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unclosed angle bracket"))),
                                }
                            },
                            (token2, pos2) => {
                                self.lexer.undo_token(token2, pos2);
                                Vec::new()
                            },
                        };
                        self.lexer.set_single_greater(saved_single_greater_flag);
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::LBrace, _) => {
                                let trait_defs = self.parse_trait_defs(&[Token::RBrace])?;
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos4) => return Err(FrontendError::Message(pos4, String::from("unexpected end of file"))),
                                    (Token::RBrace, _) => Ok(Box::new(Def::Trait(ident, Rc::new(RefCell::new(Trait(type_args, trait_defs, None))), first_pos))),
                                    (_, pos4) => return Err(FrontendError::Message(pos4, String::from("unexpected token"))),
                                }
                            },
                            (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (Token::Impl, _) => {
                // "impl", con_ident, "for", type_name, "{", impl_defs, "}"
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::ConIdent(ident), _) => {
                        match modifiers.var_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait variable mustn't have variable modifier"))),
                            None => (),
                        }
                        match modifiers.fun_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait mustn't have function modifier"))),
                            None => (),
                        }
                        match modifiers.inline_modifier_pair {
                            Some((_, tmp_pos)) => return Err(FrontendError::Message(tmp_pos, String::from("trait mustn't have inline modifier"))),
                            None => (),
                        }
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::For, _) => {
                                let type_name = self.parse_type_name()?;
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected end of file"))),
                                    (Token::LBrace, _) => {
                                        let impl_defs = self.parse_impl_defs(&[Token::RBrace])?;
                                        match self.lexer.next_token()? {
                                            (Token::Eof, pos5) => Err(FrontendError::Message(pos5, String::from("unexpected end of file"))),
                                            (Token::RBrace, _) => Ok(Box::new(Def::Impl(Rc::new(RefCell::new(Impl::Impl(ident, type_name, impl_defs, None))), first_pos))),
                                            (_, pos5) => Err(FrontendError::Message(pos5, String::from("unclosed brace"))),
                                        }
                                    },
                                    (_, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected token"))),
                                }
                            },
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_defs(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Box<Def>>>
    { self.parse_zero_or_more(&Token::Semi, end_tokens, Self::parse_def) }
        
    fn parse_type_arg(&mut self) -> FrontendResult<TypeArg>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), pos) => {
                // var_ident
                Ok(TypeArg(ident, pos))
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_one_or_more_type_args(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<TypeArg>>
    { self.parse_one_or_more(&Token::Comma, end_tokens, Self::parse_type_arg) }

    fn parse_con(&mut self, data_ident: String) -> FrontendResult<Rc<RefCell<Con>>>
    {
        match self.lexer.next_token()?{
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::ConIdent(ident), pos) => {
                // con_ident, "(", type_exprs, ")"
                // con_ident, "{", type_expr_named_field_pairs, "}"
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::LParen, _) => {
                        let type_exprs = self.parse_type_exprs(&[Token::RParen])?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RParen, _) => Ok(Rc::new(RefCell::new(Con::UnnamedField(ident, type_exprs, data_ident, pos)))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed parenthesis"))),
                        }
                    },
                    (Token::LBrace, _) => {
                        let type_expr_named_field_pairs = self.parse_named_field_pairs(&[Token::RBrace], Self::parse_type_expr)?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RBrace, _) => Ok(Rc::new(RefCell::new(Con::NamedField(ident, type_expr_named_field_pairs, data_ident, pos)))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed brace"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_one_or_more_cons(&mut self, data_ident: &str) -> FrontendResult<Vec<Rc<RefCell<Con>>>>
    { self.parse_one_or_more_without_end_sep(&Token::Bar, |parser| parser.parse_con(String::from(data_ident))) }

    fn parse_named_field_pair_with_fun_ref<T, F>(&mut self, f: &mut F) -> FrontendResult<NamedFieldPair<T>>
        where F: FnMut(&mut Self) -> FrontendResult<Box<T>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), pos) => {
                // var_ident, ":", other
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Colon, _) => Ok(NamedFieldPair(ident, f(self)?, pos)),
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_named_field_pairs<T, F>(&mut self, end_tokens: &[Token], mut f: F) -> FrontendResult<Vec<NamedFieldPair<T>>>
        where F: FnMut(&mut Self) -> FrontendResult<Box<T>>
    { self.parse_one_or_more(&Token::Comma, end_tokens, |parser| parser.parse_named_field_pair_with_fun_ref(&mut f)) }
    
    fn parse_type_expr1(&mut self) -> FrontendResult<Box<TypeExpr>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::LParen, pos) => {
                // "(", type_expr, ")"
                // "(", type_exprs, ")"
                // "(", type_exprs, ")", "->", type_expr1
                let mut type_exprs = self.parse_type_exprs(&[Token::RParen])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RParen, _) => {
                        match self.lexer.next_token()? {
                            (Token::RArrow, _) => {
                                let ret_type_expr = self.parse_type_expr1()?;
                                Ok(Box::new(TypeExpr::Fun(type_exprs, ret_type_expr, pos)))
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
                // "[", type_expr, ";", usize, "]"
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
            (Token::VarIdent(ident), pos) => {
                // var_ident
                Ok(Box::new(TypeExpr::Param(ident, pos)))
            },
            (Token::ConIdent(ident), pos) => {
                // con_ident
                // con_ident, "<", one_or_more_type_exprs, ">"
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
            (Token::Uniq, pos) => {
                // "uniq", type_expr1
                Ok(Box::new(TypeExpr::Uniq(self.parse_type_expr1()?, pos)))
            },
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

    fn parse_arg(&mut self) -> FrontendResult<Arg>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), pos) => {
                // var_ident, ":", type_expr
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Colon, _) => Ok(Arg(ident, self.parse_type_expr()?, None, pos)),
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_args(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Arg>>
    { self.parse_one_or_more(&Token::Comma, end_tokens, Self::parse_arg) }

    fn parse_where_pair(&mut self) -> FrontendResult<WherePair>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), pos) => {
                // var_ident, ":", trait_expr
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Colon, _) => Ok(WherePair(ident, self.parse_one_or_more_trait_exprs()?, pos)),
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_one_or_more_where_pairs(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<WherePair>>
    { self.parse_one_or_more(&Token::Comma, end_tokens, Self::parse_where_pair) }

    fn parse_trait_expr1(&mut self) -> FrontendResult<Box<TraitExpr>>
    {
        match self.lexer.next_token()? {
            (Token::Shared, pos) => {
                // "shared"
                Ok(Box::new(TraitExpr::Shared(pos)))
            },
            (Token::LParen, pos) => {
                // "(", type_exprs, ")", "->", type_expr
                let type_exprs = self.parse_type_exprs(&[Token::RParen])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RParen, _) => {
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RArrow, _) => {
                                let ret_type_expr = self.parse_type_expr()?;
                                Ok(Box::new(TraitExpr::Fun(type_exprs, ret_type_expr, pos)))
                            },
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                }
            },
            (Token::ConIdent(ident), pos) => {
                // con_ident, [ "<", one_or_more_type_expr, ">" ]
                match self.lexer.next_token()? {
                    (Token::Lt, _) => {
                        let type_exprs = self.parse_one_or_more_type_exprs(&[Token::Gt])?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::Gt, _) => Ok(Box::new(TraitExpr::Trait(ident, type_exprs, pos))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed angle bracket"))),
                        }
                    },
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        Ok(Box::new(TraitExpr::Trait(ident, Vec::new(), pos)))
                    },
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }

    fn parse_trait_expr(&mut self) -> FrontendResult<Box<TraitExpr>>
    {
        let saved_single_greater_flag = self.lexer.has_single_greater();
        self.lexer.set_single_greater(true);
        let res = self.parse_trait_expr1();
        self.lexer.set_single_greater(saved_single_greater_flag);
        res
    }
    
    fn parse_one_or_more_trait_exprs(&mut self) -> FrontendResult<Vec<Box<TraitExpr>>>
    { self.parse_one_or_more_without_end_sep(&Token::Plus, Self::parse_trait_expr) }
    
    fn parse_expr13(&mut self) -> FrontendResult<Box<Expr>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::Bar, pos) => {
                // "|", lambda_args, "|", [ "->", type_expr ], expr1 
                let args = self.parse_lambda_args(&[Token::Bar])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => return Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Bar, _) => {
                        let type_expr = match self.lexer.next_token()? {
                            (Token::RArrow, _) => Some(self.parse_type_expr()?),
                            (token3, pos3) => {
                                self.lexer.undo_token(token3, pos3);
                                None
                            },
                        };
                        let expr = self.parse_expr1()?;
                        Ok(Box::new(Expr::Lambda(args, type_expr, expr, None, pos)))
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed bar bracket"))),
                }
            },
            (Token::ConIdent(ident), pos) => {
                // con_ident
                // con_ident, "{", expr_named_field_pairs, "}"
                match self.lexer.next_token()? {
                    (Token::LBrace, _) => {
                        let expr_named_field_pairs = self.parse_named_field_pairs(&[Token::RBrace], Self::parse_expr)?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RBrace, _) => Ok(Box::new(Expr::NamedFieldConApp(ident, expr_named_field_pairs, None, pos))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed brace"))),
                        }
                    },
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        Ok(Box::new(Expr::Var(ident, None, pos)))
                    },
                }
            },
            (Token::VarIdent(ident), pos) => {
                // var_ident
                Ok(Box::new(Expr::Var(ident, None, pos)))
            },
            (Token::Printf, pos) => {
                // "printf", "(", exprs, ")"
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::LParen, _) => {
                        let exprs = self.parse_exprs(&[Token::RParen])?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RParen, _) => Ok(Box::new(Expr::PrintfApp(exprs, None, pos))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed parenthesis"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (token, pos) => {
                // expr_literal
                self.lexer.undo_token(token, pos.clone());
                match self.parse_literal_either(false, Self::parse_expr)? {
                    LiteralEither::Literal(literal) => Ok(Box::new(Expr::Literal(literal, None, pos))),
                    LiteralEither::Other(expr) => Ok(expr),
                }
            },
        }
    }

    fn parse_expr12(&mut self, is_getter: bool) -> FrontendResult<Box<Expr>>
    {
        let first_pos = self.lexer.pos().clone();
        let mut expr1: Box<Expr>;
        let mut idx_expr: Option<Box<Expr>> = None;
        let mut fields: Option<Vec<Field>> = None;
        let mut is_access_fun = false;
        match self.lexer.next_token()? {
            (Token::Star, _) => {
                // "*", expr12
                expr1 = self.parse_expr12(true)?;
                is_access_fun = true;
            },
            (token, pos) => {
                // expr12, "(", exprs, ")"
                // expr12, "[", expr1, "]"
                self.lexer.undo_token(token, pos);
                expr1 = self.parse_expr13()?;
                loop {
                    match self.lexer.next_token()? {
                        (Token::LParen, _) => {
                            if is_access_fun {
                                expr1 = match (fields, idx_expr) {
                                    (Some(fields), _) => Box::new(Expr::GetField(expr1, fields, None, first_pos.clone())),
                                    (_, Some(idx_expr)) => Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos.clone())),
                                    (_, _) => Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get"), None, first_pos.clone())), vec![expr1], None, first_pos.clone())),
                                };
                            }
                            idx_expr = None;
                            fields = None;
                            is_access_fun = false;
                            let exprs = self.parse_exprs(&[Token::RParen])?;
                            match self.lexer.next_token()? {
                                (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                (Token::RParen, _) => expr1 = Box::new(Expr::App(expr1, exprs, None, first_pos.clone())),
                                (_, pos2) => return Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                            }
                        },
                        (Token::LBracket, _) => {
                            if is_access_fun {
                                expr1 = match (fields, idx_expr) {
                                    (Some(fields), _) => Box::new(Expr::GetField(expr1, fields, None, first_pos.clone())),
                                    (_, Some(idx_expr)) => Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos.clone())),
                                    (_, _) => Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get"), None, first_pos.clone())), vec![expr1], None, first_pos.clone())),
                                };
                            }
                            idx_expr = Some(self.parse_expr1()?);
                            fields = None;
                            is_access_fun = true;
                            match self.lexer.next_token()? {
                                (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                (Token::RParen, _) => (),
                                (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unclosed bracket"))),
                            }
                        },
                        (Token::Dot, _) => {
                            if is_access_fun {
                                expr1 = match (fields, idx_expr) {
                                    (Some(fields), _) => Box::new(Expr::GetField(expr1, fields, None, first_pos.clone())),
                                    (_, Some(idx_expr)) => Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos.clone())),
                                    (_, _) => Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get"), None, first_pos.clone())), vec![expr1], None, first_pos.clone())),
                                }
                            }
                            idx_expr = None;
                            fields = Some(self.parse_one_or_more_fields()?);
                            is_access_fun = true;
                            match self.lexer.next_token()? {
                                (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                (Token::RParen, _) => (),
                                (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unclosed bracket"))),
                            }
                        },
                        (token2, pos2) => {
                            self.lexer.undo_token(token2, pos2);
                            break;
                        },
                    }
                }
            },
        }
        if is_access_fun {
            if !is_getter {
                let access_fun = match self.lexer.next_token()? {
                    (Token::RArrow, _) => {
                        // expr12, "->"
                        AccessFun::Get2
                    },
                    (Token::LArrow, _) => {
                        // expr12, "<-", expr12
                        AccessFun::Set(self.parse_expr12(true)?)
                    },
                    (Token::DArrow, _) => {
                        // expr12, "<->", expr12
                        // expr12, "<->", expr12, "<-"
                        let expr2 = self.parse_expr12(true)?;
                        match self.lexer.next_token()? {
                            (Token::RArrow, _) => AccessFun::UpdateGet2(expr2),
                            (token2, pos2) => {
                                self.lexer.undo_token(token2, pos2);
                                AccessFun::Update(expr2)
                            },
                        }
                    },
                    (token, pos) => {
                        // expr13
                        self.lexer.undo_token(token, pos);
                        AccessFun::Get
                    },
                };
                match (fields, idx_expr) {
                    (Some(fields), _) => {
                        match access_fun {
                            AccessFun::Get => Ok(Box::new(Expr::GetField(expr1, fields, None, first_pos))),
                            AccessFun::Get2 => Ok(Box::new(Expr::Get2Field(expr1, fields, None, first_pos))),
                            AccessFun::Set(expr2) => Ok(Box::new(Expr::SetField(expr1, fields, expr2, None, first_pos))),
                            AccessFun::Update(expr2) => Ok(Box::new(Expr::UpdateField(expr1, fields, expr2, None, first_pos))),
                            AccessFun::UpdateGet2(expr2) => Ok(Box::new(Expr::UpdateGet2Field(expr1, fields, expr2, None, first_pos))),
                        }
                    },
                    (_, Some(idx_expr)) => {
                        match access_fun {
                            AccessFun::Get => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos))),
                            AccessFun::Get2 => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("opt_get2_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos))),
                            AccessFun::Set(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_set_nth"), None, first_pos.clone())), vec![expr1, idx_expr, expr2], None, first_pos))),
                            AccessFun::Update(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_update_nth"), None, first_pos.clone())), vec![expr1, idx_expr, expr2], None, first_pos))),
                            AccessFun::UpdateGet2(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_update_get2_nth"), None, first_pos.clone())), vec![expr1, idx_expr, expr2], None, first_pos))),
                        }
                    },
                    (_, _) => {
                        match access_fun {
                            AccessFun::Get => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get"), None, first_pos.clone())), vec![expr1], None, first_pos))),
                            AccessFun::Get2 => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get2"), None, first_pos.clone())), vec![expr1], None, first_pos))),
                            AccessFun::Set(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_set"), None, first_pos.clone())), vec![expr1, expr2], None, first_pos))),
                            AccessFun::Update(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_update"), None, first_pos.clone())), vec![expr1, expr2], None, first_pos))),
                            AccessFun::UpdateGet2(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_update_get2"), None, first_pos.clone())), vec![expr1, expr2], None, first_pos))),
                        }
                    },
                }
            } else {
                match (fields, idx_expr) {
                    (Some(fields), _) => Ok(Box::new(Expr::GetField(expr1, fields, None, first_pos.clone()))),
                    (_, Some(idx_expr)) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos.clone()))),
                    (_, _) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_get"), None, first_pos.clone())), vec![expr1], None, first_pos.clone()))),
                }
            }
        } else {
            Ok(expr1)
        }
    }
        
    fn parse_expr11(&mut self) -> FrontendResult<Box<Expr>>
    {
        match self.lexer.next_token()? {
            (Token::Minus, pos) => {
                // "-", expr11
                Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_neg"), None, pos.clone())), vec![self.parse_expr11()?], None, pos)))
            },
            (Token::Ex, pos) => {
                // "!", expr11
                Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("op_not"), None, pos.clone())), vec![self.parse_expr11()?], None, pos)))
            },
            (token, pos) => {
                // expr12
                self.lexer.undo_token(token, pos);
                self.parse_expr12(false)
            },
        }
    }
    
    fn parse_expr10(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr11()?;
        loop {
            match self.lexer.next_token()? {
                (Token::Star, pos) => {
                    // expr10, "*", expr11
                    let expr2 = self.parse_expr11()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_mul"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::Slash, pos) => {
                    // expr10, "/", expr11
                    let expr2 = self.parse_expr11()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_div"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::Perc, pos) => {
                    // expr10, "%", expr11
                    let expr2 = self.parse_expr11()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_rem"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_expr9(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr10()?;
        loop {
            match self.lexer.next_token()? {
                (Token::Plus, pos) => {
                    // expr9, "+", expr10
                    let expr2 = self.parse_expr10()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_add"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::Minus, pos) => {
                    // expr9, "-", expr10
                    let expr2 = self.parse_expr10()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_sub"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_expr8(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr9()?;
        loop {
            match self.lexer.next_token()? {
                (Token::LtLt, pos) => {
                    // expr8, "<<", expr9
                    let expr2 = self.parse_expr9()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_shl"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::GtGt, pos) => {
                    // expr8, ">>", expr9
                    let expr2 = self.parse_expr9()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_shr"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_expr7(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr8()?;
        loop {
            match self.lexer.next_token()? {
                (Token::EqEq, pos) => {
                    // expr7, "==", expr8
                    let expr2 = self.parse_expr8()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_eq"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::ExEq, pos) => {
                    // expr7, "!=", expr8
                    let expr2 = self.parse_expr8()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_ne"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::Lt, pos) => {
                    // expr7, "<", expr8
                    let expr2 = self.parse_expr8()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_lt"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::GtEq, pos) => {
                    // expr7, ">=", expr8
                    let expr2 = self.parse_expr8()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_ge"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::Gt, pos) => {
                    // expr7, ">", expr8
                    let expr2 = self.parse_expr8()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_gt"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (Token::LtEq, pos) => {
                    // expr7, "<=", expr8
                    let expr2 = self.parse_expr8()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_le"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_expr6(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr7()?;
        loop {
            match self.lexer.next_token()? {
                (Token::Amp, pos) => {
                    // expr6, "&", expr7
                    let expr2 = self.parse_expr7()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_and"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_expr5(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr6()?;
        loop {
            match self.lexer.next_token()? {
                (Token::Caret, pos) => {
                    // expr5, "^", expr6
                    let expr2 = self.parse_expr6()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_xor"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_expr4(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr5()?;
        loop {
            match self.lexer.next_token()? {
                (Token::Bar, pos) => {
                    // expr4, "|", expr5
                    let expr2 = self.parse_expr5()?;
                    expr1 = Box::new(Expr::App(Box::new(Expr::Var(String::from("op_or"), None, pos.clone())), vec![expr1, expr2], None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }

    fn parse_expr3(&mut self) -> FrontendResult<Box<Expr>>
    {
        match self.lexer.next_token()? {
            (Token::Uniq, pos) => {
                // "uniq", expr3
                Ok(Box::new(Expr::Uniq(self.parse_expr3()?, None, pos)))
            },
            (Token::Shared, pos) => {
                // "shared", expr3
                Ok(Box::new(Expr::Shared(self.parse_expr3()?, None, pos)))
            },
            (token, pos) => {
                // expr4
                self.lexer.undo_token(token, pos);
                self.parse_expr4()
            },
        }
    }    
    
    fn parse_expr2(&mut self) -> FrontendResult<Box<Expr>>
    {
        let mut expr1 = self.parse_expr3()?;
        loop {
            match self.lexer.next_token()? {
                (Token::Colon, pos) => {
                    // expr2, ":", type_expr
                    let type_expr2 = self.parse_type_expr()?;
                    expr1 = Box::new(Expr::Typed(expr1, type_expr2, None, pos));
                },
                (Token::As, pos) => {
                    // expr2, "as", type_expr
                    let type_expr2 = self.parse_type_expr()?;
                    expr1 = Box::new(Expr::As(expr1, type_expr2, None, pos));
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(expr1)
    }
    
    fn parse_expr1(&mut self) -> FrontendResult<Box<Expr>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::Let, pos) => {
                // "let", binds, "in", expr1
                let binds = self.parse_one_or_more_binds(&[Token::In])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::In, _) => Ok(Box::new(Expr::Let(binds, self.parse_expr1()?, None, pos))),
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (Token::If, pos) => {
                // "if", expr1, "then", expr1, "else", expr1
                let expr1 = self.parse_expr1()?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Then, _) => {
                        let expr2 = self.parse_expr1()?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::Else, _) => {
                                let expr3 = self.parse_expr1()?;
                                Ok(Box::new(Expr::If(expr1, expr2, expr3, None, pos)))
                            },
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (token, pos) => {
                // expr1, { "match", "{", one_or_more_cases, "}" } 
                self.lexer.undo_token(token, pos.clone());
                let mut expr1 = self.parse_expr2()?;
                loop {
                    match self.lexer.next_token()? {
                        (Token::Match, _) => {
                            match self.lexer.next_token()? {
                                (Token::Eof, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                (Token::LBrace, _) => {
                                    let cases = self.parse_one_or_more_cases(&[Token::RBrace])?;
                                    match self.lexer.next_token()? {
                                        (Token::Eof, pos4) => return Err(FrontendError::Message(pos4, String::from("unexpected end of file"))),
                                        (Token::RBrace, _) => expr1 = Box::new(Expr::Match(expr1, cases, None, pos.clone())),
                                        (_, pos4) => return Err(FrontendError::Message(pos4, String::from("unclosed brace"))),
                                    }
                                },
                                (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                            }
                        },
                        (token2, pos2) => {
                            self.lexer.undo_token(token2, pos2);
                            break;
                        },
                    }
                }
                Ok(expr1)
            },
        }
    }
    
    fn parse_expr(&mut self) -> FrontendResult<Box<Expr>>
    { self.parse_expr1() }

    fn parse_exprs(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Box<Expr>>>
    { self.parse_zero_or_more(&Token::Comma, end_tokens, Self::parse_expr) }
    
    fn parse_field(&mut self) -> FrontendResult<Field>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), _) => {
                // var_ident
                Ok(Field::Named(ident))
            },
            (token, pos) => {
                // usize
                self.lexer.undo_token(token, pos);
                Ok(Field::Unnamed(self.parse_usize()?))
            },
        }
    }
    
    fn parse_one_or_more_fields(&mut self) -> FrontendResult<Vec<Field>>
    { self.parse_one_or_more_without_end_sep(&Token::Dot, Self::parse_field) }
    
    fn parse_bind(&mut self) -> FrontendResult<Bind>
    {
        let pattern = self.parse_pattern()?;
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::Eq, _) => {
                // pattern, "=", expr
                let expr = self.parse_expr()?;
                Ok(Bind(pattern, expr))
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_one_or_more_binds(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Bind>>
    { self.parse_one_or_more(&Token::Semi, end_tokens, Self::parse_bind) }

    fn parse_case(&mut self) -> FrontendResult<Case>
    {
        let pattern = self.parse_pattern()?;
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::EqGt, _) => {
                // pattern, "=>", expr
                let expr = self.parse_expr()?;
                Ok(Case(pattern, expr))
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_one_or_more_cases(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Case>>
    { self.parse_one_or_more(&Token::Semi, end_tokens, Self::parse_case) }

    fn parse_var_pattern2(&mut self, var_modifier: VarModifier) -> FrontendResult<Box<Pattern>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), pos) => {
                // var_ident
                // var_ident, "@", pattern2
                match self.lexer.next_token()? {
                    (Token::At, _) => Ok(Box::new(Pattern::At(var_modifier, ident, self.parse_pattern2()?, None, pos))),
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        Ok(Box::new(Pattern::Var(var_modifier, ident, None, pos)))
                    },
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_pattern2(&mut self) -> FrontendResult<Box<Pattern>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::ConIdent(ident), pos) => {
                // con_ident
                // con_ident, "(", patterns, ")"
                // con_ident, "{", pattern_named_field_pairs, "}"
                match self.lexer.next_token()? {
                    (Token::LParen, _) => {
                        let patterns = self.parse_patterns(&[Token::RParen])?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RParen, _) => Ok(Box::new(Pattern::UnnamedFieldCon(ident, patterns, None, pos))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed parenthesis"))),
                        }
                    },
                    (Token::LBrace, _) => {
                        let pattern_named_field_pairs = self.parse_named_field_pairs(&[Token::RBrace], Self::parse_pattern)?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RBrace, _) => Ok(Box::new(Pattern::NamedFieldCon(ident, pattern_named_field_pairs, None, pos))),
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed brace"))),
                        }
                    },
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        Ok(Box::new(Pattern::Const(ident, None, pos)))
                    },
                }
            },
            (Token::Private, _) => {
                // "private", var_pattern2
                self.parse_var_pattern2(VarModifier::Private)
            },
            (Token::Local, _) => {
                // "local", var_pattern2
                self.parse_var_pattern2(VarModifier::Local)
            },
            (Token::Global, _) => {
                // "global", var_pattern2
                self.parse_var_pattern2(VarModifier::Global)
            },
            (Token::Constant, _) => {
                // "constant", var_pattern2
                self.parse_var_pattern2(VarModifier::Constant)
            },
            (token @ Token::VarIdent(_), pos) => {
                // var_pattern2
                self.lexer.undo_token(token, pos);
                self.parse_var_pattern2(VarModifier::None)
            },
            (Token::Wildcard, pos) => {
                // "_"
                Ok(Box::new(Pattern::Wildcard(None, pos)))
            },
            (token, pos) => {
                // pattern_literal
                // pattern_literal, "as", type_expr
                self.lexer.undo_token(token, pos.clone());
                match self.parse_literal_either(false, Self::parse_pattern)? {
                    LiteralEither::Literal(literal) => {
                        match self.lexer.next_token()? {
                            (Token::As, _) => Ok(Box::new(Pattern::As(literal, self.parse_type_expr()?, None, pos))),
                            (token2, pos2) => {
                                self.lexer.undo_token(token2, pos2);
                                Ok(Box::new(Pattern::Literal(literal, None, pos)))
                            }
                        }
                    },
                    LiteralEither::Other(pattern) => Ok(pattern),
                }
            },
        }
    }
    
    fn parse_pattern1(&mut self) -> FrontendResult<Box<Pattern>>
    {
        let first_pos = self.lexer.pos().clone();
        let mut patterns: Vec<Box<Pattern>> = Vec::new();
        patterns.push(self.parse_pattern2()?);
        loop {
            match self.lexer.next_token()? {
                (Token::Bar, _) => {
                    // pattern1, "|", pattern2
                    patterns.push(self.parse_pattern2()?);
                },
                (token, pos) => {
                    self.lexer.undo_token(token, pos);
                    break;
                },
            }
        }
        Ok(Box::new(Pattern::Alt(patterns, None, first_pos)))
    }
    
    fn parse_pattern(&mut self) -> FrontendResult<Box<Pattern>>
    { self.parse_pattern1() }

    fn parse_patterns(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Box<Pattern>>>
    { self.parse_zero_or_more(&Token::Comma, end_tokens, Self::parse_pattern) }    

    fn parse_simple_literal(&mut self, is_unary_op: bool) -> FrontendResult<SimpleLiteral>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::Minus, pos) if is_unary_op => {
                // "-", simple_literal
                match self.parse_simple_literal(is_unary_op)? {
                    SimpleLiteral::Char(n) => Ok(SimpleLiteral::Char(n.overflowing_neg().0)),
                    SimpleLiteral::Int(n) => Ok(SimpleLiteral::Int(n.overflowing_neg().0)),
                    SimpleLiteral::Long(n) => Ok(SimpleLiteral::Long(n.overflowing_neg().0)),
                    SimpleLiteral::Float(n) => Ok(SimpleLiteral::Float(-n)),
                    SimpleLiteral::Double(n) => Ok(SimpleLiteral::Double(-n)),
                    _ =>  Err(FrontendError::Message(pos, String::from("illegal unary operarotor for literal type"))),
                }
            },
            (Token::Ex, pos) if is_unary_op => {
                // "!", simple_literal
                match self.parse_simple_literal(is_unary_op)? {
                    SimpleLiteral::Bool(b) => Ok(SimpleLiteral::Bool(!b)),
                    SimpleLiteral::Char(n) => Ok(SimpleLiteral::Char(!n)),
                    SimpleLiteral::Int(n) => Ok(SimpleLiteral::Int(!n)),
                    SimpleLiteral::Long(n) => Ok(SimpleLiteral::Long(!n)),
                    SimpleLiteral::Uint(n) => Ok(SimpleLiteral::Uint(!n)),
                    SimpleLiteral::Ulong(n) => Ok(SimpleLiteral::Ulong(!n)),
                    _ =>  Err(FrontendError::Message(pos, String::from("illegal unary operarotor for literal type"))),
                }
            },
            (Token::False, _) => {
                // "false"
                Ok(SimpleLiteral::Bool(false))
            },
            (Token::True, _) => {
                // "true"
                Ok(SimpleLiteral::Bool(true))
            },
            (Token::Char(n), _) => {
                // char
                Ok(SimpleLiteral::Char(n))
            },
            (Token::Int(n), _) => {
                // int
                Ok(SimpleLiteral::Int(n))
            },
            (Token::Long(n), _) => {
                // long
                Ok(SimpleLiteral::Long(n))
            },
            (Token::Uint(n), _) => {
                // uint
                Ok(SimpleLiteral::Uint(n))
            },
            (Token::Ulong(n), _) => {
                // ulong
                Ok(SimpleLiteral::Ulong(n))
            },
            (Token::Float(n), _) => {
                // float
                Ok(SimpleLiteral::Float(n))
            },
            (Token::Double(n), _) => {
                // double
                Ok(SimpleLiteral::Double(n))
            },
            (_, pos) =>  Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_literal_either<T, F>(&mut self, is_unary_op: bool, f: F) -> FrontendResult<LiteralEither<T>>
        where F: FnMut(&mut Self) -> FrontendResult<Box<T>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::String(bs), _) => {
                // string
                Ok(LiteralEither::Literal(Box::new(Literal::String(bs))))
            },
            (Token::LParen, _) => {
                // "(", other, ")"
                // "(", others, ")"
                let mut others = self.parse_zero_or_more(&Token::Comma, &[Token::RParen], f)?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RParen, _) => {
                        if others.len() == 1 {
                            Ok(LiteralEither::Other(others.remove(0)))
                        } else {
                            Ok(LiteralEither::Literal(Box::new(Literal::Tuple(others))))
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                }
            },
            (Token::LBracket, _) => {
                // "[", others, "]"
                // "[", other, ";", usize, "]"
                let mut others = self.parse_zero_or_more(&Token::Comma, &[Token::RBracket, Token::Semi], f)?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RBracket, _) => Ok(LiteralEither::Literal(Box::new(Literal::Array(others)))),
                    (Token::Semi, pos2) => {
                        if others.len() == 1 {
                            let len = self.parse_usize()?;
                            match self.lexer.next_token()? {
                                (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                                (Token::RBracket, _) => Ok(LiteralEither::Literal(Box::new(Literal::FilledArray(others.remove(0), len)))),
                                (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed bracket"))),
                            }
                        } else {
                            Err(FrontendError::Message(pos2, String::from("must be one element for filled array")))
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed bracket"))),
                }
            },
            (token, pos) => {
                // simple_literal
                self.lexer.undo_token(token, pos);
                match self.parse_simple_literal(is_unary_op)? {
                    SimpleLiteral::Bool(b) => Ok(LiteralEither::Literal(Box::new(Literal::Bool(b)))),
                    SimpleLiteral::Char(n) => Ok(LiteralEither::Literal(Box::new(Literal::Char(n)))),
                    SimpleLiteral::Int(n) => Ok(LiteralEither::Literal(Box::new(Literal::Int(n)))),
                    SimpleLiteral::Long(n) => Ok(LiteralEither::Literal(Box::new(Literal::Long(n)))),
                    SimpleLiteral::Uint(n) => Ok(LiteralEither::Literal(Box::new(Literal::Uint(n)))),
                    SimpleLiteral::Ulong(n) => Ok(LiteralEither::Literal(Box::new(Literal::Ulong(n)))),
                    SimpleLiteral::Float(n) => Ok(LiteralEither::Literal(Box::new(Literal::Float(n)))),
                    SimpleLiteral::Double(n) => Ok(LiteralEither::Literal(Box::new(Literal::Double(n)))),
                }
            },
        }
    }

    fn parse_lambda_arg(&mut self) -> FrontendResult<LambdaArg>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), pos) => {
                // var_ident, [ ":", type_expr ]
                match self.lexer.next_token()? {
                    (Token::Colon, _) => Ok(LambdaArg(ident, Some(self.parse_type_expr()?), None, pos)),
                    (token2, pos2) => {
                        self.lexer.undo_token(token2, pos2);
                        Ok(LambdaArg(ident, None, None, pos))
                    },
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_lambda_args(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<LambdaArg>>
    { self.parse_zero_or_more(&Token::Comma, end_tokens, Self::parse_lambda_arg) }

    fn parse_trait_def(&mut self) -> FrontendResult<Box<TraitDef>>
    {
        let first_pos = self.lexer.pos().clone();
        let modifiers = self.parse_modifiers()?;
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (token @ (Token::ConIdent(_) | Token::VarIdent(_)), _) => {
                // ( con_ident | var_ident ), var
                let ident = match token {
                    Token::ConIdent(tmp_ident) => tmp_ident,
                    Token::VarIdent(tmp_ident) => tmp_ident,
                    _ => return Err(FrontendError::Interal(String::from("no identifier"))),
                };
                Ok(Box::new(TraitDef(ident, self.parse_var(&modifiers, true)?, first_pos)))
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }

    fn parse_trait_defs(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Box<TraitDef>>>
    { self.parse_zero_or_more(&Token::Semi, end_tokens, Self::parse_trait_def) }

    fn parse_wildcards(&mut self, end_tokens: &[Token]) -> FrontendResult<usize>
    {
        let mut count = 0usize;
        loop {
            match self.lexer.next_token()? {
                (token, _) if end_tokens.iter().any(|t| t == &token) => break,
                (Token::Eof, pos) => return Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
                (Token::Wildcard, _) => {
                    count += 1;
                    match self.lexer.next_token()? {
                        (Token::Comma, _) => (),
                        (token2, pos2) => {
                            self.lexer.undo_token(token2, pos2);
                            break;
                        },
                    }
                }
                (_, pos) => return Err(FrontendError::Message(pos, String::from("unexpected token"))),
            }
        }
        Ok(count)
    }
    
    fn parse_type_name(&mut self) -> FrontendResult<TypeName>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::LParen, _) => {
                // "(", wildcards, ")"
                // "(", wildcards, ")", "->", "_"
                let count = self.parse_wildcards(&[Token::RParen])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RParen, _) => {
                        match self.lexer.next_token()? {
                            (Token::RArrow, _) => {
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected end of file"))),
                                    (Token::Wildcard, _) => Ok(TypeName::Fun(count)),
                                    (_, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected token"))),
                                }
                            },
                            (token3, pos3) => {
                                self.lexer.undo_token(token3, pos3);
                                Ok(TypeName::Tuple(count))
                            },
                        }
                    }
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                }
            },
            (Token::LBracket, _) => {
                // "[", "_", ";", ( usize | "_" ), "]" 
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::Wildcard, _) => {
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::Semi, _) => {
                                let len = match self.lexer.next_token()? {
                                    (Token::Wildcard, _) => None,
                                    (token4, pos4) => {
                                        self.lexer.undo_token(token4, pos4);
                                        Some(self.parse_usize()?)
                                    },
                                };
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected end of file"))),
                                    (Token::RBracket, _) => Ok(TypeName::Array(len)),
                                    (_, pos4) => Err(FrontendError::Message(pos4, String::from("unclosed bracket"))),
                                }
                            },
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected token"))),
                        }
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (Token::ConIdent(ident), _) => {
                // con_ident
                Ok(TypeName::Name(ident))
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_impl_def(&mut self) -> FrontendResult<Box<ImplDef>>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::Builtin, pos) => {
                // "builtin", ( con_ident | var_ident )
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (token @ (Token::ConIdent(_) | Token::VarIdent(_)), _) => {
                        let ident = match token {
                            Token::ConIdent(tmp_ident) => tmp_ident,
                            Token::VarIdent(tmp_ident) => tmp_ident,
                            _ => return Err(FrontendError::Interal(String::from("no identifier"))),
                        };
                        Ok(Box::new(ImplDef(ident, Rc::new(RefCell::new(ImplVar::Builtin(None))), pos)))
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            }
            (token @ (Token::ConIdent(_) | Token::VarIdent(_)), pos) => {
                // ( con_ident | var_ident ), "=", expr
                // ( con_ident | var_ident ), "(", impl_args, ")", "=", expr
                let ident = match token {
                    Token::ConIdent(tmp_ident) => tmp_ident,
                    Token::VarIdent(tmp_ident) => tmp_ident,
                    _ => return Err(FrontendError::Interal(String::from("no identifier"))),
                };
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::LParen, _) => {
                        let impl_args = self.parse_impl_args(&[Token::RParen])?;
                        match self.lexer.next_token()? {
                            (Token::Eof, pos3) => Err(FrontendError::Message(pos3, String::from("unexpected end of file"))),
                            (Token::RParen, _) => {
                                match self.lexer.next_token()? {
                                    (Token::Eof, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected end of file"))),
                                    (Token::Eq, _) => {
                                        let expr = self.parse_expr()?;
                                        Ok(Box::new(ImplDef(ident, Rc::new(RefCell::new(ImplVar::Fun(Box::new(ImplFun(impl_args, expr, None, None)), None))), pos)))
                                    },
                                    (_, pos4) => Err(FrontendError::Message(pos4, String::from("unexpected token"))),
                                }
                            },
                            (_, pos3) => Err(FrontendError::Message(pos3, String::from("unclosed parenthesis"))),
                        }
                    },
                    (Token::Eq, _) => {
                        let expr = self.parse_expr()?;
                        Ok(Box::new(ImplDef(ident, Rc::new(RefCell::new(ImplVar::Var(expr, None, None))), pos)))
                    },
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected token"))),
                }
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }

    fn parse_impl_defs(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<Box<ImplDef>>>
    { self.parse_zero_or_more(&Token::Semi, end_tokens, Self::parse_impl_def) }

    fn parse_impl_arg(&mut self) -> FrontendResult<ImplArg>
    {
        match self.lexer.next_token()? {
            (Token::Eof, pos) => Err(FrontendError::Message(pos, String::from("unexpected end of file"))),
            (Token::VarIdent(ident), pos) => {
                // var_ident
                Ok(ImplArg(ident, None, pos))
            },
            (_, pos) => Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }

    fn parse_impl_args(&mut self, end_tokens: &[Token]) -> FrontendResult<Vec<ImplArg>>
    { self.parse_zero_or_more(&Token::Comma, end_tokens, Self::parse_impl_arg) }
}
