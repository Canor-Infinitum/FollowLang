//! # Ether Intermediate Representation (IR)
//!
//! The IR is a thin wrapper around the statements that would later be fed to
//! an “Ether” runtime or code‑generator.

use super::{flow_lang::FlowExpr, action_lang::ActionExpr,
            maneuvers_lang::ManeuverExpr, ui_lang::UIExpr, elite_lang::EliteExpr};

/// An instruction in the Ether IR.
#[derive(Debug)]
pub enum EtherInstruction {
    Flow(FlowExpr),
    Action(ActionExpr),
    Maneuver(ManeuverExpr),
    UI(UIExpr),
    Elite(EliteExpr),
}

/// A simple “compiler” that turns a `FollowerProgram` into an `EtherIR`.
#[derive(Debug)]
pub struct EtherIR {
    pub instructions: Vec<EtherInstruction>,
}

impl From<super::follower_lang::FollowerProgram> for EtherIR {
    fn from(prog: super::follower_lang::FollowerProgram) -> Self {
        let instructions = prog
            .statements
            .into_iter()
            .map(|s| match s {
                super::follower_lang::Statement::Flow(e) => EtherInstruction::Flow(e),
                super::follower_lang::Statement::Action(e) => EtherInstruction::Action(e),
                super::follower_lang::Statement::Maneuver(e) => EtherInstruction::Maneuver(e),
                super::follower_lang::Statement::UI(e) => EtherInstruction::UI(e),
                super::follower_lang::Statement::Elite(e) => EtherInstruction::Elite(e),
            })
            .collect();
        EtherIR { instructions }
    }
}