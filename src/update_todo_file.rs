use std::fs::File;
use crate::print_error::print_error;
use std::io::Write;

pub fn update_todo_file(new_todo_lines: String) {
    let path = "./todos.json";
    
    let mut file_handle: File = match File::create(path) {
        Ok(v) => v,
        Err(err) => {
            print_error(err);
            return;
        }
    };

    match file_handle.write(new_todo_lines.as_bytes()) {
        Ok(_) => (),
        Err(err) => print_error(err),
    };
}