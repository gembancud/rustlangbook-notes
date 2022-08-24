use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Welcome to my guessing game");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess: ");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("I failed to read your guess");

        let input: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number, you fucking idiot");
                continue;
            }
        };

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
