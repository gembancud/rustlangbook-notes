use std::env;
use std::process;
mod lib;

use lib::Config;

fn main() {
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {} in {}", config.query, config.file_path);
    if let Err(err) = lib::run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
