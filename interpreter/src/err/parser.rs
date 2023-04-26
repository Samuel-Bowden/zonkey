use super::err_reporter::ErrReporter;
use crate::{
    parser::err::{ParserErr, ParserErrType},
    parser::value::print_type,
};

pub fn err_handler(err_reporter: &mut ErrReporter, parser_err: ParserErr) {
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
                    "There should only be start, function or class definitions in the global scope.",
                );
            }

            ParserErrType::BreakOutsideLoop(location) => {
                err_reporter.writeln(format!("Cannot break outside of a loop.",).as_str());
                err_reporter.report_token(location);
            }

            ParserErrType::CannotCreateVariableCalledSelf(location) => {
                err_reporter.writeln(
                    format!(
                        "Cannot create a variable with name 'self'. This name is reserved to reference the current object inside a constructor or method."
                    )
                    .as_str(),
                );
                err_reporter.report_token(location);
            }

            ParserErrType::ContinueOutsideLoop(location) => {
                err_reporter.writeln(format!("Cannot use continue outside of a loop.",).as_str());
                err_reporter.report_token(location);
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

            ParserErrType::ExpectedValue(before, after) => {
                err_reporter
                    .writeln(format!("Expected a value after '{}'.", before.token_type).as_str());
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            // Array errors
            ParserErrType::ArrayNonMatchingValue(
                array_type_token,
                arg_pos,
                expected_type,
                arg_type,
            ) => {
                err_reporter
                    .writeln(format!(
                        "Expected values of array to be of type '{:?}', but the value in array position {} evaluates to type '{}'.",
                        expected_type,
                        arg_pos,
                        print_type(&arg_type),
                    ).as_str());
                err_reporter.report_token(array_type_token);
            }

            ParserErrType::ArrayExpectedCommaOrRightBracket(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' to add another value or ']' to close the array after '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ArrayEmptyType(before, after) => {
                err_reporter.writeln(
                    format!("Expected the type of value this type of array will store.",).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
                err_reporter
                    .give_tip("An array type must have a type inside '[]', e.g. '[Integer]'.")
            }

            ParserErrType::ArrayTypeNotClosed(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ']' to close array type after the type this array will hold.",
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

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

            ParserErrType::BlockExpectedRightBrace(open) => {
                err_reporter.writeln(
                    "Expected block to be closed with '}', but the end of the file was reached.",
                );
                err_reporter.writeln("        The block was opened here:");
                err_reporter.report_token(open);
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
            //

            // Call errors
            ParserErrType::CallExpectedCommaOrRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' to add another argument or ')' to close argument list after '{}'.",
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
                        "Expected {expected_len} argument(s) for callable '{name}', but {arg_len} argument(s) were provided.",
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::CallArgumentIncorrectType(token, position, expr_type, name) => {
                err_reporter.writeln(
                    format!(
                        "Callable {name} does not accept a value of type {} for the parameter at position {position}.",
                        print_type(&expr_type),
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::CallNotFound(token, callable) => {
                err_reporter
                    .writeln(format!("Callable '{callable}' has not been declared.",).as_str());
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
            ParserErrType::ForExpectedLet(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected 'let' after '{}' to initialise variable in first section of for statement.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

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
                        "Expected ',' after '{}' to separate initialiser statement from test statement in for declaration.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ForExpectedComma2(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' after '{}' to separate test statement from updater statement in for declaration.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }
            //

            // Function declaration errors
            ParserErrType::FunctionRedeclared(location) => {
                err_reporter.writeln(
                    format!("A function with this name has already been declared.").as_str(),
                );
                err_reporter.report_token(location);
            }

            ParserErrType::FunctionDeclarationExpectedName(before, after) => {
                err_reporter.writeln(
                    format!("Expected a function name after '{}'.", before.token_type,).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::DeclarationExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '(' after '{}' to start parameter list of declaration.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::DeclarationExpectedParameterType(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a data type for a parameter after '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::DeclarationExpectedParameterName(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected the name of the parameter after type '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::DeclarationExpectedCommaOrRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ',' to add another parameter or ')' to close parameter list after '{}'.",
                        before.token_type,
                    ).as_str()
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::DeclarationExpectedReturnType(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected a return data type for the declaration after '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::DeclarationDidNotReturnValueInAllCases(location, req_type) => {
                err_reporter.writeln(
                    format!(
                        "Declaration did not return the required type '{}' in all branches of code block.",
                        print_type(&Some(req_type)),
                    )
                    .as_str(),
                );
                err_reporter.report_token(location);
            }

            ParserErrType::DeclarationInvalidReturnExpressionType(
                return_token,
                func_ret_type,
                expr_ret_type,
            ) => {
                err_reporter.writeln(
                    format!(
                        "This return expression evaluates to type '{}', but the declaration it is defined in has a return type of '{}'.",
                        print_type(&expr_ret_type),
                        print_type(&func_ret_type),
                    ).as_str()
                );
                err_reporter.report_token(return_token);
            }
            //

            // Operator errors
            ParserErrType::InvalidAssignmentOperator(assignment_operator, value_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot use assignment operator '{}' with value of type '{}'.",
                        assignment_operator.token_type,
                        print_type(&Some(value_type)),
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
                        "Expression to assign to variable with operator '{}' evaluated to the type '{}', but the variable is type '{}'.",
                        assignment_operator.token_type,
                        print_type(&expr_type),
                        print_type(&variable_type),
                    ).as_str()
                );
                err_reporter.report_token(assignment_operator);
            }
            //

            // Variable Declaration errors
            ParserErrType::VariableDeclarationExpectedName(before, after) => {
                err_reporter.writeln(
                    format!("Expected a variable name after '{}'.", before.token_type,).as_str(),
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
                    .give_tip("All variables must be assigned a value when they are declared.")
            }

            ParserErrType::VariableDeclarationExprEvalNone(start, end) => {
                err_reporter.writeln(
                    "The expression to assign to the variable does not evaluate to a value.",
                );
                err_reporter.report_section(start, end);
                err_reporter.give_tip("An expression to assign to a variable must evaluate to a value such as an Integer, Float, String or Boolean. You may have assigned the result of a callable that does not return a value by mistake.");
            }
            //

            // Comparision errors
            ParserErrType::ComparisionUnmatchingTypes(token, left, right) => {
                err_reporter.writeln("Cannot compare two values of different types.");
                err_reporter.report_token(token);
                err_reporter.writeln(
                    format!(
                        "        Left expression evaluates to type {}, while the right expression evaluates to type {}.",
                        print_type(&left),
                        print_type(&right),
                    )
                    .as_str(),
                );
            }

            ParserErrType::ComparisionInvalidForType(token, expr_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot perform comparision '{}' for type {}.",
                        token.token_type,
                        print_type(&expr_type),
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
                        "        Left expression evaluates to type {}, while the right expression evaluates to type {}.",
                        print_type(&left),
                        print_type(&right),
                    )
                    .as_str(),
                );
            }

            ParserErrType::OperatorInvalidForType(token, expr_type) => {
                err_reporter.writeln(
                    format!(
                        "Cannot perform operation '{}' on type {}.",
                        token.token_type,
                        print_type(&expr_type),
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }
            //

            // Grouping errors
            ParserErrType::GroupingExpectedRightParen(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ')' to end expression group started with '{}' located here.",
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
                        "Cannot perform unary operation '{}' on type {}.",
                        token.token_type,
                        print_type(&expr_type),
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            // Class errors
            ParserErrType::ClassRedeclared(location) => {
                err_reporter
                    .writeln(format!("A class with this name has already been declared.").as_str());
                err_reporter.report_token(location);
            }

            ParserErrType::InbuiltType(location) => {
                err_reporter.writeln(
                    format!("A class cannot be declared with the name of an inbuilt type.")
                        .as_str(),
                );
                err_reporter.report_token(location);
            }

            ParserErrType::ClassNotFound(location) => {
                err_reporter
                    .writeln(format!("A class with this name has not been declared.").as_str());
                err_reporter.report_token(location);
            }

            ParserErrType::ClassDeclarationExpectedName(before, after) => {
                err_reporter.writeln(
                    format!("Expected a class name after '{}'.", before.token_type,).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ClassDeclarationExpectedLeftBrace(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected '{{' after '{}' to start body of class.",
                        before.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ClassDeclarationExpectedRightBrace(before, after) => {
                err_reporter.writeln("Expected class body to be closed with '}'.");
                err_reporter.writeln("        The body was opened here:");
                err_reporter.report_token(before);
                err_reporter.writeln(
                    "        '}' was expected as the next token to close the opened body.",
                );
                err_reporter.report_next_token(after);
            }

            ParserErrType::ClassDeclarationExpectedPropertyName(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected name of property after the data type '{}'.",
                        before.token_type,
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ClassDeclarationExpectedMethodName(before, after) => {
                err_reporter.writeln(
                    format!("Expected a method name after '{}'.", before.token_type,).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ClassDeclarationUnterminatedProperty(before, after) => {
                err_reporter.writeln(
                    format!(
                        "Expected ';' after '{}' to end property declaration.",
                        before.token_type
                    )
                    .as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::ClassDeclarationRedeclaredProperty(location, name) => {
                err_reporter.writeln(
                    format!(
                        "Property with name '{}' has already been declared in this class.",
                        name,
                    )
                    .as_str(),
                );
                err_reporter.report_token(location);
            }

            ParserErrType::ClassDeclarationRedeclaredConstructor(location) => {
                err_reporter.writeln(
                    format!(
                        "This constructor has been declared when one has already been made for this class.",
                    )
                    .as_str(),
                );
                err_reporter.report_token(location);
            }

            ParserErrType::ClassDeclarationRedeclaredMethod(location, name) => {
                err_reporter.writeln(
                    format!(
                        "Method with name '{}' has already been declared in this class.",
                        name,
                    )
                    .as_str(),
                );
                err_reporter.report_token(location);
            }

            ParserErrType::ClassDeclarationNoConstructor(location) => {
                err_reporter
                    .writeln(format!("No constructor has been declared for this class.",).as_str());
                err_reporter.report_token(location);
            }

            ParserErrType::ClassDeclarationExpectPropertyTop(location) => {
                err_reporter.writeln(
                    format!("Property was not declared before the constructor or methods.")
                        .as_str(),
                );
                err_reporter.report_token(location);
                err_reporter.give_tip("All property declarations must be placed together at the top of the class declaration.");
            }

            // Method call errors
            ParserErrType::MethodCallExpectedName(before, after) => {
                err_reporter.writeln(
                    format!("Expected method name after '{}'.", before.token_type).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::MethodCallExpectedLeftParen(before, after) => {
                err_reporter.writeln(
                    format!("Expected '(' after method call '{}'.", before.token_type).as_str(),
                );
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::MethodCallNotObject(location, value) => {
                err_reporter.writeln(
                    format!(
                        "Started a method call on a value of type {:?}.",
                        print_type(&value),
                    )
                    .as_str(),
                );
                err_reporter.report_token(location);
                err_reporter.give_tip("Methods can only be called on objects.");
            }

            ParserErrType::MethodCallNotFound(token, method_name, class_name) => {
                err_reporter
                    .writeln(format!("Method with name '{method_name}' has not been declared for class '{class_name}'.",).as_str());
                err_reporter.report_token(token);
            }

            // Property accessor
            ParserErrType::PropertyAccessorExpectedName(before, after) => {
                err_reporter
                    .writeln(format!("Expected name of property to access after @.").as_str());
                err_reporter.report_token(before);
                err_reporter.report_next_token(after);
            }

            ParserErrType::PropertyNotFound(token, name) => {
                err_reporter.writeln(
                    format!(
                        "Could not find a property with name '{}' in the current class.",
                        name,
                    )
                    .as_str(),
                );
                err_reporter.report_token(token);
            }

            ParserErrType::PropertyAccessorOutsideClass(token, name) => {
                err_reporter.writeln(
                    format!(
                        "Cannot access property '{}' as it is outside a method or constructor of a class.",
                        name,
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
