use crate::logic::is_identifier;
use crate::sanitize_lines::*;
use crate::utils::*;

use anyhow::{anyhow, Context, Result};
use std::{borrow::Borrow, fmt, path::PathBuf};

#[derive(PartialEq)]
pub struct Input {
    pub rules: Vec<String>,
    pub facts: String,
    pub queries: String,
}

impl fmt::Debug for Input {
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
        content.try_into()
    }
}

impl<T> TryFrom<Vec<T>> for Input
where
    T: Borrow<str>,
{
    type Error = anyhow::Error;

    fn try_from(lines: Vec<T>) -> Result<Self, Self::Error> {
        let mut lines = sanitize_lines(&lines);

        let mut rules: Vec<String> = vec![];
        let mut facts: Option<String> = None;
        let mut queries: Option<String> = None;
        for line in lines.iter_mut() {
            match line {
                l if l.starts_with('=') || l.starts_with('?') => match l.remove(0) {
                    '=' => match facts {
                        None => facts = Some(l.to_string()),
                        Some(_) => return Err(anyhow!("Multiple facts found in input file")),
                    },
                    '?' => match queries {
                        None => queries = Some(l.to_string()),
                        Some(_) => return Err(anyhow!("Multiple queries found in input file")),
                    },
                    _ => unreachable!(),
                },
                l if !l.is_empty() => rules.push(l.to_string()),
                _ => continue,
            }
        }

        let facts = facts.context("No facts in input file")?;
        if let Some(c) = facts.chars().find(|c| !is_identifier(c)) {
            return Err(anyhow!("Invalid identifier in facts: '{}'", c));
        }
        let queries = queries.context("No queries in input file")?;
        if let Some(c) = queries.chars().find(|c| !is_identifier(c)) {
            return Err(anyhow!("Invalid identifier in query: '{}'", c));
        }

        let mut facts_set = facts.chars().map(String::from).collect::<Vec<String>>();
        facts_set.dedup();

        let mut queries_set = queries.chars().map(String::from).collect::<Vec<String>>();
        queries_set.dedup();

        Ok(Input {
            rules,
            facts: facts_set.concat(),
            queries: queries_set.concat(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils;

    use anyhow::Result;
    use pretty_assertions::assert_eq;

    #[test]
    fn from_file() -> Result<()> {
        let input_file = test_utils::input_file_path("input/valid.txt");
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
    fn error_from_file_non_exist() {
        let input_file = test_utils::input_file_path("input/non_exist.txt");
        let result = Input::try_from(input_file);
        assert!(result.is_err());
    }

    #[test]
    fn spacing() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["A=>Z", "=A", "?Z"])?,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "A".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn order() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["?Z", "=A", "A=>Z"])?,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "A".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn rule_order() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["A=>Z", "=A", "Z=>A", "?Z"])?,
            Input {
                rules: vec!["A=>Z".to_string(), "Z=>A".to_string()],
                facts: "A".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn no_rules() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["=A", "?Z"])?,
            Input {
                rules: vec![],
                facts: "A".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn valid() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["A=>Z", "=A", "?Z"])?,
            Input {
                rules: vec!["A=>Z".to_string()],
                facts: "A".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn empty_facts() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["=", "?Z"])?,
            Input {
                rules: vec![],
                facts: "".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn empty_queries() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["=A", "?"])?,
            Input {
                rules: vec![],
                facts: "A".to_string(),
                queries: "".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn duplicate_facts() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["=AA", "?"])?,
            Input {
                rules: vec![],
                facts: "A".to_string(),
                queries: "".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn duplicate_queries() -> Result<()> {
        assert_eq!(
            Input::try_from(vec!["=", "?ZZ"])?,
            Input {
                rules: vec![],
                facts: "".to_string(),
                queries: "Z".to_string(),
            }
        );
        Ok(())
    }

    #[test]
    fn error_empty() {
        let result = Input::try_from(Vec::<String>::new());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No facts in input file");
    }

    #[test]
    fn error_no_facts() {
        let result = Input::try_from(vec!["?"]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No facts in input file");
    }

    #[test]
    fn error_no_queries() {
        let result = Input::try_from(vec!["="]);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "No queries in input file");
    }

    #[test]
    fn error_double_facts() {
        let result = Input::try_from(vec!["=", "=", "?"]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Multiple facts found in input file"
        );
    }

    #[test]
    fn error_double_queries() {
        let result = Input::try_from(vec!["=", "?", "?"]);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "Multiple queries found in input file"
        );
    }
}
