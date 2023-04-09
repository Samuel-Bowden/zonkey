use super::err_reporter::ErrReporter;
use crate::err::lexer::LexerErr;

pub fn err_handler(err_reporter: &mut ErrReporter, lexer_err: LexerErr) {
    err_reporter.error_prefix();

    match lexer_err {
        LexerErr::UnexpectedGrapheme(position) => {
            err_reporter.writeln(
                format!(
                    "Unsupported character '{}' found whilst scanning source file.",
                    err_reporter.get_grapheme(position),
                )
                .as_str(),
            );

            err_reporter.report_section(position, position + 1);

            err_reporter.give_tip("A valid Zonkey program must only contain specific unicode characters. Please refer to the documentation to see which characters can be used");
        }

        LexerErr::UnterminatedString(position) => {
            err_reporter.writeln("String literal was not closed before starting a new line.");

            err_reporter.report_section(position, position + 1);
        }

        LexerErr::FloatMoreThanOneDecimalPoint(position) => {
            err_reporter.writeln("Float literal contains more than one decimal point.");

            err_reporter.report_section(position, position + 1);
        }
    }

    err_reporter.newln();
    err_reporter.aborting_prefix();
    err_reporter.writeln("Cannot start execution of script due to lexical error.");
}
