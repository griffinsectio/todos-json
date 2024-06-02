use clap::ArgMatches;
use crate::{User, Users};

pub fn register(users_json: &mut Users, sub_matches: &ArgMatches) {
    let username = match sub_matches.get_one::<String>("USERNAME") {
        Some(v) => v,
        None => {
            println!("Can't parse the username you gave!");
            std::process::exit(1);
        }
    };

    let new_user = User {
        username: username.to_owned(),
        todos: Vec::new(),
    };
    users_json.users.push(new_user);
}