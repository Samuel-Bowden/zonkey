use crate::{
    err::{self, parser::ParserErr},
    lexer::Lexer,
    parser::Parser,
    token::Token,
};
use unicode_segmentation::UnicodeSegmentation;

fn get_tokens(graphemes: &Vec<&str>) -> Vec<Token> {
    let lexer = Lexer::new(graphemes)
        .run()
        .expect("Expected lexer to succeed");
    lexer.tokens
}

fn get_failed_parser_err(tokens: Vec<Token>) -> ParserErr {
    match Parser::new(tokens).run() {
        Ok(_) => panic!("Expected parser to fail"),
        Err(e) => e,
    }
}

macro_rules! test_script_error {
    ( $x:literal ) => {
        let graphemes =
            UnicodeSegmentation::graphemes(include_str!(concat!("scripts/", $x, ".zonk")), true)
                .collect::<Vec<&str>>();
        let error = get_failed_parser_err(get_tokens(&graphemes));
        let error_message = err::handler::run(err::InterpreterErr::ParserFailed(error), &graphemes);
        assert_eq!(
            error_message,
            include_str!(concat!("expected_err_msg/", $x, ".txt"))
        );
    };
}

#[test]
fn method_call_expected_name() {
    test_script_error!("mcall_ex_name");
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
fn bad_casting() {
    test_script_error!("bad_casting");
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
fn no_value_returned() {
    test_script_error!("no_value_returned");
}

#[test]
fn continue_outside_loop() {
    test_script_error!("continue_outside_loop");
}

#[test]
fn break_outside_loop() {
    test_script_error!("break_outside_loop");
}
