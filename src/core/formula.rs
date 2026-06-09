use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Formula {
    Symbol(String),
    Integer(i64),
    Real(f64),
    Text(String),

    Unary {
        op: UnaryOp,
        expr: Box<Formula>,
    },

    Binary {
        lhs: Box<Formula>,
        op: BinaryOp,
        rhs: Box<Formula>,
    },

    Call {
        name: String,
        args: Vec<Formula>,
    },

    Power {
        base: Box<Formula>,
        exp: Box<Formula>,
    },

    Derivative {
        expr: Box<Formula>,
        var: String,
        order: usize,
        partial: bool,
    },

    Index {
        target: Box<Formula>,
        indices: Vec<String>,
    },

    Matrix(Vec<Vec<Formula>>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOp {
    Plus,
    Minus,
    Not,
    Sqrt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    RightDiv,
    LeftDiv,
    Eq,
    And,
    Or,
    Xor,
    Matches,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            UnaryOp::Plus => "+",
            UnaryOp::Minus => "-",
            UnaryOp::Not => "not",
            UnaryOp::Sqrt => "sqrt",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::RightDiv => "/",
            BinaryOp::LeftDiv => "\\",
            BinaryOp::Eq => "=",
            BinaryOp::And => "and",
            BinaryOp::Or => "or",
            BinaryOp::Xor => "xor",
            BinaryOp::Matches => "=>",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Formula::Symbol(s) => write!(f, "{s}"),
            Formula::Integer(n) => write!(f, "{n}"),
            Formula::Real(x) => write!(f, "{x}"),
            Formula::Text(s) => write!(f, "\"{s}\""),

            Formula::Unary { op, expr } => {
                if matches!(op, UnaryOp::Sqrt) {
                    write!(f, "sqrt({expr})")
                } else {
                    write!(f, "({op}{expr})")
                }
            }

            Formula::Binary { lhs, op, rhs } => {
                write!(f, "({lhs} {op} {rhs})")
            }

            Formula::Call { name, args } => {
                let joined = args.iter().map(|a| format!("{a}")).collect::<Vec<_>>().join(", ");
                write!(f, "{name}({joined})")
            }

            Formula::Power { base, exp } => {
                write!(f, "({base})^({exp})")
            }

            Formula::Derivative { expr, var, order, partial } => {
                if *partial {
                    write!(f, "partial^{}({})/partial({})^{}", order, expr, var, order)
                } else {
                    write!(f, "d^{}({})/d({})^{}", order, expr, var, order)
                }
            }

            Formula::Index { target, indices } => {
                write!(f, "{}_({})", target, indices.join(" "))
            }

            Formula::Matrix(rows) => {
                let row_strs = rows.iter()
                    .map(|row| {
                        row.iter()
                            .map(|x| format!("{x}"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .collect::<Vec<_>>();
                write!(f, "[ {} ]", row_strs.join("; "))
            }
        }
    }
}
