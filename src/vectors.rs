
use std::mem;
pub fn run() {

    let mut  numbers: Vec<i32> = vec![1,2,3,4];
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // re-assign value
    // get single val
    println!(" Single value : {}", numbers[0]);

    // get vector length
    println!("Vector length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice 
    let slice: &[i32] = &numbers[1..3];
     println!("Slice: {:?}", slice);
     
    for x in numbers.iter(){
        println!("Number: {}", x);
    }
    for x in numbers.iter_mut(){
        *x *= 2;
    }
    println!("Numbers Vec: {:?}", numbers);
}