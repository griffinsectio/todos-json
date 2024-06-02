// TODO: Implement "init" subcommand that will initialize todos.json with the first username - Done
// TODO: Proper error handling (no expect or unwrap)

mod print_error;
use del::del;
use done::done;
use login::login;
mod get_username;
mod get_user_data;
use register::register;
mod get_task_number;
use get_task_number::get_task_number;
mod get_users_json;
use crate::get_users_json::get_users_json;
mod get_todo_file;
mod update_todo_file;
use update_todo_file::update_todo_file;

mod add;
use add::add;
mod list;
use list::list;
mod done;
mod del;
mod login;
mod register;

use core::panic;
use std::{collections::HashMap, io::Write};
// use clap::error::ErrorKind;
use clap::{arg, value_parser, ArgAction, Command};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    misc: HashMap<String, String>,
    hobby: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Users {
    users: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct User {
    username: String,
    todos: Vec<HashMap<String, String>>,
}

// fn user_data<'a>(username: &str, users_json: &'a mut Users) -> &'a mut User {

// fn user_data<'a>(username: String) -> (&'a Users, &'a mut User) {
//     let users_string = get_todo_file();
//     let users_json: Users = serde_json::from_str(users_string.as_str()).unwrap();
//     let users_vec: &'a mut Vec<User>  = users_json_again.users.as_mut();

//     for user in users_vec {
//         if user.username == username {
//             return (&users_json, user);
//         }
//     };
//     panic!("Username not found");
// }

fn main() {
    let command = Command::new("todo")
    .subcommand(
        Command::new("list")
        .about("List todos")
    )
    .subcommand(
        Command::new("add")
        .arg(
            arg!([TASK])
            .required(true)
            .action(ArgAction::Append)
            .value_parser(value_parser!(String))
            .num_args(1..)
        )
        .about("Add todos to the list, each todo enclosed within string")
    )
    .about("Todo app written in Rust")
    .subcommand(
        Command::new("del")
        .arg(
            arg!([TASK_NUMBER]...)
            .required(true)
            .action(ArgAction::Append)
            .value_parser(value_parser!(usize))
        )
        .about("Delete todos from list")
    )
    .subcommand(
        Command::new("done")
        .arg(
            arg!([TASK_NUMBER]...)
            .required(true)
            // .action(ArgAction::Append)
            .value_parser(value_parser!(usize))
        )
        .about("Set todos as done")
    )
    .subcommand(
        Command::new("login")
        .arg(
            arg!([USERNAME])
            .required(true)
            .num_args(1)
            .value_parser(value_parser!(String))
        )
        .about("Log in to existing account")
    )
    .subcommand(
        Command::new("register")
        .arg(
            arg!([USERNAME])
            .required(true)
            .num_args(1)
            .value_parser(value_parser!(String))
        )
        .about("Register a new account")
    )
    .subcommand(
        Command::new("init")
        .about("Initialize todo file (or replace the if already exist)")
    )
    .about("Todo app written in Rust")
    .get_matches();

    match command.subcommand() {
        Some(("add", sub_matches)) => {
            let mut users_json = get_users_json();
            add(&mut users_json, sub_matches);

            let new_todo_content = serde_json::to_string(&users_json).unwrap();
            update_todo_file(new_todo_content);
        }
        Some(("list", _)) => {
            let mut users_json = get_users_json();
            list(&mut users_json)
        }
        Some(("done", sub_matches)) => {
            let mut users_json = get_users_json();
            done(&mut users_json, sub_matches);

            let new_todo_content = serde_json::to_string(&users_json).unwrap();
            update_todo_file(new_todo_content);
        }
        Some(("del", sub_matches)) => {
            let mut users_json = get_users_json();
            del(&mut users_json, sub_matches);
            
            // INTERESTING: Try to flip the position
            // println!("{:#?}", user);
            // println!("{:#?}", users_json);

            let new_todo_content = serde_json::to_string(&users_json).unwrap();
            update_todo_file(new_todo_content);
        }
        Some(("login", sub_matches)) => {
            let mut users_json = get_users_json();
            login(&mut users_json, sub_matches);
        }
        Some(("register", sub_matches)) => {
            let mut users_json = get_users_json();
            register(&mut users_json, sub_matches);

            let new_todo_content = serde_json::to_string(&users_json).unwrap();
            update_todo_file(new_todo_content);
        }
        Some(("init", _)) => {
            let path = "./todos.json";
            let mut file_handle = match std::fs::File::create(path) {
                Ok(v) => v,
                Err(err) => {
                    use crate::print_error::print_error;
                    print_error(err);
                    unreachable!();
                }
            };

            println!("Please enter your username: ");
            let mut username = String::new();
            std::io::stdin().read_line(&mut username).unwrap();
            let todos_json_string = format!("{{\"users\":[{{\"username\": {:?},\"todos\": []}}]}}", username.trim());
            println!("{}", todos_json_string);
            file_handle.write(todos_json_string.as_bytes()).unwrap();

            // let new_todo_content = serde_json::to_string(&users_json).unwrap();
            // update_todo_file(new_todo_content);
        }
        _ => unreachable!("This arm shouldn't be reached")
    }
}