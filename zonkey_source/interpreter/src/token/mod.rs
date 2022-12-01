use std::fmt::Display;

use self::token_type::TokenType;
use super::literal::Literal;

pub mod token_type;

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    literal: Option<Literal>,
}

impl Token {
    pub fn new(token_type: TokenType) -> Self {
        Self {
            token_type,
            literal: None,
        }
    }
    pub fn new_with_literal(token_type: TokenType, literal: Literal) -> Self {
        Self {
            token_type,
            literal: Some(literal),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            Some(Literal::String(v)) => write!(f, "{:?}: {}", self.token_type, v),
            Some(Literal::Integer(v)) => write!(f, "{:?}: {}", self.token_type, v),
            Some(Literal::Float(v)) => write!(f, "{:?}: {}", self.token_type, v),
            None => write!(f, "{:?}", self.token_type),
        }
    }
}
