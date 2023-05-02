use crate::{
    err::{InterpreterErr, InterpreterErrType},
    lexer::Lexer,
    parser::{err::ParserErr, Parser},
    token::Token,
    tree_walker::state::NullableReference,
};

fn get_failed_parser_err(tokens: Vec<Token>) -> ParserErr {
    match Parser::run(tokens) {
        Ok(_) => panic!("Expected parser to fail"),
        Err(e) => e,
    }
}

macro_rules! test_script_error {
    ( $x:literal ) => {
        let builder = std::thread::Builder::new().stack_size(crate::REQUIRED_STACK_SIZE);

        let handle = builder
            .spawn(move || {
                let source = include_str!(concat!("scripts/", $x, ".zonk"));

                let (error, graphemes) = match Lexer::run(source) {
                    (Ok(tokens), graphemes) => (
                        InterpreterErrType::ParserFailed(get_failed_parser_err(tokens)),
                        graphemes,
                    ),
                    (Err(lexer_err), graphemes) => {
                        (InterpreterErrType::LexerFailed(lexer_err), graphemes)
                    }
                };

                InterpreterErr::new(error, graphemes).get_err_messages()
            })
            .expect("Failed to create thread to test lexer and parser.");

        let error_message = handle.join().expect("Failed to join thread running test.");

        assert_eq!(
            error_message
                .chars()
                .filter(|char| !char.is_whitespace())
                .collect::<String>(),
            include_str!(concat!("expected_err_msg/", $x, ".txt"))
                .chars()
                .filter(|char| !char.is_whitespace())
                .collect::<String>(),
        );
    };
}

#[test]
fn test_sizes() {
    #[allow(dead_code)]
    enum Value {
        Integer(i64),
        Float(f64),
        String(Box<String>),
        Boolean(bool),
        Object(Box<NullableReference>),
    }
    println!(
        "Size of value enumeration: {} bytes",
        std::mem::size_of::<Value>()
    );
    println!(
        "Size of integer value: {} bytes",
        std::mem::size_of::<i64>()
    );
    println!("Size of float value: {} bytes", std::mem::size_of::<f64>());
    println!(
        "Size of string value: {} bytes",
        std::mem::size_of::<String>()
    );
    println!(
        "Size of string value in box: {} bytes",
        std::mem::size_of::<Box<String>>()
    );
    println!(
        "Size of boolean value: {} bytes",
        std::mem::size_of::<bool>()
    );
    println!(
        "Size of object nullable reference: {} bytes",
        std::mem::size_of::<NullableReference>()
    );
    println!(
        "Size of object nullable reference in box: {} bytes",
        std::mem::size_of::<Box<NullableReference>>()
    );
}

#[test]
fn method_call_expected_name() {
    test_script_error!("mcall_ex_name");
}

#[test]
fn sub_expression_limit() {
    test_script_error!("sub_expression_limit");
}

#[test]
fn scope_limit() {
    test_script_error!("scope_limit");
}

#[test]
fn class_dec_inbuilt_type() {
    test_script_error!("inbuilt_type");
}

#[test]
fn method_call_expected_left_paren() {
    test_script_error!("mcall_ex_left_paren");
}

#[test]
fn method_call_not_object() {
    test_script_error!("mcall_not_object");
}

#[test]
fn property_accessor_expected_name() {
    test_script_error!("prop_acc_ex_name");
}

#[test]
fn property_not_found() {
    test_script_error!("prop_not_found");
}

#[test]
fn property_accessor_outside_class() {
    test_script_error!("prop_acc_out_class");
}

#[test]
fn class_declaration_expected_name() {
    test_script_error!("class_dec_ex_name");
}

#[test]
fn class_declaration_expected_property_name() {
    test_script_error!("class_dec_ex_prop_name");
}

#[test]
fn class_declaration_redeclared_property() {
    test_script_error!("class_dec_redec_prop");
}

#[test]
fn class_declaration_redeclared_constructor() {
    test_script_error!("class_dec_redec_con");
}

#[test]
fn class_declaration_redeclared_method() {
    test_script_error!("class_dec_redec_mthd");
}

#[test]
fn class_declaration_expected_method_name() {
    test_script_error!("class_dec_ex_mthd_name");
}

#[test]
fn class_redeclared() {
    test_script_error!("class_redec");
}

#[test]
fn function_redeclared() {
    test_script_error!("fun_redec");
}

#[test]
fn start_redeclared() {
    test_script_error!("start_redec");
}

#[test]
fn variable_redeclared() {
    test_script_error!("var_redec");
}

#[test]
fn expected_value() {
    test_script_error!("ex_val");
}

#[test]
fn unterminated_statements() {
    test_script_error!("unterm_stmt");
}

#[test]
fn class_declaration_no_constructor() {
    test_script_error!("class_dec_no_con");
}

#[test]
fn class_declaration_properties_not_at_top() {
    test_script_error!("class_dec_prop_not_top");
}

#[test]
fn class_not_declared() {
    test_script_error!("class_not_declared");
}

#[test]
fn class_declaration_unterminated_property() {
    test_script_error!("class_dec_unterm_prop");
}

#[test]
fn class_declaration_expected_left_brace() {
    test_script_error!("class_dec_ex_left_brace");
}

#[test]
fn class_declaration_expected_right_brace() {
    test_script_error!("class_dec_ex_right_brace");
}

#[test]
fn code_outside_start_block() {
    test_script_error!("code_out_start");
}

#[test]
fn variable_not_found() {
    test_script_error!("var_not_found");
}

#[test]
fn bad_if_statement() {
    test_script_error!("bad_if");
}

#[test]
fn bad_function() {
    test_script_error!("bad_function");
}

#[test]
fn bad_class() {
    test_script_error!("bad_class");
}

#[test]
fn expected_right_brace() {
    test_script_error!("ex_right_brace");
}

#[test]
fn start_returning_invalid_value() {
    test_script_error!("start_ret_inv");
}

#[test]
fn continue_outside_loop() {
    test_script_error!("continue_outside_loop");
}

#[test]
fn break_outside_loop() {
    test_script_error!("break_outside_loop");
}

#[test]
fn array_value_wrong_type() {
    test_script_error!("array_value_wrong_type");
}

#[test]
fn array_type_not_closed() {
    test_script_error!("array_type_not_closed");
}

#[test]
fn array_expected_comma_or_right_bracket_to_close() {
    test_script_error!("array_ex_comma_or_end");
}

#[test]
fn empty_array_type() {
    test_script_error!("empty_array_type");
}

#[test]
fn nested_array_type() {
    test_script_error!("nested_array_type");
}

#[test]
fn overwriting_self() {
    test_script_error!("overwriting_self");
}

#[test]
fn while_expected_left_paren() {
    test_script_error!("while_ex_left_paren");
}

#[test]
fn while_expected_right_paren() {
    test_script_error!("while_ex_right_paren");
}

#[test]
fn while_condition_not_bool() {
    test_script_error!("while_con_not_bool");
}

#[test]
fn for_expected_let_to_start_initialiser() {
    test_script_error!("for_ex_let");
}

#[test]
fn for_expected_left_paren() {
    test_script_error!("for_ex_left_paren");
}

#[test]
fn for_expected_right_paren() {
    test_script_error!("for_ex_right_paren");
}

#[test]
fn for_expected_first_comma_to_separate_initialiser_statement_from_test_statement() {
    test_script_error!("for_ex_comma_1");
}

#[test]
fn for_expected_second_comma_to_separate_test_statement_from_updater_statement() {
    test_script_error!("for_ex_comma_2");
}

#[test]
fn for_condition_not_boolean() {
    test_script_error!("for_con_not_bool");
}

#[test]
fn block_expected_left_brace() {
    test_script_error!("block_ex_left_brace");
}

#[test]
fn block_expected_right_brace() {
    test_script_error!("block_ex_right_brace");
}

#[test]
fn function_start_class_declarations_outside_global_scope() {
    test_script_error!("dec_out_global");
}

#[test]
fn function_expected_name() {
    test_script_error!("fun_ex_name");
}

#[test]
fn declaration_expected_parameter_type() {
    test_script_error!("dec_ex_param_type");
}

#[test]
fn declaration_expected_left_paren() {
    test_script_error!("dec_ex_left_paren");
}

#[test]
fn declaration_expected_param_name() {
    test_script_error!("dec_ex_param_name");
}

#[test]
fn declaration_expected_comma_or_right_paren_for_parameter_list() {
    test_script_error!("dec_ex_comma_right_paren");
}

#[test]
fn declaration_expected_return_type() {
    test_script_error!("dec_ex_return_type");
}

#[test]
fn declaration_value_not_returned_in_all_cases() {
    test_script_error!("dec_ex_return_val");
}

#[test]
fn declaration_invalid_return_value_type() {
    test_script_error!("dec_inv_return_type");
}

#[test]
fn unary_invalid_type_for_operation() {
    test_script_error!("unary_inv_type");
}

#[test]
fn expected_right_paren_to_close_expression_group() {
    test_script_error!("grouping_ex_right_paren");
}

#[test]
fn binary_operation_types_not_matching() {
    test_script_error!("op_types_not_matching");
}

#[test]
fn binary_operation_invalid_for_type() {
    test_script_error!("op_inv_for_type");
}

#[test]
fn comparision_invalid_for_type() {
    test_script_error!("comp_inv_for_type");
}

#[test]
fn comparision_types_not_matching() {
    test_script_error!("comp_types_not_matching");
}

#[test]
fn variable_declaration_expected_name() {
    test_script_error!("var_dec_ex_name");
}

#[test]
fn variable_declaration_expected_equal() {
    test_script_error!("var_dec_ex_equal");
}

#[test]
fn variable_declaration_assignment_expression_evaluated_to_none() {
    test_script_error!("var_dec_eval_none");
}

#[test]
fn assignment_operation_invalid_for_type() {
    test_script_error!("ass_op_inv_for_type");
}

#[test]
fn assignment_operation_types_not_matching() {
    test_script_error!("ass_op_types_not_matching");
}

#[test]
fn if_condition_does_not_evaluate_to_boolean() {
    test_script_error!("if_con_not_bool");
}

#[test]
fn if_expected_left_paren() {
    test_script_error!("if_ex_left_paren");
}

#[test]
fn if_expected_right_paren() {
    test_script_error!("if_ex_right_paren");
}

#[test]
fn method_call_not_found() {
    test_script_error!("mcall_not_found");
}

#[test]
fn call_not_found() {
    test_script_error!("call_not_found");
}

#[test]
fn call_incorrect_number_of_arguments() {
    test_script_error!("call_inc_arg_num");
}

#[test]
fn call_incorrect_arguments_type() {
    test_script_error!("call_inc_arg_type");
}

#[test]
fn call_expected_comma_or_right_paren() {
    test_script_error!("call_ex_comma_right_paren");
}

#[test]
fn unexpected_token_in_global_scope() {
    test_script_error!("unex_token_global");
}

#[test]
fn array_empty_type() {
    test_script_error!("array_empty_type");
}

#[test]
fn unexpected_grapheme() {
    test_script_error!("unexpected_grapheme");
}

#[test]
fn unterminated_string() {
    test_script_error!("unterminated_string");
}

#[test]
fn float_more_than_one_decimal_point() {
    test_script_error!("float_more_than_one_dec_point");
}

#[test]
fn failed_to_parse_integer() {
    test_script_error!("failed_to_parse_integer");
}
