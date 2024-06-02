use clap::ArgMatches;
use std::collections::HashMap;
use crate::get_username::get_username;
use crate::get_user_data::get_user_data;
use crate::print_error::print_error;
use crate::Users;
use crate::get_task_number::get_task_number;

pub fn del(users_json: &mut Users, sub_matches: &ArgMatches) {
    let username = get_username();
    let user = match get_user_data(&username, users_json) {
        Ok(v) => v,
        Err(err) => {
            print_error(err);
            unreachable!();
        },
    };

    let task_number_vec = get_task_number(sub_matches);

    let mut new_todos_vec_buf: Vec<HashMap<String, String>> = vec![];
    for (index, todo) in user.todos.iter().enumerate() {
        if !task_number_vec.contains(&index) {
            new_todos_vec_buf.push(todo.to_owned());
        };
    }
    user.todos = new_todos_vec_buf;
}