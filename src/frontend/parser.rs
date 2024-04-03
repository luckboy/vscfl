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

    fn parse_named_field_pairs<T, F>(&mut self, end_token: &[Token], mut f: F) -> FrontendResult<Vec<NamedFieldPair<T>>>
        where F: FnMut(&mut Self) -> FrontendResult<Box<T>>
    { Err(FrontendError::Message(self.lexer.pos().clone(), String::from("unexpected token"))) }
    
    fn parse_expr13(&mut self) -> FrontendResult<Box<Expr>>
    {
        match self.lexer.next_token()? {
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
                    (Token::LBrace, pos2) => {
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
                let exprs = self.parse_exprs(&[Token::RParen])?;
                match self.lexer.next_token()? {
                    (Token::Eof, pos2) => Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                    (Token::RParen, _) => Ok(Box::new(Expr::PrintfApp(exprs, None, pos))),
                    (_, pos2) => Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                }
            },
            (token, pos) => {
                self.lexer.undo_token(token, pos.clone());
                match self.parse_literal_either(false, Self::parse_expr)? {
                    LiteralEither::Literal(literal) => Ok(Box::new(Expr::Literal(literal, None, pos))),
                    LiteralEither::Other(expr) => Ok(expr),
                }
            },
        }
    }

    fn parse_expr12(&mut self) -> FrontendResult<Box<Expr>>
    {
        let first_pos = self.lexer.pos().clone();
        let mut expr1: Box<Expr>;
        let mut idx_expr: Option<Box<Expr>> = None;
        let mut fields: Option<Vec<Field>> = None;
        let mut is_access_fun = false;
        match self.lexer.next_token()? {
            (Token::Star, _) => {
                // "*", expr12
                expr1 = self.parse_expr12()?;
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
                                    (_, Some(idx_expr)) => Box::new(Expr::App(Box::new(Expr::Var(String::from("get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos.clone())),
                                    (_, _) => Box::new(Expr::App(Box::new(Expr::Var(String::from("get"), None, first_pos.clone())), vec![expr1], None, first_pos.clone())),
                                };
                            }
                            idx_expr = None;
                            fields = None;
                            is_access_fun = false;
                            let exprs = self.parse_exprs(&[Token::RParen])?;
                            match self.lexer.next_token()? {
                                (Token::Eof, pos2) => return Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                                (Token::RParen, _) => expr1 = Box::new(Expr::App(expr1, exprs, None, first_pos.clone())),
                                (_, pos2) => return Err(FrontendError::Message(pos2, String::from("unclosed parenthesis"))),
                            }
                        },
                        (Token::LBracket, _) => {
                            if is_access_fun {
                                expr1 = match (fields, idx_expr) {
                                    (Some(fields), _) => Box::new(Expr::GetField(expr1, fields, None, first_pos.clone())),
                                    (_, Some(idx_expr)) => Box::new(Expr::App(Box::new(Expr::Var(String::from("get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos.clone())),
                                    (_, _) => Box::new(Expr::App(Box::new(Expr::Var(String::from("get"), None, first_pos.clone())), vec![expr1], None, first_pos.clone())),
                                };
                            }
                            idx_expr = Some(self.parse_expr1()?);
                            fields = None;
                            is_access_fun = true;
                            match self.lexer.next_token()? {
                                (Token::Eof, pos2) => return Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
                                (Token::RParen, _) => (),
                                (_, pos3) => return Err(FrontendError::Message(pos3, String::from("unclosed bracket"))),
                            }
                        },
                        (Token::Dot, _) => {
                            if is_access_fun {
                                expr1 = match (fields, idx_expr) {
                                    (Some(fields), _) => Box::new(Expr::GetField(expr1, fields, None, first_pos.clone())),
                                    (_, Some(idx_expr)) => Box::new(Expr::App(Box::new(Expr::Var(String::from("get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos.clone())),
                                    (_, _) => Box::new(Expr::App(Box::new(Expr::Var(String::from("get"), None, first_pos.clone())), vec![expr1], None, first_pos.clone())),
                                }
                            }
                            idx_expr = None;
                            fields = Some(self.parse_one_or_more_fields()?);
                            is_access_fun = true;
                            match self.lexer.next_token()? {
                                (Token::Eof, pos2) => return Err(FrontendError::Message(pos2, String::from("unexpected end of file"))),
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
            let access_fun = match self.lexer.next_token()? {
                (Token::RArrow, _) => {
                    // expr12, "->"
                    AccessFun::Get2
                },
                (Token::LArrow, _) => {
                    // expr12, "<-" expr14
                    AccessFun::Set(self.parse_expr13()?)
                },
                (Token::DArrow, _) => {
                    // expr12, "<-> expr14
                    let expr2 = self.parse_expr13()?;
                    match self.lexer.next_token()? {
                        (Token::RArrow, _) => AccessFun::UpdateGet2(expr2),
                        (token2, pos2) => {
                            self.lexer.undo_token(token2, pos2);
                            AccessFun::Update(expr2)
                        },
                    }
                },
                (token, pos) => {
                    // expr14
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
                        AccessFun::Get => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("get_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos))),
                        AccessFun::Get2 => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("get2_nth"), None, first_pos.clone())), vec![expr1, idx_expr], None, first_pos))),
                        AccessFun::Set(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("set_nth"), None, first_pos.clone())), vec![expr1, idx_expr, expr2], None, first_pos))),
                        AccessFun::Update(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("update_nth"), None, first_pos.clone())), vec![expr1, idx_expr, expr2], None, first_pos))),
                        AccessFun::UpdateGet2(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("update_get2_nth"), None, first_pos.clone())), vec![expr1, idx_expr, expr2], None, first_pos))),
                    }
                },
                (_, _) => {
                    match access_fun {
                        AccessFun::Get => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("get"), None, first_pos.clone())), vec![expr1], None, first_pos))),
                        AccessFun::Get2 => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("get2"), None, first_pos.clone())), vec![expr1], None, first_pos))),
                        AccessFun::Set(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("set"), None, first_pos.clone())), vec![expr1, expr2], None, first_pos))),
                        AccessFun::Update(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("update"), None, first_pos.clone())), vec![expr1, expr2], None, first_pos))),
                        AccessFun::UpdateGet2(expr2) => Ok(Box::new(Expr::App(Box::new(Expr::Var(String::from("update_get2"), None, first_pos.clone())), vec![expr1, expr2], None, first_pos))),
                    }
                },
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
                self.parse_expr11()
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
                // expr2, { "match", "{", one_or_more_cases, "}" } 
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
    
    fn parse_one_or_more_fields(&mut self) -> FrontendResult<Vec<Field>>
    { Err(FrontendError::Message(self.lexer.pos().clone(), String::from("unexpected token"))) }
    
    fn parse_one_or_more_binds(&mut self, end_token: &[Token]) -> FrontendResult<Vec<Bind>>
    { Err(FrontendError::Message(self.lexer.pos().clone(), String::from("unexpected token"))) }

    fn parse_one_or_more_cases(&mut self, end_token: &[Token]) -> FrontendResult<Vec<Case>>
    { Err(FrontendError::Message(self.lexer.pos().clone(), String::from("unexpected token"))) }

    fn parse_lambda_args(&mut self, end_token: &[Token]) -> FrontendResult<Vec<LambdaArg>>
    { Err(FrontendError::Message(self.lexer.pos().clone(), String::from("unexpected token"))) }

    fn parse_simple_literal(&mut self, is_unary_op: bool) -> FrontendResult<SimpleLiteral>
    {
        match self.lexer.next_token()? {
            (Token::Minus, pos) if is_unary_op => {
                match self.parse_simple_literal(is_unary_op)? {
                    SimpleLiteral::Char(n) => Ok(SimpleLiteral::Char(-n)),
                    SimpleLiteral::Int(n) => Ok(SimpleLiteral::Int(-n)),
                    SimpleLiteral::Long(n) => Ok(SimpleLiteral::Long(-n)),
                    SimpleLiteral::Float(n) => Ok(SimpleLiteral::Float(-n)),
                    SimpleLiteral::Double(n) => Ok(SimpleLiteral::Double(-n)),
                    _ =>  Err(FrontendError::Message(pos, String::from("illegal unary operarotor for literal type"))),
                }
            },
            (Token::Ex, pos) if is_unary_op => {
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
            (Token::False, _) => Ok(SimpleLiteral::Bool(false)),
            (Token::True, _) => Ok(SimpleLiteral::Bool(true)),
            (Token::Char(n), _) => Ok(SimpleLiteral::Char(n)),
            (Token::Int(n), _) => Ok(SimpleLiteral::Int(n)),
            (Token::Long(n), _) => Ok(SimpleLiteral::Long(n)),
            (Token::Uint(n), _) => Ok(SimpleLiteral::Uint(n)),
            (Token::Ulong(n), _) => Ok(SimpleLiteral::Ulong(n)),
            (Token::Float(n), _) => Ok(SimpleLiteral::Float(n)),
            (Token::Double(n), _) => Ok(SimpleLiteral::Double(n)),
            (_, pos) =>  Err(FrontendError::Message(pos, String::from("unexpected token"))),
        }
    }
    
    fn parse_literal_either<T, F>(&mut self, is_unary_op: bool, f: F) -> FrontendResult<LiteralEither<T>>
        where F: FnMut(&mut Self) -> FrontendResult<Box<T>>
    {
        match self.lexer.next_token()? {
            (Token::String(bs), _) => Ok(LiteralEither::Literal(Box::new(Literal::String(bs)))),
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
}
