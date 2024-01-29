extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
//use std::io::stdin;
//use std::str::FromStr;
//use std::any::*;
//use std::env;
use std::fs;
//use chrono::NaiveDate;

fn main() {
    let connection = establish_connection();

    let file = fs::read_to_string("/home/vulpwn/docs/dakotadb_csv/companies.tsv").unwrap();
    let lines = file.split("\n");

    let mut i = 0;
    for student in lines {
        if i < 1 {
            i += 1;
            continue;
        }
        let cut = student.split("\x09");
        let mut fields: [&str; 7] = Default::default();
        let mut j = 0;
        for slice in cut {
            fields[j] = &slice;
            j += 1;
        }
        
        let mut suite: Option<&str> = None;
        if fields[3].eq("") == false {
            suite = Some(fields[3]);
        }

        let mut old_id: Option<i32> = None;
        if fields[0].eq("") == false {
            old_id = Some(fields[0].to_string()
                              .parse::<i32>().unwrap());
        }
        
        /*
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
        */

        /*
        for x in fields {
            println!("{}", x.to_string());
        }
        */

        let phone = "";
        let email = "";
        
        let _company = create_company(&connection,
                                      fields[1],
                                      fields[2],
                                      suite,
                                      fields[4],
                                      fields[5],
                                      fields[6],
                                      &phone,
                                      None,
                                      &email,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      None,
                                      old_id.as_ref());
    
        println!("Student {} added successfully!", 
                 fields[1].to_string());
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

