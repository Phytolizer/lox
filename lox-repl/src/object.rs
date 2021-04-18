use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Object {
    /// Returns `true` if the object is [`Number`].
    pub(crate) fn is_number(&self) -> bool {
        matches!(self, Self::Number(..))
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Number(n) => n.to_string(),
                Self::String(s) => s.clone(),
                Self::Boolean(b) => b.to_string(),
            }
        )
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Number(n) => match other {
                Self::Number(m) => n == m,
                _ => false,
            },
            Self::String(s) => match other {
                Self::String(t) => s == t,
                _ => false,
            },
            Self::Boolean(b) => match other {
                Self::Boolean(c) => b == c,
                _ => false,
            },
        }
    }
}

pub(crate) trait IsTruthy {
    fn is_truthy(&self) -> bool;
}

impl IsTruthy for Option<Object> {
    fn is_truthy(&self) -> bool {
        !matches!(self, None | Some(Object::Boolean(false)))
    }
}
