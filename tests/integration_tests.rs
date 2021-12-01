mod integration_tests {
    use assert_cmd::assert::*;
    use assert_cmd::cargo::CommandCargoExt;
    use expert_system::*;
    use std::process::Command;

    macro_rules! run_cmd {
        ( $( $x:expr ),* ) => {{
            let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
            $( cmd.args($x); )*
            cmd.assert()
        }}
    }

    #[test]
    fn no_arguments() {
        run_cmd!().failure().stderr(USAGE);
    }

    #[test]
    fn two_arguments() {
        let input = ["foo", "bar"];
        run_cmd!(input).failure().stderr(USAGE);
    }
}
