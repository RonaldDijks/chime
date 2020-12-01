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
pub enum Expression {
    F64(f64),
    Bool(bool),
    Identifier(String),
    BinOp(BinOpKind, Box<Expression>, Box<Expression>),
    Assignment(String, Box<Expression>),
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    VariableDeclaration(String, Box<Expression>),
}

#[derive(Debug)]
pub struct CompilationUnit {
    pub statement: Statement,
}
