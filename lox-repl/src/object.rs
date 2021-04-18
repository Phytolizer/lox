use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum Object {
    Number(f64),
    String(String),
    Boolean(bool),
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
