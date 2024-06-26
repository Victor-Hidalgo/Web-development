//Vectors - Resizable arrays

pub fn run(){

    use std::mem;

    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    //Re-assign value
    numbers[2] = 20;

    //Add on to vector
    numbers.push(5);
    numbers.push(6);

    //Pop off last value
    numbers.pop();

    //All values
    println!("{:?}", numbers);

    //Single val
    println!("Single Value: {}", numbers[0]);
    println!("Vector length: {}", numbers.len());

    //Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    //Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    //Loop through vector values
    for x in numbers.iter(){
        println!("Number: {}", x);
    }

    //Loop & mutate values
    for x in numbers.iter_mut(){
        *x *= 2;
    }

    /*println!("Numbers Vector: {:?}", numbers);
    let s1 = "Goodbye";
    let sub1 = &s1[0..4];

    let fullname = " Rust Tutorial \r\n";
    println!("Before trim ");
    println!("length is {}",fullname.len());
    println!("After trim ");
    println!("length is {}",fullname.trim().len());*/

    use std::collections::HashMap;
    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 10);
    scores.insert(String::from("Blue"), 50);
    scores.entry(String::from("Yellow"));

    let team_name = String::from("Blue");
    let team_name2 = String::from("Yellow");
    let score = if scores.get(&team_name) > scores.get(&team_name){

        team_name;
    } else{
        team_name2;
    };
    
    
    println!("Value: {:?}", score);
}