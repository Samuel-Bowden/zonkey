use std::fmt::Display;

#[derive(Debug)]
pub enum TreeWalkerErr {
    UnsupportedOperator,
    UnsupportedExpression,
}

impl Display for TreeWalkerErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedOperator => write!(f, "An unsupported token was used as an operator"),
            Self::UnsupportedExpression => write!(f, "A currently unsupported expression was used"),
        }
    }
}
