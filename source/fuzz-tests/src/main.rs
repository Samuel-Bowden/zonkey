use afl::fuzz;
use interpreter::{UnicodeSegmentation, run_lexer, run_parser};

fn main() {
    fuzz!(|data: &[u8]| {
        if let Ok(s) = std::str::from_utf8(data) {
            let graphemes = UnicodeSegmentation::graphemes(s, true)
                .collect::<Vec<&str>>();

            let lexer_result = run_lexer(&graphemes);

            match lexer_result {
                Ok(tokens) => {
                    // Lexer survived and has been given a valid file to tokenise.
                    // Now test the parser
                    run_parser(tokens).ok();
                }
                Err(_) => {
                    // Lexer survived without panicking
                }
            }
        }
    });
}
