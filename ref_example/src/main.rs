fn main() {
    let a = 42;
    let r = &a;
    let b = a + *r;

    println!("a + a: {}, b: {}", a + a, b);
    assert_eq!(a + a, b);
}
