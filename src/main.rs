mod structs;

use anyhow::{Context, Result};
use expert_system::*;
use std::env;
use structs::parser::*;

fn main() -> Result<()> {
    if env::args().len() != 2 {
        eprint!("{}", USAGE);
        std::process::exit(1)
    }

    // let input_file = env::args().nth(1).unwrap();
    // println!("{:?}", input_file);

    let mut parser = Parser::new();
	let mut tokens = parser
		.tokenize("(!)+   ^  Aa   |")
		.context(format!("Failed while lexing"))?;
    parser.parse(&mut tokens).context(format!("Unable to parse"))?;


    Ok(())
}
