extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use std::io::stdin;

fn main() {
    let connection = establish_connection();
    
    while true {
        println!("Enter acronym for new instructor: ");
        let mut acronym = String::new();
        stdin().read_line(&mut acronym).unwrap();
        let acronym = &acronym[..(acronym.len() - 1)];
        println!("\nEnter instructor's first name: ");
        let mut first_name = String::new();
        stdin().read_line(&mut first_name).unwrap();
        let first_name = &first_name[..(first_name.len() - 1)];
        println!("\nEnter instructor's last name: ");
        let mut last_name = String::new();
        stdin().read_line(&mut last_name).unwrap();
        let last_name = &last_name[..(last_name.len() - 1)];

        let _instructor = create_instructor(&connection, 
                                 acronym, 
                                 first_name,
                                 last_name);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

