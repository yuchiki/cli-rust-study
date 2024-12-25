use assert_cmd::{cargo::CargoError, Command};
use predicates::prelude::*;
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn new_echor() -> Result<Command, CargoError> {
    Command::cargo_bin("echor")
}

#[test]
fn dies_no_args()  -> TestResult {
    let mut cmd = new_echor()?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn runs() -> TestResult {
    let mut cmd = new_echor()?;
    cmd.arg("Hello, World!").assert().success().stdout("Hello, World!\n");
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let outfile = format!("tests/expected/{}.txt", expected_file);
    let expected = fs::read_to_string(outfile)?;
    let mut cmd = new_echor()?;
    cmd.args(args).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello1() -> TestResult {
    run(&["Hello there"], "hello1")
}

#[test]
fn hello2() -> TestResult {
    run(&["Hello", "there"], "hello2")
}

#[test]
fn hello1_no_newline() -> TestResult {
    run(&["-n", "Hello there"], "hello1.n")
}

#[test]
fn hello2_no_newline() -> TestResult {
    run(&["-n", "Hello", "there"], "hello2.n")
}
