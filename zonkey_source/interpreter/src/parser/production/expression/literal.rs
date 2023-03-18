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

    fn get_variable_expr(&self, value: &Value) -> Expr {
        match value {
            Value::Integer(id) => Expr::Integer(IntegerExpr::Variable(*id)),
            Value::Float(id) => Expr::Float(FloatExpr::Variable(*id)),
            Value::String(id) => Expr::String(StringExpr::Variable(*id)),
            Value::Boolean(id) => Expr::Boolean(BooleanExpr::Variable(*id)),
            Value::Object(class, id) => Expr::Object(Rc::clone(class), ObjectExpr::Variable(*id)),
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
                        if let Some(call) = self.function_declarations.get(&name) {
                            self.call(None, name, Rc::clone(call))
                        } else {
                            self.error.add(ParserErrType::CallFunctionNotFound(
                                self.tokens[self.current - 1].clone(),
                                name.to_string(),
                            ));
                            Err(ParserStatus::Unwind)
                        }
                    }
                    Some(TokenType::Dot) => {
                        // Retrieve object to find method or property in
                        let (mut current_class, object_id) = match self.find_value(Rc::clone(&name))
                        {
                            Some(Value::Object(class, id)) => (class, id),
                            _ => {
                                panic!("Value {name} is not an object");
                            }
                        };

                        let mut id_path = vec![object_id];
                        let mut current_object = Rc::clone(&self.objects[&object_id]);

                        loop {
                            self.current += 1;

                            let property_or_method_name = if let Some(TokenType::Identifier(nm)) =
                                self.consume_token_type()
                            {
                                Rc::clone(nm)
                            } else {
                                panic!("Expected identifier for property or method name")
                            };

                            match self.current_token_type() {
                                Some(TokenType::LeftParen) => {
                                    match self.class_declarations[&current_class]
                                        .methods
                                        .get(&property_or_method_name)
                                    {
                                        Some(call) => {
                                            self.current += 1;
                                            let last_id = id_path.pop().unwrap();
                                            return self.call(
                                                Some(Expr::Object(
                                                    Rc::clone(&current_class),
                                                    ObjectExpr::Property(id_path, last_id),
                                                )),
                                                property_or_method_name,
                                                Rc::clone(&call),
                                            );
                                        }
                                        None => panic!(
                                            "Method {property_or_method_name} does not exist"
                                        ),
                                    }
                                }
                                Some(tt) => {
                                    match (
                                        current_object.properties.get(&property_or_method_name),
                                        tt,
                                    ) {
                                        (Some(Value::Object(class, id)), TokenType::Dot) => {
                                            id_path.push(*id);
                                            current_class = Rc::clone(class);
                                            current_object = Rc::clone(&current_object.objects[id]);
                                        }
                                        (Some(Value::Object(class, id)), _) => {
                                            return Ok(Expr::Object(
                                                Rc::clone(class),
                                                ObjectExpr::Property(id_path, *id),
                                            ))
                                        }
                                        (Some(Value::Integer(id)), _) => {
                                            return Ok(Expr::Integer(IntegerExpr::Property(
                                                id_path, *id,
                                            )))
                                        }
                                        (Some(Value::Float(id)), _) => {
                                            return Ok(Expr::Float(FloatExpr::Property(
                                                id_path, *id,
                                            )))
                                        }
                                        (Some(Value::String(id)), _) => {
                                            return Ok(Expr::String(StringExpr::Property(
                                                id_path, *id,
                                            )))
                                        }
                                        (Some(Value::Boolean(id)), _) => {
                                            return Ok(Expr::Boolean(BooleanExpr::Property(
                                                id_path, *id,
                                            )))
                                        }
                                        t => panic!("Found value {:?}", t),
                                    }
                                }
                                _ => panic!("Property {property_or_method_name} does not exist"),
                            }
                        }
                    }
                    _ => match self.find_value(Rc::clone(&name)) {
                        Some(value) => Ok(self.get_variable_expr(&value)),
                        None => {
                            panic!("Value {name} not found");
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
