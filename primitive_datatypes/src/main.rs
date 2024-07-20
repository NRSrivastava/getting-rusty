//Primitive datatypes
//int, float, bool, char


//Integer
//Rust has two types of integers: signed and unsigned. 
//Signed integers are represented using two's complement representation.
//Unsigned integers are represented using the binary representation.
//Unsigned integers are always positive or zero, while signed integers can be positive, negative, or zero.
//i8, i16, i32, i64, i128, isize: Signed integers
//u8, u16, u32, u64, u128, usize: Unsigned integers
fn integer(){
    let x: i32 = -42;
    let y: u64 = 100;

    //let y: u64 = -100; 
    //error[E0600]: cannot apply unary operator `-` to type `u64` 
    //note: unsigned values cannot be negated

    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    //difference between i32 and i64
    //range
    // i32: -2^31 to 2^31 - 1 : -2147483648 to 2147483647
    // i64: -2^63 to 2^63 - 1 : -9223372036854775808 to 9223372036854775807
    let a: i32 = -2147483648;
    let b: i64 = 9223372036854775807;

    println!("i32: {}", a);
    println!("i64: {}", b);

    //let a: i32 = -2147483649;
    //error: literal out of range for `i32`
    //note: the literal `-2147483649` does not fit into the type `i32` whose range is `-2147483648..=2147483647`

    //let b: i64 = 9223372036854775808;
    //error: literal out of range for `i64`
    //note: the literal `9223372036854775808` does not fit into the type `i64` whose range is `-9223372036854775808..=9223372036854775807`

}


//Floats [Floating-point numbers]
// f32, f64
fn floats(){

    let pi: f64 = 3.141592653589793238462643383279502884197169399375105820974944592307816406286;
    println!("Value of f64 pi: {}", pi);
    //Value of f64 pi: 3.141592653589793
    //Output (limited significant decimal digits due to range)

    //difference between f32 and f64
    // f32: 32-bit floating-point number
    // f32: 6-9 significant decimal digits
    // example: 3.14159265
    // f64: 64-bit floating-point number
    // f64: 15-17 significant decimal digits
    // example: 3.141592653589793
}


fn booleans(){
    let is_rust_fun: bool = true;
    let is_rust_boring: bool = false;

    println!("Is Rust fun? {}", is_rust_fun);
    println!("Is Rust boring? {}", is_rust_boring);
}


fn chars(){
    let c: char = 'A';
    println!("Character: {}", c);
    //Character: A

    //let c: char = 'AB';
    //error: character literal may only contain one codepoint
    //note: 'AB' is two codepoints
}


fn main(){
    integer();
    floats();
    booleans();
    chars();
}