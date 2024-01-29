extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use std::io::stdin;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use std::env;
use dotenv::dotenv;
use chrono::NaiveDate;

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
        println!("Enter class_id for studentclass entry: ");
        let mut class_id = String::new();
        stdin().read_line(&mut class_id).unwrap();
        let class_id = &class_id[..(class_id.len() - 1)];
        let class_id = class_id.parse::<i32>().unwrap();
        println!("Enter student_id for studentclass entry: ");
        let mut student_id = String::new();
        stdin().read_line(&mut student_id).unwrap();
        let student_id = &student_id[..(student_id.len() - 1)];
        let student_id = student_id.parse::<i32>().unwrap();
        let certification_num = None;
        let test_score = None;
        println!("\nEnter class start date (%m/%d/%Y): ");
        let mut class_start = String::new();
        stdin().read_line(&mut class_start).unwrap();
        let class_start = &class_start[..(class_start.len() - 1)];
        let class_start = NaiveDate::parse_from_str(&class_start, "%m/%d/%Y").unwrap();
        println!("\nEnter class end date (%m/%d/%Y): ");
        let mut class_end = String::new();
        stdin().read_line(&mut class_end).unwrap();
        let class_end = &class_end[..(class_end.len() - 1)];
        let class_end = NaiveDate::parse_from_str(&class_end, "%m/%d/%Y").unwrap();
        println!("\nEnter md date (%m/%d/%Y): ");
        let mut md_date = String::new();
        stdin().read_line(&mut md_date).unwrap();
        let md_date = &md_date[..(md_date.len() - 1)];
        let md_date = NaiveDate::parse_from_str(&md_date, "%m/%d/%Y").unwrap();
        println!("\nEnter va date (%m/%d/%Y): ");
        let mut va_date = String::new();
        stdin().read_line(&mut va_date).unwrap();
        let va_date = &va_date[..(va_date.len() - 1)];
        let va_date = NaiveDate::parse_from_str(&va_date, "%m/%d/%Y").unwrap();
        println!("\nEnter dc date (%m/%d/%Y): ");
        let mut dc_date = String::new();
        stdin().read_line(&mut dc_date).unwrap();
        let dc_date = &dc_date[..(dc_date.len() - 1)];
        let dc_date = NaiveDate::parse_from_str(&dc_date, "%m/%d/%Y").unwrap();

        let _studentclass_entry = create_studentclass_entry(&connection, 
                                 &class_id, 
                                 &student_id, 
                                 &certification_num, 
                                 &test_score, 
                                 &class_start,
                                 &class_end,
                                 &md_date,
                                 &va_date,
                                 &dc_date);
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

