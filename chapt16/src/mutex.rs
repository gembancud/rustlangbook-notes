use std::sync::{Arc, Mutex};
use std::thread;

pub fn run() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

///
/// This is a more complex example of using a mutex to share data between threads.
/// This uses an Atomic RC or Arc to allow multiple ownership across threads.
/// unless you are working with threads, you probably don't need to use Arc,
/// since it incurs a performance penalty.
pub fn run2() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
