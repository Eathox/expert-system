extern crate expert_system;
use expert_system::*;

use anyhow::{anyhow, Context, Result};
use parser::*;
use std::{env, path::PathBuf};

#[derive(Debug)]
pub struct Input {
    rules: Vec<String>,
    facts: Vec<String>,
    queries: Vec<String>,
}

impl TryFrom<PathBuf> for Input {
    type Error = anyhow::Error;

    fn try_from(file_path: PathBuf) -> Result<Self, Self::Error> {
        let content: Vec<String> = read_file(&file_path)
            .context(format!("Failed to read input file: '{:?}'", file_path))?;
        let lines = sanitize::sanitize_lines(&content);

        let sections: Vec<&[String]> = lines.split(|s| s.is_empty()).collect::<Vec<_>>();
        match sections.len() {
            3 => Ok(Input {
                rules: sections[0].to_vec(),
                facts: sections[1].to_vec(),
                queries: sections[2].to_vec(),
            }),
            c if c < 3 => Err(anyhow!("Too few sections in input file")),
            _ => Err(anyhow!("Too many sections in input file")),
        }
    }
}

fn handle_cli() -> String {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => args[1].clone(),
        _ => {
            eprint!("{}", USAGE);
            std::process::exit(1);
        }
    }
}

fn main() -> Result<()> {
    let input_file = handle_cli();
    let input = Input::try_from(PathBuf::from(input_file))?;

    for rule in input.rules {
        let table = TruthTable::try_from(PermutationIter::new(&rule))
            .context(format!("Failed to parse rule {}", rule))?;
        println!("{}\n{}", rule, table);
    }

    Ok(())
}

#[cfg(test)]
#[path = "../tests/test_utils/mod.rs"]
pub mod test_utils;

#[cfg(test)]
mod input {
    use super::*;

    use pretty_assertions::assert_eq;

    #[test]
    fn empty() {
        let input_file = test_utils::input_file_path("integration_test/empty.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Too few sections in input file"
        );
    }

    #[test]
    fn to_few_sections() {
        let input_file = test_utils::input_file_path("integration_test/to_few_sections.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Too few sections in input file"
        );
    }

    #[test]
    fn to_many_sections() {
        let input_file = test_utils::input_file_path("integration_test/to_many_sections.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Too many sections in input file"
        );
    }
}
