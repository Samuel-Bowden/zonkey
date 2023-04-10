use rustc_hash::FxHashMap;

use crate::{
    expr::{BooleanExpr, Expr, FloatExpr, IntegerExpr, ObjectExpr, StringExpr},
    parser::{
        declaration::{CallableDeclaration, CallableType, ClassDeclaration, ClassType},
        production::definition::prelude::*,
        value::{Value, ValueType},
    },
    standard_prelude::calls::NativeCallObject,
    stmt::Stmt,
};
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

        let mut properties = FxHashMap::default();
        let mut class_integer_next_id = 0;
        let mut class_float_next_id = 0;
        let mut class_string_next_id = 0;
        let mut class_boolean_next_id = 0;
        let mut class_object_next_id = 0;
        let mut property_default_expressions = vec![];

        while let Ok(dt) = self.data_type() {
            self.current += 1;

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
                    properties.insert(property_name, Value::Integer(class_integer_next_id));
                    class_integer_next_id += 1;
                    property_default_expressions.push(Expr::Integer(IntegerExpr::Literal(0)));
                }
                ValueType::Float => {
                    properties.insert(property_name, Value::Float(class_float_next_id));
                    class_float_next_id += 1;
                    property_default_expressions.push(Expr::Float(FloatExpr::Literal(0.)));
                }
                ValueType::String => {
                    properties.insert(property_name, Value::String(class_string_next_id));
                    class_string_next_id += 1;
                    property_default_expressions
                        .push(Expr::String(StringExpr::Literal("".to_string().into())));
                }
                ValueType::Boolean => {
                    properties.insert(property_name, Value::Boolean(class_boolean_next_id));
                    class_boolean_next_id += 1;
                    property_default_expressions.push(Expr::Boolean(BooleanExpr::Literal(false)));
                }
                ValueType::Class(class) => {
                    properties.insert(
                        property_name,
                        Value::Object(Rc::clone(&class), class_object_next_id),
                    );
                    class_object_next_id += 1;
                    property_default_expressions.push(Expr::Object(
                        Rc::clone(&class),
                        match &self
                            .class_declarations
                            .get(&class)
                            .expect("Class not declared")
                            .class_type
                        {
                            ClassType::Native => ObjectExpr::NativeCall(match class.as_str() {
                                "Page" => NativeCallObject::PageConstructor,
                                "Button" => NativeCallObject::ButtonConstructor(Box::new(
                                    StringExpr::Literal("".to_string().into()),
                                )),
                                "Text" => NativeCallObject::TextConstructor(Box::new(
                                    StringExpr::Literal("".to_string().into()),
                                )),
                                "Input" => NativeCallObject::InputConstructor(Box::new(
                                    StringExpr::Literal("".to_string().into()),
                                )),
                                "Hyperlink" => NativeCallObject::HyperlinkConstructor(
                                    Box::new(StringExpr::Literal("".to_string().into())),
                                    Box::new(StringExpr::Literal("".to_string().into())),
                                ),
                                "Row" => NativeCallObject::RowConstructor,
                                "Column" => NativeCallObject::ColumnConstructor,
                                "Image" => NativeCallObject::ImageConstructor(
                                    Box::new(StringExpr::Literal("".to_string().into())),
                                ),
                                _ => unreachable!(),
                            }),
                            ClassType::Zonkey(exprs) => ObjectExpr::Constructor(Rc::clone(&exprs)),
                        },
                    ));
                }
                ValueType::Any | ValueType::Element => {
                    unreachable!("Zonkey code cannot use this type")
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

        let class_declaration = ClassDeclaration {
            class_type: ClassType::Zonkey(Rc::clone(&property_default_expressions)),
            methods,
        };

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

                let parameters = self.parameters(self.current - 1)?;
                let mut parameter_value_types = vec![];

                let mut constructor_scope = IndexMap::new();

                constructor_scope.insert(
                    Rc::new("self".to_string()),
                    Value::Object(Rc::clone(&class_name), self.object_next_id),
                );
                self.object_next_id += 1;

                for (value_type, name) in parameters {
                    self.add_scope_parameter(&value_type, name, &mut constructor_scope)?;
                    parameter_value_types.push(value_type);
                }
                self.value_stack.push(constructor_scope);

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

                // Clean value stack after it has been parsed
                self.value_stack.clear();
                self.integer_next_id = 0;
                self.float_next_id = 0;
                self.string_next_id = 0;
                self.boolean_next_id = 0;
                self.object_next_id = 0;

                self.current_return_type = None;

                self.callables.push(Rc::new(Stmt::Block(
                    vec![
                        Stmt::ObjectVariableInitialisation(ObjectExpr::Constructor(Rc::clone(
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

                let parameters = self.parameters(self.current - 1)?;
                let return_type = self.return_type()?;

                let mut method_scope = IndexMap::new();
                let mut parameter_value_types = vec![];

                method_scope.insert(
                    Rc::new("self".to_string()),
                    Value::Object(Rc::clone(&class_name), self.object_next_id),
                );
                self.object_next_id += 1;

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

        match self.consume_token_type() {
            Some(TokenType::RightBrace) => (),
            Some(
                TokenType::IntegerType
                | TokenType::FloatType
                | TokenType::StringType
                | TokenType::BooleanType
                | TokenType::Identifier(_),
            ) => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectPropertyTop(
                        self.tokens[self.current - 1].clone(),
                    ));
                return Err(ParserStatus::Unwind);
            }
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
