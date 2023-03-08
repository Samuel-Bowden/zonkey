use crate::{parser::declaration::ClassDeclaration, parser::production::prelude::*};
use rustc_hash::FxHashMap;

impl Parser {
    pub fn class(&mut self) -> Result<(), ParserStatus> {
        debug_information!("class");

        let class_token_pos = self.current;
        self.current += 1;

        let class_name = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::Identifier(name),
                ..
            }) => name.clone(),
            t => {
                self.error.add(ParserErrType::ClassDeclarationExpectedName(
                    self.tokens[class_token_pos].clone(),
                    t.cloned(),
                ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let open_brace_pos = match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::LeftBrace,
                ..
            }) => self.current,
            t => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedLeftBrace(
                        self.tokens[self.current - 1].clone(),
                        t.cloned(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let mut properties = FxHashMap::default();

        while let Ok(dt) = self.data_type() {
            self.current += 1;

            match self.tokens.get(self.current) {
                Some(Token {
                    token_type: TokenType::Identifier(name),
                    ..
                }) => properties.insert(name.clone(), dt),
                t => {
                    self.error
                        .add(ParserErrType::ClassDeclarationExpectedPropertyName(
                            self.tokens[self.current - 1].clone(),
                            t.cloned(),
                        ));
                    return Err(ParserStatus::Unwind);
                }
            };
            self.current += 1;

            match self.tokens.get(self.current) {
                Some(Token {
                    token_type: TokenType::SemiColon,
                    ..
                }) => (),
                t => {
                    self.error
                        .add(ParserErrType::ClassDeclarationUnterminatedProperty(
                            self.tokens[self.current - 1].clone(),
                            t.cloned(),
                        ));
                    return Err(ParserStatus::Unwind);
                }
            };
            self.current += 1;
        }

        match self.tokens.get(self.current) {
            Some(Token {
                token_type: TokenType::RightBrace,
                ..
            }) => (),
            _ => {
                self.error
                    .add(ParserErrType::ClassDeclarationExpectedRightBrace(
                        self.tokens[open_brace_pos].clone(),
                        self.tokens[self.current - 1].clone(),
                    ));
                return Err(ParserStatus::Unwind);
            }
        };
        self.current += 1;

        let class_declaration = ClassDeclaration { properties };

        self.class_declarations
            .insert(class_name, class_declaration);

        Ok(())
    }
}
