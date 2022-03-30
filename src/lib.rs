pub mod parser;
pub mod sanitize;

use anyhow::{anyhow, Context, Result};
use indoc::indoc;
use std::{
    any::type_name,
    fs::File,
    io::{prelude::BufRead, BufReader},
    path::Path,
    str::FromStr,
};

pub const USAGE: &str = indoc! {"
TODO: add usage

"};

pub fn read_file<T: FromStr>(file: &impl AsRef<Path>) -> Result<Vec<T>> {
    let file = File::open(file).context("Failed to open file")?;
    let file_buf = BufReader::new(file);

    let mut result: Vec<T> = vec![];
    for line in file_buf.lines() {
        let line = line.context("Failed to read line")?;
        result.push(
            line.parse()
                .map_err(|_| anyhow!("failed to parse {:?}", type_name::<T>()))?,
        );
    }

    match result.is_empty() {
        true => Err(anyhow!("file contains no data")),
        false => Ok(result),
    }
}

#[cfg(test)]
#[path = "../tests/utils/mod.rs"]
mod test_utils;

#[cfg(test)]
mod read_file {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn text() -> Result<()> {
        let input_file = test_utils::input_file_path("read_file/text.txt");
        let result: Vec<String> = read_file(&input_file)?;
        assert_eq!(result, vec!["42", "hello world", "foo bar"]);
        Ok(())
    }

    #[test]
    fn numbers() -> Result<()> {
        let input_file = test_utils::input_file_path("read_file/numbers.txt");
        let result: Vec<i32> = read_file(&input_file)?;

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
        Ok(())
    }

    #[test]
    fn error_non_exist() -> Result<()> {
        let input_file = test_utils::input_file_path("read_file/non_exist.txt");
        let result: Result<Vec<i32>> = read_file(&input_file);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn error_empty() -> Result<()> {
        let input_file = test_utils::input_file_path("read_file/empty.txt");
        let result: Result<Vec<i32>> = read_file(&input_file);
        assert!(result.is_err());
        Ok(())
    }

    #[test]
    fn error_parse() -> Result<()> {
        let input_file = test_utils::input_file_path("read_file/foobar.txt");
        let result: Result<Vec<i32>> = read_file(&input_file);
        assert!(result.is_err());
        Ok(())
    }
}
