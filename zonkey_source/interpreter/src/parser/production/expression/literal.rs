use crate::{parser::production::expression::prelude::*, parser::value::Value};
use std::rc::Rc;

impl Parser {
    fn find_value(&self, name: Rc<String>) -> Option<Value> {
        for scope in self.value_stack.iter().rev() {
            if let Some(value) = scope.get(&name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn get_variable_expr(&self, value: Value, name: Rc<String>) -> Expr {
        match value {
            Value::Integer(id) => Expr::Integer(IntegerExpr::Variable(id)),
            Value::Float(id) => Expr::Float(FloatExpr::Variable(id)),
            Value::String(id) => Expr::String(StringExpr::Variable(id)),
            Value::Boolean(id) => Expr::Boolean(BooleanExpr::Variable(id)),
            Value::Object(obj) => Expr::Object(obj.class_declaration, name, {
                let mut expressions = vec![];
                for (name, value) in obj.properties {
                    expressions.push(self.get_variable_expr(value, name));
                }
                expressions
            }),
        }
    }

    pub fn literal(&mut self) -> Result<Expr, ParserStatus> {
        debug_information!("literal");

        match self.consume_token_type() {
            Some(TokenType::Integer(val)) => Ok(Expr::Integer(IntegerExpr::Literal(*val))),
            Some(TokenType::Float(val)) => Ok(Expr::Float(FloatExpr::Literal(*val))),
            Some(TokenType::String(val)) => Ok(Expr::String(StringExpr::Literal(Rc::clone(val)))),
            Some(TokenType::Boolean(val)) => Ok(Expr::Boolean(BooleanExpr::Literal(*val))),
            Some(TokenType::LeftParen) => {
                let left_paren_pos = self.current - 1;

                let expression = self.expression()?;

                match self.consume_token_type() {
                    Some(TokenType::RightParen) => Ok(expression),
                    _ => {
                        self.error.add(ParserErrType::GroupingExpectedRightParen(
                            self.tokens[left_paren_pos].clone(),
                            self.tokens.get(self.current - 1).cloned(),
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                }
            }
            Some(TokenType::Identifier(name)) => {
                let name = Rc::clone(name);
                match self.current_token_type() {
                    Some(TokenType::LeftParen) => {
                        self.current += 1;
                        self.call(name)
                    }
                    _ => match self.find_value(Rc::clone(&name)) {
                        Some(value) => Ok(self.get_variable_expr(value, name)),
                        None => {
                            panic!("Value not found");
                        }
                    },
                }
            }
            _ => todo!(),
        }
    }
}
