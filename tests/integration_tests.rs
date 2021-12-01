// TODO: Add integration tests

// mod integration_tests {
//     use assert_cmd::prelude::*;
//     use indoc::indoc;
//     use std::process::Command;

//     fn compare(input: &'static str, output: &'static str) {
//         let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
//         cmd.arg(input).assert().success().stdout(output);
//     }

//     #[test]
//     fn subject_example1() {
//         let input = "tests/valid_input/example_input.txt";
//         let output = indoc!("");
//     }
// }
