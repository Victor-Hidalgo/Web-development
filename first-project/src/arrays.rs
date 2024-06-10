//Arrays - Fixed list where elements are the same data types

pub fn run(){

    use std::mem;

    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    //Re-assign value
    numbers[2] = 20;

    //All values
    println!("{:?}", numbers);

    //Single val
    println!("Single Value: {}", numbers[0]);
    println!("Array length: {}", numbers.len());

    //Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}