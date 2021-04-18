use crate::ast::AcceptExprVisitor;
use crate::ast::ExprVisitor;
use crate::object::IsTruthy;
use crate::object::Object;
use crate::runtime_error::RuntimeError;
use crate::token::Token;
use crate::token_type::TokenType;

pub(crate) struct Interpreter {}

impl Interpreter {
    pub(crate) fn new() -> Self {
        Self {}
    }
    pub(crate) fn interpret(&mut self, expr: &crate::ast::Expr) {
        let res = self.evaluate(expr);
        match res {
            Ok(res) => println!(
                "{}",
                res.map(|r| r.to_string()).unwrap_or_else(|| "nil".into())
            ),
            Err(e) => crate::runtime_error(e),
        }
    }
    fn evaluate(&mut self, expr: &crate::ast::Expr) -> Result<Option<Object>, RuntimeError> {
        expr.accept(self)
    }
}

impl ExprVisitor<Result<Option<Object>, RuntimeError>> for Interpreter {
    fn visit_binary_expr(
        &mut self,
        expr: &crate::ast::BinaryExpr,
    ) -> Result<Option<Object>, RuntimeError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.kind {
            TokenType::BangEqual => Ok(Some(Object::Boolean(left != right))),
            TokenType::EqualEqual => Ok(Some(Object::Boolean(left == right))),
            TokenType::Greater => {
                let (left, right) = check_number_operands(expr.operator.clone(), left, right)?;
                Ok(Some(Object::Boolean(left > right)))
            }
            TokenType::GreaterEqual => {
                let (left, right) = check_number_operands(expr.operator.clone(), left, right)?;
                Ok(Some(Object::Boolean(left >= right)))
            }
            TokenType::Less => {
                let (left, right) = check_number_operands(expr.operator.clone(), left, right)?;
                Ok(Some(Object::Boolean(left < right)))
            }
            TokenType::LessEqual => {
                let (left, right) = check_number_operands(expr.operator.clone(), left, right)?;
                Ok(Some(Object::Boolean(left <= right)))
            }
            TokenType::Minus => {
                let (left, right) = check_number_operands(expr.operator.clone(), left, right)?;
                Ok(Some(Object::Number(left - right)))
            }
            TokenType::Slash => {
                let (left, right) = check_number_operands(expr.operator.clone(), left, right)?;
                Ok(Some(Object::Number(left / right)))
            }
            TokenType::Star => {
                let (left, right) = check_number_operands(expr.operator.clone(), left, right)?;
                Ok(Some(Object::Number(left * right)))
            }
            TokenType::Plus => {
                match left {
                    Some(Object::Number(n)) => {
                        if let Some(Object::Number(m)) = right {
                            return Ok(Some(Object::Number(n + m)));
                        }
                    }
                    Some(Object::String(s)) => {
                        if let Some(Object::String(t)) = right {
                            return Ok(Some(Object::String(s + &t)));
                        }
                    }
                    _ => {}
                }
                Err(RuntimeError::new(
                    expr.operator.clone(),
                    String::from("Operands must be two numbers or two strings."),
                ))
            }
            _ => unreachable!(),
        }
    }

    fn visit_grouping_expr(
        &mut self,
        expr: &crate::ast::GroupingExpr,
    ) -> Result<Option<Object>, RuntimeError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(
        &mut self,
        expr: &crate::ast::LiteralExpr,
    ) -> Result<Option<Object>, RuntimeError> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(
        &mut self,
        expr: &crate::ast::UnaryExpr,
    ) -> Result<Option<Object>, RuntimeError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.kind {
            TokenType::Bang => Ok(Some(Object::Boolean(!right.is_truthy()))),
            TokenType::Minus => {
                let right = check_number_operand(expr.operator.clone(), right)?;
                Ok(Some(Object::Number(-right)))
            }
            _ => unreachable!(),
        }
    }
}

fn check_number_operand(operator: Token, operand: Option<Object>) -> Result<f64, RuntimeError> {
    if let Some(Object::Number(n)) = operand {
        Ok(n)
    } else {
        Err(RuntimeError::new(
            operator,
            String::from("Operand must be a number."),
        ))
    }
}

fn check_number_operands(
    operator: Token,
    left: Option<Object>,
    right: Option<Object>,
) -> Result<(f64, f64), RuntimeError> {
    if let Some(Object::Number(n)) = left {
        if let Some(Object::Number(m)) = right {
            return Ok((n, m));
        }
    }
    Err(RuntimeError::new(
        operator,
        String::from("Operands must be numbers."),
    ))
}
