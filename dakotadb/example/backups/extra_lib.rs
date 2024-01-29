#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use actix_web::Responder;

use self::models::{Student, NewStudent, Company, NewCompany, Class,    NewClass, ClassEntry, NewClassEntry, Instructor, NewInstructor,
    StudentClass, NewStudentClass};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_studentclass_entry<'a>(conn: &PgConnection, studentclass_id: i32) -> StudentClass {
    use schema::studentclass::dsl::*;
    studentclass.find(studentclass_id).get_result(conn).expect("Error retrieving studentclass entry...")
}

pub fn update_studentclass_entry<'a>(conn: &PgConnection,
                                studentclass_id: &'a i32,
                                pos: &'a i32,
                                value: &'a str) -> QueryResult<usize> {
    use schema::studentclass::dsl::*;
    use diesel::NotFound;
    use chrono::NaiveDate;

    let target = studentclass.find(studentclass_id);
    match *pos {
        0 => diesel::update(target).set(class_id.eq(value.to_string().parse::<i32>().unwrap())).execute(conn),
        1 => diesel::update(target).set(student_id.eq(value.to_string().parse::<i32>().unwrap())).execute(conn),
        2 => diesel::update(target).set(certification_num.eq(Some(value))).execute(conn),
        3 => diesel::update(target).set(test_score.eq(Some(value.to_string().parse::<i32>().unwrap()))).execute(conn),
        4 => {
            let parsed_class_date = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(class_date.eq(&parsed_class_date.unwrap())).execute(conn)
        },
        5 => {
            let parsed_class_end_date = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(class_end_date.eq(&parsed_class_end_date.unwrap())).execute(conn)
        },
        6 => {
            let parsed_md_recert = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(md_recert_date.eq(&parsed_md_recert.unwrap())).execute(conn)
        },
        7 => {
            let parsed_va_recert = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(va_recert_date.eq(&parsed_va_recert.unwrap())).execute(conn)
        },
        8 => {
            let parsed_dc_recert = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(dc_recert_date.eq(&parsed_dc_recert.unwrap())).execute(conn)
        },
        _ => Err(NotFound),
    }
}

pub fn create_studentclass_entry<'a>(conn: &PgConnection, 
                          class_id: &'a i32, 
                          student_id: &'a i32,
                          certification_num: Option<&'a str>,
                          test_score: Option<&'a i32>,
                          class_date: &'a chrono::NaiveDate,
                          class_end_date: &'a chrono::NaiveDate,
                          md_recert_date: &'a chrono::NaiveDate,
                          va_recert_date: &'a chrono::NaiveDate,
                          dc_recert_date: &'a chrono::NaiveDate) -> StudentClass {
    use schema::studentclass;

    let new_studentclass_entry = NewStudentClass {
        class_id: class_id,
        student_id: student_id,
        certification_num: certification_num,
        test_score: test_score,
        class_date: class_date,
        class_end_date: class_end_date,
        md_recert_date: md_recert_date,
        va_recert_date: va_recert_date,
        dc_recert_date: dc_recert_date
    };

    diesel::insert_into(studentclass::table)
        .values(&new_studentclass_entry)
        .get_result(conn)
        .expect("Error saving new studentclass entry...")
}

pub fn get_instructor<'a>(conn: &PgConnection, instructor_id: i32) -> Instructor {
    use schema::instructors::dsl::*;
    instructors.find(instructor_id).get_result(conn).expect("Error retrieving instructor...")
}

pub fn update_instructor<'a>(conn: &PgConnection,
                                instructor_id: &'a i32,
                                pos: &'a i32,
                                value: &'a str) -> QueryResult<usize> {
    use schema::instructors::dsl::*;
    use diesel::NotFound;

    let target = instructors.find(instructor_id);
    match *pos {
        0 => diesel::update(target).set(acronym.eq(value)).execute(conn),
        1 => diesel::update(target).set(first_name.eq(value)).execute(conn),
        2 => diesel::update(target).set(last_name.eq(value)).execute(conn),
        _ => Err(NotFound),
    }
}

pub fn create_instructor<'a>(conn: &PgConnection, 
                          acronym: &'a str, 
                          first_name: &'a str,
                          last_name: &'a str) -> Instructor {
    use schema::instructors;

    let new_instructor = NewInstructor {
        acronym: acronym,
        first_name: first_name,
        last_name: last_name
    };

    diesel::insert_into(instructors::table)
        .values(&new_instructor)
        .get_result(conn)
        .expect("Error saving new instructor...")
}

pub fn get_class_entry<'a>(conn: &PgConnection, class_entry_id: i32) -> ClassEntry {
    use schema::class_schedule::dsl::*;
    class_schedule.find(class_entry_id).get_result(conn).expect("Error retrieving class entry from class schedule...")
}

pub fn update_class_entry<'a>(conn: &PgConnection,
                                class_entry_id: &'a i32,
                                pos: &'a i32,
                                value: &'a str) -> QueryResult<usize> {
    use schema::class_schedule::dsl::*;
    use diesel::NotFound;
    use chrono::NaiveDate;

    let target = class_schedule.find(class_entry_id);
    match *pos {
        0 => diesel::update(target).set(class_id.eq(value.to_string().parse::<i32>().unwrap())).execute(conn),
        1 => {
            let p_cd = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(class_date.eq(&p_cd.unwrap())).execute(conn)
        },
        2 => {
            let p_ced = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(class_end_date.eq(&p_ced.unwrap())).execute(conn)
        },
        3 => {
            let p_mdrd = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(md_recert_date.eq(&p_mdrd.unwrap())).execute(conn)
        },
        4 => {
            let p_vard = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(va_recert_date.eq(&p_vard.unwrap())).execute(conn)
        },
        5 => {
            let p_dcrd = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(dc_recert_date.eq(&p_dcrd.unwrap())).execute(conn)
        },
        6 => diesel::update(target).set(instructor_id.eq(value.to_string().parse::<i32>().unwrap())).execute(conn),
        7 => diesel::update(target).set(second_instructor_id.eq(Some(value.to_string().parse::<i32>().unwrap()))).execute(conn),
        8 => diesel::update(target).set(cancelled.eq(value.to_string().parse::<bool>().unwrap())).execute(conn),
        _ => Err(NotFound),
    }
}

pub fn create_class_entry<'a>(conn: &PgConnection, 
                          class_id: &'a i32, 
                          class_date: &'a chrono::NaiveDate,
                          class_end_date: &'a chrono::NaiveDate,
                          md_recert_date: &'a chrono::NaiveDate,
                          va_recert_date: &'a chrono::NaiveDate,
                          dc_recert_date: &'a chrono::NaiveDate,
                          instructor_id: &'a i32,
                          second_instructor_id: Option<&'a i32>,
                          cancelled: &'a bool) -> ClassEntry {
    use schema::class_schedule;

    let new_class_entry = NewClassEntry {
        class_id: class_id,
        class_date: class_date,
        class_end_date: class_end_date,
        md_recert_date: md_recert_date,
        va_recert_date: va_recert_date,
        dc_recert_date: dc_recert_date,
        instructor_id: instructor_id,
        second_instructor_id: second_instructor_id,
        cancelled: cancelled
    };

    diesel::insert_into(class_schedule::table)
        .values(&new_class_entry)
        .get_result(conn)
        .expect("Error saving new class entry to class schedule...")
}

pub fn get_class<'a>(conn: &PgConnection, class_id: i32) -> Class {
    use schema::classes::dsl::*;
    classes.find(class_id).get_result(conn).expect("Error retrieving class...")
}

pub fn update_class<'a>(conn: &PgConnection,
                                class_id: &'a i32,
                                pos: &'a i32,
                                value: &'a str) -> QueryResult<usize> {
    use schema::classes::dsl::*;
    use diesel::NotFound;

    let target = classes.find(class_id);
    match *pos {
        0 => diesel::update(target).set(class_title.eq(value)).execute(conn),
        1 => diesel::update(target).set(class_language.eq(value)).execute(conn),
        2 => diesel::update(target).set(md_approval_num.eq(Some(value))).execute(conn),
        3 => diesel::update(target).set(va_approval_num.eq(Some(value))).execute(conn),
        4 => diesel::update(target).set(dc_approval_num.eq(Some(value))).execute(conn),
        5 => diesel::update(target).set(md_recert_yrs.eq(Some(value.to_string().parse::<i32>().unwrap()))).execute(conn),
        6 => diesel::update(target).set(va_recert_yrs.eq(Some(value.to_string().parse::<i32>().unwrap()))).execute(conn),
        7 => diesel::update(target).set(dc_recert_yrs.eq(Some(value.to_string().parse::<i32>().unwrap()))).execute(conn),
        _ => Err(NotFound),
    }
}

pub fn create_class<'a>(conn: &PgConnection, 
                          class_title: &'a str, 
                          class_language: &'a str,
                          md_approval_num: Option<&'a str>,
                          va_approval_num: Option<&'a str>,
                          dc_approval_num: Option<&'a str>,
                          md_recert_yrs: Option<&'a i32>,
                          va_recert_yrs: Option<&'a i32>,
                          dc_recert_yrs: Option<&'a i32>) -> Class {
    use schema::classes;

    let new_class = NewClass {
        class_title: class_title,
        class_language: class_language,
        md_approval_num: md_approval_num,
        va_approval_num: va_approval_num,
        dc_approval_num: dc_approval_num,
        md_recert_yrs: md_recert_yrs,
        va_recert_yrs: va_recert_yrs,
        dc_recert_yrs: dc_recert_yrs
    };

    diesel::insert_into(classes::table)
        .values(&new_class)
        .get_result(conn)
        .expect("Error saving new class...")
}

pub fn get_company<'a>(conn: &PgConnection, company_id: i32) -> Company {
    use schema::companies::dsl::*;
    companies.find(company_id).get_result(conn).expect("Error retrieving company...")
}

pub fn update_company<'a>(conn: &PgConnection,
                                company_id: &'a i32,
                                pos: &'a i32,
                                value: &'a str) -> QueryResult<usize> {
    use schema::companies::dsl::*;
    use diesel::NotFound;

    let target = companies.find(company_id);
    match *pos {
        0 => diesel::update(target).set(company_name.eq(value)).execute(conn),
        1 => diesel::update(target).set(address.eq(value)).execute(conn),
        2 => diesel::update(target).set(suite.eq(Some(value))).execute(conn),
        3 => diesel::update(target).set(city.eq(value)).execute(conn),
        4 => diesel::update(target).set(state.eq(value)).execute(conn),
        5 => diesel::update(target).set(zip_code.eq(value)).execute(conn),
        6 => diesel::update(target).set(phone.eq(value)).execute(conn),
        7 => diesel::update(target).set(phone_ext.eq(Some(value))).execute(conn),
        8 => diesel::update(target).set(email.eq(value)).execute(conn),
        9 => diesel::update(target).set(poc_firstname.eq(Some(value))).execute(conn),
        10 => diesel::update(target).set(poc_lastname.eq(Some(value))).execute(conn),
        11 => diesel::update(target).set(cc_holdername.eq(Some(value))).execute(conn),
        12 => diesel::update(target).set(cc_num.eq(Some(value))).execute(conn),
        13 => diesel::update(target).set(cc_expdate.eq(Some(value))).execute(conn),
        14 => diesel::update(target).set(cc_cvv.eq(Some(value))).execute(conn),
        15 => diesel::update(target).set(cc_zipcode.eq(Some(value))).execute(conn),
        16 => diesel::update(target).set(notes.eq(Some(value))).execute(conn),
        _ => Err(NotFound),
    }
}

pub fn create_company<'a>(conn: &PgConnection,
                          company_name: &'a str,
                          address: &'a str,
                          suite: Option<&'a str>,
                          city: &'a str,
                          state: &'a str,
                          zip_code: &'a str,
                          phone: &'a str,
                          phone_ext: Option<&'a str>,
                          email: &'a str,
                          poc_firstname: Option<&'a str>,
                          poc_lastname: Option<&'a str>,
                          cc_holdername: Option<&'a str>,
                          cc_num: Option<&'a str>,
                          cc_expdate: Option<&'a str>,
                          cc_cvv: Option<&'a str>,
                          cc_zipcode: Option<&'a str>,
                          notes: Option<&'a str>) -> Company {
    use schema::companies;

    let new_company = NewCompany {
        company_name: company_name,
        address: address,
        suite: suite,
        city: city,
        state: state,
        zip_code: zip_code,
        phone: phone,
        phone_ext: phone_ext,
        email: email,
        poc_firstname: poc_firstname,
        poc_lastname: poc_lastname,
        cc_holdername: cc_holdername,
        cc_num: cc_num,
        cc_expdate: cc_expdate,
        cc_cvv: cc_cvv,
        cc_zipcode: cc_zipcode,
        notes: notes
    };

    diesel::insert_into(companies::table)
        .values(&new_company)
        .get_result(conn)
        .expect("Error saving new company...")
}

pub fn get_student<'a>(conn: &PgConnection, student_id: i32) -> Student {
    use schema::students::dsl::*;
    students.find(student_id).get_result(conn).expect("Error retrieving student...")
}

pub fn update_student<'a>(conn: &PgConnection,
                                student_id: &'a i32,
                                pos: &'a i32,
                                value: &'a str) -> QueryResult<usize> {
    use schema::students::dsl::*;
    use diesel::NotFound;
    use chrono::NaiveDate;

    let target = students.find(student_id);
    match *pos {
        0 => diesel::update(target).set(social.eq(value)).execute(conn),
        1 => diesel::update(target).set(first_name.eq(value)).execute(conn),
        2 => diesel::update(target).set(last_name.eq(value)).execute(conn),
        3 => diesel::update(target).set(address.eq(value)).execute(conn),
        4 => diesel::update(target).set(suite.eq(Some(value))).execute(conn),
        5 => diesel::update(target).set(city.eq(value)).execute(conn),
        6 => diesel::update(target).set(state.eq(value)).execute(conn),
        7 => diesel::update(target).set(zip_code.eq(value)).execute(conn),
        8 => diesel::update(target).set(phone.eq(value)).execute(conn),
        9 => {
            let parsed_dob = NaiveDate::parse_from_str(value, "%m/%d/%Y");
            diesel::update(target).set(dob.eq(&parsed_dob.unwrap())).execute(conn)
        },
        10 => diesel::update(target).set(company_id.eq(Some(value.to_string().parse::<i32>().unwrap()))).execute(conn),
        11 => diesel::update(target).set(email.eq(Some(value))).execute(conn),
        12 => diesel::update(target).set(photo.eq(Some(value))).execute(conn),
        _ => Err(NotFound),
    }
}

pub fn create_student<'a>(conn: &PgConnection, 
                          social: &'a str, 
                          first_name: &'a str,
                          last_name: &'a str,
                          address: &'a str,
                          suite: Option<&'a str>,
                          city: &'a str,
                          state: &'a str,
                          zip_code: &'a str,
                          phone: &'a str,
                          dob: &'a chrono::NaiveDate,
                          company_id: Option<&'a i32>,
                          email: Option<&'a str>,
                          photo: Option<&'a str>) -> Student {
    use schema::students;

    let new_student = NewStudent {
        social: social,
        first_name: first_name,
        last_name: last_name,
        address: address,
        suite: suite,
        city: city,
        state: state,
        zip_code: zip_code,
        phone: phone,
        dob: dob,
        company_id: company_id,
        email: email,
        photo: photo
    };

    diesel::insert_into(students::table)
        .values(&new_student)
        .get_result(conn)
        .expect("Error saving new student...")
}
