/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

use crate::ast::*;
use crate::lexer::{Lexer, Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Result<Self, String> {
        let mut lexer = Lexer::new(input);
        lexer.skip_whitespace_and_comments_safe()?;
        let current = lexer.next_token()?;
        Ok(Self { lexer, current })
    }

    fn advance(&mut self) -> Result<(), String> {
        self.lexer.skip_whitespace_and_comments_safe()?;
        self.current = self.lexer.next_token()?;
        Ok(())
    }

    fn matches(&self, tok: &Token) -> bool {
        std::mem::discriminant(&self.current) == std::mem::discriminant(tok)
    }

    fn consume(&mut self, expected: Token) -> Result<(), String> {
        if self.matches(&expected) {
            self.advance()?;
            Ok(())
        } else {
            Err(format!("Expected token {:?}, found {:?}", expected, self.current))
        }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut elements = Vec::new();
        while !self.matches(&Token::EOF) {
            elements.push(self.parse_top_level()?);
        }
        Ok(Program { elements })
    }

    fn parse_top_level(&mut self) -> Result<TopLevel, String> {
        if self.matches(&Token::Colon) {
            self.advance()?;
            if let Token::StringLit(s) = &self.current {
                let path = s.clone();
                self.advance()?;
                self.consume(Token::Semi)?;
                return Ok(TopLevel::Import(path));
            } else if let Token::Ident(id) = &self.current {
                let mut path = vec![id.clone()];
                self.advance()?;
                while self.matches(&Token::Dot) {
                    self.advance()?;
                    if let Token::Ident(next_id) = &self.current {
                        path.push(next_id.clone());
                        self.advance()?;
                    } else {
                        return Err("Expected identifier after '.' in macro".into());
                    }
                }
                let mut expr = None;
                if !self.matches(&Token::Semi) {
                    expr = Some(self.parse_expr()?);
                }
                self.consume(Token::Semi)?;
                return Ok(TopLevel::Macro { path, expr });
            }
        }
        
        let atom = self.parse_atom()?;
        Ok(TopLevel::Atom(atom))
    }

    fn parse_atom(&mut self) -> Result<Atom, String> {
        let core = self.parse_atom_core()?;
        let mut definition = None;
        if self.matches(&Token::Colon) || self.matches(&Token::Defines) {
            self.advance()?;
            definition = Some(self.parse_expr()?);
        }
        if self.matches(&Token::Semi) {
            self.consume(Token::Semi)?;
        }
        Ok(Atom { core, definition })
    }

    fn parse_atom_core(&mut self) -> Result<AtomCore, String> {
        let head = if let Some(sigil) = self.parse_sigil() {
            let ident = if let Token::Ident(id) = &self.current {
                let name = id.clone();
                self.advance()?;
                Some(name)
            } else {
                None
            };
            let mut path = Vec::new();
            while self.matches(&Token::Dot) {
                self.advance()?;
                if let Token::Ident(next_id) = &self.current {
                    path.push(next_id.clone());
                    self.advance()?;
                } else {
                    return Err("Expected identifier in path".into());
                }
            }
            AtomCoreHead::Sigil(SigilNs { sigil, ident, path })
        } else if let Token::Ident(id) = &self.current {
            let name = id.clone();
            self.advance()?;
            AtomCoreHead::Ident(name)
        } else {
            return Err(format!("Expected Sigil or Ident at atom core, found {:?}", self.current));
        };

        let mut invocations = Vec::new();
        loop {
            if let Some(kind) = self.get_group_kind() {
                self.advance()?;
                let mut inner = None;
                if !self.matches(&self.get_closing_token(kind)) {
                    inner = Some(self.parse_expr()?);
                }
                self.consume(self.get_closing_token(kind))?;
                invocations.push(Invocation::Grouping(kind, inner.map(Box::new)));
            } else if self.matches(&Token::Dot) {
                self.advance()?;
                if let Token::Ident(id) = &self.current {
                    let name = id.clone();
                    self.advance()?;
                    let mut args = None;
                    if self.get_group_kind().is_some() {
                        let kind = self.get_group_kind().unwrap();
                        self.advance()?;
                        if !self.matches(&self.get_closing_token(kind)) {
                            args = Some(Box::new(self.parse_expr()?));
                        }
                        self.consume(self.get_closing_token(kind))?;
                    }
                    invocations.push(Invocation::MethodCall { op: ".".into(), ident: name, args });
                } else {
                    return Err("Expected identifier after '.' in method call".into());
                }
            } else if let Some(sigil) = self.parse_sigil() {
                if let Token::Ident(id) = &self.current {
                    let name = id.clone();
                    self.advance()?;
                    let mut args = None;
                    if self.get_group_kind().is_some() {
                        let kind = self.get_group_kind().unwrap();
                        self.advance()?;
                        if !self.matches(&self.get_closing_token(kind)) {
                            args = Some(Box::new(self.parse_expr()?));
                        }
                        self.consume(self.get_closing_token(kind))?;
                    }
                    let op = match sigil {
                        Sigil::Type => "&",
                        Sigil::Function => "%",
                        Sigil::Variable => "$",
                        Sigil::Identity => "~",
                        Sigil::Category => "#",
                        Sigil::Symbol => "@",
                        Sigil::Pointer => "^",
                        Sigil::Rule => "?",
                        Sigil::Predicate => "!",
                        Sigil::Generic => "_",
                    };
                    invocations.push(Invocation::MethodCall { op: op.into(), ident: name, args });
                } else {
                    return Err("Expected identifier after Sigil in method call".into());
                }
            } else {
                break;
            }
        }

        let mut templates = Vec::new();
        while self.matches(&Token::DblColon) {
            self.advance()?;
            templates.push(self.parse_expr()?);
        }

        Ok(AtomCore { head, invocations, templates })
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_pipe_level()
    }

    fn parse_pipe_level(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_semi_level()?;
        while self.matches(&Token::Pipe) {
            self.advance()?;
            let rhs = self.parse_semi_level()?;
            expr = Expr::Binary { op: "|".into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_semi_level(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_colon_level()?;
        while self.matches(&Token::Semi) {
            self.advance()?;
            let rhs = self.parse_colon_level()?;
            expr = Expr::Binary { op: ";".into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_colon_level(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_dot_level()?;
        while self.matches(&Token::Colon) {
            self.advance()?;
            let rhs = self.parse_dot_level()?;
            expr = Expr::Binary { op: ":".into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_dot_level(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comma_level()?;
        while self.matches(&Token::Dot) {
            self.advance()?;
            let rhs = self.parse_comma_level()?;
            expr = Expr::Binary { op: ".".into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_comma_level(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_expression_level()?;
        while self.matches(&Token::Comma) {
            self.advance()?;
            let rhs = self.parse_expression_level()?;
            expr = Expr::Binary { op: ",".into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_expression_level(&mut self) -> Result<Expr, String> {
        let expr = self.parse_logical_or()?;
        if self.matches(&Token::Matches) {
            self.advance()?;
            let rhs = self.parse_logical_or()?;
            return Ok(Expr::Binary { op: "=>".into(), lhs: Box::new(expr), rhs: Box::new(rhs) });
        }
        Ok(expr)
    }

    fn parse_logical_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_logical_and()?;
        loop {
            let op = match &self.current {
                Token::Or => "or",
                Token::Xor => "xor",
                Token::Nor => "nor",
                Token::Xnor => "xnor",
                _ => break,
            };
            self.advance()?;
            let rhs = self.parse_logical_and()?;
            expr = Expr::Binary { op: op.into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_logical_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_math_expr()?;
        loop {
            let op = match &self.current {
                Token::And => "and",
                Token::Nand => "nand",
                _ => break,
            };
            self.advance()?;
            let rhs = self.parse_math_expr()?;
            expr = Expr::Binary { op: op.into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_math_expr(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_math_term()?;
        loop {
            let op = match &self.current {
                Token::Plus => "+",
                Token::Minus => "-",
                _ => break,
            };
            self.advance()?;
            let rhs = self.parse_math_term()?;
            expr = Expr::Binary { op: op.into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_math_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_math_factor()?;
        loop {
            let op = match &self.current {
                Token::Multiply => "*",
                Token::RightDiv => "/",
                Token::LeftDiv => "\\",
                _ => break,
            };
            self.advance()?;
            let rhs = self.parse_math_factor()?;
            expr = Expr::Binary { op: op.into(), lhs: Box::new(expr), rhs: Box::new(rhs) };
        }
        Ok(expr)
    }

    fn parse_math_factor(&mut self) -> Result<Expr, String> {
        let prefix_op = match &self.current {
            Token::Plus => Some("+"),
            Token::Minus => Some("-"),
            Token::Not => Some("not"),
            _ => None,
        };

        if let Some(op) = prefix_op {
            self.advance()?;
            let expr = self.parse_math_factor()?;
            return Ok(Expr::Unary { op: op.into(), expr: Box::new(expr) });
        }

        if self.matches(&Token::LParen) {
            // Might be grouping
            let _tail = self.lexer.skip_whitespace_and_comments_safe(); // Skip just in case
            // If the next token doesn't form an expression, check for groupings
        }

        if let Some(kind) = self.get_group_kind() {
            self.advance()?;
            let mut inner = None;
            if !self.matches(&self.get_closing_token(kind)) {
                inner = Some(self.parse_expr()?);
            }
            self.consume(self.get_closing_token(kind))?;
            return Ok(Expr::Grouping(kind, inner.map(Box::new)));
        }

        if let Token::StringLit(s) = &self.current {
            let lit = Literal::String(s.clone());
            self.advance()?;
            return Ok(Expr::Literal(lit));
        } else if let Token::Number(n) = &self.current {
            let lit = Literal::Number(n.clone());
            self.advance()?;
            return Ok(Expr::Literal(lit));
        } else if let Token::Boolean(b) = &self.current {
            let lit = Literal::Boolean(*b);
            self.advance()?;
            return Ok(Expr::Literal(lit));
        } else if let Token::Maybe(b) = &self.current {
            let lit = Literal::Maybe(*b);
            self.advance()?;
            return Ok(Expr::Literal(lit));
        } else if let Token::BarDot(bd) = &self.current {
            let lit = Literal::BarDot(bd.clone());
            self.advance()?;
            return Ok(Expr::Literal(lit));
        }

        // Otherwise it must be an AtomCore
        if let Ok(core) = self.parse_atom_core() {
            return Ok(Expr::AtomCore(core));
        }

        Ok(Expr::Empty)
    }

    fn parse_sigil(&mut self) -> Option<Sigil> {
        let sigil = match &self.current {
            Token::Type => Some(Sigil::Type),
            Token::Function => Some(Sigil::Function),
            Token::Variable => Some(Sigil::Variable),
            Token::Identity => Some(Sigil::Identity),
            Token::Category => Some(Sigil::Category),
            Token::Symbol => Some(Sigil::Symbol),
            Token::Pointer => Some(Sigil::Pointer),
            Token::Rule => Some(Sigil::Rule),
            Token::Predicate => Some(Sigil::Predicate),
            Token::Generic => Some(Sigil::Generic),
            _ => None,
        };
        if sigil.is_some() {
            let _ = self.advance();
        }
        sigil
    }

    fn get_group_kind(&self) -> Option<GroupKind> {
        match &self.current {
            Token::LParen => Some(GroupKind::Paren),
            Token::LBrack => Some(GroupKind::Bracket),
            Token::LBrace => Some(GroupKind::Brace),
            Token::LessThan => Some(GroupKind::Angle),
            _ => None,
        }
    }

    fn get_closing_token(&self, kind: GroupKind) -> Token {
        match kind {
            GroupKind::Paren => Token::RParen,
            GroupKind::Bracket => Token::RBrack,
            GroupKind::Brace => Token::RBrace,
            GroupKind::Angle => Token::GreaterThan,
        }
    }
}
