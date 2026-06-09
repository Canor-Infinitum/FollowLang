//! # Follower Language (High‑level)
//!
//! A simple container that mixes the lower‑level language ASTs.

use super::{flow_lang::FlowExpr, action_lang::ActionExpr,
            maneuvers_lang::ManeuverExpr, ui_lang::UIExpr, elite_lang::EliteExpr};

/// One statement in a Follower program.
#[derive(Debug)]
pub enum Statement {
    Flow(FlowExpr),
    Action(ActionExpr),
    Maneuver(ManeuverExpr),
    UI(UIExpr),
    Elite(EliteExpr),
}

/// A sequence of statements – the top‑level “program”.
#[derive(Debug)]
pub struct FollowerProgram {
    pub statements: Vec<Statement>,
}

impl FollowerProgram {
    /// Create an empty program.
    pub fn new() -> Self { FollowerProgram { statements: Vec::new() } }

    /// Append a Flow statement.
    pub fn add_flow(&mut self, expr: FlowExpr) {
        self.statements.push(Statement::Flow(expr));
    }
    /// Append an Action statement.
    pub fn add_action(&mut self, expr: ActionExpr) {
        self.statements.push(Statement::Action(expr));
    }
    /// Append a Maneuver statement.
    pub fn add_maneuver(&mut self, expr: ManeuverExpr) {
        self.statements.push(Statement::Maneuver(expr));
    }
    /// Append a UI statement.
    pub fn add_ui(&mut self, expr: UIExpr) {
        self.statements.push(Statement::UI(expr));
    }
    /// Append an Elite statement.
    pub fn add_elite(&mut self, expr: EliteExpr) {
        self.statements.push(Statement::Elite(expr));
    }
}