use std::thread;
use std::time::Duration;

pub fn run(){

    let v = vec![5, 10, 15, 20];

    let handle = thread::spawn( move || {
        println!("Vector: {:?}", v);
    });

    let arr = ['a', 'b', 'c', 'd', 'e'];

    for index in 0..arr.len(){
        println!("{}", arr[index]);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}