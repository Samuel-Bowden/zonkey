use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::{error::Error, path::Path, process::Command};

fn test_file(script_name: &str, success: bool) -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("zonkey")?;
    cmd.arg(format!("scripts/{}.zonk", script_name));
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
        "Failed to read file - No such file or directory (os error 2)\n",
    ));

    Ok(())
}

#[test]
fn and_or_script() -> Result<(), Box<dyn Error>> {
    test_file("and_or", true)
}

#[test]
fn add_and_remove_elements() -> Result<(), Box<dyn Error>> {
    test_file("add_remove_el", true)
}
