fn loop_and_panic(numbers: Vec<i32>) {
    for num in numbers {
        if num < 0 {
            panic!("Negative number found");
        } else {
            println!("Number: {}", num);
        }
    }
}

fn main() {
    let nums = vec![1, 2, 3, 4, -5];
    loop_and_panic(nums);
}
