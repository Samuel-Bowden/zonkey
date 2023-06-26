use assert_cmd::Command;
use std::{error::Error, fs::read_to_string};

macro_rules! test_success {
    ( $script_name:literal, $argument:literal) => {
        assert_eq!(
            std::str::from_utf8(
                &Command::cargo_bin("zonkey")
                    .unwrap()
                    .arg("run")
                    .arg($argument)
                    .assert()
                    .success()
                    .get_output()
                    .stdout
            )
            .unwrap()
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect::<String>(),
            include_str!(concat!("expected_output/", $script_name, ".txt"))
                .chars()
                .filter(|char| !char.is_whitespace())
                .collect::<String>()
        )
    };
}

macro_rules! test_fail {
    ( $script_name:literal, $argument:literal) => {
        assert_eq!(
            std::str::from_utf8(
                &Command::cargo_bin("zonkey")
                    .unwrap()
                    .arg("run")
                    .arg($argument)
                    .assert()
                    .failure()
                    .get_output()
                    .stderr
            )
            .unwrap()
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect::<String>(),
            include_str!(concat!("expected_output/", $script_name, ".txt"))
                .chars()
                .filter(|char| !char.is_whitespace())
                .collect::<String>()
        )
    };
}

#[cfg(target_os = "linux")]
#[test]
fn file_does_not_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zonkey")?;

    cmd.arg("run").arg("/a-directory-that-does-not-exist");
    cmd.assert()
        .failure()
        .stderr("Failed to read file - No such file or directory (os error 2)");

    Ok(())
}

#[cfg(target_os = "windows")]
#[test]
fn file_does_not_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zonkey")?;

    cmd.arg("run").arg("/a-directory-that-does-not-exist");
    cmd.assert()
        .failure()
        .stderr("Failed to read file - The system cannot find the file specified. (os error 2)");

    Ok(())
}

#[test]
fn learning_var_types_expr() -> Result<(), Box<dyn Error>> {
    test_success!(
        "learning_var_type_expr",
        "tests/scripts/learning_var_type_expr.zonk"
    );
    Ok(())
}

#[test]
fn learning_arrays() -> Result<(), Box<dyn Error>> {
    test_success!("learning_arrays", "tests/scripts/learning_arrays.zonk");
    Ok(())
}

#[test]
fn and_or_script() -> Result<(), Box<dyn Error>> {
    test_success!("and_or", "tests/scripts/and_or.zonk");
    Ok(())
}

#[test]
fn first_page() -> Result<(), Box<dyn Error>> {
    test_success!("first_page", "tests/scripts/first_page.zonk");
    Ok(())
}

#[test]
fn method_chain_stress_test() -> Result<(), Box<dyn Error>> {
    test_success!("method_chain", "tests/scripts/method_chain.zonk");
    Ok(())
}

#[test]
fn learning_classes() -> Result<(), Box<dyn Error>> {
    test_success!("learning_classes", "tests/scripts/learning_classes.zonk");
    Ok(())
}

#[test]
fn unary() -> Result<(), Box<dyn Error>> {
    test_success!("unary", "tests/scripts/unary.zonk");
    Ok(())
}

#[test]
fn nested_subexpression_scope_approaching_limit() -> Result<(), Box<dyn Error>> {
    test_success!(
        "nested_subexpr_scope_limit",
        "tests/scripts/nested_subexpr_scope_limit.zonk"
    );
    Ok(())
}

#[test]
fn power() -> Result<(), Box<dyn Error>> {
    test_success!("power", "tests/scripts/power.zonk");
    Ok(())
}

#[test]
fn powerf() -> Result<(), Box<dyn Error>> {
    test_success!("powerf", "tests/scripts/powerf.zonk");
    Ok(())
}

#[test]
fn learning_circle_area() -> Result<(), Box<dyn Error>> {
    test_success!(
        "learning_circle_area",
        "tests/scripts/learning_circle_area.zonk"
    );
    Ok(())
}

#[test]
fn add_and_remove_elements_from_page() -> Result<(), Box<dyn Error>> {
    test_success!(
        "add_remove_el_page",
        "tests/scripts/add_remove_el_page.zonk"
    );
    Ok(())
}

#[test]
fn add_and_remove_elements_from_row_and_column() -> Result<(), Box<dyn Error>> {
    test_success!(
        "add_remove_el_row_col",
        "tests/scripts/add_remove_el_row_col.zonk"
    );
    Ok(())
}

#[test]
fn integer_array() -> Result<(), Box<dyn Error>> {
    test_success!("integer_array", "tests/scripts/integer_array.zonk");
    Ok(())
}

#[test]
fn float_array() -> Result<(), Box<dyn Error>> {
    test_success!("float_array", "tests/scripts/float_array.zonk");
    Ok(())
}

#[test]
fn string_array() -> Result<(), Box<dyn Error>> {
    test_success!("string_array", "tests/scripts/string_array.zonk");
    Ok(())
}

#[test]
fn boolean_array() -> Result<(), Box<dyn Error>> {
    test_success!("boolean_array", "tests/scripts/boolean_array.zonk");
    Ok(())
}

#[test]
fn zonkey_object_array() -> Result<(), Box<dyn Error>> {
    test_success!(
        "zonkey_object_array",
        "tests/scripts/zonkey_object_array.zonk"
    );
    Ok(())
}

#[test]
fn native_object_array() -> Result<(), Box<dyn Error>> {
    test_success!(
        "native_object_array",
        "tests/scripts/native_object_array.zonk"
    );
    Ok(())
}

#[test]
fn good_casting() -> Result<(), Box<dyn Error>> {
    test_success!("good_casting", "tests/scripts/good_casting.zonk");
    Ok(())
}

#[test]
fn failed_string_to_float_cast() -> Result<(), Box<dyn Error>> {
    test_fail!(
        "string_to_float_cast_failed",
        "tests/scripts/string_to_float_cast_failed.zonk"
    );
    Ok(())
}

#[test]
fn failed_string_to_integer_cast() -> Result<(), Box<dyn Error>> {
    test_fail!(
        "string_to_integer_cast_failed",
        "tests/scripts/string_to_integer_cast_failed.zonk"
    );
    Ok(())
}

#[test]
fn factorial() -> Result<(), Box<dyn Error>> {
    test_success!("factorial", "tests/scripts/factorial.zonk");
    Ok(())
}

#[test]
fn fibonacci() -> Result<(), Box<dyn Error>> {
    test_success!("fibonacci", "tests/scripts/fibonacci.zonk");
    Ok(())
}

#[test]
fn property_not_initialised() -> Result<(), Box<dyn Error>> {
    test_fail!(
        "property_not_initialised",
        "tests/scripts/property_not_initialised.zonk"
    );
    Ok(())
}

#[test]
fn index_invalid_position() -> Result<(), Box<dyn Error>> {
    test_fail!(
        "index_invalid_position",
        "tests/scripts/index_invalid_position.zonk"
    );
    Ok(())
}

#[test]
fn divide_by_zero() -> Result<(), Box<dyn Error>> {
    test_fail!("divide_by_zero", "tests/scripts/divide_by_zero.zonk");
    Ok(())
}

#[test]
fn invalid_hex_colour() -> Result<(), Box<dyn Error>> {
    test_fail!("invalid_colour", "tests/scripts/invalid_colour.zonk");
    Ok(())
}

#[test]
fn complex_expressions() -> Result<(), Box<dyn Error>> {
    test_success!(
        "complex_expressions",
        "tests/scripts/complex_expressions.zonk"
    );
    Ok(())
}

#[test]
fn get_request() -> Result<(), Box<dyn Error>> {
    test_success!("get_request", "tests/scripts/get_request.zonk");
    let written_data = read_to_string("get_request_response.txt").expect("Unable to read file");
    assert!(written_data.contains(r#"arg1": "hello"#));
    std::fs::remove_file("get_request_response.txt")?;
    Ok(())
}

#[test]
fn post_request() -> Result<(), Box<dyn Error>> {
    test_success!("post_request", "tests/scripts/post_request.zonk");
    let written_data = read_to_string("post_request_response.txt").expect("Unable to read file");
    assert!(written_data.contains(r#"data": "Hello from the client"#));
    std::fs::remove_file("post_request_response.txt")?;
    Ok(())
}

#[test]
fn insufficient_permission_level_for_network_script() -> Result<(), Box<dyn Error>> {
    test_fail!(
        "insufficient_perm_level",
        "https://codeberg.org/Sam-Bowden/pages/raw/branch/master/insufficient_perm_level.zonk"
    );
    Ok(())
}

#[test]
fn read_and_write_file() -> Result<(), Box<dyn Error>> {
    std::fs::write("test.txt", "Here is a sequence of numbers from 1 to 10:")
        .expect("Unable to write file");
    test_success!(
        "read_and_write_file",
        "tests/scripts/read_and_write_file.zonk"
    );
    let written_data = read_to_string("test.txt").expect("Unable to read file");
    assert_eq!(
        "Here is a sequence of numbers from 1 to 10: 1 2 3 4 5 6 7 8 9 10",
        written_data
    );
    std::fs::remove_file("test.txt")?;
    Ok(())
}

#[test]
fn input() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        std::str::from_utf8(
            &Command::cargo_bin("zonkey")
                .unwrap()
                .arg("run")
                .arg("tests/scripts/input.zonk")
                .write_stdin("Sam Bowden")
                .assert()
                .success()
                .get_output()
                .stdout
        )
        .unwrap()
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>(),
        include_str!("expected_output/input.txt")
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect::<String>()
    );

    Ok(())
}

#[test]
fn args() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        std::str::from_utf8(
            &Command::cargo_bin("zonkey")
                .unwrap()
                .arg("run")
                .arg("tests/scripts/args.zonk")
                .arg("one")
                .arg("two")
                .arg("three")
                .assert()
                .success()
                .get_output()
                .stdout
        )
        .unwrap()
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>(),
        include_str!("expected_output/args.txt")
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect::<String>()
    );

    Ok(())
}

#[test]
fn learning_loops() -> Result<(), Box<dyn Error>> {
    assert_eq!(
        std::str::from_utf8(
            &Command::cargo_bin("zonkey")
                .unwrap()
                .arg("run")
                .arg("tests/scripts/learning_loops.zonk")
                .write_stdin("exit")
                .assert()
                .success()
                .get_output()
                .stdout
        )
        .unwrap()
        .chars()
        .filter(|char| !char.is_whitespace())
        .collect::<String>(),
        include_str!("expected_output/learning_loops.txt")
            .chars()
            .filter(|char| !char.is_whitespace())
            .collect::<String>()
    );

    Ok(())
}
