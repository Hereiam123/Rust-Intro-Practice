//pub - Public function
pub fn run() {
    //Print to console
    println!("Hello, from the print.rs file");
    //String literal
    println!("Number: {} and {}", 1, 2);

    //Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Beau", "Florida", "code"
    );

    //Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "Beau",
        activity = "Baseball"
    );
}
