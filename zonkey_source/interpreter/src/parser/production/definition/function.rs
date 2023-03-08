use crate::{
    parser::declaration::FunctionDeclaration,
    parser::production::prelude::*,
    parser::value::ValueType,
    parser::value::{Object, Value},
};
use indexmap::IndexMap;

impl Parser {
    pub fn function(&mut self) -> Result<(), ParserStatus> {
        debug_information!("function");

        // First stage - parse function
        let function_token_pos = self.current;
        self.current += 1;

        let function_name = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            t => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedName(
                        self.tokens[function_token_pos].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftParen,
                start,
                ..
            }) => *start,
            t => {
                self.error
                    .add(ParserErrType::FunctionDeclarationExpectedLeftParen(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        // Get function parameters
        let mut parameters = vec![];

        match self.current_token_type() {
            Some(TokenType::RightParen) => {
                self.current += 1;
            }
            _ => loop {
                let parameter_data_type = match self.data_type() {
                    Ok(data_type) => data_type,
                    Err(t) => {
                        self.error
                            .add(ParserErrType::FunctionDeclarationExpectedParameterType(
                                self.tokens[self.current - 1].clone(),
                                t,
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
                self.current += 1;

                let parameter_name = match self.tokens.get(self.current) {
                    Some(Token {
                        token_type: TokenType::Identifier(name),
                        ..
                    }) => name.clone(),
                    t => {
                        self.error
                            .add(ParserErrType::FunctionDeclarationExpectedParameterName(
                                self.tokens[function_token_pos].clone(),
                                t.cloned(),
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                };
                self.current += 1;

                parameters.push((parameter_data_type, parameter_name));

                match self.tokens.get(self.current) {
                    Some(Token {
                        token_type: TokenType::Comma,
                        ..
                    }) => {
                        self.current += 1;
                        continue;
                    }
                    Some(Token {
                        token_type: TokenType::RightParen,
                        ..
                    }) => {
                        self.current += 1;
                        break;
                    }
                    t => {
                        self.error.add(
                            ParserErrType::FunctionDeclarationExpectedCommaOrRightParen(
                                self.tokens[function_token_pos].clone(),
                                t.cloned(),
                            ),
                        );
                        return Err(ParserStatus::Unwind);
                    }
                };
            },
        }

        // Get return type if present
        let return_data_type = if let Some(TokenType::Arrow) = self.current_token_type() {
            self.current += 1;

            match self.data_type() {
                Ok(return_type) => {
                    self.current += 1;
                    Some(return_type)
                }
                Err(t) => {
                    self.error
                        .add(ParserErrType::FunctionDeclarationExpectedReturnType(
                            self.tokens[self.current - 1].clone(),
                            t,
                        ));
                    return Err(ParserStatus::Unwind);
                }
            }
        } else {
            None
        };

        // Second stage - parse function body
        // Add parameters to the first value scope of function body
        let mut function_scope = IndexMap::new();
        for (value_type, name) in &parameters {
            self.add_function_parameter(value_type, name, &mut function_scope);
        }

        self.value_stack.push(function_scope);

        let function_declaration = FunctionDeclaration {
            id: self.callables.len(),
            parameters,
            return_data_type,
        };

        self.function_declarations
            .insert(function_name, function_declaration.clone());

        self.current_function_declaration = Some(function_declaration);

        // Parse the function block
        let block = self.block()?;

        // Clean value stack after it has been parsed
        self.value_stack.clear();
        self.integer_next_id = 0;
        self.float_next_id = 0;
        self.string_next_id = 0;
        self.boolean_next_id = 0;

        self.current_function_declaration = None;

        // Finally add function to callables
        self.callables.push(block);

        Ok(())
    }

    fn add_function_parameter(
        &mut self,
        value_type: &ValueType,
        name: &str,
        scope: &mut IndexMap<String, Value>,
    ) {
        match value_type {
            ValueType::Integer => {
                scope.insert(name.to_string(), Value::Integer(self.integer_next_id));
                self.integer_next_id += 1;
            }
            ValueType::Float => {
                scope.insert(name.to_string(), Value::Float(self.float_next_id));
                self.float_next_id += 1;
            }
            ValueType::String => {
                scope.insert(name.to_string(), Value::String(self.string_next_id));
                self.string_next_id += 1;
            }
            ValueType::Boolean => {
                scope.insert(name.to_string(), Value::Boolean(self.boolean_next_id));
                self.boolean_next_id += 1;
            }
            ValueType::Class(class_type) => match self.class_declarations.remove(class_type) {
                Some(cd) => {
                    let mut properties = IndexMap::new();

                    for (name, value_type) in &cd.properties {
                        self.add_function_parameter(value_type, name, &mut properties);
                    }

                    self.class_declarations.insert(class_type.to_string(), cd);

                    scope.insert(
                        name.to_string(),
                        Value::Object(Object {
                            class_declaration: class_type.to_string(),
                            properties,
                        }),
                    );
                }
                None => panic!("Function parameter of undefined class type"),
            },
        }
    }
}
