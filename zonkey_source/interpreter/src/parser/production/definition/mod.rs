mod class;
mod function;
mod prelude;
mod start;

use std::rc::Rc;

use crate::{
    parser::production::definition::prelude::*,
    parser::value::ValueType,
    parser::value::{Object, Value},
};

impl Parser {
    // Helper functions used by some definitions to convert token_type to a value_type
    fn data_type(&self) -> Result<ValueType, ()> {
        match self.current_token_type() {
            Some(TokenType::IntegerType) => Ok(ValueType::Integer),
            Some(TokenType::FloatType) => Ok(ValueType::Float),
            Some(TokenType::StringType) => Ok(ValueType::String),
            Some(TokenType::BooleanType) => Ok(ValueType::Boolean),
            Some(TokenType::Identifier(type_name)) => Ok(ValueType::Class(Rc::clone(type_name))),
            _ => Err(()),
        }
    }

    fn return_type(&mut self) -> Result<Option<ValueType>, ParserStatus> {
        if let Some(TokenType::Arrow) = self.current_token_type() {
            self.current += 1;

            match self.data_type() {
                Ok(return_type) => {
                    self.current += 1;
                    Ok(Some(return_type))
                }
                Err(_) => {
                    self.error
                        .add(ParserErrType::FunctionDeclarationExpectedReturnType(
                            self.tokens[self.current - 1].clone(),
                            self.tokens.get(self.current).cloned(),
                        ));
                    Err(ParserStatus::Unwind)
                }
            }
        } else {
            Ok(None)
        }
    }

    fn parameters(
        &mut self,
        main_token_pos: usize,
    ) -> Result<Vec<(ValueType, Rc<String>)>, ParserStatus> {
        match self.consume_token_type() {
            Some(TokenType::LeftParen) => (),
            _ => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedLeftParen(
                        self.tokens[self.current - 2].clone(),
                        self.tokens.get(self.current - 1).cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };

        let mut parameters = vec![];

        match self.current_token_type() {
            Some(TokenType::RightParen) => {
                self.current += 1;
            }
            _ => loop {
                let parameter_data_type = match self.data_type() {
                    Ok(data_type) => data_type,
                    Err(_) => {
                        self.error
                            .add(ParserErrType::FunctionDeclarationExpectedParameterType(
                                self.tokens[self.current - 1].clone(),
                                self.tokens.get(self.current).cloned(),
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
                self.current += 1;

                let parameter_name = match self.consume_token_type() {
                    Some(TokenType::Identifier(name)) => Rc::clone(name),
                    _ => {
                        self.error
                            .add(ParserErrType::FunctionDeclarationExpectedParameterName(
                                self.tokens[main_token_pos].clone(),
                                self.tokens.get(self.current - 1).cloned(),
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };

                parameters.push((parameter_data_type, parameter_name));

                match self.consume_token_type() {
                    Some(TokenType::Comma) => continue,
                    Some(TokenType::RightParen) => break,
                    _ => {
                        self.error.add(
                            ParserErrType::FunctionDeclarationExpectedCommaOrRightParen(
                                self.tokens[main_token_pos].clone(),
                                self.tokens.get(self.current - 1).cloned(),
                            ),
                        );
                        return Err(ParserStatus::Unwind);
                    }
                };
            },
        }

        Ok(parameters)
    }

    fn add_scope_parameter(
        &mut self,
        value_type: &ValueType,
        name: Rc<String>,
        scope: &mut IndexMap<Rc<String>, Value>,
    ) {
        match value_type {
            ValueType::Integer => {
                scope.insert(name, Value::Integer(self.integer_next_id));
                self.integer_next_id += 1;
            }
            ValueType::Float => {
                scope.insert(name, Value::Float(self.float_next_id));
                self.float_next_id += 1;
            }
            ValueType::String => {
                scope.insert(name, Value::String(self.string_next_id));
                self.string_next_id += 1;
            }
            ValueType::Boolean => {
                scope.insert(name, Value::Boolean(self.boolean_next_id));
                self.boolean_next_id += 1;
            }
            ValueType::Any => unreachable!("Zonkey code cannot use the Any type"),
            ValueType::Class(class_type) => match self.class_declarations.remove(class_type) {
                Some(cd) => {
                    let mut properties = IndexMap::new();

                    for (name, value_type) in &cd.properties {
                        self.add_scope_parameter(value_type, Rc::clone(name), &mut properties);
                    }

                    self.class_declarations.insert(Rc::clone(class_type), cd);

                    scope.insert(
                        name,
                        Value::Object(Object {
                            class_declaration: Rc::clone(class_type),
                            properties,
                        }),
                    );
                }
                None => panic!("Function parameter of undefined class type"),
            },
        }
    }
}
