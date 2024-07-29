fn own_vec(vector: &mut Vec<i32>) {
    vector.push(10);
}

fn own_integer(x: &mut i32) {
    *x += 1;
}

fn own_string(s: &String) {
    println!("{}", s);
}

fn main() {

    let mut my_int = 10;
    println!("my_int before: {}", my_int);
    own_integer(&mut my_int);
    println!("my_int after: {}", my_int);

    let mut my_vec = vec![1, 2, 3, 4, 5];
    println!("before {:?}", my_vec);
    own_vec(&mut my_vec);
    println!("after {:?}", my_vec); // this will not compile without borrowing in prior function call!

    let my_string = String::from("Hellow, World");
    own_string(&my_string);
    println!("{}", my_string); // this will not compile without borrowing in prior function call!
}
