extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use self::models::*;
use self::diesel::prelude::*;
use std::fs;

fn main() {

    use dakotadb::schema::companies::dsl::*;
    

    let connection = establish_connection();

    let file = fs::read_to_string("/home/vulpwn/docs/dakotadb_csv/company_poc.tsv").unwrap();
    let lines = file.split("\n");

    let mut i = 0;
    for student in lines {
        if i < 1 {
            i += 1;
            continue;
        }
        if i == 3298 {
            break;
        }
        i += 1;
        let cut = student.split("\x09");
        let mut fields: [&str; 11] = Default::default();
        let mut j = 0;
        for slice in cut {
            fields[j] = &slice;
            j += 1;
        }
        
        if fields[0].eq("") == true {
            continue;
        }

        let company_id: i32 = fields[0].to_string()
                                .parse::<i32>().unwrap();

        let results = companies.filter(old_id.eq(&company_id))
            .limit(1)
            .load::<Company>(&connection);
        
        let company = match results {
            Ok(item) => item,
            Err(_error) => {
                println!("Error loading the company");
                continue;
            },
        };
        
        for c in company {
            
            /*
            let mut c_suite: Option<&str> = None;
            if fields[2].eq("") == false {
               c_suite = Some(fields[3]);
            }
            let mut c_poc_f: Option<&str> = None;
            if fields[6].eq("") == false {
                c_poc_f = Some(fields[6]);
            }
            let mut c_poc_l: Option<&str> = None;
            if fields[7].eq("") == false {
                c_poc_l = Some(fields[7]);
            }
            let mut c_phone_ext: Option<&str> = None;
            if fields[9].eq("") == false {
                c_phone_ext = Some(fields[9]);
            }
            
            let target = companies.filter(old_id.eq(&company_id));
            let _set_address = diesel::update(target)
                .set(address.eq(fields[1])).execute(&connection);
            let _set_suite = diesel::update(target)
                .set(suite.eq(c_suite)).execute(&connection);
            let _set_city = diesel::update(target)
                .set(city.eq(fields[3])).execute(&connection);
            let _set_state = diesel::update(target)
                .set(state.eq(fields[4])).execute(&connection);
            let _set_zip_code = diesel::update(target)
                .set(zip_code.eq(fields[5])).execute(&connection);
            let _set_poc_firstname = diesel::update(target)
                .set(poc_firstname.eq(c_poc_f)).execute(&connection);
            let _set_poc_lastname = diesel::update(target)
                .set(poc_lastname.eq(c_poc_l)).execute(&connection);
            let _set_phone = diesel::update(target)
                .set(phone.eq(fields[8])).execute(&connection);
            let _set_phone_ext = diesel::update(target)
                .set(phone_ext.eq(c_phone_ext)).execute(&connection);
            let _set_email = diesel::update(target)
                .set(email.eq(fields[10])).execute(&connection);
            
            println!("id:{}, old_id:{}, name:{} UPDATED w/ {}!", 
                        c.id, c.old_id.unwrap(), c.company_name, company_id);
            */
            let temp_poc_first = match c.poc_firstname {
                Some(s) => s,
                None => "".to_string(),
            };
            let temp_poc_last = match c.poc_lastname {
                Some(s) => s,
                None => "".to_string(),
            };
            println!("Name:{}, Old_ID:{}, Address:{}, POC:{} {}",
                     c.company_name, c.old_id.unwrap(), c.address, temp_poc_first, temp_poc_last);
            /*
            let pull_again = companies.filter(id.eq(c.id))
                                .limit(1)
                                .load::<Company>(&connection);
            let check = match pull_again {
                Ok(item) => item,
                Err(_error) => {
                    println!("Error loading the company again");
                    continue;
                },
            };
            for ch in check {
                println!("Address:{}", ch.address);
            };
            */
        };
    }
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

