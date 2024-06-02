use clap::ArgMatches;
use crate::get_username::get_username;
use crate::get_user_data::get_user_data;
use crate::print_error::print_error;
use crate::Users;
use crate::get_task_number;

pub fn done(users_json: &mut Users, sub_matches: &ArgMatches) {
    let username = get_username();
    let user = match get_user_data(&username, users_json) {
        Ok(v) => v,
        Err(err) => {
            print_error(err);
            unreachable!();
        },
    };

    let task_number_vec = get_task_number(sub_matches);
    for task_number in task_number_vec {
        let todo_ref_mut = &mut user.todos[task_number];
        *todo_ref_mut.get_mut("done").unwrap() = "true".to_string();
    }
}