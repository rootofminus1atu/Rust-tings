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
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}


pub struct Config {
    query: String,
    file_path: String,
    case_sensitive: bool,
}
impl Config {
    pub fn new(query: &str, file_path: &str) -> Config {
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            case_sensitive,
        }
    }


    pub fn from_args_slice<'a>(args: &'a [String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        let query = &args[1];
        let file_path = &args[2];

        Ok(Config::new(query, file_path))
    }

    pub fn from_args(args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut args = args.skip(1); // Skip the program name.

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config::new(&query, &file_path))
    }
}