use std::rc::Rc;

use super::token::{Token, TokenType};
use crate::{err::lexer::LexerErr, lexer_debug};

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    graphemes: &'a Vec<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(graphemes: &'a Vec<&'a str>) -> Self {
        Self {
            tokens: vec![],
            start: 0,
            current: 0,
            graphemes,
        }
    }

    pub fn run(mut self) -> Result<Self, LexerErr> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        lexer_debug!("Printing tokens");

        #[cfg(debug_assertions)]
        for (i, token) in self.tokens.iter().enumerate() {
            println!("  {}: {:?}", i + 1, token);
        }

        Ok(self)
    }

    fn scan_token(&mut self) -> Result<(), LexerErr> {
        let grapheme = self.graphemes[self.current];
        self.current += 1;

        match grapheme {
            // Single grapheme
            "(" => self.add_token(TokenType::LeftParen),
            ")" => self.add_token(TokenType::RightParen),
            "{" => self.add_token(TokenType::LeftBrace),
            "}" => self.add_token(TokenType::RightBrace),
            "[" => self.add_token(TokenType::LeftBracket),
            "]" => self.add_token(TokenType::RightBracket),
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            ";" => self.add_token(TokenType::SemiColon),
            ":" => self.add_token(TokenType::Colon),
            "&" => self.add_token(TokenType::And),
            "|" => self.add_token(TokenType::Or),
            "@" => self.add_token(TokenType::At),
            // Single or double graphemes
            "!" => {
                let token = match self.next_grapheme("=") {
                    true => TokenType::BangEqual,
                    false => TokenType::Bang,
                };
                self.add_token(token);
            }
            "=" => {
                let token = match self.next_grapheme("=") {
                    true => TokenType::EqualEqual,
                    false => TokenType::Equal,
                };
                self.add_token(token);
            }
            "<" => {
                let token = match self.next_grapheme("=") {
                    true => TokenType::LessEqual,
                    false => TokenType::Less,
                };
                self.add_token(token);
            }
            ">" => {
                let token = match self.next_grapheme("=") {
                    true => TokenType::MoreEqual,
                    false => TokenType::More,
                };
                self.add_token(token);
            }
            "+" => {
                let token = match self.next_grapheme("=") {
                    true => TokenType::PlusEqual,
                    false => TokenType::Plus,
                };
                self.add_token(token);
            }
            "-" => {
                match self.next_grapheme("=") {
                    true => self.add_token(TokenType::MinusEqual),
                    false => match self.next_grapheme(">") {
                        true => self.add_token(TokenType::Arrow),
                        false => match self.next_grapheme_number() {
                            true => {
                                self.number()?;
                            }
                            false => self.add_token(TokenType::Minus),
                        },
                    },
                };
            }
            "*" => {
                let token = match self.next_grapheme("=") {
                    true => TokenType::StarEqual,
                    false => TokenType::Star,
                };
                self.add_token(token);
            }
            "/" => {
                let token = match self.next_grapheme("=") {
                    true => TokenType::SlashEqual,
                    false => TokenType::Slash,
                };
                self.add_token(token);
            }
            // String literals
            "\"" => self.string()?,
            // Number literals - e.g. Integer or Float
            "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.number()?,
            // Identifier
            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n"
            | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B"
            | "C" | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P"
            | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "_" => {
                self.identifier()?
            }
            // Comments - ignore all characters until the next line
            "#" => {
                while !self.is_at_end() && self.graphemes[self.current] != "\n" {
                    self.current += 1
                }
            }
            // Whitespace and newlines
            " " | "\r" | "\t" => (),
            "\n" => (),
            _ => return Err(LexerErr::UnexpectedGrapheme(self.current - 1)),
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.graphemes.len()
    }

    fn next_grapheme(&mut self, expected: &str) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.graphemes[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn next_grapheme_number(&mut self) -> bool {
        if self.is_at_end() {
            return false;
        }

        if let "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" =
            self.graphemes[self.current]
        {
            return true;
        }

        false
    }

    fn string(&mut self) -> Result<(), LexerErr> {
        while !self.is_at_end() && self.graphemes[self.current] != "\"" {
            if self.graphemes[self.current] == "\n" {
                return Err(LexerErr::UnterminatedString(self.current - 1));
            }
            self.current += 1;
        }

        if self.is_at_end() {
            return Err(LexerErr::UnterminatedString(self.current - 1));
        }

        self.current += 1;

        let mut literal = String::new();

        for i in self.start + 1..self.current - 1 {
            literal.push_str(self.graphemes[i]);
        }

        self.add_token(TokenType::String(Rc::new(literal)));

        Ok(())
    }

    fn number(&mut self) -> Result<(), LexerErr> {
        let mut float = false;

        loop {
            if self.is_at_end() {
                break;
            }

            match self.graphemes[self.current] {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => self.current += 1,
                "." => {
                    if !float {
                        float = true;
                        self.current += 1;
                    } else {
                        return Err(LexerErr::FloatMoreThanOneDecimalPoint(self.current));
                    }
                }
                _ => {
                    break;
                }
            }
        }

        let mut literal = String::new();
        for i in self.start..self.current {
            literal.push_str(self.graphemes[i]);
        }

        if float {
            let val = literal.parse::<f64>().unwrap();
            self.add_token(TokenType::Float(val));
        } else {
            let val = match literal.parse::<i64>() {
                Ok(v) => v,
                Err(e) => return Err(LexerErr::FailedToParseInteger(self.start, self.current, e)),
            };
            self.add_token(TokenType::Integer(val));
        }

        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LexerErr> {
        loop {
            if self.is_at_end() {
                break;
            }

            match self.graphemes[self.current] {
                "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "a" | "b" | "c"
                | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p"
                | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "A" | "B" | "C"
                | "D" | "E" | "F" | "G" | "H" | "I" | "J" | "K" | "L" | "M" | "N" | "O" | "P"
                | "Q" | "R" | "S" | "T" | "U" | "V" | "W" | "X" | "Y" | "Z" | "_" => {
                    self.current += 1;
                }
                _ => break,
            }
        }

        let mut literal = String::new();
        for i in self.start..self.current {
            literal.push_str(self.graphemes[i]);
        }

        match literal.as_str() {
            "function" => self.add_token(TokenType::Function),
            "start" => self.add_token(TokenType::Start),
            "loop" => self.add_token(TokenType::Loop),
            "if" => self.add_token(TokenType::If),
            "else" => self.add_token(TokenType::Else),
            "for" => self.add_token(TokenType::For),
            "while" => self.add_token(TokenType::While),
            "break" => self.add_token(TokenType::Break),
            "return" => self.add_token(TokenType::Return),
            "continue" => self.add_token(TokenType::Continue),
            "false" => self.add_token(TokenType::Boolean(false)),
            "true" => self.add_token(TokenType::Boolean(true)),
            "let" => self.add_token(TokenType::Let),
            "class" => self.add_token(TokenType::Class),
            "method" => self.add_token(TokenType::Method),
            "constructor" => self.add_token(TokenType::Constructor),
            _ => self.add_token(TokenType::Identifier(Rc::new(literal))),
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            start: self.start,
            end: self.current,
        });
    }
}
