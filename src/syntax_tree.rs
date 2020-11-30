#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

impl BinOpKind {
    pub fn precedence(&self) -> (u8, u8) {
        match self {
            BinOpKind::Add | BinOpKind::Sub => (1, 2),
            BinOpKind::Mul | BinOpKind::Div => (3, 4),
        }
    }
}

#[derive(Debug)]
pub enum SyntaxTree {
    F64(f64),
    BinOp(BinOpKind, Box<SyntaxTree>, Box<SyntaxTree>),
}
