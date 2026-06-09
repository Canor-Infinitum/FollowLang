use crate::core::formula::{BinaryOp, Formula, UnaryOp};

use crate::united::UnitedInterchangeSpec;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EliteOp {
    Mov,
    Jmp,
    Cmp,
    Add,
    Sub,
    Mul,
    Div,
    Label(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EliteInstruction {
    pub op: EliteOp,
    pub operands: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EliteSchemaSpec {
    pub united: UnitedInterchangeSpec,
    pub radius_formula: Formula,
}

impl EliteSchemaSpec {
    pub fn new(united: UnitedInterchangeSpec) -> Self {
        // r_f = L * sqrt(pi * (n + sqrt(n * (1 + n))))
        let radius_formula = Formula::Binary {
            lhs: Box::new(Formula::Symbol("L".into())),
            op: BinaryOp::Mul,
            rhs: Box::new(Formula::Unary {
                op: UnaryOp::Sqrt,
                expr: Box::new(Formula::Binary {
                    lhs: Box::new(Formula::Symbol("pi".into())),
                    op: BinaryOp::Mul,
                    rhs: Box::new(Formula::Binary {
                        lhs: Box::new(Formula::Symbol("n".into())),
                        op: BinaryOp::Add,
                        rhs: Box::new(Formula::Unary {
                            op: UnaryOp::Sqrt,
                            expr: Box::new(Formula::Binary {
                                lhs: Box::new(Formula::Symbol("n".into())),
                                op: BinaryOp::Mul,
                                rhs: Box::new(Formula::Binary {
                                    lhs: Box::new(Formula::Integer(1)),
                                    op: BinaryOp::Add,
                                    rhs: Box::new(Formula::Symbol("n".into())),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        };

        Self {
            united,
            radius_formula,
        }
    }
}