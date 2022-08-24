// does the fizzbuzz goodies with hashmap
fn fb(hmp: &HashMap<i32, &str>) {
    let mut v = vec![];
    for kv in hmp {
        v.push(kv.0);
    }
    v.sort_by(|a, b| b.cmp(a));

    'passby: for i in 0..100 {
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
