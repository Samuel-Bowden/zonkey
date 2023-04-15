use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{error::Error, fs::read_to_string, path::Path, process::Command};

fn test_file(script_name: &str, argument: &str, success: bool) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zonkey")?;
    cmd.arg(argument);
    if success {
        cmd.assert()
            .success()
            .stdout(predicates::path::eq_file(Path::new(&format!(
                "expected_output/{}.txt",
                script_name
            ))));
    } else {
        cmd.assert()
            .failure()
            .stderr(predicates::path::eq_file(Path::new(&format!(
                "expected_output/{}.txt",
                script_name
            ))));
    }
    Ok(())
}

#[test]
fn file_does_not_exist() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zonkey")?;

    cmd.arg("/a-directory-that-does-not-exist");
    cmd.assert().failure().stderr(predicate::eq(
        "Failed to read file - No such file or directory (os error 2)",
    ));

    Ok(())
}

#[test]
fn and_or_script() -> Result<(), Box<dyn Error>> {
    test_file("and_or", "scripts/and_or.zonk", true)
}

#[test]
fn add_and_remove_elements_from_page() -> Result<(), Box<dyn Error>> {
    test_file(
        "add_remove_el_page",
        "scripts/add_remove_el_page.zonk",
        true,
    )
}

#[test]
fn add_and_remove_elements_from_row_and_column() -> Result<(), Box<dyn Error>> {
    test_file(
        "add_remove_el_row_col",
        "scripts/add_remove_el_row_col.zonk",
        true,
    )
}

#[test]
fn integer_array() -> Result<(), Box<dyn Error>> {
    test_file("integer_array", "scripts/integer_array.zonk", true)
}

#[test]
fn float_array() -> Result<(), Box<dyn Error>> {
    test_file("float_array", "scripts/float_array.zonk", true)
}

#[test]
fn string_array() -> Result<(), Box<dyn Error>> {
    test_file("string_array", "scripts/string_array.zonk", true)
}

#[test]
fn boolean_array() -> Result<(), Box<dyn Error>> {
    test_file("boolean_array", "scripts/boolean_array.zonk", true)
}

#[test]
fn zonkey_object_array() -> Result<(), Box<dyn Error>> {
    test_file(
        "zonkey_object_array",
        "scripts/zonkey_object_array.zonk",
        true,
    )
}

#[test]
fn native_object_array() -> Result<(), Box<dyn Error>> {
    test_file(
        "native_object_array",
        "scripts/native_object_array.zonk",
        true,
    )
}

#[test]
fn good_casting() -> Result<(), Box<dyn Error>> {
    test_file("good_casting", "scripts/good_casting.zonk", true)
}

#[test]
fn failed_string_to_float_cast() -> Result<(), Box<dyn Error>> {
    test_file(
        "string_to_float_cast_failed",
        "scripts/string_to_float_cast_failed.zonk",
        false,
    )
}

#[test]
fn failed_string_to_integer_cast() -> Result<(), Box<dyn Error>> {
    test_file(
        "string_to_integer_cast_failed",
        "scripts/string_to_integer_cast_failed.zonk",
        false,
    )
}

#[test]
fn factorial() -> Result<(), Box<dyn Error>> {
    test_file("factorial", "scripts/factorial.zonk", true)
}

#[test]
fn fibonacci() -> Result<(), Box<dyn Error>> {
    test_file("fibonacci", "scripts/fibonacci.zonk", true)
}

#[test]
fn property_not_initialised() -> Result<(), Box<dyn Error>> {
    test_file(
        "property_not_initialised",
        "scripts/property_not_initialised.zonk",
        false,
    )
}

#[test]
fn index_invalid_position() -> Result<(), Box<dyn Error>> {
    test_file(
        "index_invalid_position",
        "scripts/index_invalid_position.zonk",
        false,
    )
}

#[test]
fn divide_by_zero() -> Result<(), Box<dyn Error>> {
    test_file("divide_by_zero", "scripts/divide_by_zero.zonk", false)
}

#[test]
fn get_request() -> Result<(), Box<dyn Error>> {
    test_file("get_request", "scripts/get_request.zonk", true).expect("Unexpected output");
    let written_data = read_to_string("get_request_response.txt").expect("Unable to read file");
    assert!(written_data.contains(r#"arg1": "hello"#));
    Ok(())
}

#[test]
fn post_request() -> Result<(), Box<dyn Error>> {
    test_file("post_request", "scripts/post_request.zonk", true).expect("Unexpected output");
    let written_data = read_to_string("post_request_response.txt").expect("Unable to read file");
    assert!(written_data.contains(r#"data": "Hello from the client"#));
    Ok(())
}

#[test]
fn insufficient_permission_level_for_network_script() -> Result<(), Box<dyn Error>> {
    test_file(
        "insufficient_perm_level",
        "https://codeberg.org/Sam-Bowden/pages/raw/branch/master/insufficient_perm_level.zonk",
        false,
    )
}

#[test]
fn read_and_write_file() -> Result<(), Box<dyn Error>> {
    std::fs::write("test.txt", "Here is a sequence of numbers from 1 to 10:")
        .expect("Unable to write file");
    test_file(
        "read_and_write_file",
        "scripts/read_and_write_file.zonk",
        true,
    )
    .expect("Unexpected output");
    let written_data = read_to_string("test.txt").expect("Unable to read file");
    assert_eq!(
        "Here is a sequence of numbers from 1 to 10: 1 2 3 4 5 6 7 8 9 10",
        written_data
    );
    Ok(())
}
