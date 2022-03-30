extern crate expert_system;
use expert_system::USAGE;

mod test_utils;

use assert_cmd::assert::*;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

macro_rules! _run_cmd {
    ( $( $e:expr ),+ ) => {{
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        $( cmd.args($e); )+
        cmd.assert()
    }};
}

macro_rules! run_cmd {
    ( $( $l:literal ),* ) => {{
        let temp_vec: Vec<String> = vec![ $( $l.to_string() ),* ];
        _run_cmd!(temp_vec)
    }};
    ( $( $e:expr ),* ) => {{
        let temp_vec: Vec<String> = vec![ $( $e ),* ];
        _run_cmd!(temp_vec)
    }};
}

#[test]
fn no_arguments() {
    let expected = USAGE;
    run_cmd!().failure().stderr(expected);
}

#[test]
fn to_many_arguments() {
    let expected = USAGE;
    run_cmd!("foo", "bar").failure().stderr(expected);
}

#[test]
fn error_empty() {
    let input_file = test_utils::input_file_path("integration_test/empty.txt");
    run_cmd!(input_file.display().to_string()).failure();
}

#[test]
fn error_invalid_rule() {
    let input_file = test_utils::input_file_path("integration_test/invalid_rule.txt");
    run_cmd!(input_file.display().to_string()).failure();
}
