/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Sigils
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

    // Operators
    Plus,       // +
    Minus,      // -
    Multiply,   // *
    RightDiv,   // /
    LeftDiv,    // \
    Equals,     // =
    LessThan,   // <
    GreaterThan,// >

    // Logical Operators
    And,        // &&, "and"
    Or,         // ||, "or"
    Xor,        // "xor"
    Nand,       // "nand"
    Nor,        // "nor"
    Xnor,       // "xnor"
    Not,        // "not"

    // Brackets
    LParen,     // (
    RParen,     // )
    LBrack,     // [
    RBrack,     // ]
    LBrace,     // {
    RBrace,     // }

    // Separators
    Comma,      // ,
    Dot,        // .
    Semi,       // ;
    Colon,      // :
    Pipe,       // |
    DblColon,   // ::
    Matches,    // =>
    Defines,    // :=

    // Literals & Identifiers
    Ident(String),
    StringLit(String),
    Number(String),
    Boolean(bool),
    Maybe(bool),
    BarDot(String),

    EOF,
}

pub struct Lexer<'a> {
    input: &'a str,
    chars: std::str::Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            chars: input.chars(),
        }
    }

    fn next_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek_char(&mut self) -> Option<char> {
        self.chars.clone().next()
    }

    pub fn next_token(&mut self) -> Result<Token, String> {
        self.skip_whitespace_and_comments_safe()?;

        let Some(c) = self.next_char() else {
            return Ok(Token::EOF);
        };

        match c {
            '&' => {
                if self.peek_char() == Some('&') {
                    self.next_char();
                    Ok(Token::And)
                } else {
                    Ok(Token::Type)
                }
            }
            '%' => Ok(Token::Function),
            '$' => Ok(Token::Variable),
            '~' => Ok(Token::Identity),
            '#' => Ok(Token::Category),
            '@' => Ok(Token::Symbol),
            '^' => Ok(Token::Pointer),
            '?' => Ok(Token::Rule),
            '!' => Ok(Token::Predicate),
            '_' => Ok(Token::Generic),
            '+' => Ok(Token::Plus),
            '-' => Ok(Token::Minus),
            '*' => Ok(Token::Multiply),
            '/' => Ok(Token::RightDiv),
            '\\' => Ok(Token::LeftDiv),
            '=' => {
                if self.peek_char() == Some('>') {
                    self.next_char();
                    Ok(Token::Matches)
                } else {
                    Ok(Token::Equals)
                }
            }
            '<' => Ok(Token::LessThan),
            '>' => Ok(Token::GreaterThan),
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '[' => Ok(Token::LBrack),
            ']' => Ok(Token::RBrack),
            '{' => Ok(Token::LBrace),
            '}' => Ok(Token::RBrace),
            ',' => Ok(Token::Comma),
            '.' => Ok(Token::Dot),
            ';' => Ok(Token::Semi),
            ':' => {
                if self.peek_char() == Some(':') {
                    self.next_char();
                    Ok(Token::DblColon)
                } else if self.peek_char() == Some('=') {
                    self.next_char();
                    Ok(Token::Defines)
                } else {
                    Ok(Token::Colon)
                }
            }
            '|' => {
                if self.peek_char() == Some('|') {
                    self.next_char();
                    Ok(Token::Or)
                } else {
                    // Check for BarDot syntax: '|', { ! "." } , "."
                    let mut content = String::new();
                    while let Some(nc) = self.peek_char() {
                        if nc == '.' {
                            self.next_char();
                            content.push('.');
                            return Ok(Token::BarDot(content));
                        } else if nc == '\n' || nc == '\r' {
                            break;
                        } else {
                            content.push(self.next_char().unwrap());
                        }
                    }
                    Ok(Token::Pipe)
                }
            }
            '"' => {
                let mut s = String::new();
                while let Some(nc) = self.next_char() {
                    if nc == '"' {
                        return Ok(Token::StringLit(s));
                    } else if nc == '\\' {
                        let esc = self.next_char().ok_or("Unterminated string escape")?;
                        match esc {
                            'n' => s.push('\n'),
                            't' => s.push('\t'),
                            '"' => s.push('"'),
                            '\\' => s.push('\\'),
                            _ => return Err(format!("Unknown escape: \\{}", esc)),
                        }
                    } else {
                        s.push(nc);
                    }
                }
                Err("Unterminated string literal".into())
            }
            _ if c.is_digit(10) => {
                let mut num = c.to_string();
                while let Some(nc) = self.peek_char() {
                    if nc.is_digit(10) {
                        num.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
                Ok(Token::Number(num))
            }
            _ if c.is_alphabetic() || c == '_' => {
                let mut ident = c.to_string();
                while let Some(nc) = self.peek_char() {
                    if nc.is_alphanumeric() || nc == '_' {
                        ident.push(self.next_char().unwrap());
                    } else {
                        break;
                    }
                }
                match ident.as_str() {
                    "and" => Ok(Token::And),
                    "or" => Ok(Token::Or),
                    "xor" => Ok(Token::Xor),
                    "nand" => Ok(Token::Nand),
                    "nor" => Ok(Token::Nor),
                    "xnor" => Ok(Token::Xnor),
                    "not" => Ok(Token::Not),
                    "TRUE" => Ok(Token::Boolean(true)),
                    "FALSE" => Ok(Token::Boolean(false)),
                    "MAYBE" => {
                        self.skip_whitespace_and_comments_safe()?;
                        let next = self.next_token()?;
                        match next {
                            Token::Boolean(b) => Ok(Token::Maybe(b)),
                            _ => Err("Expected BOOLEAN after MAYBE".into()),
                        }
                    }
                    _ => Ok(Token::Ident(ident)),
                }
            }
            _ => Err(format!("Unexpected character: {}", c)),
        }
    }

    pub fn skip_whitespace_and_comments_safe(&mut self) -> Result<(), String> {
        loop {
            let tail = self.chars.as_str();
            if tail.is_empty() {
                break;
            }
            if let Some(c) = tail.chars().next() {
                if c.is_whitespace() {
                    self.next_char();
                } else if tail.starts_with("//") {
                    self.next_char();
                    self.next_char();
                    while let Some(nc) = self.next_char() {
                        if nc == '\n' {
                            break;
                        }
                    }
                } else if tail.starts_with("/*") {
                    self.next_char();
                    self.next_char();
                    let mut closed = false;
                    while let Some(nc) = self.next_char() {
                        if nc == '*' && self.peek_char() == Some('/') {
                            self.next_char();
                            closed = true;
                            break;
                        }
                    }
                    if !closed {
                        return Err("Unterminated comment /*".into());
                    }
                } else if tail.starts_with("(*") {
                    self.next_char();
                    self.next_char();
                    let mut closed = false;
                    while let Some(nc) = self.next_char() {
                        if nc == '*' && self.peek_char() == Some(')') {
                            self.next_char();
                            closed = true;
                            break;
                        }
                    }
                    if !closed {
                        return Err("Unterminated comment (*".into());
                    }
                } else {
                    break;
                }
            }
        }
        Ok(())
    }
}
