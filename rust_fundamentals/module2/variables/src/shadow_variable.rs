pub fn print_age(age: i32) {
    let result = if age >= 18 {
        "Adult"
    } else {
        "Child"
    };

    println!("Print age as category: {:?}", result);
}

pub fn health_shadowing(height: i32) {
    let health = if height >= 180 {"Healthy"} else {"Unhealthy"};
    println!("My health based on height is: {health}", health=health);
    let health = if height >= 180 {true} else {false};
    println!("My health as a boolean is: {health}");
}