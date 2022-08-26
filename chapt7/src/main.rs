mod living;
mod science;

fn main() {
    let animal = science::create_animal("Dog");
    let animal2 = science::Animal::new("Cat");

    println!("{}", animal.name);
    println!("{}", animal2.name);
}
