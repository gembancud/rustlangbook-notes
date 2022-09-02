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
