use crate::{
    expr::{Expr, ObjectExpr},
    parser::declaration::ClassDeclaration,
    parser::{
        declaration::{CallableDeclaration, CallableType},
        production::definition::prelude::*,
        value::{Value, ValueType},
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

        let open_brace_pos = match self.consume_token_type() {
            Some(TokenType::LeftBrace) => self.current,
            _ => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedLeftBrace(
                        self.tokens[self.current - 2].clone(),
                        self.tokens.get(self.current - 1).cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };

        // Parse properties
        let mut properties = FxHashMap::default();
        while let Ok(dt) = self.data_type() {
            self.current += 1;

            match self.consume_token_type() {
                Some(TokenType::Identifier(name)) => properties.insert(Rc::clone(name), dt),
                _ => {
                    self.error
                        .add(ParserErrType::ClassDeclarationExpectedPropertyName(
                            self.tokens[self.current - 2].clone(),
                            self.tokens.get(self.current - 1).cloned(),
                        ));
                    return Err(ParserStatus::Unwind);
                }
            };

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

        let class_declaration = ClassDeclaration {
            properties,
            methods,
        };

        self.class_declarations
            .insert(Rc::clone(&class_name), class_declaration);

        // Parse constructor
        if let Some(TokenType::Constructor) = self.current_token_type() {
            self.current += 1;

            let parameters = self.parameters(self.current - 1)?;
            let mut parameter_value_types = vec![];

            let mut constructor_scope = IndexMap::new();

            let id = self.object_next_id;
            self.object_next_id += 1;
            let (object, types) = self.create_object(Rc::clone(&class_name))?;
            self.objects.insert(id, Rc::new(object));
            constructor_scope.insert(
                Rc::new("self".to_string()),
                Value::Object(Rc::clone(&class_name), id),
            );

            for (value_type, name) in parameters {
                self.add_scope_parameter(&value_type, name, &mut constructor_scope)?;
                parameter_value_types.push(value_type);
            }

            self.value_stack.push(constructor_scope);

            let return_type = Some(ValueType::Class(Rc::clone(&class_name)));

            let constructor_declaration = CallableDeclaration {
                callable_type: CallableType::Zonkey(self.callables.len()),
                parameters: parameter_value_types,
                return_type: return_type.clone(),
            };

            self.function_declarations
                .insert(Rc::clone(&class_name), Rc::new(constructor_declaration));

            self.current_return_type = return_type;

            let block = self.block()?;

            // Clean value stack after it has been parsed
            self.value_stack.clear();
            self.objects.clear();
            self.integer_next_id = 0;
            self.float_next_id = 0;
            self.string_next_id = 0;
            self.boolean_next_id = 0;
            self.object_next_id = 0;

            self.current_return_type = None;

            self.callables.push(Rc::new(Stmt::Block(
                vec![
                    Stmt::DefaultConstructor(types),
                    block,
                    Stmt::Return(Some(Expr::Object(
                        Rc::clone(&class_name),
                        ObjectExpr::Variable(0),
                    ))),
                ],
                self.stack(),
            )))
        }

        // Parse methods
        while let Some(TokenType::Method) = self.current_token_type() {
            self.current += 1;

            let method_name = match self.consume_token_type() {
                Some(TokenType::Identifier(name)) => Rc::clone(name),
                _ => {
                    panic!("Expected class name")
                }
            };

            let parameters = self.parameters(self.current - 1)?;
            let return_type = self.return_type()?;

            let mut method_scope = IndexMap::new();
            let mut parameter_value_types = vec![];

            // Add self as parameter
            self.add_scope_parameter(
                &ValueType::Class(Rc::clone(&class_name)),
                Rc::new("self".to_string()),
                &mut method_scope,
            )?;
            parameter_value_types.push(ValueType::Class(Rc::clone(&class_name)));

            for (value_type, name) in parameters {
                self.add_scope_parameter(&value_type, name, &mut method_scope)?;
                parameter_value_types.push(value_type);
            }

            self.value_stack.push(method_scope);

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

            // Clean value stack after it has been parsed
            self.value_stack.clear();
            self.objects.clear();
            self.integer_next_id = 0;
            self.float_next_id = 0;
            self.string_next_id = 0;
            self.boolean_next_id = 0;
            self.object_next_id = 0;

            self.current_return_type = None;

            self.callables.push(block.into());
        }

        match self.consume_token_type() {
            Some(TokenType::RightBrace) => (),
            _ => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                        self.tokens[self.current - 2].clone(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };

        Ok(())
    }
}
