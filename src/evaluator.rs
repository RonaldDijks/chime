use std::collections::HashMap;

use crate::{
    parser::ParserError,
    syntax_tree::{BinOpKind, SyntaxTree},
};

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
    UnknownIdentifier(String),
}

pub struct Evaluator {
    scope: HashMap<String, EvaluatorValue>,
}

impl Evaluator {
    pub fn new() -> Self {
        let mut scope = HashMap::new();
        scope.insert("a".into(), EvaluatorValue::Float(10.0));

        Evaluator { scope }
    }

    pub fn evaluate(&mut self, syntax_tree: &SyntaxTree) -> Result<EvaluatorValue, EvaluatorError> {
        match syntax_tree {
            SyntaxTree::F64(value) => Ok(EvaluatorValue::Float(*value)),
            SyntaxTree::Bool(value) => Ok(EvaluatorValue::Bool(*value)),
            SyntaxTree::BinOp(kind, left, right) => self.evaluate_binop(kind, left, right),
            SyntaxTree::Identifier(ident) => self.evaluate_identifier(ident),
            SyntaxTree::Assignment(ident, expr) => self.evaluate_assignment_expression(ident, expr),
        }
    }

    fn evaluate_binop(
        &mut self,
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

    fn evaluate_identifier(&self, expr: &String) -> Result<EvaluatorValue, EvaluatorError> {
        match self.scope.get(expr) {
            Some(&value) => Ok(value),
            None => Err(EvaluatorError::UnknownIdentifier(expr.clone())),
        }
    }
    fn evaluate_assignment_expression(
        &mut self,
        ident: &String,
        expr: &SyntaxTree,
    ) -> Result<EvaluatorValue, EvaluatorError> {
        let expr = self.evaluate(expr)?;

        match self.scope.get(ident) {
            Some(_) => {
                self.scope.insert((*ident).clone(), expr);
                Ok(expr)
            }
            None => Err(EvaluatorError::UnknownIdentifier((*ident).clone())),
        }
    }
}
