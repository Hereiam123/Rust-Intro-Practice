/*Primitive str = Immutable fixed-length
string somewhere in memeory*/
/*String = Growable, heap-allocated data
structure - Use when you need to modify
or own string data*/

pub fn run() {
    //Standard str
    let hello = "Hello";

    //Growable String
    let mut hello2 = String::from("Hello ");

    //Get length of string
    println!("Length: {}", hello.len());

    //Pushing individual character
    //to string primitive doesn't work
    //hello.push("a");

    //Pushing individual character
    //to String
    hello2.push('W');

    let hello3 = "orld!";

    //Pushing string to String
    hello2.push_str(hello3);

    //Capacity of string in bytes
    println!("Capacity {}", hello2.capacity());

    //Check if empty
    println!("Is Empty {}", hello2.is_empty());

    //Contains substring
    println!("Contains 'World' {}", hello2.contains("World"));

    //Replace
    println!("Replace: {}", hello2.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello2.split_whitespace() {
        println!("{}", word);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //Assert string length
    assert_eq!(2, s.len());

    //Assert fail
    //Should throw compile error
    //assert_eq!(3, s.len());

    //Assert capacity
    assert_eq!(10, s.capacity());

    println!("{:?}", (hello, hello2, s));
}
