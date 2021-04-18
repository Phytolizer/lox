#[derive(Debug, Clone)]
pub(super) enum Type {
    Integer(i32),
    String(String),
    Boolean(bool),
    Nil,
}

impl PartialEq for Type {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Type::Integer(i) => match other {
                Type::Integer(j) => i == j,
                _ => false,
            },
            Type::String(s) => match other {
                Type::String(t) => s == t,
                _ => false,
            },
            Type::Boolean(b) => match other {
                Type::Boolean(c) => b == c,
                _ => false,
            },
            Type::Nil => match other {
                Type::Nil => true,
                _ => false,
            },
        }
    }
}
