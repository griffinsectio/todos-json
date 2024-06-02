use std::io::Error;

pub fn print_error(err: Error) {
    println!("An error occurred: {}", err);
    std::process::exit(1);
}