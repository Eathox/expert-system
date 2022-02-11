use std::borrow::Borrow;

fn remove_spaces(line: impl Borrow<str>) -> String {
    line.borrow().split_whitespace().collect()
}

fn remove_comment(line: impl Borrow<str>) -> String {
    line.borrow().split_terminator('#').take(1).collect()
}

pub fn sanitize_lines(lines: &[impl Borrow<str>]) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut prev = "".to_string();
    for line in lines.iter() {
        let mut sanitized = line.borrow().to_string();
        sanitized = remove_comment(sanitized);
        sanitized = remove_spaces(sanitized);
        if !result.is_empty() && prev.is_empty() && !sanitized.is_empty() {
            result.push("".to_string());
        }
        if !sanitized.is_empty() {
            result.push(sanitized.clone());
        }
        prev = sanitized
    }

    result
}

#[path = "../tests/common/mod.rs"]
mod common;

#[cfg(test)]
mod remove_spaces {
    use super::*;

    #[test]
    fn only_whitespace() {
        let input = " \t\n\r";
        let result: String = remove_spaces(input);

        assert_eq!(result, "");
    }

    #[test]
    fn no_whitespace() {
        let input = "hello-world";
        let result: String = remove_spaces(input);

        assert_eq!(result, input);
    }

    #[test]
    fn mixed() {
        let input = "  h e l l o\nw o r l d  ";
        let result: String = remove_spaces(input);

        assert_eq!(result, "helloworld");
    }
}

#[cfg(test)]
mod remove_comment {
    use super::*;

    #[test]
    fn only_comment() {
        let input = "#hello world";
        let result: String = remove_comment(input);

        assert_eq!(result, "");
    }

    #[test]
    fn no_comment() {
        let input = "hello world";
        let result: String = remove_comment(input);

        assert_eq!(result, input);
    }

    #[test]
    fn mixed() {
        let input = "hello #world";
        let result: String = remove_comment(input);

        assert_eq!(result, "hello ");
    }
}

#[cfg(test)]
mod sanitize_lines {
    use super::*;
    use anyhow::Result;
    use expert_system::read_file;

    #[test]
    fn empty_lines() {
        let input: Vec<&str> = vec!["  ", "", "\t"];
        let result: Vec<String> = sanitize_lines(&input);

        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn empty_lines_at_end() {
        let input: Vec<&str> = vec!["hello", "  ", "", "\t"];
        let result: Vec<String> = sanitize_lines(&input);

        assert_eq!(result, vec!["hello"]);
    }

    #[test]
    fn merge_empty_lines() {
        let input: Vec<&str> = vec!["hello", "  ", "", "\t", "world"];
        let result: Vec<String> = sanitize_lines(&input);

        assert_eq!(result, vec!["hello", "", "world"]);
    }

    #[test]
    fn every_other_empty() {
        let input: Vec<&str> = vec!["f", "", "o", "", "o"];
        let result: Vec<String> = sanitize_lines(&input);

        assert_eq!(result, vec!["f", "", "o", "", "o"]);
    }

    #[test]
    fn example_input() -> Result<()> {
        let input_file = common::input_file_path("sanitize_lines/example_input.txt");
        let input: Vec<String> = read_file(input_file)?;

        let expect_file = common::input_file_path("sanitize_lines/example_input_expected.txt");
        let expected: Vec<String> = read_file(expect_file)?;

        let result: Vec<String> = sanitize_lines(&input);
        assert_eq!(result, expected);
        Ok(())
    }
}
