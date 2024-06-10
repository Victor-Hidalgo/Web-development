use std::fs::File;
use std::io::ErrorKind;

pub fn run(){

    /*let f = File::open("test.jpg");
    match f{
        Ok(f)=>{
            println!("file found {:?}", f);
        },
        Err(e)=>{
            println!("Not found \n{:?}", e);
        }
    }*/
    println!("End of main");

    let sol = is_prime(3);
    
    /*match sol{
        Ok(d) =>{
            println!("{}, it is prime", d);
        },
        Err(msg) =>{
            println!("Error: {}", msg);
        }
    }*/

    //let g = File::open("hello.txt").expect("Failed to open hello.txt");

    //let f = File::open("hello.txt");

    /*let b = match f{

        Ok(file) => file,

        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };*/

    let i = 20;

    match foo(i) {

        Ok(b) => b,

        Err(error) => match is_even(i){
            Ok(c) => c,
            Err(e) => panic!("{:?} The number is an odd positive", e),
        }

        second_error => {
            panic!("Could not determined the type of {:?}", second_error);
        }
    };

    let mut n: i32 = 3;
while n < 10{
        println!("{}", n);
        n -= 1;
}
}

fn foo(n: i32) -> Result<bool, String>{
    
    if n < 0{
        return Ok(true);
    }
    else{
        return Err("NOT_A_NEGATIVE".to_string());
    }
}

fn is_even(n: i32) -> Result<bool, String>{

    if n % 2 == 0{
        return Ok(true);
    }
    else{
        return Err("NOT_EVEN".to_string());
    }
}

fn is_prime(num: u32) -> Result<bool, String>{

    let mut flag :bool = false;

    if num > 1{
        
        for i in 2..num{

            if num % i == 0{
                flag = true;
                break;
            }
        }
    }

    if flag == false{
        return Ok(true);
    }

    else{
        return Err("NOT_A_PRIME".to_string());
    }

}
