use std::fs;
use assert_cmd::Command;
use predicates::prelude::*;
type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn dies_no_args_test() -> TestResult{
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}
#[test]
fn runs_test() -> TestResult {
    let mut cmd = Command::cargo_bin("echor")?;
    cmd.arg("hello").assert().success();
    Ok(())
}
fn runs(args: &[&str], expected_file: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("echor")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}
#[test]
fn hello1_test() -> TestResult{
    runs(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2_test() -> TestResult {
    runs(&["Hello", "there"], "tests/expected/hello2.txt")
}
#[test]
fn hello1_no_newline_test() -> TestResult {
    runs(&["-n", "Hello  there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline_test() -> TestResult {
    runs(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
