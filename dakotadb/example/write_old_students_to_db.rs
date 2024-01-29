extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
//use std::io::stdin;
//use std::str::FromStr;
//use std::any::*;
//use std::env;
use std::fs;
use chrono::NaiveDate;

fn main() {
    let connection = establish_connection();

    let file = fs::read_to_string("/home/vulpwn/docs/dakotadb_csv/students.tsv").unwrap();
    let lines = file.split("\n");

    let mut i = 0;
    for student in lines {
        if i < 1 {
            i += 1;
            continue;
        }
        let cut = student.split("\x09");
        let mut fields: [&str; 13] = Default::default();
        let mut j = 0;
        for slice in cut {
            fields[j] = &slice;
            j += 1;
        }
        
        let mut dob = NaiveDate::from_ymd(2022, 03, 12);
        if fields[9].eq("") == false {
            let mut date = fields[9].to_string();
            let mut mdy: [&str; 3] = Default::default();
            j = 0;
            for f in date.split("/") {
                mdy[j] = &f;
                j += 1;
            }
            dob = NaiveDate::from_ymd(mdy[2].to_string()
                                      .parse::<i32>().unwrap(),
                                      mdy[0].to_string()
                                      .parse::<u32>().unwrap(),
                                      mdy[1].to_string()
                                      .parse::<u32>().unwrap());
        }

        let mut suite: Option<&str> = None;
        if fields[4].eq("") == false {
            suite = Some(fields[4]);
        }

        let mut company_id: Option<i32> = None;
        if fields[10].eq("") == false {
            company_id = Some(fields[10].to_string()
                              .parse::<i32>().unwrap());
        }

        let mut email: Option<&str> = None;
        if fields[11].eq("") == false {
            email = Some(fields[11]);
        }
        
        let mut old_id: Option<i32> = None;
        if fields[12].eq("") == false {
            let mut temp = fields[12].to_string();
            temp.pop();
            old_id = Some(temp.parse::<i32>().unwrap());
        }

        /*
        for x in fields {
            println!("{}", x.to_string());
        }
        */
        
        let _student = create_student(&connection,
                                      fields[0],
                                      fields[1],
                                      fields[2],
                                      fields[3],
                                      suite,
                                      fields[5],
                                      fields[6],
                                      fields[7],
                                      fields[8],
                                      &dob,
                                      company_id.as_ref(),
                                      email,
                                      None,
                                      old_id.as_ref());
    
        println!("Student {} {} added successfully!", 
                 fields[1].to_string(), fields[2].to_string());
    }

    /*
    let _student = create_student(&connection, 
                                 social, 
                                 first_name,
                                 last_name,
                                 address,
                                 Some(suite),
                                 city,
                                 state,
                                 zip_code,
                                 phone,
                                 &dob,
                                 Some(&company_id),
                                 Some(email),
                                 None);
    */
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

