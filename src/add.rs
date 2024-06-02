use clap::ArgMatches;
use std::collections::HashMap;
use crate::get_username::get_username;
use crate::get_user_data::get_user_data;
use crate::print_error::print_error;
use crate::Users;

pub fn add(users_json: &mut Users, sub_matches: &ArgMatches) {
    let username = get_username();
    let user = match get_user_data(&username, users_json) {
        Ok(v) => v,
        Err(err) => {
            print_error(err);
            unreachable!();
        },
    };

    if sub_matches.get_many::<String>("TASK").unwrap().len() > 0 {
        let values = sub_matches.get_many::<String>("TASK").unwrap_or_default().map(|v| v.to_string()).collect::<Vec<_>>();

        for v in values {
            let mut new_todo = HashMap::new();
            new_todo.insert("text".to_string(), v);
            new_todo.insert("done".to_string(), "false".to_string());

            user.todos.push(new_todo);
        }
    } else {
        panic!("Please provide a task to add!");
    }
}