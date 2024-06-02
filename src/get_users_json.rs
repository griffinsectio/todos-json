use crate::Users;
use crate::get_todo_file::get_todo_file;

pub fn get_users_json() -> Users {
    let users_string = get_todo_file().unwrap();
    let users_json: Users = serde_json::from_str(users_string.as_str()).unwrap();
    return users_json
}