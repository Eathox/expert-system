extern crate expert_system;
use expert_system::*;

use anyhow::{anyhow, Context, Result};
use core::fmt;
use parser::*;
use std::{collections::HashSet, env, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Input {
    rules: Vec<String>,
    facts: String,
    queries: String,
}

impl fmt::Display for Input {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Rules:")?;
        for rule in self.rules.iter() {
            writeln!(f, "  {}", rule)?;
        }
        writeln!(f, "Facts: {}", self.facts)?;
        writeln!(f, "Queries: {}", self.queries)?;
        Ok(())
    }
}

impl TryFrom<PathBuf> for Input {
    type Error = anyhow::Error;

    fn try_from(file_path: PathBuf) -> Result<Self, Self::Error> {
        let content: Vec<String> = read_file(&file_path)
            .context(format!("Failed to read input file: '{:?}'", file_path))?;
        let mut lines = sanitize::sanitize_lines(&content);

        let mut rules: Vec<String> = vec![];
        let mut facts: Option<String> = None;
        let mut queries: Option<String> = None;
        for line in lines.iter_mut() {
            match line {
                l if l.starts_with("=") || l.starts_with("?") => match l.remove(0) {
                    '=' => match facts {
                        None => facts = Some(l.to_string()),
                        Some(_) => Err(anyhow!("Multiple facts found in input file"))?,
                    },
                    '?' => match queries {
                        None => queries = Some(l.to_string()),
                        Some(_) => Err(anyhow!("Multiple queries found in input file"))?,
                    },
                    _ => unreachable!(),
                },
                l if !l.is_empty() => rules.push(l.to_string()),
                _ => continue,
            }
        }

        let facts = facts.context("No facts in input file")?;
        if let Some(c) = facts.chars().find(|c| !is_identifier(c)) {
            Err(anyhow!("Invalid identifier in facts: '{}'", c))?
        }
        let queries = queries.context("No queries in input file")?;
        if let Some(c) = queries.chars().find(|c| !is_identifier(c)) {
            Err(anyhow!("Invalid identifier in query: '{}'", c))?
        }

        let mut fact_set = HashSet::new();
        let mut queries_set = HashSet::new();
        Ok(Input {
            rules,
            facts: facts
                .chars()
                .filter(|c| fact_set.insert(c.to_owned()))
                .collect(),
            queries: queries
                .chars()
                .filter(|c| queries_set.insert(c.to_owned()))
                .collect(),
        })
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

    println!("{}", input);
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

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn order() -> Result<()> {
        let input_file = test_utils::input_file_path("input/order.txt");
        let result = Input::try_from(input_file)?;
        assert_eq!(
            result,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "ABC".to_string(),
                queries: "ZYX".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn rule_order() -> Result<()> {
        let input_file = test_utils::input_file_path("input/rule_order.txt");
        let result = Input::try_from(input_file)?;
        assert_eq!(
            result,
            Input {
                rules: vec!["A=>Z".to_string(), "Z=>A".to_string()],
                facts: "ABC".to_string(),
                queries: "ZYX".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn empty_facts() -> Result<()> {
        let input_file = test_utils::input_file_path("input/empty_facts.txt");
        let result = Input::try_from(input_file)?;
        assert_eq!(
            result,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "".to_string(),
                queries: "ZYX".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn empty_queries() -> Result<()> {
        let input_file = test_utils::input_file_path("input/empty_queries.txt");
        let result = Input::try_from(input_file)?;
        assert_eq!(
            result,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "ABC".to_string(),
                queries: "".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn duplicate_facts() -> Result<()> {
        let input_file = test_utils::input_file_path("input/duplicate_facts.txt");
        let result = Input::try_from(input_file)?;
        assert_eq!(
            result,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "A".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn duplicate_queries() -> Result<()> {
        let input_file = test_utils::input_file_path("input/duplicate_queries.txt");
        let result = Input::try_from(input_file)?;
        assert_eq!(
            result,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "A".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn no_rules() -> Result<()> {
        let input_file = test_utils::input_file_path("input/no_rules.txt");
        let result = Input::try_from(input_file)?;
        assert_eq!(
            result,
            Input {
                rules: vec![],
                facts: "ABC".to_string(),
                queries: "ZYX".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn error_empty() {
        let input_file = test_utils::input_file_path("input/empty.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No facts in input file");
    }

    #[test]
    fn error_no_facts() {
        let input_file = test_utils::input_file_path("input/no_facts.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No facts in input file");
    }

    #[test]
    fn error_no_queries() {
        let input_file = test_utils::input_file_path("input/no_queries.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No queries in input file");
    }

    #[test]
    fn error_double_facts() {
        let input_file = test_utils::input_file_path("input/double_facts.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Multiple facts found in input file"
        );
    }

    #[test]
    fn error_double_queries() {
        let input_file = test_utils::input_file_path("input/double_queries.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Multiple queries found in input file"
        );
    }
}
