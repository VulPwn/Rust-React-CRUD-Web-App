extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use std::fs;

fn main() {
    let connection = establish_connection();

    let file = fs::read_to_string("/home/vulpwn/docs/dakotadb_csv/classes.tsv").unwrap();
    let lines = file.split("\n");

    let mut i = 0;
    for class in lines {
        if i < 1 {
            i += 1;
            continue;
        }
        let cut = class.split("\x09");
        let mut fields: [&str; 8] = Default::default();
        let mut j = 0;
        for slice in cut {
            fields[j] = &slice;
            j += 1;
        }
        
        let mut md_approval_num: Option<&str> = None;
        if fields[2].eq("") == false {
            md_approval_num = Some(fields[2]);
        }
        let mut va_approval_num: Option<&str> = None;
        if fields[3].eq("") == false {
            va_approval_num = Some(fields[3]);
        }
        let mut dc_approval_num: Option<&str> = None;
        if fields[4].eq("") == false {
            dc_approval_num = Some(fields[4]);
        }

        let mut md_recert_yrs: Option<i32> = None;
        if fields[5].eq("") == false {
            md_recert_yrs = Some(fields[5].to_string()
                              .parse::<i32>().unwrap());
        }
        let mut va_recert_yrs: Option<i32> = None;
        if fields[6].eq("") == false {
            va_recert_yrs = Some(fields[6].to_string()
                              .parse::<i32>().unwrap());
        }
        let mut dc_recert_yrs: Option<i32> = None;
        if fields[7].eq("") == false {
            let mut temp = fields[7].to_string();
            let pop = temp.pop().unwrap();
            println!("{}", pop as u32);
            if pop.eq(&'\u{000D}') == false {
                dc_recert_yrs = Some(pop.to_string()
                                     .parse::<i32>().unwrap());
            } else {
                dc_recert_yrs = Some(temp.parse::<i32>().unwrap());
            }
        }

        let _class = create_class(&connection,
                                      fields[0],
                                      fields[1],
                                      md_approval_num,
                                      va_approval_num,
                                      dc_approval_num,
                                      md_recert_yrs.as_ref(),
                                      va_recert_yrs.as_ref(),
                                      dc_recert_yrs.as_ref());
    
        println!("Class {} added successfully!", 
                 fields[0].to_string());
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

