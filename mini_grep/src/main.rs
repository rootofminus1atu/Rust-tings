use std::{env, process};

use mini_grep::{Config, run};

mod tests;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    } 
}



