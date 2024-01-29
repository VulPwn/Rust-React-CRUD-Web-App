extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use std::io::stdin;
use argon2::{self, Config};
use rand::{distributions::Alphanumeric, Rng};
use crate::Pool;
use actix_web::web;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;

pub fn establish_pg_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let connection = establish_pg_connection();

    while true {
        println!("Enter email for user: ");
        let mut email = String::new();
        stdin().read_line(&mut email).unwrap();
        let email = &email[..(email.len() - 1)];
        println!("Enter password for user: ");
        let mut password = String::new();
        stdin().read_line(&mut password).unwrap();
        let password = &password[..(password.len() - 1)];


        let _user = create_user(&connection, 
                                 email, 
                                 password);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

