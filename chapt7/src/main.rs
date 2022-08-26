mod living;
mod science;

fn main() {
    let animal = science::create_animal("Dog");
    let animal2 = science::Animal::new("Cat");

    dbg!(animal);
    println!("{}", animal2.name);
}
