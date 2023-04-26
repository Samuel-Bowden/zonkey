use crate::{
    expr::{Expr, ObjectExpr},
    parser::{
        declaration::{CallableDeclaration, CallableType, ClassDeclaration, ConstructionType},
        location::Location,
        production::definition::prelude::*,
        value::ValueType,
    },
    stmt::Stmt,
};
use rustc_hash::FxHashMap;
use std::rc::Rc;

impl Parser {
    pub fn class(&mut self) -> Result<(), ParserStatus> {
        debug_information!("class");

        let class_token_pos = self.current;
        self.current += 1;

        let class_name = match self.consume_token_type() {
            Some(TokenType::Identifier(name)) => Rc::clone(name),
            _ => {
                self.error.add(ParserErrType::ClassDeclarationExpectedName(
                    self.tokens[class_token_pos].clone(),
                    self.tokens.get(self.current - 1).cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };

        if let Some(_) = self.class_declarations.get(&class_name) {
            self.error.add(ParserErrType::ClassRedeclared(
                self.tokens[self.current - 1].clone(),
            ));
            return Err(ParserStatus::Unwind);
        }

        if let "Integer" | "Float" | "String" | "Boolean" | "Printable" | "Element" =
            class_name.as_str()
        {
            self.error.add(ParserErrType::InbuiltType(
                self.tokens[self.current - 1].clone(),
            ));
            return Err(ParserStatus::Unwind);
        }

        let open_brace_pos = match self.consume_token_type() {
            Some(TokenType::LeftBrace) => self.current - 1,
            _ => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedLeftBrace(
                        self.tokens[self.current - 2].clone(),
                        self.tokens.get(self.current - 1).cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };

        let mut properties = FxHashMap::default();
        let mut class_integer_next_id = 0;
        let mut class_float_next_id = 0;
        let mut class_string_next_id = 0;
        let mut class_boolean_next_id = 0;
        let mut class_object_next_id = 0;
        let mut property_default_expressions = vec![];

        while let Some(dt) = self.data_type()? {
            self.current += 1;

            let property_name_pos = self.current;
            let property_name = match self.consume_token_type() {
                Some(TokenType::Identifier(name)) => Rc::clone(name),
                _ => {
                    self.error
                        .add(ParserErrType::ClassDeclarationExpectedPropertyName(
                            self.tokens[self.current - 2].clone(),
                            self.tokens.get(self.current - 1).cloned(),
                        ));
                    return Err(ParserStatus::End);
                }
            };

            if let Some(_) = properties.get(&property_name) {
                self.error
                    .add(ParserErrType::ClassDeclarationRedeclaredProperty(
                        self.tokens[self.current - 1].clone(),
                        property_name.to_string(),
                    ));
                return Err(ParserStatus::End);
            }

            match dt {
                ValueType::Integer => {
                    properties.insert(property_name, Location::Integer(class_integer_next_id));
                    class_integer_next_id += 1;
                    property_default_expressions.push(ConstructionType::Integer);
                }
                ValueType::Float => {
                    properties.insert(property_name, Location::Float(class_float_next_id));
                    class_float_next_id += 1;
                    property_default_expressions.push(ConstructionType::Float);
                }
                ValueType::String => {
                    properties.insert(property_name, Location::String(class_string_next_id));
                    class_string_next_id += 1;
                    property_default_expressions.push(ConstructionType::String);
                }
                ValueType::Boolean => {
                    properties.insert(property_name, Location::Boolean(class_boolean_next_id));
                    class_boolean_next_id += 1;
                    property_default_expressions.push(ConstructionType::Boolean);
                }
                ValueType::Class(class) => {
                    properties.insert(
                        property_name.clone(),
                        Location::Object(Rc::clone(&class), class_object_next_id),
                    );
                    class_object_next_id += 1;
                    property_default_expressions.push(ConstructionType::NullPointer(
                        self.tokens[property_name_pos].clone(),
                    ));
                }
                ValueType::Printable | ValueType::Element | ValueType::Generic => {
                    unreachable!("Zonkey code cannot use these types")
                }
            }

            match self.consume_token_type() {
                Some(TokenType::SemiColon) => (),
                _ => {
                    self.error
                        .add(ParserErrType::ClassDeclarationUnterminatedProperty(
                            self.tokens[self.current - 2].clone(),
                            self.tokens.get(self.current - 1).cloned(),
                        ));
                    return Err(ParserStatus::Unwind);
                }
            };
        }

        let methods = FxHashMap::default();

        let property_default_expressions = Rc::new(property_default_expressions);

        let class_declaration = ClassDeclaration { methods };

        self.current_properties = Some(properties);

        self.class_declarations
            .insert(Rc::clone(&class_name), class_declaration);

        let mut constructor_declared = false;

        while let Some(TokenType::Constructor | TokenType::Method) = self.current_token_type() {
            if let Some(TokenType::Constructor) = self.current_token_type() {
                if constructor_declared {
                    self.error
                        .add(ParserErrType::ClassDeclarationRedeclaredConstructor(
                            self.tokens[self.current].clone(),
                        ));
                    return Err(ParserStatus::End);
                }

                constructor_declared = true;

                self.current += 1;

                let parameters = self.parameters()?;
                let mut parameter_value_types = vec![];

                let mut constructor_scope = FxHashMap::default();

                constructor_scope.insert(
                    Rc::new("self".to_string()),
                    Location::Object(Rc::clone(&class_name), self.object_next_id),
                );
                self.object_next_id += 1;

                for (value_type, name) in parameters {
                    self.add_scope_parameter(&value_type, name, &mut constructor_scope)?;
                    parameter_value_types.push(value_type);
                }
                self.environments.push(constructor_scope);

                let constructor_declaration = CallableDeclaration {
                    callable_type: CallableType::Zonkey(self.callables.len()),
                    parameters: parameter_value_types,
                    return_type: Some(ValueType::Class(Rc::clone(&class_name))),
                };

                self.function_declarations
                    .insert(Rc::clone(&class_name), constructor_declaration);

                // The newly constructed object is returned automatically, the user cannot return it in
                // a constructor
                self.current_return_type = None;

                let block = self.block()?;

                // Clean environments after it has been parsed
                self.environments.clear();
                self.integer_next_id = 0;
                self.float_next_id = 0;
                self.string_next_id = 0;
                self.boolean_next_id = 0;
                self.object_next_id = 0;

                self.current_return_type = None;

                self.callables.push(Rc::new(Stmt::Block(
                    vec![
                        Stmt::SelfInitialisation(ObjectExpr::Constructor(Rc::clone(
                            &property_default_expressions,
                        ))),
                        block,
                        Stmt::Return(Some(Expr::Object(
                            Rc::clone(&class_name),
                            ObjectExpr::Variable(0),
                        ))),
                    ],
                    self.stack(),
                )))
            } else {
                let method_token_pos = self.current;
                self.current += 1;

                let method_name = match self.consume_token_type() {
                    Some(TokenType::Identifier(name)) => Rc::clone(name),
                    _ => {
                        self.error
                            .add(ParserErrType::ClassDeclarationExpectedMethodName(
                                self.tokens[self.current - 2].clone(),
                                self.tokens.get(self.current - 1).cloned(),
                            ));
                        return Err(ParserStatus::End);
                    }
                };

                if let Some(_) = self
                    .class_declarations
                    .get_mut(&class_name)
                    .unwrap()
                    .methods
                    .get(&method_name)
                {
                    self.error
                        .add(ParserErrType::ClassDeclarationRedeclaredMethod(
                            self.tokens[self.current - 1].clone(),
                            method_name.to_string(),
                        ));
                    return Err(ParserStatus::End);
                }

                let parameters = self.parameters()?;
                let return_type = self.return_type()?;

                let mut method_scope = FxHashMap::default();
                let mut parameter_value_types = vec![];

                method_scope.insert(
                    Rc::new("self".to_string()),
                    Location::Object(Rc::clone(&class_name), self.object_next_id),
                );
                self.object_next_id += 1;

                for (value_type, name) in parameters {
                    self.add_scope_parameter(&value_type, name, &mut method_scope)?;
                    parameter_value_types.push(value_type);
                }

                self.environments.push(method_scope);

                let method_declaration = CallableDeclaration {
                    callable_type: CallableType::Zonkey(self.callables.len()),
                    parameters: parameter_value_types,
                    return_type: return_type.clone(),
                };

                self.class_declarations
                    .get_mut(&class_name)
                    .unwrap()
                    .methods
                    .insert(Rc::clone(&method_name), Rc::new(method_declaration));

                self.current_return_type = return_type;

                let block = self.block()?;

                if let Some(return_type) = &self.current_return_type {
                    if !self.returned_value {
                        self.error
                            .add(ParserErrType::DeclarationDidNotReturnValueInAllCases(
                                self.tokens[method_token_pos + 1].clone(),
                                return_type.clone(),
                            ));
                        return Err(ParserStatus::Unwind);
                    }
                }

                // Clean value stack after it has been parsed
                self.environments.clear();
                self.integer_next_id = 0;
                self.float_next_id = 0;
                self.string_next_id = 0;
                self.boolean_next_id = 0;
                self.object_next_id = 0;

                self.current_return_type = None;

                self.callables.push(block.into());
            }
        }

        if constructor_declared == false {
            self.error.add(ParserErrType::ClassDeclarationNoConstructor(
                self.tokens[class_token_pos].clone(),
            ));
            return Err(ParserStatus::Unwind);
        }

        self.current_properties = None;

        match self.current_token_type() {
            Some(TokenType::RightBrace) => (),
            Some(TokenType::Identifier(_)) => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectPropertyTop(
                        self.tokens[self.current].clone(),
                    ));
                return Err(ParserStatus::Unwind);
            }
            _ => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                        self.tokens.get(self.current).cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };

        self.current += 1;

        Ok(())
    }
}
