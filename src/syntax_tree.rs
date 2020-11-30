#[derive(Debug, Copy, Clone)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,

    LogicalAnd,
    LogicalOr,
}

impl BinOpKind {
    pub fn precedence(&self) -> (u8, u8) {
        match self {
            BinOpKind::Add | BinOpKind::Sub => (1, 2),
            BinOpKind::Mul | BinOpKind::Div => (3, 4),
            BinOpKind::LogicalAnd => (5, 6),
            BinOpKind::LogicalOr => (7, 8),
        }
    }
}

#[derive(Debug)]
pub enum SyntaxTree {
    F64(f64),
    Bool(bool),
    Identifier(String),
    BinOp(BinOpKind, Box<SyntaxTree>, Box<SyntaxTree>),
    Assignment(String, Box<SyntaxTree>),
}
