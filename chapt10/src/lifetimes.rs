pub fn run() {
    let a = String::from("Hello");
    let b = "World";

    let c = longest(&a, b);
    println!("Longest: {}", c);
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
