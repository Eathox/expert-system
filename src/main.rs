extern crate expert_system;
use expert_system::*;

use anyhow::{Context, Result};
use std::env;

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
    let content: Vec<String> =
        read_file(&input_file).context(format!("failed to read input file: '{}'", input_file))?;
    let lines = sanitize::sanitize_lines(&content);
    println!("{:?}", lines);
    Ok(())
}
