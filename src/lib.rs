use std::{
    any::type_name,
    fmt,
    fs::File,
    io,
    io::{prelude::*, BufReader},
    path::Path,
    result::Result,
    str::FromStr,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ReadFileError {
    #[error("file contains no data")]
    Empty,
    #[error("failed to convert <{0}>")]
    Parse(&'static str),
    #[error("failed to read file")]
    Read { source: io::Error },
    #[error(transparent)]
    IOError(#[from] io::Error),
}

pub fn read_file<T: FromStr>(file: &impl AsRef<Path>) -> Result<Vec<T>, ReadFileError>
where
    T::Err: fmt::Debug,
{
    let file = File::open(file).map_err(|source| ReadFileError::Read { source })?;
    let file_buf = BufReader::new(file);

    let mut result = vec![];
    for line in file_buf.lines() {
        result.push(
            line.map_err(|source| ReadFileError::Read { source })?
                .parse::<T>()
                .map_err(|_| ReadFileError::Parse(type_name::<T>()))?,
        )
    }

    match result.is_empty() {
        true => Err(ReadFileError::Empty),
        false => Ok(result),
    }
}

#[path = "../tests/common/mod.rs"]
mod common;

#[cfg(test)]
mod read_file {
    use super::*;
    use anyhow::Result;

    #[test]
    fn text() -> Result<()> {
        let input_file = common::input_file_path("read_file/text.txt");
        let result: Vec<String> = read_file(&input_file)?;

        assert_eq!(result, vec!["42", "hello world", "foo bar"]);
        Ok(())
    }

    #[test]
    fn numbers() -> Result<()> {
        let input_file = common::input_file_path("read_file/numbers.txt");
        let result: Vec<i32> = read_file(&input_file)?;

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        Ok(())
    }

    #[test]
    fn error_non_exist() {
        let input_file = common::input_file_path("read_file/non_exist.txt");
        let result: Result<Vec<i32>, ReadFileError> = read_file(&input_file);
        assert!(matches!(
            result,
            Err(crate::ReadFileError::Read { source: _ })
        ))
    }

    #[test]
    fn error_empty() {
        let input_file = common::input_file_path("read_file/empty.txt");
        let result: Result<Vec<i32>, ReadFileError> = read_file(&input_file);
        assert!(matches!(result, Err(crate::ReadFileError::Empty)))
    }

    #[test]
    fn error_parse() {
        let input_file = common::input_file_path("read_file/foobar.txt");
        let result: Result<Vec<i32>, ReadFileError> = read_file(&input_file);
        assert!(matches!(result, Err(crate::ReadFileError::Parse("i32"))))
    }
}
