use lox_generated::ast::Primary;
use lox_generated::ast::UnaryOperator;

use super::types::Type;

#[derive(Debug)]
pub(crate) enum RuntimeError {
    IncompatibleTypes(Type, Type),
    UndefinedSymbol(String),
    BadCallTarget(Type),
    BadIntegerLiteral(String),
    MismatchedUnaryOperator(UnaryOperator, Type),
    InvalidType(Type),
}
