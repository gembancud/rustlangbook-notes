use std::fs::File;
fn main() {
    let file = File::open("hello.txt");
    match file {
        Ok(file) => {
            println!("File opened successfully");
        }
        Err(error) => println!("{:?}", error),
    }
}
