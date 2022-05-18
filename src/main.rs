extern crate expert_system;
use expert_system::*;

use anyhow::{anyhow, Context, Result};
use core::fmt;
use std::hash::Hasher;
use parser::*;
use std::{borrow::Borrow, collections::HashSet, env, path::PathBuf};

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
        Self::try_from(content)
    }
}

impl<T> TryFrom<Vec<T>> for Input
where
    T: Borrow<str>,
{
    type Error = anyhow::Error;

    fn try_from(lines: Vec<T>) -> Result<Self, Self::Error> {
        let mut lines = sanitize::sanitize_lines(&lines);

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

    // let rulemap = input.rules.try_from().context("Failed to build Rulemap from input rules")?;

    for rule in input.rules {
        let table = TruthTable::try_from(PermutationIter::new(&rule))
            .context(format!("Failed to parse rule {}", rule))?;
        println!("{}\n{}", rule, table);
    }

    Ok(())
}

#[derive(Debug)]
struct Executor {
    rulemap: RuleMap,
    facts: HashSet<char>,
    queries: Vec<char>
}

#[macro_use] extern crate lazy_static;//
use std::sync::Mutex;//

lazy_static! {//
    static ref CHECKED: Mutex<HashSet<u64>> = Mutex::new(HashSet::new());//
}//

use std::collections::hash_map::DefaultHasher;//
use std::hash::Hash;//

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Proposition {
    Contingency(bool),
    Undetermined,
}

// impl From<Proposition> for bool {
//     fn from(item: Proposition) -> Self {
//         match item {
//             Proposition::Contradiction => false,
//             Proposition::Contingency(true) => true,
//             Proposition::Contingency(false) => false,
//             Proposition::Undetermined => false,//
//         }
//     }
// }

impl Executor {

    fn solve_query(&self, query: char) -> Proposition {
        if self.facts.contains(&query) {
            println!("fact: {}", query);
            return Proposition::Contingency(true);
        }
        println!("No fact: {}", query);

        let mut results = vec![];
        // Iterate through the variable's rule span
        for rule in self.rulemap.get_span_ref(query).unwrap().iter() {
            let mut reduced = rule.as_ref().clone();
            let mut s = DefaultHasher::new();
            (**rule).hash(&mut s);
            let h = s.finish();
            if !CHECKED.lock().unwrap().contains(&h) {
                CHECKED.lock().unwrap().insert(h);
                for var in rule.variables.iter() {
                    if *var != query {
                        println!("before\n{}", reduced);
                        reduced = match self.solve_query(*var) {
                            Proposition::Contingency(b) => reduced.get_reduced(*var, b).unwrap(),
                            _ => reduced,
                        };
                        println!("after\n{}", reduced);
                        match reduced.results.as_slice() {
                            // [false, false] => Proposition::Contradiction,
                            [false, true] => results.push(Proposition::Contingency(true)),
                            [true, false] => results.push(Proposition::Contingency(false)),
                            // [true, true] => Proposition::Tautology,
                            [_, _] => results.push(Proposition::Undetermined),
                            _ => ()
                        }
                    }
                }
            }
        }
        println!("results {}\n{:?}", query, results);

        if results.is_empty() {
            // If it is not possible to prove query as fact by inference, assume it to be false
            Proposition::Contingency(false)
        } else if results.iter().all(|&item| item == Proposition::Contingency(true)) {
            Proposition::Contingency(true)
        } else if results.iter().all(|&item| item == Proposition::Contingency(false)) {
            Proposition::Contingency(false)
        } else {
            Proposition::Undetermined
        }
    }

    pub fn solve(&mut self) -> Vec<Proposition> {
        self.queries.iter().map(|query| self.solve_query(*query)).collect()
    }

}

impl TryFrom<Input> for Executor {
    type Error = anyhow::Error;

    fn try_from(input: Input) -> Result<Self, Self::Error> {
        let rulemap = RuleMap::try_from(input.rules).context("Failed to build executor")?;
        Ok(Executor {
            rulemap,
            facts: input.facts.chars().collect(),
            queries: input.queries.chars().collect()
        })
    }
}

#[cfg(test)]
mod executor {
    use super::*;

    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use super::Proposition::*;

    fn get_input(rules: Vec<&str>, facts: &str, queries: &str) -> Input {
        Input {
            rules: rules.into_iter().map(|rule| rule.into()).collect(),
            facts: facts.into(),
            queries: queries.into(),
        }
    }

    #[test]
    fn test_unidirectional() -> Result<()> {
        let input = get_input(vec![
            "A => B", // 0 => ?
            "C => D", // 1 => ?
            "E => F", // ? => 0
            "G => H", // ? => 1
            ], "CH", "BDEG");
        let mut executor = Executor::try_from(input)?;
        assert_eq!(executor.solve(), vec![
            Undetermined, // 0 => ?
            Contingency(true), // 1 => ?
            Contingency(false), // ? => 0
            Undetermined // ? => 1
        ]);

        Ok(())
    }

    #[test]
    fn test_bidirectional() -> Result<()> {
        let input = get_input(vec![
            "A <=> B", // 0 <=> ?
            "C <=> D", // 1 <=> ?
            "E <=> F", // ? <=> 0
            "G <=> H", // ? <=> 1
            ], "CH", "BDEG");
        let mut executor = Executor::try_from(input)?;
        assert_eq!(executor.solve(), vec![
            Contingency(false), // 0 <=> ?
            Contingency(true), // 1 <=> ?
            Contingency(false), // ? <=> 0
            Contingency(true) // ? <=> 1
        ]);

        Ok(())
    }

    #[test]
    fn test_linear_chain3() -> Result<()> {
        // chain of 3 variables
        let input = get_input(vec![
            "A => B",
            "B => C", // 1 => B => ?
            "D => E",
            "E => F", // 0 => E => ?
            ], "A", "CF");
        let mut executor = Executor::try_from(input)?;
        assert_eq!(executor.solve(), vec![
            Contingency(true),
            Contingency(false),
        ]);

        Ok(())
    }

    #[test]
    fn test_linear_chain4() -> Result<()> {
        // chain of 4 variables
        let input = get_input(vec![
            "A => B",
            "B => C",
            "C => D", // 1 => B => C => ?
            "E => F",
            "F => G",
            "G => H", // 0 => F => G => ?
            ], "A", "DH");
        let mut executor = Executor::try_from(input)?;
        assert_eq!(executor.solve(), vec![
            Contingency(true),
            Undetermined,
        ]);

        Ok(())
    }

    #[test]
    fn test_parallel() -> Result<()> {
        // chain of 3 variables
        let input = get_input(vec![
            "A => B",
            "C => D",
            "B + D => E",
            ], "AC", "E");
        let mut executor = Executor::try_from(input)?;
        assert_eq!(executor.solve(), vec![
            Contingency(true),
        ]);

        Ok(())
    }

    #[test]
    fn test_parallel_with_noise() -> Result<()> {
        // chain of 3 variables
        let input = get_input(vec![
            "!A => B",
            "A => B",
            "B => C",
            ], "A", "C");
        let mut executor = Executor::try_from(input)?;
        assert_eq!(executor.solve(), vec![
            Undetermined,
        ]);

        Ok(())
    }
























    #[test]
    fn executor() -> Result<()> {
        let input = Input {
            rules: vec![
                "A + Z => B".into(),
                "B => C".into(),
                "C <=> D + E".into(),
                "E | F => G".into(),
                "G => H".into(),
            ],
            facts: "AZ".into(),
            queries: "H".into(),
        };
        let mut executor = Executor::try_from(input)?;
        // println!("executor: {:#?}", executor.rulemap);
        executor.solve();

        Ok(())
    }

    #[test]
    fn executor2() -> Result<()> {
        let input = Input {
            rules: vec![
                "P => Q".into(),
                "L + M => P".into(),
                "B + L => M".into(),
                "A + P => L".into(),
                "A + B => L".into(),
            ],
            facts: "AB".into(),
            queries: "Q".into(),
        };
        let mut executor = Executor::try_from(input)?;
        // println!("executor: {:#?}", executor);
        executor.solve();

        Ok(())
    }

    #[test]
    fn executor3() -> Result<()> {
        let input = Input {
            rules: vec![
                "A => B".into(),
                "B => C".into(),
                "D => E".into(),
                "E + F => G".into(),
                "C + G => H".into(),
            ],
            facts: "ADF".into(),
            queries: "H".into(),
        };
        let mut executor = Executor::try_from(input)?;
        // println!("executor: {:#?}", executor);
        executor.solve();

        Ok(())
    }

    #[test]
    fn executor4() -> Result<()> {
        let input = Input {
            rules: vec![
                "A => B".into(),
                "B => C".into(),
                "C | D => E".into(),
            ],
            facts: "D".into(),
            queries: "E".into(),
        };
        let mut executor = Executor::try_from(input)?;
        // println!("executor: {:#?}", executor);
        executor.solve();

        Ok(())
    }

    #[test]
    fn executor5() -> Result<()> {
        let input = Input {
            rules: vec![
                "!A => B".into(),
                "A => C".into(),
                "B + C => D".into(),
            ],
            facts: "A".into(),
            queries: "D".into(),
        };
        let mut executor = Executor::try_from(input)?;
        // println!("executor: {:#?}", executor);
        executor.solve();

        Ok(())
    }
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
