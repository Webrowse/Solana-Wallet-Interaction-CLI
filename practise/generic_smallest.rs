//Simple approch

// fn smallest<T: Ord + Copy>(a: T, b: T, c: T) -> T {
//     if a>b {
//         if a<c {a} else {c}
//     } else {
//         if b<c {b} else {c}
//     }
// }

// fn main() {
//     println!("{}", smallest(10, 20, 5));   // Expected: 5
//     println!("{}", smallest('x', 'm', 'a')); // Expected: 'a'
// }


// Using Rust's built-in std::cmp::min function
fn smallest<T: Ord + Copy>(a: T, b: T, c: T) -> T {
    std::cmp::min(a, std::cmp::min(b,c))
}


fn main() {
    println!("{:?}", smallest(10, 20, 5));   // Expected: 5
    println!("{:?}", smallest('x', 'm', 'a')); // Expected: 'a'
}