pub fn run(){

    let color = "red ";

    match color{

        "red" | "yellow"=> println!("Printing first case..."),
        "blue" => println!("Printing second case..."),
        "red" => println!("Printing third case..."),
        _ => println!("No color selected")
    }

    /*match number{

        5 | 10 => println!("Class A"),
        30 => println!("Class B"),
        21..=40 => println!("Class C"),
        _ => println!("Class not registered")
    }*/

    /*enum Coin{
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: Coin) -> u32{
        match coin{
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }*/

    //println!("Value of quarter is: {}", value_in_cents(Coin::Quarter));

    let dish = ("Eggs","Ham");

    if let ("Eggs", b) = dish {
        
        println!("Eggs are served with {}", b);
    }

    /*else{
        println!("No eggs will be served");
    }*/

    let x = Some(3);
    let a = if let Some(1) = x {
        1
    }
    else if x == Some(2){
        2
    }
    else if let Some(y) = x{
        y
    }
    else{
        -1
    };

    assert_eq!(a,3);

    //println!("This is {}", a);

    let car_color = "green";
 
    let car_message = if let "yellow" = car_color {
        concat!("yellow ","car")
        
    } else if let "purple" = car_color {
        concat!("purple ","car")
        
    } else if let "green" = car_color {
        concat!("green ","car")

    } else {
        concat!("unavailable ","car")
    };
    
    // printing the gfg_answer variable
    println!("{}",car_message);
}