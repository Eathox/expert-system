use assert_cmd::assert::*;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

mod common;

#[path = "../src/main.rs"]
#[allow(dead_code, unused_imports)]
mod main;

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
fn invalid_arguments() {
    let expected = main::USAGE;
    run_cmd!().failure().stderr(expected);
    run_cmd!("foo", "bar").failure().stderr(expected);
}
