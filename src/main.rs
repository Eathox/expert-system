extern crate expert_system;
use expert_system::*;
use input::Input;
use parser::TruthTable;
use permutation_iter::PermutationIter;

use anyhow::{Context, Result};
use std::{env, path::PathBuf};

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

    println!("{:?}", input);
    for rule in input.rules {
        let table = TruthTable::try_from(PermutationIter::new(&rule))
            .context(format!("Failed to parse rule {}", rule))?;
        println!("{}\n{:?}", rule, table);
    }

    Ok(())
}
