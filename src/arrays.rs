//Arrays - Fixed list where elements
//are the same data types

pub fn run() {
    //Not correct length
    //let numbers: [i32; 5] = [1, 2, 3, 4];

    //Not correct type
    //let numbers: [i32; 5] = [1, 2, 3, 4, ""];

    //[type; length]
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    //Re-assign value
    numbers[2] = 20;

    //Single value
    println!("Single value: {}", numbers[0]);

    //Get array length
    println!("Array length: {}", numbers.len());

    //Get amount of memeory allocated in stack
    println!("Array mem usage: {}", std::mem::size_of_val(&numbers));

    println!("{:?}", numbers);
}
