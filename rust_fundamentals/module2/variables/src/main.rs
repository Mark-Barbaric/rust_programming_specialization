mod shadow_variable;

fn main () {
    let name = "Alfredo";
    let weight = 190.0;
    let _kilos = weight / 2.2;
    let mut _height = 190;
    _height = 210;
    println!("Name: {name}, Weight: {weight}, Height: {height}", name=name, weight=weight, height=_height);
    println!("Print Age \n\n");
    shadow_variable::print_age(19);
    shadow_variable::print_age(12);
    println!("Health Shadowing \n\n");
    shadow_variable::health_shadowing(190);
    shadow_variable::health_shadowing(160);
}