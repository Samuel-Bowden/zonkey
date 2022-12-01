pub mod err;

use unicode_segmentation::UnicodeSegmentation;
use super::{token::{Token, token_type::TokenType}, literal::Literal};
use self::err::LexerErr;

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
    graphemes: Vec<&'a str>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            graphemes: source.graphemes(true).collect(),
        }
    }

    pub fn run(mut self) -> Result<Self, LexerErr> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.add_token(TokenType::EOF);
        
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
            "," => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            ";" => self.add_token(TokenType::SemiColon),
            "?" => self.add_token(TokenType::QuestionMark),
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
                let token = match self.next_grapheme("=") {
                    true => TokenType::MinusEqual,
                    false => match self.next_grapheme(">") {
                        true => TokenType::Arrow,
                        false => TokenType::Minus,
                    },
                };
                self.add_token(token);
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
            "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => self.number()?,
            // Identifier
            "a"|"b"|"c"|"d"|"e"|"f"|"g"|"h"|"i"|"j"|
                "k"|"l"|"m"|"n"|"o"|"p"|"q"|"r"|"s"|
                "t"|"u"|"v"|"w"|"x"|"y"|"z"|"A"|"B"|
                "C"|"D"|"E"|"F"|"G"|"H"|"I"|"J"|"K"|
                "L"|"M"|"N"|"O"|"P"|"Q"|"R"|"S"|"T"|
                "U"|"V"|"W"|"X"|"Y"|"Z"|"_"
                => self.identifier()?,
            // Comments - ignore all characters until the next line
            "#" => {
                while !self.is_at_end() && self.graphemes[self.current] != "\n" {
                    self.current += 1
                }
            },
            // Whitespace and newlines
            " " | "\r" | "\t" => (),
            "\n" => self.line += 1,
            unexpected_grapheme => return Err(
                LexerErr::UnexpectedGrapheme(self.line, String::from(unexpected_grapheme))
            ),
        }

        Ok(())
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.graphemes.len()
    }

    fn next_grapheme(&mut self, expected: &str) -> bool {
        if self.is_at_end() { return false }

        if self.graphemes[self.current] != expected { return false }

        self.current += 1;
        true
    }

    fn string(&mut self) -> Result<(), LexerErr> {
        while !self.is_at_end() && self.graphemes[self.current] != "\"" {
            if self.graphemes[self.current] == "\n" {
                return Err(LexerErr::UnterminatedString(self.line))
            }
            self.current += 1;
        }
        
        if self.is_at_end() {
            return Err(LexerErr::UnterminatedString(self.line))
        }

        self.current += 1;

        let mut literal = String::new();

        for i in self.start + 1 .. self.current - 1 {
            literal.push_str(self.graphemes[i]);
        }

        self.add_token_with_literal(TokenType::String, Literal::String(literal));
        
        Ok(())
    }

    fn number(&mut self) -> Result<(), LexerErr> {
        let mut float = false;

        loop {
            if self.is_at_end() { break; }

            match self.graphemes[self.current] {
                "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9" => self.current += 1,
                "." => {
                    float = true;
                    self.current += 1;
                }
                _ => {
                    break;
                }
            }
        }

        let mut literal = String::new();
        for i in self.start .. self.current {
            literal.push_str(self.graphemes[i]);
        }

        if float {
            let val = literal.parse::<f64>().unwrap();
            self.add_token_with_literal(TokenType::Float, Literal::Float(val));
        } else {
            let val = literal.parse::<i64>().unwrap();
            self.add_token_with_literal(TokenType::Integer, Literal::Integer(val));
        }

        Ok(())
    }

    fn identifier(&mut self) -> Result<(), LexerErr> {
        loop {
            if self.is_at_end() { break; }

            match self.graphemes[self.current] {
                "0"|"1"|"2"|"3"|"4"|"5"|"6"|"7"|"8"|"9"|
                "a"|"b"|"c"|"d"|"e"|"f"|"g"|"h"|"i"|"j"|
                "k"|"l"|"m"|"n"|"o"|"p"|"q"|"r"|"s"|"t"|
                "u"|"v"|"w"|"x"|"y"|"z"|"A"|"B"|"C"|"D"|
                "E"|"F"|"G"|"H"|"I"|"J"|"K"|"L"|"M"|"N"|
                "O"|"P"|"Q"|"R"|"S"|"T"|"U"|"V"|"W"|"X"|
                "Y"|"Z"|"_" => {
                    self.current += 1;
                }
                _ => break,
            }
        }

        let mut literal = String::new();
        for i in self.start .. self.current {
            literal.push_str(self.graphemes[i]);
        }

        match literal.as_str() {
            "function" => self.add_token(TokenType::Function),
            "start" => self.add_token(TokenType::Start),
            "loop" => self.add_token(TokenType::Loop),
            "infinite" => self.add_token(TokenType::Infinite),
            "if" => self.add_token(TokenType::If),
            "else" => self.add_token(TokenType::Else),
            "for" => self.add_token(TokenType::For),
            "while" => self.add_token(TokenType::While),
            "break" => self.add_token(TokenType::Break),
            "return" => self.add_token(TokenType::Return),
            "continue" => self.add_token(TokenType::Continue),
            "Integer" => self.add_token(TokenType::IntegerType),
            "Float" => self.add_token(TokenType::FloatType),
            "String" => self.add_token(TokenType::StringType),
            "Boolean" => self.add_token(TokenType::BooleanType),
            _ => self.add_token_with_literal(TokenType::Identifier, Literal::String(literal)),
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token::new(token_type));
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) {
        self.tokens.push(Token::new_with_literal(token_type, literal));
    }
}

#[cfg(test)]
mod tests {
    use crate::{token::token_type::TokenType, literal::Literal};
    use super::*;

    #[derive(Debug)]
    enum LexerTestErr {
        Incorrect,
        LexerFailed(LexerErr),
    }

    impl Literal {
        pub fn string(string: &str) -> Self {
            Self::String(String::from(string))
        }
    }

    #[test]
    fn test_lexer_functions() -> Result<(), LexerTestErr> {
        let source = "function hello(String name) {
            print(\"Hello \" + name);
        }";

        let lexer = Lexer::new(source).run();

        let source = match lexer {
            Ok(lexer) => lexer.tokens.into_iter(),
            Err(e) => return Err(LexerTestErr::LexerFailed(e)),
        };

        let test = vec![
            Token::new(TokenType::Function),
            Token::new_with_literal(TokenType::Identifier, Literal::string("hello")),
            Token::new(TokenType::LeftParen),
            Token::new(TokenType::StringType),
            Token::new_with_literal(TokenType::Identifier, Literal::string("name")),
            Token::new(TokenType::RightParen),
            Token::new(TokenType::LeftBrace),
            Token::new_with_literal(TokenType::Identifier, Literal::string("print")),
            Token::new(TokenType::LeftParen),
            Token::new_with_literal(TokenType::String, Literal::string("Hello ")),
            Token::new(TokenType::Plus),
            Token::new_with_literal(TokenType::Identifier, Literal::string("name")),
            Token::new(TokenType::RightParen),
            Token::new(TokenType::SemiColon),
            Token::new(TokenType::RightBrace),
            Token::new(TokenType::EOF),
        ].into_iter();

        if source.eq(test) {
            Ok(())
        } else {
            Err(LexerTestErr::Incorrect)
        }
    }

    #[test]
    fn test_lexer_start() -> Result<(), LexerTestErr> {
        let source = "start(String? name) {
            if (name.exists()) {
                print(name);
            }
        }";

        let lexer = Lexer::new(source).run();

        let source = match lexer {
            Ok(lexer) => lexer.tokens.into_iter(),
            Err(e) => return Err(LexerTestErr::LexerFailed(e)),
        };

        let test = vec![
            Token::new(TokenType::Start),
            Token::new(TokenType::LeftParen),
            Token::new(TokenType::StringType),
            Token::new(TokenType::QuestionMark),
            Token::new_with_literal(TokenType::Identifier, Literal::string("name")),
            Token::new(TokenType::RightParen),
            Token::new(TokenType::LeftBrace),
            Token::new(TokenType::If),
            Token::new(TokenType::LeftParen),
            Token::new_with_literal(TokenType::Identifier, Literal::string("name")),
            Token::new(TokenType::Dot),
            Token::new_with_literal(TokenType::Identifier, Literal::string("exists")),
            Token::new(TokenType::LeftParen),
            Token::new(TokenType::RightParen),
            Token::new(TokenType::RightParen),
            Token::new(TokenType::LeftBrace),
            Token::new_with_literal(TokenType::Identifier, Literal::string("print")),
            Token::new(TokenType::LeftParen),
            Token::new_with_literal(TokenType::Identifier, Literal::string("name")),
            Token::new(TokenType::RightParen),
            Token::new(TokenType::SemiColon),
            Token::new(TokenType::RightBrace),
            Token::new(TokenType::RightBrace),
            Token::new(TokenType::EOF),
        ].into_iter();

        if source.eq(test) {
            Ok(())
        } else {
            Err(LexerTestErr::Incorrect)
        }
    }
}
