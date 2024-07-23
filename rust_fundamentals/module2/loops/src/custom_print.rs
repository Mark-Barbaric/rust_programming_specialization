use std::io;

pub fn print_nums(size: i32) {

    println!("Printing loop");

    for i in 0..size {
        println!("i = {i}", i=i);
    }

    println!("Printing reverse inclusive");
    for i in (0..=size).rev() {
        println!("i = {i}", i=i);
    }
}

pub fn print_word() {
    let mut input = String::new();

    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word to print. To stop execution, type 'stop': ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {s}", s=input);
    }

    println!("Goodbye.");
}