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
fn spacing() {
    let input_file = test_utils::input_file_path("integration_test/spacing.txt");
    run_cmd!(input_file.display().to_string()).success();
}

#[test]
fn white_space() {
    let input_file = test_utils::input_file_path("integration_test/white_space.txt");
    run_cmd!(input_file.display().to_string()).success();
}

#[test]
fn example_input() {
    let input_file = test_utils::input_file_path("integration_test/example_input.txt");
    run_cmd!(input_file.display().to_string()).success();
}

#[test]
fn error_usage_no_arguments() {
    run_cmd!().failure().stderr(USAGE);
}

#[test]
fn error_usage_to_many_arguments() {
    run_cmd!("foo", "bar").failure().stderr(USAGE);
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

#[test]
fn error_invalid_facts() {
    let input_file = test_utils::input_file_path("integration_test/invalid_facts.txt");
    run_cmd!(input_file.display().to_string()).failure();
}

#[test]
fn error_invalid_queries() {
    let input_file = test_utils::input_file_path("integration_test/invalid_queries.txt");
    run_cmd!(input_file.display().to_string()).failure();
}
