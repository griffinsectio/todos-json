use colored::Colorize;
use crate::get_username::get_username;
use crate::get_user_data::get_user_data;
use crate::print_error::print_error;
use crate::Users;

pub fn list(users_json: &mut Users) {
    let username = get_username();
    let user = match get_user_data(&username, users_json) {
        Ok(v) => v,
        Err(err) => {
            print_error(err);
            unreachable!();
        },
    };

    if user.todos.len() == 0 {
        println!("You've got no todos, use the command {} to add one", "\"add\"".green().bold());
        println!("Examples:\n{}", "todo add \"Buy groceries\"".blue().bold());
        println!("{}", "todo add \"Buy some snacks\" \"Study for exam\"".blue().bold());
    } else {
        for (index, todo) in user.todos.iter().enumerate() {
            if todo.get("done").unwrap() == "true" {
                println!("{}. {}", index + 1, todo.get("text").unwrap().blue().bold().strikethrough());
            } else {
                println!("{}. {}", index + 1, todo.get("text").unwrap());
            }
        }
    }
}