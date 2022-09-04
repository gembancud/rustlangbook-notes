#[derive(Debug)]
pub struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> &T {
        &self.x
    }
}

impl Point<i32> {
    pub fn sum(&self) -> i32 {
        self.x + self.y
    }
}

impl Point<f64> {
    pub fn sum(&self) -> f64 {
        self.x + self.y
    }
}

pub fn run() {
    let pi32 = Point::new(1, 2);
    println!("The int point is {:?}", pi32);

    let int_sum = pi32.sum();
    println!("The sum is {}", int_sum);

    let pf64 = Point::new(1.0, 2.0);
    println!("The floating point is {:?}", pf64);

    let float_sum = pf64.sum();
    println!("The sum is {}", float_sum);
}
