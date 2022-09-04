mod mytrait;
mod point;

use mytrait::{Summary, Tweet};
fn main() {
    // point::run();
    mytrait::run();

    let tweet = mytrait::Tweet::new(String::from("main tweet"));
    println!("{}", tweet.summarize());
}
