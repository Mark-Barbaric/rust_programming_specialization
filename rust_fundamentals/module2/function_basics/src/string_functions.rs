pub fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    result.expect("Ooops. Something went wront").to_string()
}