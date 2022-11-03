use std::env;
use std::error::Error;

use std::fs;
//derive debug
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query: String = args[1].clone();
        let file_path: String = args[2].clone();

        let case_sensitive: bool = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            file_path,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn two_results() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.duct";

        assert_eq!(
            vec!["safe, fast, productive.", "Pick three.duct"],
            search(query, contents)
        );
    }

    #[test]
    fn case_sensitive() {
        let query = "R";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.duct";

        assert_eq!(vec!["Rust:"], search(query, contents));
    }
    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.duct
trust me";

        assert_eq!(
            vec!["Rust:", "trust me"],
            search_case_insensitive(query, contents)
        );
    }
}
