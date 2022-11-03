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
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

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
