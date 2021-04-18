use std::fmt::Display;

use crate::object::Object;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub(crate) struct Token {
    pub(crate) kind: TokenType,
    pub(crate) lexeme: String,
    pub(crate) literal: Option<Object>,
    pub(crate) line: usize,
}

impl Token {
    pub(crate) fn new<O>(kind: TokenType, lexeme: String, literal: O, line: usize) -> Self
    where
        O: Into<Option<Object>>,
    {
        Self {
            kind,
            lexeme,
            literal: literal.into(),
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {} {}",
            self.kind,
            self.lexeme,
            self.literal
                .as_ref()
                .map(|l| l.to_string())
                .unwrap_or_default()
        )
    }
}
