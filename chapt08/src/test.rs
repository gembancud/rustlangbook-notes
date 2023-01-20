use std::collections::HashMap;

fn test() {
    let mut hmp = HashMap::new();

    hmp.insert("blue", 10);
    hmp.insert("blue", 25);
    dbg!(&hmp);
    // There are two ways in modifying entries. One is to use the `entry` method to get a mutable reference to the value in the hash map.
    // The other is to use and_modify to use a lambda function
    let e = hmp.entry("blue").or_insert(5);
    *e = 200;
    hmp.entry("red").or_insert(5);
    dbg!(&hmp);
    hmp.entry("red").and_modify(|v| {
        *v += 10;
    });
    dbg!(&hmp);
}
