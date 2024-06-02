use clap::ArgMatches;
use crate::print_error::print_error;
use crate::Users;
use std::fs::File;
use std::io::{Write, Error, ErrorKind};

pub fn login(users_json: &mut Users, sub_matches: &ArgMatches) {
    let username = match sub_matches.get_one::<String>("USERNAME") {
        Some(v) => v,
        None => {
            println!("Can't parse the username you gave!");
            std::process::exit(1);
        }
    };

    for user in &users_json.users {
        if user.username == *username {
            let path = "./username";
            let mut file_handle = File::create(path).unwrap();
            match file_handle.write(username.as_bytes()) {
                Ok(_) => (),
                Err(err) => print_error(err),
            };
            return;
        }
    }
    print_error(Error::new(ErrorKind::NotFound, "Username not found!"));
}