extern crate expert_system;
use expert_system::*;

use anyhow::{anyhow, Context, Result};
use parser::*;
use std::{env, path::PathBuf};

pub struct Input {
    rules: Vec<String>,
    facts: Vec<String>,
    queries: Vec<String>,
}

impl TryFrom<PathBuf> for Input {
    type Error = anyhow::Error;

    fn try_from(file_path: PathBuf) -> Result<Self, Self::Error> {
        let content: Vec<String> = read_file(&file_path)
            .context(format!("failed to read input file: '{:?}'", file_path))?;
        let lines = sanitize::sanitize_lines(&content);

        let mut sections = lines.split(|s| s.is_empty());
        let rules = sections
            .next()
            .context(format!("missing rules in: {:?}", file_path))?;
        let facts = sections
            .next()
            .context(format!("missing facts in: {:?}", file_path))?;
        let queries = sections
            .next()
            .context(format!("missing queries in: {:?}", file_path))?;
        if sections.next().is_some() {
            return Err(anyhow!("too many sections in input"));
        }

        Ok(Input {
            rules: rules.to_vec(),
            facts: facts.to_vec(),
            queries: queries.to_vec(),
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

    for rule in input.rules {
        let table = TruthTable::from(PermutationIter::new(&rule));
        println!("{}\n{}", rule, table);
    }

    Ok(())
}
