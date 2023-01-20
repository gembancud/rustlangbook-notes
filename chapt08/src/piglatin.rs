use std::io;

pub fn run() {
    let mut input = String::new();
    let vowels = vec!['a', 'e', 'i', 'o', 'u'];

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    let pig_input = to_pig(input, &vowels);
    println!("{}", pig_input);
}

fn to_pig(s: &str, vowels: &[char]) -> String {
    let first = s.chars().next().unwrap();
    if vowels.contains(&first) {
        format!("{}-hay", s)
    } else {
        format!("{}-{}ay", &s[1..], &first)
    }
}
