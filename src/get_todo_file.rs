use std::fs::File;
use std::io::Error;
use colored::Colorize;
use std::io::Read;

pub fn get_todo_file() -> Result<String, Error>  {
    let path = "./todos.json";
    let mut json_string = String::new();
    let mut file_handle = match File::open(path) {
        Ok(v) => v,
        Err(_) => {
            let error_string = format!("Can't open todo file, if it's your first time to use this app please run: {}", "todo init".blue().bold());
            return Err(Error::new(std::io::ErrorKind::NotFound, error_string));
            // print_error(err);
        }
    };
    file_handle.read_to_string(&mut json_string).unwrap();

    return Ok(json_string);
    // json_string
}