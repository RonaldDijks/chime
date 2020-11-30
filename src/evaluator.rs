use crate::syntax_tree::{BinOpKind, SyntaxTree};

#[derive(Debug, Copy, Clone)]
pub enum EvaluatorValue {
    Float(f64),
    Bool(bool),
}

impl EvaluatorValue {
    pub fn to_string(&self) -> String {
        match self {
            EvaluatorValue::Float(value) => format!("{} : float", value),
            EvaluatorValue::Bool(value) => format!("{} : bool", value),
        }
    }
}

#[derive(Debug)]
pub enum EvaluatorError {
    UnknownBinaryOperator(BinOpKind, EvaluatorValue, EvaluatorValue),
}

pub struct Evaluator {}

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }

    pub fn evaluate(&self, syntax_tree: &SyntaxTree) -> Result<EvaluatorValue, EvaluatorError> {
        match syntax_tree {
            SyntaxTree::F64(value) => Ok(EvaluatorValue::Float(*value)),
            SyntaxTree::BinOp(kind, left, right) => self.evaluate_binop(kind, left, right),
            SyntaxTree::Bool(value) => Ok(EvaluatorValue::Bool(*value)),
        }
    }

    fn evaluate_binop(
        &self,
        kind: &BinOpKind,
        left: &SyntaxTree,
        right: &SyntaxTree,
    ) -> Result<EvaluatorValue, EvaluatorError> {
        use BinOpKind::*;
        use EvaluatorValue::*;
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        match (left, right, *kind) {
            (Float(l), Float(r), Add) => Ok(EvaluatorValue::Float(l + r)),
            (Float(l), Float(r), Sub) => Ok(EvaluatorValue::Float(l - r)),
            (Float(l), Float(r), Mul) => Ok(EvaluatorValue::Float(l * r)),
            (Float(l), Float(r), Div) => Ok(EvaluatorValue::Float(l / r)),
            (Bool(l), Bool(r), LogicalAnd) => Ok(EvaluatorValue::Bool(l && r)),
            (Bool(l), Bool(r), LogicalOr) => Ok(EvaluatorValue::Bool(l || r)),
            _ => Err(EvaluatorError::UnknownBinaryOperator(*kind, left, right)),
        }
    }
}
