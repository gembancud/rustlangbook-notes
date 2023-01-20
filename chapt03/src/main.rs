use std::collections::HashMap;
use std::io;
fn main() {
    // do_fb();
    let mut fib_dp: HashMap<u32, u32> = HashMap::new();
    loop {
        println!("Enter a number to get the fibonacci number");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Input error");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
        let res = fib(input, &mut fib_dp);
        println!("the {}th number is {}", input, res);
    }
}

fn fib(n: u32, fib_dp: &mut HashMap<u32, u32>) -> u32 {
    if n == 0 || n == 1 {
        return n;
    }
    let rec_res = fib(n - 1, fib_dp) + fib(n - 2, fib_dp);
    // [&n] = fib(n - 1, fib_dp) + fib(n - 2, fib_dp);
    fib_dp.insert(n, rec_res);
    let out = fib_dp.get(&n).unwrap();
    *out
}

/// Does a fizz buzzy on a map inside the function
fn do_fb() {
    let mut hmp: HashMap<i32, &str> = HashMap::new();
    hmp.insert(3, "fizz");
    hmp.insert(5, "buzz");
    hmp.insert(15, "fizzbuzz");
    hmp.insert(30, "jizzguzz");

    fb(&hmp);
}

// does the fizzbuzz goodies with hashmap
fn fb(hmp: &HashMap<i32, &str>) {
    let mut v = vec![];
    for kv in hmp {
        v.push(kv.0);
    }
    v.sort_by(|a, b| b.cmp(a));

    for i in 0..100 {
        let mut flag = false;
        for j in &v {
            if i % *j == 0 {
                println!("{}", hmp[j]);
                flag = true;
                break;
            }
        }
        if !flag {
            println!("{}", i);
        }
    }
}
