use anyhow::Result;
use expert_system::*;

mod common;

mod read_file {
    use super::*;

    #[test]
    fn text() -> Result<()> {
        let input_file = common::crate_input_file_path("text.txt");
        let result: Vec<String> = read_file(input_file)?;

        assert_eq!(result, vec!["42", "hello world", "foo bar"]);
        Ok(())
    }

    #[test]
    fn numbers() -> Result<()> {
        let input_file = common::crate_input_file_path("numbers.txt");
        let result: Vec<i32> = read_file(input_file)?;

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        Ok(())
    }

    #[test]
    fn error_non_exist() {
        let input_file = common::crate_input_file_path("non_exist.txt");
        let result: Result<Vec<i32>, ReadFileError> = read_file(input_file);
        assert!(matches!(
            result,
            Err(crate::ReadFileError::Read { source: _ })
        ))
    }

    #[test]
    fn error_empty() {
        let input_file = common::crate_input_file_path("empty.txt");
        let result: Result<Vec<i32>, ReadFileError> = read_file(input_file);
        assert!(matches!(result, Err(crate::ReadFileError::Empty)))
    }

    #[test]
    fn error_parse() {
        let input_file = common::crate_input_file_path("foobar.txt");
        let result: Result<Vec<i32>, ReadFileError> = read_file(input_file);
        assert!(matches!(result, Err(crate::ReadFileError::Parse("i32"))))
    }
}
