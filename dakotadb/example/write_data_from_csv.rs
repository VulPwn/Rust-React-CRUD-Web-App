extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use self::models::*;
use self::diesel::prelude::*;
use std::fs;

fn main() {

    use dakotadb::schema::companies::dsl::*;
    

    let connection = establish_connection();

    let file = fs::read_to_string("/home/vulpwn/docs/dakotadb_csv/company_poc_rearranged.tsv").unwrap();
    let lines = file.split("\n");

    let mut i = 0;
    for student in lines {
        if i < 20 {
            i += 1;
            continue;
        }
        if i == 21 {
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
            
            let look = get_company(&connection, 1);
            println!("From get_company: {}", look.company_name);

            /*
            let mut c_suite: Option<&str> = None;
            if fields[3].eq("") == false {
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
            */
            /*
            let mut c_suite = "".to_string();
            if c.suite.eq(&None) == false {
                c_suite = c.suite.unwrap().to_string();
            }
            let mut c_phone_ext = "".to_string();
            if c.phone_ext.eq(&None) == false {
                c_phone_ext = c.phone_ext.unwrap().to_string();
            }
            let mut c_poc_f = "".to_string();
            if c.poc_firstname.eq(&None) == false {
                c_poc_f = c.poc_firstname.unwrap().to_string();
            }
            let mut c_poc_l = "".to_string();
            if c.poc_lastname.eq(&None) == false {
                c_poc_l = c.poc_lastname.unwrap().to_string();
            }
            let mut c_cc_hn = "".to_string();
            if c.cc_holdername.eq(&None) == false {
                c_cc_hn = c.cc_holdername.unwrap().to_string();
            }
            let mut c_notes = "".to_string();
            if c.notes.eq(&None) == false {
                c_notes = c.notes.unwrap().to_string();
            }
            
            println!("id:{}, company_name:{}, address:{}, suite:{}, city:{}, zip_code:{}, phone:{}", 
                     i, c.company_name, c.address, &c_suite, c.city, c.zip_code, c.phone); 
            println!("phone_ext:{}, poc_f:{}, poc_l:{}, cc_hn:{}, notes:{}, old_id:{}",
                     &c_phone_ext, &c_poc_f, &c_poc_l, &c_cc_hn, &c_notes, c.old_id.unwrap());
            //let _set_name = update_company_entry(&connection, &c.id, &0, "MYO TEMP SERVICE");
            */
            /*
            let mut x: i32 = 0;
            for field in fields {
                if x == 0 {
                    x += 1;
                    continue;
                };
                //let temp = x;
                let _set_field = update_company_entry(&connection, &c.id, &x, field);
                x += 1;
            }
            */
            /*
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
            /*
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
            */
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

