use std::io;

fn main() {
    println!("Hello, this is cargo!!");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Your input was wrong");
    let input: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };
    oten(input);
    oten(23);
}

fn oten(i: i32) {
    println!("Hello");
    println!("logging {i}")
}
