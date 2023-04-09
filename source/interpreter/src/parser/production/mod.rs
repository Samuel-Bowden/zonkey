mod definition;
mod expression;
mod prelude;
mod statement;

use crate::{
    expr::Expr,
    parser::{production::prelude::*, value::ValueType},
    parser_debug,
    stack::Stack,
};
use std::rc::Rc;

impl Parser {
    pub fn program(&mut self) {
        debug_information!("program");

        while !self.is_at_end() {
            let result = match self.current_token_type() {
                Some(TokenType::Start) => self.start(),
                Some(TokenType::Function) => self.function(),
                Some(TokenType::Class) => self.class(),
                Some(_) => {
                    self.error.add(ParserErrType::UnexpectedTokenInGlobal(
                        self.tokens[self.current].clone(),
                    ));
                    Err(ParserStatus::Unwind)
                }
                None => Ok(()),
            };

            match result {
                Ok(()) => (),
                // Synchronise
                Err(_) => {
                    parser_debug!("Synchronising global");
                    loop {
                        if let Some(TokenType::Start | TokenType::Function | TokenType::Class)
                        | None = self.current_token_type()
                        {
                            break;
                        }

                        self.current += 1;
                    }
                }
            };
        }
    }

    // Helper functions
    fn expr_type(&self, expr: &Expr) -> Option<ValueType> {
        match expr {
            Expr::Integer(_) => Some(ValueType::Integer),
            Expr::Float(_) => Some(ValueType::Float),
            Expr::String(_) => Some(ValueType::String),
            Expr::Boolean(_) => Some(ValueType::Boolean),
            Expr::None(_) => None,
            Expr::Object(type_name, ..) => Some(ValueType::Class(Rc::clone(type_name))),
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn current_token_type(&self) -> Option<&TokenType> {
        if let Some(t) = self.tokens.get(self.current) {
            Some(&t.token_type)
        } else {
            None
        }
    }

    fn consume_token_type(&mut self) -> Option<&TokenType> {
        self.current += 1;
        if let Some(t) = self.tokens.get(self.current - 1) {
            Some(&t.token_type)
        } else {
            None
        }
    }

    fn stack(&self) -> Stack {
        Stack {
            integer: self.integer_next_id,
            float: self.float_next_id,
            string: self.string_next_id,
            boolean: self.boolean_next_id,
            object: self.object_next_id,
        }
    }
}
