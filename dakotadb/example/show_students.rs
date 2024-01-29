extern crate diesel;
extern crate dakotadb;

use self::dakotadb::*;
use self::models::*;
use self::diesel::prelude::*;

fn main()
{
    use dakotadb::schema::students::dsl::*;

    let connection = establish_connection();
    let results = students.filter(first_name.eq("HEDRAS E."))
        .limit(5)
        .load::<Student>(&connection)
        .expect("Error loading students");

    println!("Displaying {} students", results.len());
    for student in results {
        println!("{}", student.first_name);
        println!("{}", student.last_name);
        println!("{}", student.dob);
    }
}
