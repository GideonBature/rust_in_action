use std::ops::{Add};

fn add<T: Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let ans = add(-14.5, 7.8);
    println!("add: {}", ans);
}
