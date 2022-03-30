extern crate expert_system;
use expert_system::USAGE;

mod utils;

use assert_cmd::assert::*;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

macro_rules! run_cmd {
    ( $( $l:literal ),* ) => {{
        let temp_vec: Vec<String> = vec![ $( $l.to_string() ),* ];
        run_cmd!(temp_vec)
    }};
    ( $( $e:expr ),* ) => {{
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        $( cmd.args($e); )*
        cmd.assert()
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
