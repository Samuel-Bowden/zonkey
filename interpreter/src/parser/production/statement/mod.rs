mod block;
mod expression;
mod r#for;
mod r#if;
mod r#loop;
mod prelude;
mod r#return;
mod variable_init;
mod r#while;

use crate::parser::production::statement::prelude::*;

impl Parser {
    pub fn statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("statement");

        match self.current_token_type() {
            Some(TokenType::LeftBrace) => self.block(),
            Some(TokenType::If) => {
                self.current += 1;
                self.if_statement()
            }
            Some(TokenType::While) => {
                self.current += 1;
                self.while_statement()
            }
            Some(TokenType::Loop) => {
                self.current += 1;
                self.loop_statement()
            }
            Some(TokenType::For) => {
                self.current += 1;
                self.for_statement()
            }
            _ => Ok(self.terminated_statement()?),
        }
    }

    pub fn terminated_statement(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("terminated_statement");

        let statement = match self.current_token_type() {
            Some(TokenType::Return) => self.return_statement()?,
            Some(TokenType::Break) => {
                if self.loop_count > 0 {
                    self.current += 1;
                    Stmt::Break
                } else {
                    self.error.add(ParserErrType::BreakOutsideLoop(
                        self.tokens[self.current].clone(),
                    ));
                    return Err(ParserStatus::Unwind)
                }
            }
            Some(TokenType::Continue) => {
                if self.loop_count > 0 {
                    self.current += 1;
                    Stmt::Continue
                } else {
                    self.error.add(ParserErrType::ContinueOutsideLoop(
                        self.tokens[self.current].clone(),
                    ));
                    return Err(ParserStatus::Unwind)
                }
            }
            Some(TokenType::Let) => self.variable_init()?,
            _ => self.expression_statement()?,
        };

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::SemiColon,
                ..
            }) => {
                self.current += 1;
                return Ok(statement);
            }
            t => self.error.add(ParserErrType::UnterminatedStatement(
                self.tokens[self.current - 1].clone(),
                t.cloned(),
            )),
        }

        Err(ParserStatus::Unwind)
    }
}
