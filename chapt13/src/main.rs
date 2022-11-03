// use std::thread;
//
// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);
//
//     thread::spawn(move || println!("From thread: {:?}", list))
//         .join()
//         .unwrap();
// }
//
fn main() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);
    {
        let mut borrows_mutably = || list.push(7);
        borrows_mutably();
    }

    println!("After calling closure: {:?}", list);
}
