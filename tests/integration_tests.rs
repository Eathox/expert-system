mod integration_tests {
    use assert_cmd::assert::*;
    use assert_cmd::cargo::CommandCargoExt;
    use expert_system::*;
    use std::process::Command;

    macro_rules! run_cmd {
        ( $( $x:literal ),* ) => {{
            let temp_vec: Vec<String> = vec![ $( $x.to_string() ),* ];
            run_cmd!(temp_vec)
        }};
        ( $( $i:expr ),* ) => {{
            let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
            $( cmd.args($i); )*
            cmd.assert()
        }};
    }

    #[test]
    fn invalid_arguments() {
        let output = USAGE;
        run_cmd!().failure().stderr(output);
        run_cmd!("foo", "bar").failure().stderr(output);
    }
}
