mod class;
mod function;
mod prelude;
mod start;

use std::rc::Rc;

use crate::{
    parser::production::definition::prelude::*, parser::value::{Value, print_type}, parser::value::ValueType, standard_prelude::classes::array};

impl Parser {
    // Helper functions used by some definitions to convert token_type to a value_type
    fn data_type(&mut self) -> Result<Option<ValueType>, ParserStatus> {
        match self.current_token_type() {
            Some(TokenType::Identifier(value_type)) => {
                match value_type.as_str() {
                    "Integer" => Ok(Some(ValueType::Integer)),
                    "Float" => Ok(Some(ValueType::Float)),
                    "String" => Ok(Some(ValueType::String)),
                    "Boolean" => Ok(Some(ValueType::Boolean)),
                    _ => match self.class_declarations.get(value_type) {
                        Some(_) => Ok(Some(ValueType::Class(Rc::clone(&value_type)))),
                        None => {
                            self.error.add(ParserErrType::ClassNotFound(
                                self.tokens[self.current].clone(),
                            ));
                            Err(ParserStatus::Unwind)
                        }
                    }
                }
            }
            Some(TokenType::LeftBracket) => {
                // An array type
                self.current += 1;
                let data_type = self.data_type()?;
                
                let value_type = match &data_type {
                    Some(vt) => vt,
                    None => {
                        self.error.add(ParserErrType::ArrayEmptyType(
                            self.tokens[self.current - 1].clone(),
                            self.tokens.get(self.current).cloned(),
                        ));
                        return Err(ParserStatus::Unwind)
                    },
                };

                self.current += 1;
                match self.current_token_type() {
                    Some(TokenType::RightBracket) => {
                        let class_name = Rc::new(format!("[{}]", print_type(&data_type)));

                        self.class_declarations
                            .insert(Rc::clone(&class_name), array::new(class_name.clone(), value_type.clone()));

                        Ok(Some(ValueType::Class(
                            Rc::clone(&class_name)
                        )))
                    }
                    _ => {
                        self.error.add(ParserErrType::ArrayTypeNotClosed(
                            self.tokens[self.current - 1].clone(),
                            self.tokens.get(self.current).cloned(),
                        ));
                        return Err(ParserStatus::Unwind)
                    }
                }
            }
            _ => Ok(None),
        }
    }

    fn return_type(&mut self) -> Result<Option<ValueType>, ParserStatus> {
        if let Some(TokenType::Arrow) = self.current_token_type() {
            self.current += 1;

            match self.data_type()? {
                Some(return_type) => {
                    self.current += 1;
                    Ok(Some(return_type))
                }
                None => {
                    self.error
                        .add(ParserErrType::DeclarationExpectedReturnType(
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

    fn parameters(&mut self) -> Result<Vec<(ValueType, Rc<String>)>, ParserStatus> {
        match self.consume_token_type() {
            Some(TokenType::LeftParen) => (),
            _ => {
                self.error
                    .add(ParserErrType::DeclarationExpectedLeftParen(
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
                let parameter_data_type = match self.data_type()? {
                    Some(data_type) => data_type,
                    None => {
                        self.error
                            .add(ParserErrType::DeclarationExpectedParameterType(
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
                            .add(ParserErrType::DeclarationExpectedParameterName(
                                self.tokens[self.current - 2].clone(),
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
                            ParserErrType::DeclarationExpectedCommaOrRightParen(
                                self.tokens[self.current - 2].clone(),
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
    ) -> Result<(), ParserStatus> {
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
            ValueType::Class(class) => {
                scope.insert(name, Value::Object(Rc::clone(class), self.object_next_id));
                self.object_next_id += 1;
            }
            ValueType::Printable | ValueType::Element | ValueType::Generic => {
                unreachable!("Zonkey code cannot use these types")
            }
        }

        Ok(())
    }
}
