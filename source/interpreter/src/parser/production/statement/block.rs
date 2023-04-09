use crate::{parser::production::statement::prelude::*, parser_debug};
use indexmap::IndexMap;

impl Parser {
    pub fn block(&mut self) -> Result<Stmt, ParserStatus> {
        debug_information!("block");

        let open_brace_pos = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftBrace,
                ..
            }) => self.current,
            t => {
                self.error.add(ParserErrType::BlockExpectedLeftBrace(
                    self.tokens[self.current - 1].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let mut statements = vec![];
        self.value_stack.push(IndexMap::new());

        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;

        loop {
            match self.current_token_type() {
                Some(TokenType::RightBrace) => {
                    self.current += 1;
                    self.value_stack.pop();

                    self.integer_next_id = integer_point;
                    self.float_next_id = float_point;
                    self.string_next_id = string_point;
                    self.boolean_next_id = boolean_point;

                    return Ok(Stmt::Block(statements, self.stack()));
                }
                Some(_) => {
                    match self.statement() {
                        Ok(s) => statements.push(s),
                        Err(ParserStatus::Unwind) => {
                            // Best effort to synchronise on the end or start of statements
                            parser_debug!("Synchronising inside block");

                            let mut braces_seen = 0;

                            loop {
                                match self.current_token_type() {
                                    // Statement end
                                    Some(TokenType::SemiColon) => {
                                        if braces_seen == 0 {
                                            self.current += 1;
                                            break;
                                        }
                                    }
                                    // Statement start
                                    Some(
                                        TokenType::Let
                                        | TokenType::If
                                        | TokenType::For
                                        | TokenType::Return
                                        | TokenType::Loop
                                        | TokenType::While,
                                    ) => {
                                        if braces_seen == 0 {
                                            break;
                                        }
                                    }
                                    Some(TokenType::RightBrace) => {
                                        if braces_seen == 0 {
                                            break;
                                        } else {
                                            braces_seen -= 1;
                                        }
                                    }
                                    Some(TokenType::LeftBrace) => {
                                        braces_seen += 1;
                                    }
                                    None => break,
                                    _ => (),
                                }

                                self.current += 1;
                            }

                            parser_debug!("Synchronised inside block successfully");
                        }
                        Err(ParserStatus::End) => return Err(ParserStatus::End),
                    }
                }
                None => {
                    self.error.add(ParserErrType::BlockExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                        self.tokens[self.current - 1].clone(),
                    ));

                    return Err(ParserStatus::End);
                }
            }
        }
    }
}
