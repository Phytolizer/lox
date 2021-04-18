use lox_generated::ast::Primary;
use lox_generated::ast::UnaryOperator;

use super::types::Type;
use super::Symbol;

#[derive(Debug)]
pub(crate) enum RuntimeError {
    IncompatibleTypes(Type, Type),
    UndefinedSymbol(String),
    BadCallTarget(Symbol),
    BadIntegerLiteral(String),
    MismatchedUnaryOperator(UnaryOperator, Type),
    InvalidType(Type),
}
