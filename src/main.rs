mod parser;

use anyhow::{Context, Result};
use expert_system::*;
use std::env;
use parser::RuleParser;

fn main() -> Result<()> {
    if env::args().len() != 2 {
        eprint!("{}", USAGE);
        std::process::exit(1)
    }

    let input_file = env::args().nth(1).unwrap();
    println!("{:?}", input_file);

    let mut _parser = RuleParser::new();

    let temp = _parser.parse("A => B");

    Ok(())
}
