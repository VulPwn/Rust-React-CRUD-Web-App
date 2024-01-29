extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use self::models::*;
use self::diesel::prelude::*;
use std::fs;

fn main() {

    use dakotadb::schema::students::dsl::*;
    use dakotadb::schema::companies::dsl::*;
    
    let connection = establish_connection();

    let results = students.order(dakotadb::schema::students::dsl::id.asc()).load::<Student>(&connection);
    let file = fs::read_to_string("/home/vulpwn/docs/dakotadb_csv/students.tsv").unwrap();
    let lines = file.split("\n").collect::<Vec<_>>();

    let mut i = 0;
    for student in results.unwrap() {
        /*
        if i < 50 {
            i += 1;
            continue;
        }
        if i == 52 {
            break;
        }
        */
        i += 1;
        
        let old_data = lines[i];
        let cut = old_data.split("\x09");
        let mut fields: [&str; 13] = Default::default();
        let mut j = 0;
        for slice in cut {
            fields[j] = &slice;
            j += 1;
        }
        //println!("{} | {}", fields[1], student.first_name);
        assert_eq!(fields[1], student.first_name);
        assert_eq!(fields[2], student.last_name);
        
        let id_from_file = fields[10].to_string().parse::<i32>().unwrap();
        if student.company_id.eq(&Some(id_from_file)) == false {
            continue;
        };
        
        let independent: Option<i32> = Some(639);
        let mut company = companies.filter(dakotadb::schema::companies::dsl::old_id.eq(&independent))
                                   .first::<Company>(&connection);
        //let mut company = companies.filter(dakotadb::schema::companies::dsl::old_id.eq(&Some(id_from_file)))
        //                           .first::<Company>(&connection);
        
        let mut company = match company {
            Ok(item) => item,
            Err(e) => {
                println!("Error loading company {}: {:?}", id_from_file, e);
                continue;
            },
        };
        
        //println!("{}", company.company_name);
        
        let update: Result<Student, diesel::result::Error> = diesel::update(&student).set(company_id.eq(Some(company.id))).get_result(&connection);
        
        match update {
            Ok(update) => {
                println!("Successfully updated company_id for {} {}: {} => {}", 
                    update.first_name, update.last_name, student.company_id.unwrap(), update.company_id.unwrap());
                println!("Data check: {} => {}", update.id, student.id);
            },
            Err(e) => {
                println!("{:?}", e); 
            }
        }
    }
}
