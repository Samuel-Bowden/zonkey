use self::{token::Token, lexer::{err::LexerErr, Lexer}, err::InterpreterErr};

pub mod lexer;
pub mod token;
pub mod err;
pub mod literal;

pub struct Interpreter {
    debug: bool,
}

impl Interpreter {
    pub fn new(debug: bool) -> Self {
        Self {
            debug,
        }
    }

    pub fn run(&self, source: &str) -> Result<(), InterpreterErr> {
        let tokens = match self.run_lexer(source) {
            Ok(t) => t,
            Err(e) => return Err(InterpreterErr::LexerFailed(e)),
        };

        if self.debug {
            println!("Tokens:");
            for (i, token) in tokens.iter().enumerate() {
                println!("{}. {token}", i+1);
            }
        }

        Ok(())
    }

    fn run_lexer(&self, source: &str) -> Result<Vec<Token>, LexerErr> {
        Lexer::new(source).run()
    }
}
