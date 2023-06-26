#[cfg(target_os = "linux")]
fn main() {
    // Please note that fuzz testing will not work on Windows as it is unsupported by afl-rs

    use afl::fuzz;
    use interpreter::{
        err::{InterpreterErr, InterpreterErrType},
        lexer::Lexer,
        parser::Parser,
    };

    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let mut source = s.to_string();
            let (lexer_result, graphemes) = Lexer::run(&mut source);

            let err_type = match lexer_result {
                Ok(tokens) => {
                    // Lexer survived and has been given a valid file to tokenise.
                    // Now test the parser
                    match Parser::run(tokens) {
                        Ok(_) => return,
                        Err(e) => InterpreterErrType::ParserFailed(e),
                    }
                }
                Err(e) => InterpreterErrType::LexerFailed(e),
            };

            eprintln!(
                "{}",
                InterpreterErr::new(err_type, graphemes).get_err_messages()
            );
        }
    });
}

#[cfg(target_os = "windows")]
fn main() {
    println!("Fuzz testing does not work on Windows");
}
