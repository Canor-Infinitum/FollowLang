use crate::core::formula::{BinaryOp, Formula};
use crate::semantic::SemanticNode;

#[derive(Debug, Clone, PartialEq)]
pub struct ActionNode {
    pub name: String,
    pub relation: String,
    pub semantic: SemanticNode,
}

impl ActionNode {
    pub fn normalize(&self) -> String {
        self.relation.trim().to_string()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActionSymbolSpec {
    pub c_n1: Formula,
    pub c_n2: Formula,
    pub function_name: String,
    pub variable: String,
    pub equation: Formula,
}

impl ActionSymbolSpec {
    pub fn arbitrary_f() -> Self {
        let f_of_f = Formula::Call {
            name: "F".into(),
            args: vec![Formula::Symbol("f".into())],
        };

        let d1 = Formula::Derivative {
            expr: Box::new(f_of_f.clone()),
            var: "f".into(),
            order: 1,
            partial: true,
        };

        let d2 = Formula::Derivative {
            expr: Box::new(f_of_f.clone()),
            var: "f".into(),
            order: 2,
            partial: true,
        };

        // C_N1 * (dF/df)^2 - C_N2 * F[f] * (d²F/df²) = 0
        let lhs = Formula::Binary {
            lhs: Box::new(Formula::Binary {
                lhs: Box::new(Formula::Symbol("C_N1".into())),
                op: BinaryOp::Mul,
                rhs: Box::new(Formula::Power {
                    base: Box::new(d1),
                    exp: Box::new(Formula::Integer(2)),
                }),
            }),
            op: BinaryOp::Sub,
            rhs: Box::new(Formula::Binary {
                lhs: Box::new(Formula::Binary {
                    lhs: Box::new(Formula::Symbol("C_N2".into())),
                    op: BinaryOp::Mul,
                    rhs: Box::new(f_of_f.clone()),
                }),
                op: BinaryOp::Mul,
                rhs: Box::new(d2),
            }),
        };

        let equation = Formula::Binary {
            lhs: Box::new(lhs),
            op: BinaryOp::Eq,
            rhs: Box::new(Formula::Integer(0)),
        };

        Self {
            c_n1: Formula::Symbol("C_N1".into()),
            c_n2: Formula::Symbol("C_N2".into()),
            function_name: "F".into(),
            variable: "f".into(),
            equation,
        }
    }
}