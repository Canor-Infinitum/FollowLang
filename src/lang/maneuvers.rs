use crate::core::formula::{BinaryOp, Formula};
use crate::semantic::SemanticNode;

use crate::{action::ActionSymbolSpec, flow::FlowScaleSpec};

#[derive(Debug, Clone, PartialEq)]
pub struct ManeuverNode {
    pub name: String,
    pub composition: String,
    pub weight: Option<f64>,
    pub semantic: SemanticNode,
}

impl ManeuverNode {
    pub fn eval(&self) -> String {
        match self.weight {
            Some(w) => format!("{} @ {}", self.composition, w),
            None => self.composition.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ManeuverGroupSpec {
    pub flow: FlowScaleSpec,
    pub action: ActionSymbolSpec,
    pub delta_a: Formula,
}

impl ManeuverGroupSpec {
    pub fn new(flow: FlowScaleSpec, action: ActionSymbolSpec) -> Self {
        // DELTA(A) = 32 * pi^2 * l_p^2 + 64 * pi^3 * (l_p^4 / r_s^2)
        let delta_a = Formula::Binary {
            lhs: Box::new(Formula::Binary {
                lhs: Box::new(Formula::Integer(32)),
                op: BinaryOp::Mul,
                rhs: Box::new(Formula::Binary {
                    lhs: Box::new(Formula::Power {
                        base: Box::new(Formula::Symbol("pi".into())),
                        exp: Box::new(Formula::Integer(2)),
                    }),
                    op: BinaryOp::Mul,
                    rhs: Box::new(Formula::Power {
                        base: Box::new(flow.l_p.clone()),
                        exp: Box::new(Formula::Integer(2)),
                    }),
                }),
            }),
            op: BinaryOp::Add,
            rhs: Box::new(Formula::Binary {
                lhs: Box::new(Formula::Integer(64)),
                op: BinaryOp::Mul,
                rhs: Box::new(Formula::Binary {
                    lhs: Box::new(Formula::Power {
                        base: Box::new(Formula::Symbol("pi".into())),
                        exp: Box::new(Formula::Integer(3)),
                    }),
                    op: BinaryOp::Mul,
                    rhs: Box::new(Formula::Binary {
                        lhs: Box::new(Formula::Power {
                            base: Box::new(flow.l_p.clone()),
                            exp: Box::new(Formula::Integer(4)),
                        }),
                        op: BinaryOp::RightDiv,
                        rhs: Box::new(Formula::Power {
                            base: Box::new(Formula::Symbol("r_s".into())),
                            exp: Box::new(Formula::Integer(2)),
                        }),
                    }),
                }),
            }),
        };

        Self {
            flow,
            action,
            delta_a,
        }
    }
}