use crate::{parser::production::expression::prelude::*, parser::value::Value};

impl Parser {
    fn find_value(&self, name: &str) -> Option<Value> {
        for scope in self.value_stack.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value.clone());
            }
        }
        None
    }

    fn get_variable_expr(&self, value: Value, name: String) -> Expr {
        match value {
            Value::Integer(id) => Expr::Integer(IntegerExpr::Variable(id)),
            Value::Float(id) => Expr::Float(FloatExpr::Variable(id)),
            Value::String(id) => Expr::String(StringExpr::Variable(id)),
            Value::Boolean(id) => Expr::Boolean(BooleanExpr::Variable(id)),
            Value::Object(obj) => Expr::Object(obj.class_declaration.clone(), name.to_string(), {
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

        match self.current_token_type().cloned() {
            Some(TokenType::Integer(val)) => {
                self.current += 1;
                Ok(Expr::Integer(IntegerExpr::Literal(val)))
            }
            Some(TokenType::Float(val)) => {
                self.current += 1;
                Ok(Expr::Float(FloatExpr::Literal(val)))
            }
            Some(TokenType::String(val)) => {
                self.current += 1;
                Ok(Expr::String(StringExpr::Literal(val)))
            }
            Some(TokenType::Boolean(val)) => {
                self.current += 1;
                Ok(Expr::Boolean(BooleanExpr::Literal(val)))
            }
            Some(TokenType::LeftParen) => {
                let left_paren_pos = self.current;
                self.current += 1;

                let expression = self.expression()?;

                match self.current_token_type() {
                    Some(TokenType::RightParen) => {
                        self.current += 1;
                        Ok(expression)
                    }
                    _ => {
                        self.error.add(ParserErrType::GroupingExpectedRightParen(
                            self.tokens[left_paren_pos].clone(),
                            self.tokens.get(self.current).cloned(),
                        ));
                        return Err(ParserStatus::Unwind);
                    }
                }
            }
            Some(TokenType::Identifier(mut name)) => {
                self.current += 1;

                match self.current_token_type() {
                    Some(TokenType::LeftParen) => self.call(&name, None, self.current),
                    Some(TokenType::Colon) => {
                        self.current += 1;

                        match self.current_token_type() {
                            Some(TokenType::Identifier(second_name)) => {
                                let second_name = second_name.clone();
                                self.current += 1;

                                match self.current_token_type() {
                                    Some(TokenType::LeftParen) => self.call(
                                        &second_name,
                                        Some(name.clone()),
                                        self.current - 1,
                                    ),
                                    _ => {
                                        self.error.add(ParserErrType::ModuleExpectedLeftParen(
                                            self.tokens[self.current - 1].clone(),
                                            self.tokens.get(self.current).cloned(),
                                        ));
                                        Err(ParserStatus::Unwind)
                                    }
                                }
                            }
                            _ => {
                                self.error.add(ParserErrType::ModuleExpectedIdentifier(
                                    self.tokens[self.current - 2].clone(),
                                    self.tokens.get(self.current - 1).cloned(),
                                ));
                                Err(ParserStatus::Unwind)
                            }
                        }
                    }
                    Some(TokenType::Dot) => {
                        let mut object = match self.find_value(&name) {
                            Some(Value::Object(obj)) => obj,
                            _ => {
                                panic!("Value {name} is not an object");
                            }
                        };

                        loop {
                            self.current += 1;

                            name = if let Some(TokenType::Identifier(name)) =
                                self.current_token_type()
                            {
                                name.clone()
                            } else {
                                panic!("Expected identifier for property name");
                            };

                            self.current += 1;

                            match (object.properties.get(&name), self.current_token_type()) {
                                (Some(Value::Object(obj)), Some(TokenType::Dot)) => {
                                    object = obj.clone();
                                    continue;
                                }
                                (Some(v), _) => return Ok(self.get_variable_expr(v.clone(), name)),
                                (_, Some(TokenType::Dot)) => {
                                    panic!("Value {name} is not an object");
                                }
                                (None, _) => {
                                    panic!("Property does not exist");
                                }
                            }
                        }
                    }
                    _ => match self.find_value(&name) {
                        Some(value) => Ok(self.get_variable_expr(value, name)),
                        None => {
                            panic!("Value not found");
                        }
                    },
                }
            }
            _ => {
                self.error.add(ParserErrType::ExpectedLiteralVariableCall(
                    self.tokens[self.current - 1].clone(),
                    self.tokens.get(self.current).cloned(),
                ));
                Err(ParserStatus::Unwind)
            }
        }
    }
}
