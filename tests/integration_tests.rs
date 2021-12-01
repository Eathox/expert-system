mod integration_tests {
    use assert_cmd::assert::*;
    use std::ffi::OsStr;

    use assert_cmd::cargo::CommandCargoExt;
    use expert_system::*;
    use std::process::Command;

    fn run_cmd_args<T, S>(input: T) -> Assert
    where
        T: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        cmd.args(input).assert()
    }

    fn run_cmd() -> Assert {
        run_cmd_args::<Vec<String>, String>(vec![])
    }

    #[test]
    fn no_arguments() {
        let output = USAGE;

        run_cmd().failure().stderr(output);
    }

    #[test]
    fn two_arguments() {
        let input = ["foo", "bar"];
        let output = USAGE;

        run_cmd_args(input).failure().stderr(output);
    }
}
