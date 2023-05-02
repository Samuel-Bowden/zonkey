use crate::{parser::production::statement::prelude::*, parser_debug};
use rustc_hash::FxHashMap;

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
        self.environments.push(FxHashMap::default());

        let integer_point = self.integer_next_id;
        let float_point = self.float_next_id;
        let string_point = self.string_next_id;
        let boolean_point = self.boolean_next_id;
        let object_point = self.object_next_id;

        loop {
            match self.current_token_type() {
                Some(TokenType::RightBrace) => {
                    self.current += 1;
                    self.environments.pop();

                    self.integer_next_id = integer_point;
                    self.float_next_id = float_point;
                    self.string_next_id = string_point;
                    self.boolean_next_id = boolean_point;
                    self.object_next_id = object_point;

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
                                parser_debug!(format!(
                                    "Current Token {:?}",
                                    self.current_token_type()
                                )
                                .as_str());
                                match self.current_token_type() {
                                    // Statement end
                                    Some(TokenType::SemiColon) => {
                                        if braces_seen == 0 {
                                            self.current += 1;
                                            parser_debug!(format!(
                                                "Found semicolon at {}",
                                                self.current
                                            )
                                            .as_str());
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

                            parser_debug!(format!(
                                "Synchronised inside block successfully at {:?}",
                                self.current_token_type()
                            )
                            .as_str());
                        }
                        Err(ParserStatus::End) => return Err(ParserStatus::End),
                    }
                }
                None => {
                    self.error.add(ParserErrType::BlockExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                    ));

                    return Err(ParserStatus::End);
                }
            }
        }
    }
}
