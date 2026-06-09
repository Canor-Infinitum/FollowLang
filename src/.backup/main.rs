/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

mod ast;
mod lexer;
mod parser;

fn main() {
    let code = "&char::( &u8, &u16, &u32, &u64 );";
    let mut parser = parser::Parser::new(code).unwrap();
    let ast = parser.parse_program().unwrap();
    println!("Parsed AST: {:#?}", ast);
}
