use crate::{parser::production::statement::prelude::*, parser::value::Value};
use std::rc::Rc;

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

        if name.as_str() == "self" {
            self.error
                .add(ParserErrType::CannotCreateVariableCalledSelf(
                    self.tokens[self.current].clone(),
                ));
            return Err(ParserStatus::Unwind);
        }

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
                        self.tokens[self.current - 1].end,
                    ));
                Err(ParserStatus::Unwind)
            }
            Expr::Object(class, val) => {
                let id = self.object_next_id;
                self.object_next_id += 1;
                self.value_stack
                    .last_mut()
                    .unwrap()
                    .insert(name, Value::Object(class, id));
                Ok(Stmt::ObjectVariableInitialisation(val))
            }
        }
    }
}
