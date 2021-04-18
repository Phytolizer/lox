use std::fmt::Display;

use crate::token::Token;

#[derive(Debug)]
pub(crate) struct RuntimeError {
    token: Token,
    message: String,
}

impl RuntimeError {
    pub(crate) fn new(token: Token, message: String) -> Self {
        Self { token, message }
    }

    pub(crate) fn message(&self) -> &str {
        &self.message
    }

    pub(crate) fn token(&self) -> &Token {
        &self.token
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for RuntimeError {}
