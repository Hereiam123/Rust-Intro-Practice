/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memeory)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays
*/

//Rust is statically typed, which means
//That it must know the types of all
//variables at compile time, however, the compiler
//can usually infer what type is wanted

pub fn run() {
    //Default is "i32"
    let x = 1;

    //Default is "f64"
    let y = 2.5;

    //Add explicit type
    let z: i64 = 4545454545454545;

    //Find max size
    println!("Max i32 {}", std::i32::MAX);
    println!("Max i32 {}", std::i64::MAX);

    //Boolean
    //Could set its type
    let is_true: bool = true;

    //Get boolean from expression
    let is_true_expression = 10 > 2;

    //Char type
    let a1 = 'a';

    //Unicode character
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_true, is_true_expression, a1, face))
}
