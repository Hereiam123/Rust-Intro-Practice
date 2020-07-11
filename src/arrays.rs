//Arrays - Fixed list where elements
//are the same data types

pub fn run() {
    //Not correct length
    //let numbers: [i32; 5] = [1, 2, 3, 4];

    //Not correct type
    //let numbers: [i32; 5] = [1, 2, 3, 4, ""];

    //[type; length]
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("Single value: {}", numbers[0]);

    println!("{:?}", numbers);
}
