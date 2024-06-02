use std::fs::File;
use std::io::Read;
use colored::Colorize;

pub fn get_username() -> String {
    let path = "./username";
    let mut username = String::new();

    let mut file_handle = match File::open(path) {
        Ok(v) => v,
        Err(_) => {
            println!("Please first log in with your username\nusing the command {}", "\"login\"".green().bold());
            std::process::exit(1);
        },
    };

    file_handle.read_to_string(&mut username).unwrap();
    
    username = username.trim().to_string();
    username
}