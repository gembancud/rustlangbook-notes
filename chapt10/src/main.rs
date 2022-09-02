mod point;

fn main() {
    // let v = vec![1, 2, 3];
    // let v_largest = largest(&v);
    // println!("The largest number is {}", v_largest);

    let pi32 = point::Point::new(1, 2);
    println!("The int point is {:?}", pi32);

    let int_sum = pi32.sum();
    println!("The sum is {}", int_sum);

    let pf64 = point::Point::new(1.0, 2.0);
    println!("The floating point is {:?}", pf64);

    let float_sum = pf64.sum();
    println!("The sum is {}", float_sum);
}

// fn largest<T>(v: &[T]) -> T {
//     let mut largest = v[0];
//     for item in v {
//         if item > &largest {
//             largest = *item;
//         }
//     }
//     largest
// }
