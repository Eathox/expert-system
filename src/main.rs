extern crate expert_system;
use expert_system::*;

use anyhow::Result;
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
    let input = Input::try_from(PathBuf::from(input_file))
        .map_err(|e| e.context("Unable to read input file"))?;

    println!("{:?}", input);
    let map = RuleMap::try_from(input.rules).map_err(|e| e.context("Failed to parse rule map"))?;
    println!("{:?}", map);

    Ok(())
}
