fn deconstruct_f32(value: f32) -> (u32, i32, u32) {
    let bits = value.to_bits();
    let sign = (bits >> 31) & 1;
    let exponent = ((bits >> 23) & 0xff) as i32 - 127;
    let mantissa = bits & 0x7fffff;
    (sign, exponent, mantissa)
}

fn main() {
    let value: f32 = 42.42;
    let (sign, exponent, mantissa) = deconstruct_f32(value);

    println!("Sign: {}", sign);
    println!("Exponent: {}", exponent);
    println!("Mantissa: {:x}", mantissa);
}
