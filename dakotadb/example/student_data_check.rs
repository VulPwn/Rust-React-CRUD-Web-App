extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use self::models::*;
use self::diesel::prelude::*;

fn main() {

    use dakotadb::schema::students::dsl::*;
    use dakotadb::schema::companies::dsl::*;
    
    let connection = establish_connection();
    
    let results = students.order(dakotadb::schema::students::dsl::id.asc()).load::<Student>(&connection);
    let count = students.count().get_result(&connection);
    assert_eq!(Ok(9042), count);
    
    let mut i = 0;
    for student in results.unwrap() {
        if i < 50 {
            i += 1;
            continue;
        }
        if i == 52 {
            break;
        }
        i += 1;
        
        println!("i => {} & student_id => {}", i, student.id);
        
        let mut company = companies.filter(dakotadb::schema::companies::dsl::id.eq(student.company_id.unwrap()))
                                   .first::<Company>(&connection);
        
        let mut company = match company {
            Ok(item) => item,
            Err(e) => {
                println!("Error loading the company: {:?}", e);
                continue;
            },
        };

        println!("Company Name: {}, id: {}", company.company_name, company.id);
        
        //println!("{} {} {} {} {}", student.id, student.first_name, student.last_name, student.company_id.unwrap(), student.social); 
    }
    /*
        let mut company = companies.filter(dakotadb::schema::companies::dsl::old_id.eq(student.company_id))
                                   .limit(1)
                                   .load::<Company>(&connection);
        
        let mut company = match company {
            Ok(item) => item,
            Err(e) => {
                println!("Error loading the company: {:?}", e);
                continue;
            },
        };
        let company = company.pop().unwrap();
        //println!("{}", company.company_name);
        let update: Result<Student, diesel::result::Error> = diesel::update(&student).set(company_id.eq(company.id)).get_result(&connection);
        
        match update {
            Ok(update) => {
                println!("Successfully updated company_id for {} {}: {} -> {}", 
                    update.first_name, update.last_name, student.company_id.unwrap(), update.company_id.unwrap());
                println!("Data check: {} => {}", update.id, student.id);
            },
            Err(e) => {
                println!("{:?}", e); 
            }
        }
    }
    */
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

