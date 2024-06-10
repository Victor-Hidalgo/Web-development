pub fn run(){

    let mut number: i32 = 0;
    let mut count = 0;

    while number < 21 {

        if number % 2 != 0{
            break;
        }
        count += 1;
        number += 1;
    }

    println!("Total odds: {}", count);

    /*let mut num = Some(0);

    while let Some(i) = num{

        if i > 9{
            println!("The number contains more than one digit");
            num = None;
        }
        else{
            println!("The number is: {:?}", i);
            num = Some(i + 1);
        }
    }*/

    let names = vec!["Bob","Ferris", "Frank"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("Hi {}", name),
            _ => println!("Hello {}", name),
        }
    }

    let mut s = foo(3, 0);

    if s != 0{
        println!("Calculating result of the operation...");
    }

    else{
        println!("The result could not be determined");
    }
    
    
}

fn foo(x: i32, y: i32) -> i32{

    if y == 0{
        panic!("error detected");
    }
    else{
        x/y
    }
}

fn foolo(n: u32, base: u32) -> u32 {

    let mut power = base;
    let mut count = 1;

    while n >= power {
        count += 1;

        if let Some(new_power) = power.checked_mul(base) {
            power = new_power;

        } else {
            break;
        }
    }
    count
}