use super::err_reporter::ErrReporter;
use interpreter::err::parser::{ParserErr, ParserErrType};

pub fn err_handler(mut err_reporter: ErrReporter, parser_err: ParserErr) {
    let len = parser_err.get_length();

    for error in parser_err.errors {
        err_reporter.error_prefix();

        match error {
            // Miscellaneous/Global errors
            ParserErrType::UnterminatedStatement(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ';' after '{}' to end statement.",
                        before.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::UnexpectedTokenInGlobal(unexpected_token) => {
                err_reporter.writeln(
                    format!(
                        "Unexpected token '{}' in global scope.",
                        unexpected_token.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(unexpected_token);
                err_reporter.give_tip(
                    "There should only be 'start' or 'function' blocks in the global scope.",
                );
            }

            ParserErrType::VariableNotFound(token, name) => {
                err_reporter.writeln(
                    format!(
                        "Could not find a variable with name '{}' in the current scope.",
                        name,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::ExpectedLiteralVariableCall(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a function call, literal or variable name after '{}'.",
                        before.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }
            //

            // Block errors
            ParserErrType::BlockExpectedLeftBrace(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '{{' after '{}' to start block.",
                        before.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::BlockExpectedRightBrace(open, before) => {
                err_reporter.writeln(
                    "Expected block to be closed with '}', but the end of the file was reached.",
                );
                err_reporter.writeln("        The block was opened here:");
                err_reporter.report_token(open);
                err_reporter.writeln("        But '}' was expected after the last character below to close the opened block:");
                err_reporter.report_token(before);
            }
            //

            // Start errors
            ParserErrType::RedefinedStart(first, other) => {
                err_reporter.writeln("Start block was defined more than once.");
                err_reporter.writeln("        The first definition of start was here:");
                err_reporter.report_token(first);
                err_reporter.writeln("        But it was defined again here:");
                err_reporter.report_token(other);
            }

            ParserErrType::NoStartBlock => {
                err_reporter.writeln("No start block was found in the source file.");
            }

            ParserErrType::StartCannotReturn(return_token) => {
                err_reporter.writeln("Start blocks cannot have return statements.");
                err_reporter.report_token(return_token);
            }
            //

            // Call errors
            ParserErrType::CallExpectedCommaOrRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' to add another value or ')' to close value list after '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::CallIncorrectArgumentsNum(token, arg_len, expected_len, name) => {
                err_reporter.writeln(
                    format!(
                        "Expected {expected_len} argument(s) for function call '{name}', but {arg_len} argument(s) were provided.",
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::CallArgumentIncorrectType(token, position, expr_type, name) => {
                err_reporter.writeln(
                    format!(
                        "Function call {name} does not accept a value of type {:?} for the parameter at position {position}.",
                        expr_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::CallModuleFunctionNotFound(token, function, module) => {
                err_reporter.writeln(
                    format!("Function '{function}' does not exist in module '{module}'.",).as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::CallModuleNotFound(token, module) => {
                err_reporter.writeln(format!("Module '{module}' does not exist.",).as_str());
                err_reporter.report_token(token);
            }

            ParserErrType::CallFunctionNotFound(token, function) => {
                err_reporter
                    .writeln(format!("Function '{function}' has not been declared.",).as_str());
                err_reporter.report_token(token);
            }
            //

            // If statement errors
            ParserErrType::IfExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start the condition of the if statement.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::IfExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' after '{}' to end the condition of the if statement.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::IfConditionNotBool(start, end) => {
                err_reporter
                    .writeln("The condition of the if statement does not evaluate to a boolean.");
                err_reporter.report_section(start, end);
            }
            //

            // While statement errors
            ParserErrType::WhileExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start the condition of the while statement.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::WhileExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' after '{}' to end the condition of the while statement.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::WhileConditionNotBool(start, end) => {
                err_reporter.writeln("While statement condition does not evaluate to a boolean.");
                err_reporter.report_section(start, end);
            }
            //

            // For statement errors
            ParserErrType::ForExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start the clauses of the for statement.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ForExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' after '{}' to end the clauses of the for statement.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ForConditionNotBool(start, end) => {
                err_reporter
                    .writeln("For statement test condition does not evaluate to a boolean.");
                err_reporter.report_section(start, end);
            }

            ParserErrType::ForExpectedComma1(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' after '{}' to separate initialiser from test condition in for statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ForExpectedComma2(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' after '{}' to separate test condition from updater in for statement.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }
            //

            // Function declaration errors
            ParserErrType::FunctionDeclarationExpectedName(before, after) => {
                err_reporter.writeln(
                    format!("Expected a function name after '{}'.", before.token_type,).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start parameter list of function declaration.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedParameterType(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a data type for a function parameter after '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedParameterName(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected the name of the function parameter after '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedCommaOrRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' to add another function parameter or ')' to close parameter list after '{}'.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationExpectedReturnType(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a return data type for the function after '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::FunctionDeclarationInvalidReturnExpressionType(
                return_token,
                func_ret_type,
                expr_ret_type,
            ) => {
                err_reporter.writeln(
                    format!(
                        "This return expression evaluates to type '{:?}', but the function it is declared in has a return type of {:?}.",
                        expr_ret_type,
                        func_ret_type,
                    ).as_str()
                );
                err_reporter.report_token(return_token);
            }
            //

            // Operator errors
            ParserErrType::InvalidAssignmentOperator(assignment_operator, value_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot use assignment operator '{}' with value of type {:?}.",
                        assignment_operator.token_type, value_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(assignment_operator);
            }

            ParserErrType::UnmatchingTypesAssignmentOperatator(
                assignment_operator,
                variable_type,
                expr_type,
            ) => {
                err_reporter.writeln(
                    format!(
                        "Expression to assign to variable with operator '{}' evaluated to the type {:?}, but the variable is of type {:?}.",
                        assignment_operator.token_type,
                        expr_type,
                        variable_type,
                    ).as_str()
                );
                err_reporter.report_token(assignment_operator);
            }
            //

            // Variable Declaration errors
            ParserErrType::VariableDeclarationExpectedName(before, after) => {
                err_reporter.writeln(
                    format!("Expected a function name after '{}'.", before.token_type,).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::VariableDeclarationAlreadyDeclared(token, name) => {
                err_reporter.writeln(
                    format!(
                        "Attempted to declare a variable with name '{}', but a variable with this name has already been declared previously.",
                        name
                    ).as_str()
                );
                err_reporter.report_token(token);
            }
            ParserErrType::VariableDeclarationExpectedEqual(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '=' after '{}' to assign a value to the declared variable.",
                        before.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
                err_reporter
                    .give_tip("All variables must be assigned a value when they are declared")
            }

            ParserErrType::VariableDeclarationExprEvalNone(start, end) => {
                err_reporter.writeln(
                    "The expression to assign to the variable does not evaluate to a value",
                );
                err_reporter.report_section(start, end);
                err_reporter.give_tip("An expression to assign to a variable must evaluate to a value such as an Integer, Float, String or Boolean. You may have assigned the result of a function that does not return a value by mistake");
            }
            //

            // Comparision errors
            ParserErrType::ComparisionUnmatchingTypes(token, left, right) => {
                err_reporter.writeln("Cannot compare two values of different types.");
                err_reporter.report_token(token);
                err_reporter.writeln(
                    format!(
                        "        Left expression evaluates to type {:?}, while the right expression evaluates to type {:?}.",
                        left,
                        right,
                    )
                    .as_str(),
                );
            }

            ParserErrType::ComparisionInvalidForType(token, expr_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot perform comparision '{}' for type {:?}.",
                        token.token_type, expr_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }
            //

            // Operator errors
            ParserErrType::OperatorUnmatchingTypes(token, left, right) => {
                err_reporter.writeln(
                    format!(
                        "Cannot use operator '{}' on two values of different types.",
                        token.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
                err_reporter.writeln(
                    format!(
                        "        Left expression evaluates to type {:?}, while the right expression evaluates to type {:?}.",
                        left,
                        right,
                    )
                    .as_str(),
                );
            }

            ParserErrType::OperatorInvalidForType(token, expr_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot perform operation '{}' on type {:?}.",
                        token.token_type, expr_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }
            //

            // Module errors
            ParserErrType::ModuleExpectedIdentifier(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected an identifier after '{}' to specify a function within the module.",
                        before.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ModuleExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start the parameter list of module function call.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            // Grouping errors
            ParserErrType::GroupingExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' after '{}' to end grouping of expression started with '('.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }
            //

            // Unary operator errors
            ParserErrType::UnaryOperatorInvalidForType(token, expr_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot perform unary operation '{}' on type {:?}.",
                        token.token_type, expr_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            // Casting errors
            ParserErrType::CastNotPossible(token, cast_to_type, expr_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot cast evaluated value of expression from {:?} to {:?}.",
                        expr_type, cast_to_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::CastPointless(token, cast_to_type) => {
                err_reporter.writeln(
                    format!(
                        "Casting evaluated value of expression from {:?} to {:?} is pointless",
                        cast_to_type, cast_to_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }
        }
        err_reporter.newln();
    }

    err_reporter.aborting_prefix();
    err_reporter
        .writeln(format!("Cannot start execution of script due to {} error(s).", len).as_str());
}
