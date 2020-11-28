use crate::syntax_tree::{BinOpKind, SyntaxTree};

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn evaluate(&self, syntax_tree: &SyntaxTree) -> f64 {
        match syntax_tree {
            SyntaxTree::F64(value) => *value,
            SyntaxTree::BinOp(kind, left, right) => {
                let left = self.evaluate(&left);
                let right = self.evaluate(&right);
                match kind {
                    BinOpKind::Add => left + right,
                    BinOpKind::Sub => left - right,
                    BinOpKind::Mul => left * right,
                    BinOpKind::Div => left / right,
                }
            }
        }
    }
}
