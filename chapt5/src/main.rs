#[derive(Debug)]
struct User {
    name: String,
    email: String,
    age: u8,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // let user1 = User {
    //     name: String::from("Alex Mercer"),
    //     email: String::from("Alex@gmail.com"),
    //     age: 23,
    // };
    //
    // dbg!(user1);
    // println!("{:#?}", user1);

    let rect = Rectangle {
        width: 3,
        height: 4,
    };
    println!("{:#?}", rect);

    println!("a = {}", rect.area());
    println!("p = {}", rect.perimeter());

    let sq = Rectangle::square(3);
    dbg!(sq);
}
