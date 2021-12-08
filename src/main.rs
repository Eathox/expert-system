use anyhow::{Context, Result};
use expert_system::*;
use indoc::indoc;
use std::env;

pub const USAGE: &str = indoc! {"
    TODO: add usage

"};

fn main() {
    if env::args().len() != 2 {
        eprint!("{}", USAGE);
        std::process::exit(1)
    }

    let input_file = env::args().nth(1).unwrap();
    println!("{:?}", input_file);
}
