use crate::{
    parser::production::statement::prelude::*,
    parser::value::{Object, Value, ValueType},
    stmt::ConstructionType,
};
use indexmap::IndexMap;

impl Parser {
    pub fn variable_init(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("variable_init");
        self.current += 1;

        let name = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
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
                    name,
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
                    let (object, types) = self.create_object(class_name)?;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name, Value::Object(object));

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
                        .insert(name.clone(), Value::Integer(id));
                    Ok(Stmt::IntegerVariableInitialisation(val))
                }
                Expr::Float(val) => {
                    let id = self.float_next_id;
                    self.float_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), Value::Float(id));
                    Ok(Stmt::FloatVariableInitialisation(val))
                }
                Expr::String(val) => {
                    let id = self.string_next_id;
                    self.string_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), Value::String(id));
                    Ok(Stmt::StringVariableInitialisation(val))
                }
                Expr::Boolean(val) => {
                    let id = self.boolean_next_id;
                    self.boolean_next_id += 1;
                    self.value_stack
                        .last_mut()
                        .unwrap()
                        .insert(name.clone(), Value::Boolean(id));
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
                Expr::Object(..) => {
                    todo!()
                }
            }
        }
    }

    fn create_object(
        &mut self,
        class_name: String,
    ) -> Result<(Object, Vec<ConstructionType>), ParserStatus> {
        let declaration = match self.class_declarations.get(&class_name) {
            Some(declaration) => declaration,
            None => {
                panic!("Class not declared")
            }
        };

        let mut object = Object {
            class_declaration: class_name.clone(),
            properties: IndexMap::new(),
        };

        let mut types = vec![];

        for (name, value_type) in declaration.properties.clone() {
            match value_type {
                ValueType::Integer => {
                    object
                        .properties
                        .insert(name.to_string(), Value::Integer(self.integer_next_id));
                    self.integer_next_id += 1;
                    types.push(ConstructionType::Integer);
                }
                ValueType::Float => {
                    object
                        .properties
                        .insert(name.to_string(), Value::Float(self.float_next_id));
                    self.float_next_id += 1;
                    types.push(ConstructionType::Float);
                }
                ValueType::String => {
                    object
                        .properties
                        .insert(name.to_string(), Value::String(self.string_next_id));
                    self.string_next_id += 1;
                    types.push(ConstructionType::String);
                }
                ValueType::Boolean => {
                    object
                        .properties
                        .insert(name.to_string(), Value::Boolean(self.boolean_next_id));
                    self.boolean_next_id += 1;
                    types.push(ConstructionType::Boolean);
                }
                ValueType::Class(class_type) => {
                    let (this_object, these_types) = self.create_object(class_type)?;
                    types.push(ConstructionType::Class(these_types));
                    object.properties.insert(name, Value::Object(this_object));
                }
            }
        }

        Ok((object, types))
    }
}
