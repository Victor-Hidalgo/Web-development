/*
Primitive Types--
Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take
in memory; u stands for unassigned, meaning no negative part)
Floats: f32, f64
Boolean (bool)
Characters (char)
Tuples
Arrays - fixed length
Rust is a statiscally typed language, which means that it must know the types of all
variables at compile time, however,  the compiler can usually infer what type we want
to use based on the value and how we use it.
*/

pub fn run(){

    //Default is "i32"
    let x = 1;
    //Default is "f64"
    let y = 2.5;
    //Explicitly
    let z: i64 = 454545;
    let n: i32;

    //Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //Boolean
    let is_active: bool = true;

    //Get from expression
    let is_greater: bool = 10 < 5;
    let a1 = 'a'; //Character
    let face = '\u{1F600}';

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));

    let mut num = Some(5);

    while let Some(i) = num{

        if i > 9{
            println!("The number contains more than one digit");
            //num = None;
        }
        else{
            println!("The number is: {:?}", i);
            num = Some(i + 1);
        }
    }
}