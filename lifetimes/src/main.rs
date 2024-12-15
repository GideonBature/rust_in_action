fn add_two<'a, 'b>(x: &'a i32, y: &'b i32) -> i32 {
    x + y
}


fn main() {
    let n1 = 15;
    let n2 = 10;

    println!("n1 + n2 = {}", add_two(&n1, &n2));
}
