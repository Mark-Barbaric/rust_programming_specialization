fn borrow_vec(vector: &mut Vec<i32>) {
    vector.push(10);
}

fn borrow_integer(x: &mut i32) {
    *x += 1;
}

fn borrow_string(s: &mut String) {
    s.push('!');
    println!("{}", s);
}

fn own_string(s: String) {
    println!("{}", s);
}

fn own_vec(mut vector: Vec<i32>) {
    vector.push(10);
    println!("{:?}", vector);
}

fn main() {

    let mut my_int = 10;
    println!("my_int before: {}", my_int);
    borrow_integer(&mut my_int);
    println!("my_int after: {}", my_int);

    let mut my_vec = vec![1, 2, 3, 4, 5];
    println!("before {:?}", my_vec);
    borrow_vec(&mut my_vec);
    println!("after {:?}", my_vec); 
    own_vec(my_vec);
    //println!("own_vec: {:?}", my_vec); // this will not compile without borrowing in prior function call!

    let mut my_string = String::from("Hellow, World");
    borrow_string(&mut my_string);
    println!("borrow_string: {}", my_string); // this will not compile without borrowing in prior function call!
    own_string(my_string);
    //println!("own_string: {}", my_string); // this will not compile without borrowing in prior function call!
}
