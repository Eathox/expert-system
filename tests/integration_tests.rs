#[cfg(test)]
mod integration_tests {
    use assert_cmd::assert::*;
    use assert_cmd::cargo::CommandCargoExt;
    use expert_system::*;
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
    fn invalid_arguments() {
        let output = USAGE;
        run_cmd!().failure().stderr(output);
        run_cmd!("foo", "bar").failure().stderr(output);
    }
}
