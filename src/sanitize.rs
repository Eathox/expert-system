fn remove_spaces(line: impl AsRef<str>) -> String {
    line.as_ref().split_whitespace().collect()
}

fn remove_comment(line: impl AsRef<str>) -> String {
    line.as_ref().split_terminator("#").take(1).collect()
}

pub fn sanitize_lines(lines: &Vec<impl AsRef<str>>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut prev = "".to_string();
    for line in lines.iter() {
        let mut sanitized = line.as_ref().to_string();
        sanitized = remove_comment(&sanitized);
        sanitized = remove_spaces(&sanitized);
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
        let result: String = remove_spaces(&input);

        assert_eq!(result, "");
    }

    #[test]
    fn no_whitespace() {
        let input = "hello-world";
        let result: String = remove_spaces(&input);

        assert_eq!(result, input);
    }

    #[test]
    fn mixed() {
        let input = "  h e l l o\nw o r l d  ";
        let result: String = remove_spaces(&input);

        assert_eq!(result, "helloworld");
    }
}

#[cfg(test)]
mod remove_comment {
    use super::*;

    #[test]
    fn only_comment() {
        let input = "#hello world";
        let result: String = remove_comment(&input);

        assert_eq!(result, "");
    }

    #[test]
    fn no_comment() {
        let input = "hello world";
        let result: String = remove_comment(&input);

        assert_eq!(result, input);
    }

    #[test]
    fn mixed() {
        let input = "hello #world";
        let result: String = remove_comment(&input);

        assert_eq!(result, "hello ");
    }
}
