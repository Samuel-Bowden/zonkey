pub mod err;

use crate::lexer_debug;

use self::err::LexerErr;
use super::token::Token;
use unicode_segmentation::UnicodeSegmentation;

pub struct Lexer<'a> {
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: u64,
    graphemes: Vec<&'a str>,
    #[cfg(debug_assertions)]
    debug: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, _debug: bool) -> Self {
        Self {
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            graphemes: source.graphemes(true).collect(),
            #[cfg(debug_assertions)]
            debug: _debug,
        }
    }

    pub fn run(mut self) -> Result<Self, LexerErr> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        lexer_debug!(self.debug, "Printing tokens");

        #[cfg(debug_assertions)]
        if self.debug {
            for (i, token) in self.tokens.iter().enumerate() {
                println!("  {}: {:?}", i + 1, token);
            }
        }

        Ok(self)
    }

    fn scan_token(&mut self) -> Result<(), LexerErr> {
        let grapheme = self.graphemes[self.current];
        self.current += 1;

        match grapheme {
            // Single grapheme
            "(" => self.add_token(Token::LeftParen),
            ")" => self.add_token(Token::RightParen),
            "{" => self.add_token(Token::LeftBrace),
            "}" => self.add_token(Token::RightBrace),
            "," => self.add_token(Token::Comma),
            "." => self.add_token(Token::Dot),
            ";" => self.add_token(Token::SemiColon),
            "?" => self.add_token(Token::QuestionMark),
            // Single or double graphemes
            "!" => {
                let token = match self.next_grapheme("=") {
                    true => Token::BangEqual,
                    false => Token::Bang,
                };
                self.add_token(token);
            }
            "=" => {
                let token = match self.next_grapheme("=") {
                    true => Token::EqualEqual,
                    false => Token::Equal,
                };
                self.add_token(token);
            }
            "<" => {
                let token = match self.next_grapheme("=") {
                    true => Token::LessEqual,
                    false => Token::Less,
                };
                self.add_token(token);
            }
            ">" => {
                let token = match self.next_grapheme("=") {
                    true => Token::MoreEqual,
                    false => Token::More,
                };
                self.add_token(token);
            }
            "+" => {
                let token = match self.next_grapheme("=") {
                    true => Token::PlusEqual,
                    false => Token::Plus,
                };
                self.add_token(token);
            }
            "-" => {
                match self.next_grapheme("=") {
                    true => self.add_token(Token::MinusEqual),
                    false => match self.next_grapheme(">") {
                        true => self.add_token(Token::Arrow),
                        false => match self.next_grapheme_number() {
                            true => {
                                self.number()?;
                            }
                            false => self.add_token(Token::Minus),
                        },
                    },
                };
            }
            "*" => {
                let token = match self.next_grapheme("=") {
                    true => Token::StarEqual,
                    false => Token::Star,
                };
                self.add_token(token);
            }
            "/" => {
                let token = match self.next_grapheme("=") {
                    true => Token::SlashEqual,
                    false => Token::Slash,
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
            "\n" => self.line += 1,
            unexpected_grapheme => {
                return Err(LexerErr::UnexpectedGrapheme(
                    self.line,
                    String::from(unexpected_grapheme),
                ))
            }
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
                return Err(LexerErr::UnterminatedString(self.line));
            }
            self.current += 1;
        }

        if self.is_at_end() {
            return Err(LexerErr::UnterminatedString(self.line));
        }

        self.current += 1;

        let mut literal = String::new();

        for i in self.start + 1..self.current - 1 {
            literal.push_str(self.graphemes[i]);
        }

        self.add_token(Token::String(literal));

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
                        return Err(LexerErr::FloatMoreThanOneDecimalPoint);
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
            self.add_token(Token::Float(val));
        } else {
            let val = literal.parse::<i64>().unwrap();
            self.add_token(Token::Integer(val));
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
            "function" => self.add_token(Token::Function),
            "start" => self.add_token(Token::Start),
            "loop" => self.add_token(Token::Loop),
            "if" => self.add_token(Token::If),
            "else" => self.add_token(Token::Else),
            "for" => self.add_token(Token::For),
            "while" => self.add_token(Token::While),
            "break" => self.add_token(Token::Break),
            "return" => self.add_token(Token::Return),
            "continue" => self.add_token(Token::Continue),
            "Integer" => self.add_token(Token::IntegerType),
            "Float" => self.add_token(Token::FloatType),
            "String" => self.add_token(Token::StringType),
            "Boolean" => self.add_token(Token::BooleanType),
            "false" => self.add_token(Token::Boolean(false)),
            "true" => self.add_token(Token::Boolean(true)),
            "print" => self.add_token(Token::Print),
            "exit" => self.add_token(Token::Exit),
            _ => self.add_token(Token::Identifier(literal)),
        }

        Ok(())
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[derive(Debug)]
    enum LexerTestErr {
        Incorrect,
        LexerFailed(LexerErr),
    }

    #[test]
    fn test_lexer_functions() -> Result<(), LexerTestErr> {
        let source = "function hello(String name) {
            print(\"Hello \" + name);
        }";

        let lexer = Lexer::new(source, false).run();

        let source = match lexer {
            Ok(lexer) => lexer.tokens.into_iter(),
            Err(e) => return Err(LexerTestErr::LexerFailed(e)),
        };

        let test = vec![
            Token::Function,
            Token::Identifier("hello".to_string()),
            Token::LeftParen,
            Token::StringType,
            Token::Identifier("name".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::Print,
            Token::LeftParen,
            Token::String("Hello ".to_string()),
            Token::Plus,
            Token::Identifier("name".to_string()),
            Token::RightParen,
            Token::SemiColon,
            Token::RightBrace,
        ]
        .into_iter();

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

        let lexer = Lexer::new(source, false).run();

        let source = match lexer {
            Ok(lexer) => lexer.tokens.into_iter(),
            Err(e) => return Err(LexerTestErr::LexerFailed(e)),
        };

        let test = vec![
            Token::Start,
            Token::LeftParen,
            Token::StringType,
            Token::QuestionMark,
            Token::Identifier("name".to_string()),
            Token::RightParen,
            Token::LeftBrace,
            Token::If,
            Token::LeftParen,
            Token::Identifier("name".to_string()),
            Token::Dot,
            Token::Identifier("exists".to_string()),
            Token::LeftParen,
            Token::RightParen,
            Token::RightParen,
            Token::LeftBrace,
            Token::Print,
            Token::LeftParen,
            Token::Identifier("name".to_string()),
            Token::RightParen,
            Token::SemiColon,
            Token::RightBrace,
            Token::RightBrace,
        ]
        .into_iter();

        if source.eq(test) {
            Ok(())
        } else {
            Err(LexerTestErr::Incorrect)
        }
    }
}
