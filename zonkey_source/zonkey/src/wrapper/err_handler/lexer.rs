use interpreter::{err::lexer::LexerErr, token::{Token, TokenType}};
use super::err_reporter::ErrReporter;

pub fn err_handler(mut err_reporter: ErrReporter, lexer_err: LexerErr) {
    err_reporter.error_prefix();

    match lexer_err {
        LexerErr::UnexpectedGrapheme(position) => {
            err_reporter.writeln(
                format!(
                    "Unsupported character '{}' found whilst scanning source file.",
                    err_reporter.get_grapheme(position),
                ).as_str()
            );

            err_reporter.report_line(Token {
                token_type: TokenType::None,
                start: position,
                end: position + 1,
            });

            err_reporter.give_tip("A valid Zonkey program must only contain specific unicode characters.\nPlease refer to the documentation to see which characters can be used".to_string());
        }

        LexerErr::UnterminatedString(position) => {
            err_reporter.writeln("String literal was not closed before starting a new line.");

            err_reporter.report_line(Token {
                token_type: TokenType::None,
                start: position,
                end: position + 1,
            });
        }

        LexerErr::FloatMoreThanOneDecimalPoint(position) => {
            err_reporter.writeln("Float literal contains more than one decimal point.");

            err_reporter.report_line(Token {
                token_type: TokenType::None,
                start: position,
                end: position + 1,
            });
        }
    }
    
    err_reporter.newln();
    err_reporter.aborting_prefix();
    err_reporter.writeln("Cannot start execution of script due to lexical error.");
}
