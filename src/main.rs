use expert_system::*;
use std::env;

fn main() {
    if env::args().len() != 2 {
        eprint!("{}", USAGE);
        std::process::exit(1)
    }

    let input_file = env::args().nth(1).unwrap();
    println!("{:?}", input_file);
}
