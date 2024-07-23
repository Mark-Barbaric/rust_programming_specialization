use std::io;

pub fn print_match() {
    println!("Please enter a greeting: ");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    
    match name.trim() {
        "Good Bye" => println!("Sorry to see you go."),
        "hello" => println!("Its good to meet you"),
        _ => println!("I can't find a greeting, goodbye")
    }
}


pub fn print_loops(size: i32) {

    println!("Printing loop");
    for i in 0..size {
        println!("i = {i}", i=i);
    }

    println!("Printing reverse inclusive");
    for i in (0..=size).rev() {
        println!("i = {i}", i=i);
    }

    println!("Printing from vector");
    let numbers = vec![1, 2, 3, 4, 5];

    for n in numbers {
        println!("{}", n);
    }
}

pub fn print_while() {
    let mut input = String::new();

    while input.trim() != "stop" {
        input.clear();
        println!("Please enter a word to print. To stop execution, type 'stop': ");
        io::stdin().read_line(&mut input).expect("Failed to read input");
        println!("You entered: {s}", s=input);
    }

    println!("Goodbye.");
}