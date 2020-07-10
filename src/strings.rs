/*Primitive str = Immutable fixed-length
string somewhere in memeory*/
/*String = Growable, heap-allocated data
structure - Use when you need to modify
or own string data*/

pub fn run() {
    //Standard str
    let mut hello = "Hello";

    //Growable String
    let mut hello2 = String::from("Hello");

    //Get length of string
    println!("Length: {}", hello.len());

    //Pushing individual character
    //to string primitive doesn't work
    //hello.push("a");

    //Pushing individual character
    //to String
    hello2.push('a');

    println!("{:?}", (hello, hello2));
}
