/*
 * Copyright (c) 2026 Canor Aether Infinitum
 *
 * SPDX-License-Identifier: GNU AGPL v3.0-or-later
 */

// Self-referential crate alias aliases to satisfy external-like imports in submodules
pub use crate as follow_syntax;
pub use crate as follow_core;

#[path = "lang/flow.rs"]
pub mod flow;
#[path = "lang/action.rs"]
pub mod action;
#[path = "lang/maneuvers.rs"]
pub mod maneuvers;
#[path = "lang/united.rs"]
pub mod united;
#[path = "lang/elite.rs"]
pub mod elite;
#[path = "lang/follower.rs"]
pub mod follower;
#[path = "lang/ether.rs"]
pub mod ether;

pub mod registry;
pub mod math;
pub mod semantic;
pub mod ast;
pub mod lexer;
pub mod parser;
pub mod ffi;
pub mod core;
