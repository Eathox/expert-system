use anyhow::{Context, Result};
use expert_system::*;
use indoc::indoc;
use std::env;

mod sanitize;

mod errors {
    pub const INPUT_FILE_READ: &str = "failed to read input file";
}

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
    let content: Vec<String> = read_file(input_file).context(errors::INPUT_FILE_READ)?;
    let lines = sanitize::sanitize_lines(&content);
    println!("{:?}", lines);
    Ok(())
}
