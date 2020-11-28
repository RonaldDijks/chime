#[derive(Debug)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum SyntaxTree {
    F64(f64),
    BinOp(BinOpKind, Box<SyntaxTree>, Box<SyntaxTree>),
}
