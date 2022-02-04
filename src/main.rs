use anyhow::{Context, Result};
use expert_system::*;
use indoc::indoc;
use std::env;

mod sanitize;

pub const USAGE: &str = indoc! {"
    TODO: add usage

"};

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
    let content: Vec<String> = read_file(input_file).context("Failed to read file")?;
    let lines = sanitize::sanitize_lines(&content);
    println!("{:?}", lines);
    Ok(())
}
