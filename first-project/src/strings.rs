/*Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - Use when you need to modify or
own string data
*/
pub fn run(){

    //Growable type definition
    let mut hello = String::from("Hello ");
    
    println!("Length: {}", hello.len());
    
    //Push char
    hello.push('W');

    //Push string
    hello.push_str("orld!");

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    println!("Is Empty: {}", hello.is_empty());
    println!("Contains 'World': {}", hello.contains("World"));
    println!("Replace: {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    /*for word in hello.split_whitespace(){
        println!("{}", word);
    }*/

    let mut s = String::from("Hello, ");

    s.push('W');
    s.push_str("orld!");
    s.replace("World", "There");
    println!("End: {}", s);

    //Assertion testing
    /*assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());*/

    let s1 = String::from("Apple, ");
    let s2 = String::from("Banana");
    let s3 = s1 + &s2;
    
    //println!("Last s: {}", s1);
    //println!("{}", s1);
}