use std::collections::HashMap;
use std::io::{self, Write};
pub fn run() {
    let mut company_values = HashMap::new();
    print!("q - quit\na add name eg. a name branch\np - print all\n");
    loop {
        let mut input = String::new();
        print!("Enter command: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Input readline failed");
        let input: Vec<&str> = input.split_whitespace().collect();
        match input[0] {
            "q" => break,
            "a" => {
                let name = String::from(input[1]);
                let branch = input[2].parse::<String>().expect("Not a number");
                company_values.entry(branch).or_insert(vec![]).push(name);
            }
            "p" => {
                for (key, value) in &company_values {
                    println!("{}", key);
                    for name in value {
                        println!("\t{}", name);
                    }
                }
            }
            _ => println!("Invalid command"),
        }
    }
}
