mod class;
mod function;
mod start;

use crate::{parser::production::prelude::*, parser::value::ValueType};

impl Parser {
    // Helper functions used by some definitions to convert token_type to a value_type
    fn data_type(&mut self) -> Result<ValueType, Option<Token>> {
        match self.current_token_type() {
            Some(TokenType::IntegerType) => Ok(ValueType::Integer),
            Some(TokenType::FloatType) => Ok(ValueType::Float),
            Some(TokenType::StringType) => Ok(ValueType::String),
            Some(TokenType::BooleanType) => Ok(ValueType::Boolean),
            Some(TokenType::Identifier(type_name)) => Ok(ValueType::Class(type_name.to_string())),
            _ => Err(self.tokens.get(self.current).cloned()),
        }
    }
}
