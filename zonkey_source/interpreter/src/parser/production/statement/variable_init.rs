use std::rc::Rc;

use crate::{
    parser::production::statement::prelude::*,
    parser::value::{Object, Value, ValueType},
    stmt::ConstructionType,
};
use rustc_hash::FxHashMap;

impl Parser {
    pub fn variable_init(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("variable_init");
        self.current += 1;

        let name = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => Rc::clone(name),
            t => {
                self.error
                    .add(ParserErrType::VariableDeclarationExpectedName(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        if let Some(_) = self.value_stack.last().unwrap().get(&name) {
            self.error
                .add(ParserErrType::VariableDeclarationAlreadyDeclared(
                    self.tokens[self.current - 1].clone(),
                    name.to_string(),
                ));
            return Err(ParserStatus::Unwind);
        }

        let equal_pos = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Equal,
                ..
            }) => self.current,
            t => {
                self.error
                    .add(ParserErrType::VariableDeclarationExpectedEqual(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        if let Some(TokenType::New) = self.current_token_type() {
            self.current += 1;

            match self.current_token_type().cloned() {
                Some(TokenType::Identifier(class_name)) => {
                    self.current += 1;
                    let (object, types) = self.create_object(Rc::clone(&class_name))?;

                    let object_id = self.object_next_id;
                    self.object_next_id += 1;

                    self.objects.insert(object_id, Rc::new(object));

                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::Object(class_name, object_id));

                    Ok(Stmt::ClassVariableInitialisation(types))
                }
                _ => {
                    panic!("Expected identifier")
                }
            }
        } else {
            let expr = self.expression()?;

            match expr {
                Expr::Integer(val) => {
                    let id = self.integer_next_id;
                    self.integer_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::Integer(id));
                    Ok(Stmt::IntegerVariableInitialisation(val))
                }
                Expr::Float(val) => {
                    let id = self.float_next_id;
                    self.float_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::Float(id));
                    Ok(Stmt::FloatVariableInitialisation(val))
                }
                Expr::String(val) => {
                    let id = self.string_next_id;
                    self.string_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::String(id));
                    Ok(Stmt::StringVariableInitialisation(val))
                }
                Expr::Boolean(val) => {
                    let id = self.boolean_next_id;
                    self.boolean_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::Boolean(id));
                    Ok(Stmt::BooleanVariableInitialisation(val))
                }
                Expr::None(_) => {
                    self.error
                        .add(ParserErrType::VariableDeclarationExprEvalNone(
                            self.tokens[equal_pos].end,
                            self.tokens[self.current].end,
                        ));
                    Err(ParserStatus::Unwind)
                }
                Expr::Object(class, val) => {
                    let id = self.object_next_id;
                    self.object_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::Object(Rc::clone(&class), id));
                    Ok(Stmt::ObjectVariableInitialisation(val))
                }
            }
        }
    }

    pub fn create_object(
        &mut self,
        class_name: Rc<String>,
    ) -> Result<(Object, Vec<ConstructionType>), ParserStatus> {
        let declaration = match self.class_declarations.get(&class_name) {
            Some(declaration) => declaration,
            None => {
                self.error.add(ParserErrType::ClassNotFound(
                    self.tokens[self.current - 1].clone(),
                    class_name.to_string(),
                ));

                return Err(ParserStatus::Unwind);
            }
        };

        let mut object = Object {
            properties: FxHashMap::default(),
            objects: FxHashMap::default(),
            integer_next_id: 0,
            float_next_id: 0,
            string_next_id: 0,
            boolean_next_id: 0,
            object_next_id: 0,
        };

        let mut types = vec![];

        for (name, value_type) in declaration.properties.clone() {
            match value_type {
                ValueType::Integer => {
                    object
                        .properties
                        .insert(name, Value::Integer(object.integer_next_id));
                    object.integer_next_id += 1;
                    types.push(ConstructionType::Integer);
                }
                ValueType::Float => {
                    object
                        .properties
                        .insert(name, Value::Float(object.float_next_id));
                    object.float_next_id += 1;
                    types.push(ConstructionType::Float);
                }
                ValueType::String => {
                    object
                        .properties
                        .insert(name, Value::String(object.string_next_id));
                    object.string_next_id += 1;
                    types.push(ConstructionType::String);
                }
                ValueType::Boolean => {
                    object
                        .properties
                        .insert(name, Value::Boolean(object.boolean_next_id));
                    object.boolean_next_id += 1;
                    types.push(ConstructionType::Boolean);
                }
                ValueType::Any => unreachable!("Zonkey code cannot use the Any type"),
                ValueType::Class(class) => {
                    let (new_object, new_types) = self.create_object(Rc::clone(&class))?;

                    object
                        .properties
                        .insert(name, Value::Object(class, object.object_next_id));
                    object
                        .objects
                        .insert(self.object_next_id, Rc::new(new_object));
                    object.object_next_id += 1;
                    types.push(ConstructionType::Class(new_types));
                }
            }
        }

        Ok((object, types))
    }
}
