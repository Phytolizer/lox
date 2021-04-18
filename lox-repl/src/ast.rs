use lox_repl_macros::generate_ast;

use crate::object::Object;
use crate::token::Token;

generate_ast! {
    Expr
    Binary {
        left: Box<Expr>
        operator: Token
        right: Box<Expr>
    }
    Grouping {
        expression: Box<Expr>
    }
    Literal {
        value: Option<Object>
    }
    Unary {
        operator: Token
        right: Box<Expr>
    }
}

pub(crate) struct ExprPrinter;
impl ExprPrinter {
    pub(crate) fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut s = format!("({}", name);
        for expr in exprs {
            s.push_str(&format!(" {}", expr.accept(self)));
        }
        s.push(')');
        s
    }
}
impl ExprVisitor<String> for ExprPrinter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.left, &expr.right])
    }
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, &[&expr.right])
    }
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> String {
        self.parenthesize("group", &[&expr.expression])
    }
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
        match &expr.value {
            None => String::from("nil"),
            Some(value) => value.to_string(),
        }
    }
}
