use std::io;

pub fn process_numbers(numbers: &[i32]) {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    println!("The sum of the numbers is {s}", s=sum);

    if sum % 2 == 0 {
        println!("The sum is even");
    } else {
        println!("The sum is odd");
    }
}

pub fn read_numbers_from_user() -> Vec<i32> {
    let mut nums: Vec<i32> = Vec::new();

    println!("Please enter the size of the vector: ");
    let mut size_input = String::new();
    io::stdin()
        .read_line(&mut size_input)
        .expect("Cannot read value");
    println!("User input for size: {s}", s=size_input);
    let vec_size: i32 = size_input.trim().parse().expect("Input not an integer");

    for i in 0..vec_size {
        println!("Please enter the {i}th value: ", i=i);
        let mut value_string = String::new();
        io::stdin().read_line(&mut value_string).expect("Cannot read value");
        let cur_value: i32 = value_string.trim().parse().expect("Input is not an integer");
        nums.push(cur_value);
    }
    nums
}