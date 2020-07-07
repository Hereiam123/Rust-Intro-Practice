//Variable hold primitive data or references to data
//Variables are immutable by default
//Rust is a block-scoped language
pub fn run() {
    let name = "Beau";

    //mut - Makes variable mutable
    let mut age = 30;
    println!("My name is {} and I am {}", name, age);

    age = 31;
    println!("My name is {} and I am {}", name, age);

    //Define constant
    //i32 - 32 bit interger type
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables
    let (my_name, my_age) = ("Beau", 31);
    println!("{} is {}", my_name, my_age);
}
