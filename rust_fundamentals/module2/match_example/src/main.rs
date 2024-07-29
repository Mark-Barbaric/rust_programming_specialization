use std::io::BufRead;

fn main() {
    let file: Result<std::fs::File, std::io::Error> = std::fs::File::open("non_existent_file.txt");

    let file: std::fs::File = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    panic!("File not found: {}", error);
                }
                _ => {
                    panic!("Error opening file: {}", error);
                }
            }
        }
    };

    let reader: std::io::BufReader<std::fs::File> = std::io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}