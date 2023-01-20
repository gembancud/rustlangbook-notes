use std::io;
fn main() {
    println!("Hello, world!");
    loop {
        println!("Enter word: ");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read_line");
        let res = first_word(&input[..]);
        println!("{}", res);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    s
}
