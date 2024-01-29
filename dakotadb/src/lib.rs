#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate chrono;
extern crate argon2;
extern crate rand;

pub mod schema;
pub mod models;
pub mod handlers;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
use argon2::{Config};
use rand::{distributions::Alphanumeric, Rng};
use serde::ser::StdError;

use self::models::{Student, NewStudent, Company, NewCompany, Class, NewClass, ClassEntry, NewClassEntry, Instructor, NewInstructor, StudentClass, NewStudentClass, User, NewUser};

pub type DbError = Box<dyn std::error::Error + Send + Sync>;
pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn establish_connection() -> Result<Pool, DbError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool...");
    
    Ok(pool)
}

#[tokio::main]
pub async fn make_detailed_address_api_call<'a>(input: &'a str) -> Result<Option<serde_json::Value>, Box<(dyn StdError + Send + Sync + 'static)>> {
    let mut url: String = "https://maps.googleapis.com/maps/api/place/details/json?place_id=".to_string();
    url += input;
    url += "&fields=formatted_address&key=SECRET_KEY";
    let body = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    //let result: String = body;
    Ok(Some(body))
}
#[tokio::main]

pub async fn make_address_api_call<'a>(input: &'a str) -> Result<Option<serde_json::Value>, Box<(dyn StdError + Send + Sync + 'static)>> {
    let mut url: String = "https://maps.googleapis.com/maps/api/place/autocomplete/json?input=".to_string();
    url += input;
    url += "&components=country:us&key=SECRET_KEY";
    let body = reqwest::get(url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    //let result: String = body;
    Ok(Some(body))
}

pub fn delete_user<'a>(conn: &PgConnection, user_id: i32) -> Result<Option<i32>, DbError> {
    use schema::users::dsl::*;
    let deleted_id =diesel::delete(users.filter(id.eq(user_id)))
                   .returning(id)
                   .get_result(conn)
                   .optional()?;
    Ok(deleted_id)
}

pub fn get_all_users<'a>(conn: &PgConnection) -> Result<Option<User>, DbError> {
    use schema::users::dsl::*;
    let all_users = users.get_result(conn).optional()?;

    Ok(all_users)
}

pub fn get_user<'a>(conn: &PgConnection, user_id: i32) -> Result<Option<User>, DbError> {
    use schema::users::dsl::*;
    let user = users.find(user_id).get_result(conn).optional()?;

    Ok(user)
}

pub fn update_user<'a>(conn: &PgConnection,
                       user_id: &'a i32,
                       new_pass: &'a str
                       ) -> Result<Option<User>, DbError> {
    use schema::users::dsl::*;

    let pass_bytes = new_pass.to_string().into_bytes();
    let new_salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let salt_bytes = new_salt.to_owned().into_bytes();
    let a2_config = Config::default();
    let pass_hash = argon2::hash_encoded(&pass_bytes, &salt_bytes, &a2_config).unwrap();

    let target = users.find(user_id);
    let user = diesel::update(target).set((pass.eq(pass_hash), salt.eq(new_salt)))
                                     .get_result(conn)
                                     .optional()?;

    Ok(user)
}

pub fn create_user<'a>(conn: &PgConnection, 
                       email: &'a str, 
                       pass: &'a str
                       ) -> Result<Option<User>, DbError> {
    use schema::users;

    let pass_bytes = pass.to_string().into_bytes();
    let salt: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();
    let salt_bytes = salt.to_owned().into_bytes();
    let a2_config = Config::default();
    let pass_hash = argon2::hash_encoded(&pass_bytes, &salt_bytes, &a2_config).unwrap();

    let new_user = NewUser {
        email: email.to_string(),
        pass: pass_hash,
        salt: salt
    };

    let user = diesel::insert_into(users::table)
                        .values(&new_user)
                        .get_result(conn)
                        .optional()?;

    Ok(user)
}

pub fn delete_studentclass_entry<'a>(conn: &PgConnection, studentclass_id: i32) -> Result<Option<i32>, DbError> {
    use schema::studentclass::dsl::*;
    let deleted_id =diesel::delete(studentclass.filter(id.eq(studentclass_id)))
                   .returning(id)
                   .get_result(conn)
                   .optional()?;
    Ok(deleted_id)
}

pub fn get_studentclass_entry<'a>(conn: &PgConnection, studentclass_id: i32) -> Result<Option<StudentClass>, DbError> {
    use schema::studentclass::dsl::*;
    let studentclass_entry = studentclass.find(studentclass_id).get_result(conn).optional()?;

    Ok(studentclass_entry)
}

pub fn update_studentclass_entry<'a>(conn: &PgConnection,
                                studentclass_id: &'a i32,
                                new_class_id: &'a i32,
                                new_student_id: &'a i32,
                                new_certification_num: &'a Option<String>,
                                new_test_score: &'a Option<i32>,
                                new_class_date: &'a chrono::NaiveDate,
                                new_class_end_date: &'a chrono::NaiveDate,
                                new_md_recert_date: &'a chrono::NaiveDate,
                                new_va_recert_date: &'a chrono::NaiveDate,
                                new_dc_recert_date: &'a chrono::NaiveDate
                                ) -> Result<Option<StudentClass>, DbError> {
    use schema::studentclass::dsl::*;

    let target = studentclass.find(studentclass_id);

    let new_certification_num = match new_certification_num {
        &None => None,
        _ => Some(new_certification_num.as_ref().unwrap().to_string()),
    };
    let new_test_score = match new_test_score {
        &None => None,
        _ => Some(*new_test_score.as_ref().unwrap()),
    };

    let studentclass_entry = diesel::update(target).set((class_id.eq(*new_class_id),
                                                         student_id.eq(*new_student_id),
                                                         certification_num.eq(new_certification_num),
                                                         test_score.eq(new_test_score),
                                                         class_date.eq(*new_class_date),
                                                         class_end_date.eq(*new_class_end_date),
                                                         md_recert_date.eq(*new_md_recert_date),
                                                         va_recert_date.eq(*new_va_recert_date),
                                                         dc_recert_date.eq(*new_dc_recert_date)))
                                                         .get_result(conn)
                                                         .optional()?;

    Ok(studentclass_entry)
}

pub fn create_studentclass_entry<'a>(conn: &PgConnection, 
                          class_id: &'a i32, 
                          student_id: &'a i32,
                          certification_num: &'a Option<String>,
                          test_score: &'a Option<i32>,
                          class_date: &'a chrono::NaiveDate,
                          class_end_date: &'a chrono::NaiveDate,
                          md_recert_date: &'a chrono::NaiveDate,
                          va_recert_date: &'a chrono::NaiveDate,
                          dc_recert_date: &'a chrono::NaiveDate
                          ) -> Result<Option<StudentClass>, DbError> {
    use schema::studentclass;

    let certification_num = match certification_num {
        &None => None,
        _ => Some(certification_num.as_ref().unwrap().to_string()),
    };
    let test_score = match test_score {
        &None => None,
        _ => Some(*test_score.as_ref().unwrap()),
    };

    let new_studentclass_entry = NewStudentClass {
        class_id: *class_id,
        student_id: *student_id,
        certification_num: certification_num,
        test_score: test_score,
        class_date: *class_date,
        class_end_date: *class_end_date,
        md_recert_date: *md_recert_date,
        va_recert_date: *va_recert_date,
        dc_recert_date: *dc_recert_date
    };

    let studentclass_entry = diesel::insert_into(studentclass::table)
                                .values(&new_studentclass_entry)
                                .get_result(conn)
                                .optional()?;

    Ok(studentclass_entry)
}

pub fn delete_instructor<'a>(conn: &PgConnection, instructor_id: i32) -> Result<Option<i32>, DbError> {
    use schema::instructors::dsl::*;
    let deleted_id =diesel::delete(instructors.filter(id.eq(instructor_id)))
                   .returning(id)
                   .get_result(conn)
                   .optional()?;
    Ok(deleted_id)
}

pub fn get_instructor_all<'a>(conn: &PgConnection) -> Result<Option<Vec<Instructor>>, DbError> {
    use schema::instructors::dsl::*;
    let instructor_all = instructors.load::<Instructor>(conn).optional()?;

    Ok(instructor_all)
}

pub fn get_instructor<'a>(conn: &PgConnection, instructor_id: i32) -> Result<Option<Instructor>, DbError> {
    use schema::instructors::dsl::*;
    let instructor = instructors.find(instructor_id).get_result(conn).optional()?;

    Ok(instructor)
}

pub fn update_instructor<'a>(conn: &PgConnection,
                                instructor_id: &'a i32,
                                new_acronym: &'a str,
                                new_first_name: &'a str,
                                new_last_name: &'a str
                                ) -> Result<Option<Instructor>, DbError> {
    use schema::instructors::dsl::*;

    let target = instructors.find(instructor_id);
    
    let instructor = diesel::update(target).set((acronym.eq(new_acronym.to_string()),
                                                 first_name.eq(new_first_name.to_string()),
                                                 last_name.eq(new_last_name.to_string())))
                                                 .get_result(conn)
                                                 .optional()?;
    
    Ok(instructor)
}

pub fn create_instructor<'a>(conn: &PgConnection, 
                          acronym: &'a str, 
                          first_name: &'a str,
                          last_name: &'a str
                          ) -> Result<Option<Instructor>, DbError> {
    use schema::instructors;

    let new_instructor = NewInstructor {
        acronym: acronym.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string()
    };

    let instructor = diesel::insert_into(instructors::table)
                                .values(&new_instructor)
                                .get_result(conn)
                                .optional()?;

    Ok(instructor)
}


pub fn delete_class_entry<'a>(conn: &PgConnection, class_entry_id: i32) -> Result<Option<i32>, DbError> {
    use schema::class_schedule::dsl::*;
    let deleted_id =diesel::delete(class_schedule.filter(id.eq(class_entry_id)))
                   .returning(id)
                   .get_result(conn)
                   .optional()?;
    Ok(deleted_id)
}

pub fn get_class_entry<'a>(conn: &PgConnection, class_entry_id: i32) -> Result<Option<ClassEntry>, DbError> {
    use schema::class_schedule::dsl::*;
    let class_entry = class_schedule.find(class_entry_id).get_result(conn).optional()?;

    Ok(class_entry)
}

pub fn get_studentclass_entries_for_class_entry<'a>(conn: &PgConnection, class_entry_id: i32) -> Result<Option<Vec<StudentClass>>, DbError> {
    use schema::studentclass::dsl::*;
    let class_entry_students = studentclass.filter(class_id.eq(class_entry_id)).load::<StudentClass>(conn).optional()?;

    Ok(class_entry_students)
}

pub fn update_class_entry<'a>(conn: &PgConnection,
                                class_entry_id: &'a i32,
                                new_class_id: &'a i32, 
                                new_class_date: &'a chrono::NaiveDate,
                                new_class_end_date: &'a chrono::NaiveDate,
                                new_md_recert_date: &'a chrono::NaiveDate,
                                new_va_recert_date: &'a chrono::NaiveDate,
                                new_dc_recert_date: &'a chrono::NaiveDate,
                                new_instructor_id: &'a i32,
                                new_second_instructor_id: &'a Option<i32>,
                                new_cancelled: &'a bool
                                ) -> Result<Option<ClassEntry>, DbError> {
    use schema::class_schedule::dsl::*;

    let new_second_instructor_id = match new_second_instructor_id {
        &None => None,
        _ => Some(*new_second_instructor_id.as_ref().unwrap()),
    };

    let target = class_schedule.find(class_entry_id);
    
    let class_entry = diesel::update(target).set((class_id.eq(*new_class_id),
                                                class_date.eq(*new_class_date),
                                                class_end_date.eq(*new_class_end_date),
                                                md_recert_date.eq(*new_md_recert_date),
                                                va_recert_date.eq(*new_va_recert_date),
                                                dc_recert_date.eq(*new_dc_recert_date),
                                                instructor_id.eq(*new_instructor_id),
                                                second_instructor_id.eq(new_second_instructor_id),
                                                cancelled.eq(*new_cancelled)))
                                                .get_result(conn)
                                                .optional()?;

    Ok(class_entry)
}

pub fn create_class_entry<'a>(conn: &PgConnection, 
                          class_id: &'a i32, 
                          class_date: &'a chrono::NaiveDate,
                          class_end_date: &'a chrono::NaiveDate,
                          md_recert_date: &'a chrono::NaiveDate,
                          va_recert_date: &'a chrono::NaiveDate,
                          dc_recert_date: &'a chrono::NaiveDate,
                          instructor_id: &'a i32,
                          second_instructor_id: &'a Option<i32>,
                          cancelled: &'a bool) -> Result<Option<ClassEntry>, DbError> {
    use schema::class_schedule;

    let second_instructor_id = match second_instructor_id {
        &None => None,
        _ => Some(*second_instructor_id.as_ref().unwrap()),
    };

    let new_class_entry = NewClassEntry {
        class_id: *class_id,
        class_date: *class_date,
        class_end_date: *class_end_date,
        md_recert_date: *md_recert_date,
        va_recert_date: *va_recert_date,
        dc_recert_date: *dc_recert_date,
        instructor_id: *instructor_id,
        second_instructor_id: second_instructor_id,
        cancelled: *cancelled
    };

    let class_entry = diesel::insert_into(class_schedule::table)
                                .values(&new_class_entry)
                                .get_result(conn)
                                .optional()?;

    Ok(class_entry)
}


pub fn delete_class<'a>(conn: &PgConnection, class_id: i32) -> Result<Option<i32>, DbError> {
    use schema::classes::dsl::*;
    let deleted_id =diesel::delete(classes.filter(id.eq(class_id)))
                   .returning(id)
                   .get_result(conn)
                   .optional()?;
    Ok(deleted_id)
}

pub fn get_class_all<'a>(conn: &PgConnection) -> Result<Option<Vec<Class>>, DbError> {
    use schema::classes::dsl::*;
    let class_all = classes.order(id.asc()).load::<Class>(conn).optional()?;

    Ok(class_all)
}

pub fn get_class<'a>(conn: &PgConnection, class_id: i32) -> Result<Option<Class>, DbError> {
    use schema::classes::dsl::*;
    let class = classes.find(class_id).get_result(conn).optional()?;

    Ok(class)
}

pub fn update_class<'a>(conn: &PgConnection,
                                class_id: &'a i32,
                                new_class_title: &'a str, 
                                new_class_language: &'a str,
                                new_md_approval_num: &'a Option<String>,
                                new_va_approval_num: &'a Option<String>,
                                new_dc_approval_num: &'a Option<String>,
                                new_md_recert_yrs: &'a Option<i32>,
                                new_va_recert_yrs: &'a Option<i32>,
                                new_dc_recert_yrs: &'a Option<i32>,
                                new_hours: &'a i32
                                ) -> Result<Option<Class>, DbError> {
    use schema::classes::dsl::*;

    let new_md_approval_num = match new_md_approval_num {
        &None => None,
        _ => Some(new_md_approval_num.as_ref().unwrap().to_string()),
    };
    let new_va_approval_num = match new_va_approval_num {
        &None => None,
        _ => Some(new_va_approval_num.as_ref().unwrap().to_string()),
    };
    let new_dc_approval_num = match new_dc_approval_num {
        &None => None,
        _ => Some(new_dc_approval_num.as_ref().unwrap().to_string()),
    };
    let new_md_recert_yrs = match new_md_recert_yrs {
        &None => None,
        _ => Some(*new_md_recert_yrs.as_ref().unwrap()),
    };
    let new_va_recert_yrs = match new_va_recert_yrs {
        &None => None,
        _ => Some(*new_va_recert_yrs.as_ref().unwrap()),
    };
    let new_dc_recert_yrs = match new_dc_recert_yrs {
        &None => None,
        _ => Some(*new_dc_recert_yrs.as_ref().unwrap()),
    };
    
    let target = classes.find(class_id);
    let class = diesel::update(target).set((class_title.eq(new_class_title.to_string()),
                                            class_language.eq(new_class_language.to_string()),
                                            md_approval_num.eq(new_md_approval_num),
                                            va_approval_num.eq(new_va_approval_num),
                                            dc_approval_num.eq(new_dc_approval_num),
                                            md_recert_yrs.eq(new_md_recert_yrs),
                                            va_recert_yrs.eq(new_va_recert_yrs),
                                            dc_recert_yrs.eq(new_dc_recert_yrs),
                                            hours.eq(*new_hours)))
                                           .get_result(conn)
                                           .optional()?;

    Ok(class)
}

pub fn create_class<'a>(conn: &PgConnection, 
                          class_title: &'a str, 
                          class_language: &'a str,
                          md_approval_num: &'a Option<String>,
                          va_approval_num: &'a Option<String>,
                          dc_approval_num: &'a Option<String>,
                          md_recert_yrs: &'a Option<i32>,
                          va_recert_yrs: &'a Option<i32>,
                          dc_recert_yrs: &'a Option<i32>,
                          hours: &'a i32
                          ) -> Result<Option<Class>, DbError> {
    use schema::classes;

    let md_approval_num = match md_approval_num {
        &None => None,
        _ => Some(md_approval_num.as_ref().unwrap().to_string()),
    };
    let va_approval_num = match va_approval_num {
        &None => None,
        _ => Some(va_approval_num.as_ref().unwrap().to_string()),
    };
    let dc_approval_num = match dc_approval_num {
        &None => None,
        _ => Some(dc_approval_num.as_ref().unwrap().to_string()),
    };
    let md_recert_yrs = match md_recert_yrs {
        &None => None,
        _ => Some(*md_recert_yrs.as_ref().unwrap()),
    };
    let va_recert_yrs = match va_recert_yrs {
        &None => None,
        _ => Some(*va_recert_yrs.as_ref().unwrap()),
    };
    let dc_recert_yrs = match dc_recert_yrs {
        &None => None,
        _ => Some(*dc_recert_yrs.as_ref().unwrap()),
    };

    let new_class = NewClass {
        class_title: class_title.to_string(),
        class_language: class_language.to_string(),
        md_approval_num: md_approval_num,
        va_approval_num: va_approval_num,
        dc_approval_num: dc_approval_num,
        md_recert_yrs: md_recert_yrs,
        va_recert_yrs: va_recert_yrs,
        dc_recert_yrs: dc_recert_yrs,
        hours: *hours
    };

    let class = diesel::insert_into(classes::table)
                        .values(&new_class)
                        .get_result(conn)
                        .optional()?;

    Ok(class)
}

pub fn delete_company<'a>(conn: &PgConnection, company_id: i32) -> Result<Option<i32>, DbError> {
    use schema::companies::dsl::*;
    let deleted_id =diesel::delete(companies.filter(id.eq(company_id)))
                   .returning(id)
                   .get_result(conn)
                   .optional()?;
    Ok(deleted_id)
}

pub fn get_company<'a>(conn: &PgConnection, company_id: i32) -> Result<Option<Company>, DbError> {
    use schema::companies::dsl::*;
    let company = companies.find(company_id).get_result(conn).optional()?;

    Ok(company)
}

pub fn update_company<'a>(conn: &PgConnection,
                                company_id: &'a i32,
                                new_company_name: &'a str,
                                new_address: &'a str,
                                new_suite: &'a Option<String>,
                                new_city: &'a str,
                                new_state: &'a str,
                                new_zip_code: &'a str,
                                new_phone: &'a str,
                                new_phone_ext: &'a Option<String>,
                                new_email: &'a str,
                                new_poc_firstname: &'a Option<String>,
                                new_poc_lastname: &'a Option<String>,
                                new_cc_holdername: &'a Option<String>,
                                new_cc_num: &'a Option<String>,
                                new_cc_expdate: &'a Option<String>,
                                new_cc_cvv: &'a Option<String>,
                                new_cc_zipcode: &'a Option<String>,
                                new_notes: &'a Option<String>
                                ) -> Result<Option<Company>, DbError> {
    use schema::companies::dsl::*;

    let new_suite = match new_suite {
        &None => None,
        _ => Some(new_suite.as_ref().unwrap().to_string()),
    };
    let new_phone_ext = match new_phone_ext {
        &None => None,
        _ => Some(new_phone_ext.as_ref().unwrap().to_string()),
    };
    let new_poc_firstname = match new_poc_firstname {
        &None => None,
        _ => Some(new_poc_firstname.as_ref().unwrap().to_string()),
    };
    let new_poc_lastname = match new_poc_lastname {
        &None => None,
        _ => Some(new_poc_lastname.as_ref().unwrap().to_string()),
    };
    let new_cc_holdername = match new_cc_holdername {
        &None => None,
        _ => Some(new_cc_holdername.as_ref().unwrap().to_string()),
    };
    let new_cc_num = match new_cc_num {
        &None => None,
        _ => Some(new_cc_num.as_ref().unwrap().to_string()),
    };
    let new_cc_expdate = match new_cc_expdate {
        &None => None,
        _ => Some(new_cc_expdate.as_ref().unwrap().to_string()),
    };
    let new_cc_cvv = match new_cc_cvv {
        &None => None,
        _ => Some(new_cc_cvv.as_ref().unwrap().to_string()),
    };
    let new_cc_zipcode = match new_cc_zipcode {
        &None => None,
        _ => Some(new_cc_zipcode.as_ref().unwrap().to_string()),
    };
    let new_notes = match new_notes {
        &None => None,
        _ => Some(new_notes.as_ref().unwrap().to_string()),
    };
    
    let target = companies.find(company_id);
    
    let company = diesel::update(target)
        .set((company_name.eq(new_company_name.to_string()),
        address.eq(new_address.to_string()),
        suite.eq(new_suite),
        city.eq(new_city.to_string()),
        state.eq(new_state.to_string()),
        zip_code.eq(new_zip_code.to_string()),
        phone.eq(new_phone.to_string()),
        phone_ext.eq(new_phone_ext),
        email.eq(new_email.to_string()),
        poc_firstname.eq(new_poc_firstname),
        poc_lastname.eq(new_poc_lastname),
        cc_holdername.eq(new_cc_holdername),
        cc_num.eq(new_cc_num),
        cc_expdate.eq(new_cc_expdate),
        cc_cvv.eq(new_cc_cvv),
        cc_zipcode.eq(new_cc_zipcode),
        notes.eq(new_notes)))
        .get_result(conn)
        .optional()?;
    
    Ok(company)
}

pub fn create_company<'a>(conn: &PgConnection,
                          company_name: &'a str,
                          address: &'a str,
                          suite: &'a Option<String>,
                          city: &'a str,
                          state: &'a str,
                          zip_code: &'a str,
                          phone: &'a str,
                          phone_ext: &'a Option<String>,
                          email: &'a str,
                          poc_firstname: &'a Option<String>,
                          poc_lastname: &'a Option<String>,
                          cc_holdername: &'a Option<String>,
                          cc_num: &'a Option<String>,
                          cc_expdate: &'a Option<String>,
                          cc_cvv: &'a Option<String>,
                          cc_zipcode: &'a Option<String>,
                          notes: &'a Option<String>
                          ) -> Result<Option<Company>, DbError> {
    use schema::companies;

    let suite = match suite {
        &None => None,
        _ => Some(suite.as_ref().unwrap().to_string()),
    };
    let phone_ext = match phone_ext {
        &None => None,
        _ => Some(phone_ext.as_ref().unwrap().to_string()),
    };
    let poc_firstname = match poc_firstname {
        &None => None,
        _ => Some(poc_firstname.as_ref().unwrap().to_string()),
    };
    let poc_lastname = match poc_lastname {
        &None => None,
        _ => Some(poc_lastname.as_ref().unwrap().to_string()),
    };
    let cc_holdername = match cc_holdername {
        &None => None,
        _ => Some(cc_holdername.as_ref().unwrap().to_string()),
    };
    let cc_num = match cc_num {
        &None => None,
        _ => Some(cc_num.as_ref().unwrap().to_string()),
    };
    let cc_expdate = match cc_expdate {
        &None => None,
        _ => Some(cc_expdate.as_ref().unwrap().to_string()),
    };
    let cc_cvv = match cc_cvv {
        &None => None,
        _ => Some(cc_cvv.as_ref().unwrap().to_string()),
    };
    let cc_zipcode = match cc_zipcode {
        &None => None,
        _ => Some(cc_zipcode.as_ref().unwrap().to_string()),
    };
    let notes = match notes {
        &None => None,
        _ => Some(notes.as_ref().unwrap().to_string()),
    };

    let new_company = NewCompany {
        company_name: company_name.to_string(),
        address: address.to_string(),
        suite: suite,
        city: city.to_string(),
        state: state.to_string(),
        zip_code: zip_code.to_string(),
        phone: phone.to_string(),
        phone_ext: phone_ext,
        email: email.to_string(),
        poc_firstname: poc_firstname,
        poc_lastname: poc_lastname,
        cc_holdername: cc_holdername,
        cc_num: cc_num,
        cc_expdate: cc_expdate,
        cc_cvv: cc_cvv,
        cc_zipcode: cc_zipcode,
        notes: notes
    };

    let company = diesel::insert_into(companies::table)
                    .values(&new_company)
                    .get_result(conn)
                    .optional()?;
    Ok(company)
}

pub fn delete_student<'a>(conn: &PgConnection, student_id: i32) -> Result<Option<i32>, DbError> {
    use schema::students::dsl::*;
    let deleted_id =diesel::delete(students.filter(id.eq(student_id)))
                   .returning(id)
                   .get_result(conn)
                   .optional()?;
    Ok(deleted_id)
}

pub fn get_student<'a>(conn: &PgConnection, student_id: i32) -> Result<Option<Student>, DbError> {
    use schema::students::dsl::*;
    let student = students.find(student_id).get_result(conn).optional()?;

    Ok(student)
}

pub fn update_student<'a>(conn: &PgConnection,
                                student_id: &'a i32,
                                new_social: &'a str,
                                new_first_name: &'a str,
                                new_last_name: &'a str,
                                new_address: &'a str,
                                new_suite: &'a Option<String>,
                                new_city: &'a str,
                                new_state: &'a str,
                                new_zip_code: &'a str,
                                new_phone: &'a str,
                                new_dob: &'a chrono::NaiveDate,
                                new_company_id: &'a Option<i32>,
                                new_email: &'a Option<String>,
                                new_photo: &'a Option<String>
                                ) -> Result<Option<Student>, DbError> {
    use schema::students::dsl::*;

    let new_suite = match new_suite {
        &None => None,
        _ => Some(new_suite.as_ref().unwrap().to_string()),
    };
    let new_company_id = match new_company_id {
        &None => None,
        _ => Some(*new_company_id.as_ref().unwrap()),
    };
    let new_email = match new_email {
        &None => None,
        _ => Some(new_email.as_ref().unwrap().to_string()),
    };
    let new_photo = match new_photo {
        &None => None,
        _ => Some(new_photo.as_ref().unwrap().to_string()),
    };

    let target = students.find(student_id);
    let student = diesel::update(target).set((social.eq(new_social.to_string()),
                    first_name.eq(new_first_name.to_string()),
                    last_name.eq(new_last_name.to_string()),
                    address.eq(new_address.to_string()),
                    suite.eq(new_suite),
                    city.eq(new_city.to_string()),
                    state.eq(new_state.to_string()),
                    zip_code.eq(new_zip_code.to_string()),
                    phone.eq(new_phone.to_string()),
                    dob.eq(*new_dob),
                    company_id.eq(new_company_id),
                    email.eq(new_email),
                    photo.eq(new_photo)))
                    .get_result(conn)
                    .optional()?;
    Ok(student)
}

pub fn create_student<'a>(conn: &PgConnection, 
                          social: &'a str, 
                          first_name: &'a str,
                          last_name: &'a str,
                          address: &'a str,
                          suite: &'a Option<String>,
                          city: &'a str,
                          state: &'a str,
                          zip_code: &'a str,
                          phone: &'a str,
                          dob: &'a chrono::NaiveDate,
                          company_id: &'a Option<i32>,
                          email: &'a Option<String>,
                          photo: &'a Option<String>
                          ) -> Result<Option<Student>, DbError> {
    use schema::students;

    let suite = match suite {
        &None => None,
        _ => Some(suite.as_ref().unwrap().to_string()),
    };
    let company_id = match company_id {
        &None => None,
        _ => Some(*company_id.as_ref().unwrap()),
    };
    let email = match email {
        &None => None,
        _ => Some(email.as_ref().unwrap().to_string()),
    };
    let photo = match photo {
        &None => None,
        _ => Some(photo.as_ref().unwrap().to_string()),
    };

    let new_student = NewStudent {
        social: social.to_string(),
        first_name: first_name.to_string(),
        last_name: last_name.to_string(),
        address: address.to_string(),
        suite: suite,
        city: city.to_string(),
        state: state.to_string(),
        zip_code: zip_code.to_string(),
        phone: phone.to_string(),
        dob: *dob,
        company_id: company_id,
        email: email,
        photo: photo
    };

    let student = diesel::insert_into(students::table)
                    .values(&new_student)
                    .get_result(conn)
                    .optional()?;

    Ok(student)
}
