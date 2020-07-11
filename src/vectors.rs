//Vectors - Resizable vectors

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    //Re-assign value
    numbers[2] = 20;

    //Add onto vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value
    numbers.pop();

    //Single value
    println!("Single value: {}", numbers[0]);

    //Get vector length
    println!("Vector length: {}", numbers.len());

    //Get amount of memeory allocated in stack
    println!("Vector mem usage: {}", mem::size_of_val(&numbers));

    //Get slice of vector, in range
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

    //Loop and mutate vector values
    for x in numbers.iter_mut() {
        *x *= 2
    }

    println!("{:?}", numbers);
}
