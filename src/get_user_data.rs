use std::io::{Error, ErrorKind};
use colored::Colorize;

use crate::{Users, User};

pub fn get_user_data<'a>(username: &str, users_json: &'a mut Users) -> Result<&'a mut User, Error> {
    let users_vec: &mut Vec<User> = users_json.users.as_mut();
    let user: Result<&mut User, Error>; 

    for user_i in users_vec {
        if user_i.username == username {
            // return user;
            user = Ok(user_i);
            return user;
        }
    };
    let error_message = format!("Username not found! Please make sure your username is correct\nAnd relogin using the command {}", "todo login USERNAME".green().bold());
    user = Err(Error::new(ErrorKind::NotFound, error_message));
    return user;
}