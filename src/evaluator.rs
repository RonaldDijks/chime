use std::collections::HashMap;

use crate::syntax_tree::{BinOpKind, CompilationUnit, Expression, Statement};

#[derive(Debug, Copy, Clone)]
pub enum EvaluatorValue {
    Unit,
    Float(f64),
    Bool(bool),
}

impl EvaluatorValue {
    pub fn to_string(&self) -> String {
        match self {
            EvaluatorValue::Unit => format!("() : unit"),
            EvaluatorValue::Float(value) => format!("{} : float", value),
            EvaluatorValue::Bool(value) => format!("{} : bool", value),
        }
    }
}

#[derive(Debug)]
pub enum EvaluatorError {
    UnknownBinaryOperator(BinOpKind, EvaluatorValue, EvaluatorValue),
    UnknownIdentifier(String),
    VariableAlreadyDeclared(String),
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

    pub fn evaluate(&mut self, unit: &CompilationUnit) -> Result<EvaluatorValue, EvaluatorError> {
        self.evaluate_statement(&unit.statement)
    }

    pub fn evaluate_statement(
        &mut self,
        statement: &Statement,
    ) -> Result<EvaluatorValue, EvaluatorError> {
        match statement {
            Statement::VariableDeclaration(ident, expr) => {
                self.evaluate_variable_declaration(ident, expr)
            }
            Statement::ExpressionStatement(expr) => self.evaluate_expression(expr),
        }
    }

    pub fn evaluate_variable_declaration(
        &mut self,
        ident: &String,
        expr: &Expression,
    ) -> Result<EvaluatorValue, EvaluatorError> {
        if self.scope.contains_key(ident) {
            return Err(EvaluatorError::VariableAlreadyDeclared((*ident).clone()));
        }

        let value = self.evaluate_expression(expr)?;
        self.scope.insert((*ident).clone(), value);
        Ok(EvaluatorValue::Unit)
    }

    pub fn evaluate_expression(
        &mut self,
        expression: &Expression,
    ) -> Result<EvaluatorValue, EvaluatorError> {
        match expression {
            Expression::F64(value) => Ok(EvaluatorValue::Float(*value)),
            Expression::Bool(value) => Ok(EvaluatorValue::Bool(*value)),
            Expression::BinOp(kind, left, right) => self.evaluate_binop(kind, left, right),
            Expression::Identifier(ident) => self.evaluate_identifier(ident),
            Expression::Assignment(ident, expr) => self.evaluate_assignment_expression(ident, expr),
        }
    }

    fn evaluate_binop(
        &mut self,
        kind: &BinOpKind,
        left: &Expression,
        right: &Expression,
    ) -> Result<EvaluatorValue, EvaluatorError> {
        use BinOpKind::*;
        use EvaluatorValue::*;
        let left = self.evaluate_expression(left)?;
        let right = self.evaluate_expression(right)?;
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
        expr: &Expression,
    ) -> Result<EvaluatorValue, EvaluatorError> {
        let expr = self.evaluate_expression(expr)?;

        match self.scope.get(ident) {
            Some(_) => {
                self.scope.insert((*ident).clone(), expr);
                Ok(expr)
            }
            None => Err(EvaluatorError::UnknownIdentifier((*ident).clone())),
        }
    }
}
