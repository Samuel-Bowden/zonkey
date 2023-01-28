use interpreter::err::parser::{ParserErr, ParserErrType};
use super::err_reporter::ErrReporter;

pub fn err_handler(mut err_reporter: ErrReporter, parser_err: ParserErr) {
    let len = parser_err.get_length();

    for error in parser_err.errors {
        err_reporter.error_prefix();

        match error {
            // Miscellaneous/Global errors
            ParserErrType::UnterminatedStatement(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected token ';' after token '{}' to end statement.",
                        before.token_type
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::UnexpectedTokenInGlobal(unexpected_token) => {
                err_reporter.writeln(
                    format!(
                        "Unexpected token '{}' in global scope",
                        unexpected_token.token_type
                    ).as_str()
                );
                err_reporter.report_line(unexpected_token);
                err_reporter.give_tip("There should only be 'start' or 'function' blocks in the global scope".to_string());
            }
            //

            // Start declaration errors
            ParserErrType::RedefinedStart(first, other) => {
                err_reporter.writeln("Start block was defined more than once.");
                err_reporter.writeln("        The first definition of start was here:");
                err_reporter.report_line(first);
                err_reporter.writeln("        But it was defined again here:");
                err_reporter.report_line(other);
            }

            ParserErrType::NoStartBlock => {
                err_reporter.writeln("No start block was found in the source file.");
            }
            //

            // If statement errors
            ParserErrType::IfExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start the condition of the if statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::IfExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' after '{}' to end the condition of the if statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::IfConditionNotBool(condition) => {
                err_reporter.writeln("The condition of the if statement does not evaluate to a boolean.");
                err_reporter.report_line(condition);
            }
            //

            // While statement errors
            ParserErrType::WhileExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start the condition of the while statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::WhileExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' after '{}' to end the condition of the while statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::WhileConditionNotBool(condition) => {
                err_reporter.writeln("While statement condition does not evaluate to a boolean.");
                err_reporter.report_line(condition);
            }
            //
            
            // For statement errors
            ParserErrType::ForExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start the clauses of the for statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ForExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' after '{}' to end the clauses of the for statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ForConditionNotBool(condition) => {
                err_reporter.writeln("For statement test condition does not evaluate to a boolean.");
                err_reporter.report_line(condition);
            }

            ParserErrType::ForExpectedComma1(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' after '{}' to separate initialiser from test condition in for statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ForExpectedComma2(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' after '{}' to separate test condition from updater in for statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }
            //
            
            // Function declaration errors
            ParserErrType::FunctionDeclarationExpectedName(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a function name after '{}'.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start parameter list of function declaration.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedParameterType(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a data type for a function parameter after '{}'.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedParameterName(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected the name of the function parameter after '{}'.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedCommaOrRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' to add another function parameter or ')' to close parameter list after '{}'.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedReturnType(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a return data type for the function after '{}'.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_line(before);
                err_reporter.report_next_token(after);
            }
        }
        err_reporter.newln();
    }

    err_reporter.aborting_prefix();
    err_reporter.writeln(
        format!(
            "Cannot start execution of script due to {} error(s).",
            len
        ).as_str()
    );
}
