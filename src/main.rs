mod input;
mod logic;
mod sanitize_lines;
mod usage;
mod utils;

use input::Input;
use logic::RuleMap;

use anyhow::{Context, Result};
use std::{env, path::PathBuf};

fn handle_cli() -> String {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => args[1].clone(),
        _ => {
            eprint!("{}", usage::USAGE);
            std::process::exit(1);
        }
    }
}

fn main() -> Result<()> {
    let input_file = handle_cli();
    let input = Input::try_from(PathBuf::from(input_file)).context("Unable to read input file")?;

    println!("{:?}", input);
    let map = RuleMap::try_from(input.rules).context("Failed to parse rule")?;
    println!("{:?}", map);

    Ok(())
}
