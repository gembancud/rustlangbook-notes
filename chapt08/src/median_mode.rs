use std::collections::HashMap;

fn run() {
    let i = vec![1, 2, 3, 4, 5, 6, 8, 7, 8, 9, 10];
    let median = median(&i);
    let mode = mode(&i);
    println!("median: {}, mode {}", median, mode);
}

fn median(i: &[i32]) -> i32 {
    let mut i = i.to_vec();
    i.sort();
    i[i.len() / 2]
}

fn mode(i: &Vec<i32>) -> i32 {
    let mut mp = HashMap::new();
    for num in i {
        let count = mp.entry(num).or_insert(0);
        *count += 1;
    }
    let mut maxk: i32 = std::i32::MIN;
    let mut maxv: i32 = std::i32::MIN;
    for (k, v) in mp {
        if v > maxv {
            maxk = *k;
            maxv = v;
        }
    }
    maxk
}
