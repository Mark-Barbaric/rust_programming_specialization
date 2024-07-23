mod number_functions;
mod string_functions;

fn main() {
    let numbers = vec![1, 2, 3];
    number_functions::process_numbers(&numbers);
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    number_functions::process_numbers(&nums);
    let string1 = "this, is, my, string".to_string();
    println!("Original string: {s}", s=string1);
    let split = string_functions::split_string(string1, ',', 1);
    println!("Split string: {s}", s=split);
    println!("Reading nums from user");
    let user_nums = number_functions::read_numbers_from_user();
    println!("Printing user numbers");
    number_functions::process_numbers(&user_nums);
}