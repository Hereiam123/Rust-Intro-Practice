use std::env;

pub fn run() {
    //First arg is target of executable
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    //println!("Command: {}", command);
    let name = "Billy";
    let status = "100%";

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command")
    }
}
