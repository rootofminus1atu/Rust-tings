use std::env;
use std::fs;

pub fn display() {
    let args: Vec<String> = env::args().collect();

    let config = Config::from_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        std::process::exit(1);
    } 
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for {} in {}", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let lines: Vec<&str> = contents.split("\n").filter(|line| line.contains(config.query)).collect();

    match lines.len() {
        0 => println!("No matches found"),
        _ => {
            println!("Found {} matches:", lines.len());
            for line in lines {
                println!("{}", line);
            }
        }
    }

    Ok(())
}


struct Config<'a> {
    query: &'a str,
    file_path: &'a str
}

impl Config<'_> {
    fn new<'a>(query: &'a str, file_path: &'a str) -> Config<'a> {
        Config { query, file_path }
    }

    fn from_args<'a>(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config::new(query, file_path))
    }
}

fn parse_args(args: &[String]) -> Result<Config, &'static str> {
    let query: &String = args.get(1).ok_or("No query string")?;
    let file_path: &String = args.get(2).ok_or("No file path")?;

    Ok(Config::new(query, file_path))
}