use std::{env, process};

use mini_grep::{Config, run};

mod tests;

fn main() {
    
    let config = Config::from_args(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    } 
}



