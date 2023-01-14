fn main() {
    // Comparing string types
    let not_a_string = "some text"; // This is a "string slice" (or an &str type)
    let a_string: String = "some text".to_string(); // This is the equivalent of "string" in Rust (which is a String struct)
    println!("&str: {}", not_a_string);
    println!("String: {}", a_string);

    // Comparing numeric types
    let i8 = 1; // This is an integer of unknown type, the type is going to be 
                // whatever fits best into memory (in our case  it will be i8,
                // since 1 is a small number
    let unsigned_32_bit_integer: u32 = 1;
    let signed_32_bit_integer: i32 = 1;
    println!("i8: {}", i8);
    println!("u32: {}", unsigned_32_bit_integer);
    println!("i32: {}", signed_32_bit_integer);
}