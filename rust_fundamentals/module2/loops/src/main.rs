mod custom_print;

fn main() {
    custom_print::print_loops(8);
    custom_print::print_match();
    let mut _maybe_number : Option<Option<()>> = Some(None);
    // let maybe_number = Some(42);
    if let Some(number) = _maybe_number {
        println!("The number is {:?}", number);
    } else {
        println!("There is no number");
    }
    custom_print::print_while();
}