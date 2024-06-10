pub fn run(){

    let age: u8;
    age = 22;
    let check_id: bool = false;
    let knows_person_of_age: bool = true;

    //If/Else
    if age >= 21 && check_id || knows_person_of_age{
        println!("Bartender: What would you like to drink?");
    }
    else if age < 21 && check_id{
        println!("Bartender: Sorry, you have to leave");
    }

    else{
        println!("Bartender: I'll need to see your ID");
    }

    //Shorthand If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);

    add_numbers(5, 42);

    foo();

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }
    println!("{:?}", v);
}
fn print_number(num: i8){
    println!("The number is: {}", num);
}

fn add_numbers(num1: i8, num2: i8){
    print_number(num1 + num2);
}

fn foo(){
    let x = 2;
    let x = 3;
}
