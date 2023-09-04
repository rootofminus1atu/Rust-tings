use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Searching for `{}` in `{}`", config.query, config.file_path);

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    match results.len() {
        0 => println!("No matches found"),
        num => {
            match num {
                1 => println!("Found {} match", num),
                _ => println!("Found {} matches", num),
            }

            for line in results {
                println!("{}", line);
            }
        }
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


pub struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    case_sensitive: bool,
}

impl Config<'_> {
    pub fn new<'a>(query: &'a str, file_path: &'a str) -> Config<'a> {
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        Config { query, file_path, case_sensitive }
    }

    pub fn from_args<'a>(args: &'a [String]) -> Result<Config<'a>, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config::new(query, file_path))
    }
}