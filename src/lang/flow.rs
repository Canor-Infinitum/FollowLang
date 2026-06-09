use crate::core::formula::Formula;
use crate::semantic::SemanticNode;

#[derive(Debug, Clone, PartialEq)]
pub struct FlowNode {
    pub name: String,
    pub pattern: String,
    pub semantic: SemanticNode,
}

impl FlowNode {
    pub fn expand(&self, n: usize) -> String {
        self.pattern.chars().cycle().take(n).collect()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FlowScaleSpec {
    pub q: Formula,
    pub l_p: Formula,
    pub r_s_over_l_p: Formula,
    pub r_s: Formula,
}

impl FlowScaleSpec {
    pub fn fundamental() -> Self {
        let q = Formula::Symbol("q".into());
        let l_p = Formula::Real(0.07881256452824544);

        // r_s / l_p := 2 * sqrt(pi * (q + sqrt(q * (1 + q))))
        let r_s_over_l_p = Formula::Binary {
            lhs: Box::new(Formula::Integer(2)),
            op: crate::core::formula::BinaryOp::Mul,
            rhs: Box::new(Formula::Unary {
                op: crate::core::formula::UnaryOp::Sqrt,
                expr: Box::new(Formula::Binary {
                    lhs: Box::new(Formula::Symbol("pi".into())),
                    op: crate::core::formula::BinaryOp::Mul,
                    rhs: Box::new(Formula::Binary {
                        lhs: Box::new(q.clone()),
                        op: crate::core::formula::BinaryOp::Add,
                        rhs: Box::new(Formula::Unary {
                            op: crate::core::formula::UnaryOp::Sqrt,
                            expr: Box::new(Formula::Binary {
                                lhs: Box::new(q.clone()),
                                op: crate::core::formula::BinaryOp::Mul,
                                rhs: Box::new(Formula::Binary {
                                    lhs: Box::new(Formula::Integer(1)),
                                    op: crate::core::formula::BinaryOp::Add,
                                    rhs: Box::new(q.clone()),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        };

        // r_s := 2 * l_p * sqrt(...)
        let r_s = Formula::Binary {
            lhs: Box::new(Formula::Integer(2)),
            op: crate::core::formula::BinaryOp::Mul,
            rhs: Box::new(Formula::Binary {
                lhs: Box::new(l_p.clone()),
                op: crate::core::formula::BinaryOp::Mul,
                rhs: Box::new(Formula::Unary {
                    op: crate::core::formula::UnaryOp::Sqrt,
                    expr: Box::new(Formula::Binary {
                        lhs: Box::new(Formula::Symbol("pi".into())),
                        op: crate::core::formula::BinaryOp::Mul,
                        rhs: Box::new(Formula::Binary {
                            lhs: Box::new(q.clone()),
                            op: crate::core::formula::BinaryOp::Add,
                            rhs: Box::new(Formula::Unary {
                                op: crate::core::formula::UnaryOp::Sqrt,
                                expr: Box::new(Formula::Binary {
                                    lhs: Box::new(q.clone()),
                                    op: crate::core::formula::BinaryOp::Mul,
                                    rhs: Box::new(Formula::Binary {
                                        lhs: Box::new(Formula::Integer(1)),
                                        op: crate::core::formula::BinaryOp::Add,
                                        rhs: Box::new(q.clone()),
                                    }),
                                }),
                            }),
                        }),
                    }),
                }),
            }),
        };

        Self {
            q,
            l_p,
            r_s_over_l_p,
            r_s,
        }
    }
}
