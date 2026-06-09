/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Sigil {
    Type,       // &
    Function,   // %
    Variable,   // $
    Identity,   // ~
    Category,   // #
    Symbol,     // @
    Pointer,    // ^
    Rule,       // ?
    Predicate,  // !
    Generic,    // _
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(String),
    Boolean(bool),
    Maybe(bool),
    BarDot(String),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum GroupKind {
    Paren,    // ( )
    Bracket,  // [ ]
    Brace,    // { }
    Angle,    // < >
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Binary {
        op: String,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
    Unary {
        op: String,
        expr: Box<Expr>,
    },
    Literal(Literal),
    Grouping(GroupKind, Option<Box<Expr>>),
    AtomCore(AtomCore),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SigilNs {
    pub sigil: Sigil,
    pub ident: Option<String>,
    pub path: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AtomCoreHead {
    Sigil(SigilNs),
    Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Invocation {
    Grouping(GroupKind, Option<Box<Expr>>),
    MethodCall {
        op: String, // "." or Sigil
        ident: String,
        args: Option<Box<Expr>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AtomCore {
    pub head: AtomCoreHead,
    pub invocations: Vec<Invocation>,
    pub templates: Vec<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub core: AtomCore,
    pub definition: Option<Expr>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopLevel {
    Atom(Atom),
    Macro {
        path: Vec<String>,
        expr: Option<Expr>,
    },
    Import(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub elements: Vec<TopLevel>,
}
