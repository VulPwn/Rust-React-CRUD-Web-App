use serde::{Deserialize, Serialize};
use super::schema::students;
use super::schema::companies;
use super::schema::classes;
use super::schema::class_schedule;
use super::schema::instructors;
use super::schema::studentclass;
use super::schema::users;

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub email: String,
    pub pass: String,
    pub salt: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub email: String,
    pub pass: String,
    pub salt: String,
}

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "studentclass"]
pub struct StudentClass {
    pub id: i32,
    pub class_id: i32,
    pub student_id: i32,
    pub certification_num: Option<String>,
    pub test_score: Option<i32>,
    pub class_date: chrono::NaiveDate,
    pub class_end_date: chrono::NaiveDate,
    pub md_recert_date: chrono::NaiveDate,
    pub va_recert_date: chrono::NaiveDate,
    pub dc_recert_date: chrono::NaiveDate,
}

#[derive(Deserialize, Insertable)]
#[table_name = "studentclass"]
pub struct NewStudentClass {
    pub class_id: i32,
    pub student_id: i32,
    pub certification_num: Option<String>,
    pub test_score: Option<i32>,
    pub class_date: chrono::NaiveDate,
    pub class_end_date: chrono::NaiveDate,
    pub md_recert_date: chrono::NaiveDate,
    pub va_recert_date: chrono::NaiveDate,
    pub dc_recert_date: chrono::NaiveDate,
}

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "instructors"]
pub struct Instructor {
    pub id: i32,
    pub acronym: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Deserialize, Insertable)]
#[table_name = "instructors"]
pub struct NewInstructor {
    pub acronym: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "class_schedule"]
pub struct ClassEntry {
    pub id: i32,
    pub class_id: i32,
    pub class_date: chrono::NaiveDate,
    pub class_end_date: chrono::NaiveDate,
    pub md_recert_date: chrono::NaiveDate,
    pub va_recert_date: chrono::NaiveDate,
    pub dc_recert_date: chrono::NaiveDate,
    pub instructor_id: i32,
    pub second_instructor_id: Option<i32>,
    pub cancelled: bool,
}

#[derive(Deserialize, Insertable)]
#[table_name = "class_schedule"]
pub struct NewClassEntry {
    pub class_id: i32,
    pub class_date: chrono::NaiveDate,
    pub class_end_date: chrono::NaiveDate,
    pub md_recert_date: chrono::NaiveDate,
    pub va_recert_date: chrono::NaiveDate,
    pub dc_recert_date: chrono::NaiveDate,
    pub instructor_id: i32,
    pub second_instructor_id: Option<i32>,
    pub cancelled: bool,
}

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "classes"]
pub struct Class {
    pub id: i32,
    pub class_title: String,
    pub class_language: String,
    pub md_approval_num: Option<String>,
    pub va_approval_num: Option<String>,
    pub dc_approval_num: Option<String>,
    pub md_recert_yrs: Option<i32>,
    pub va_recert_yrs: Option<i32>,
    pub dc_recert_yrs: Option<i32>,
    pub hours: i32,
}

#[derive(Deserialize, Insertable)]
#[table_name = "classes"]
pub struct NewClass {
    pub class_title: String,
    pub class_language: String,
    pub md_approval_num: Option<String>,
    pub va_approval_num: Option<String>,
    pub dc_approval_num: Option<String>,
    pub md_recert_yrs: Option<i32>,
    pub va_recert_yrs: Option<i32>,
    pub dc_recert_yrs: Option<i32>,
    pub hours: i32,
}

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "companies"]
pub struct Company {
    pub id: i32,
    pub company_name: String,
    pub address: String,
    pub suite: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub phone: String,
    pub phone_ext: Option<String>,
    pub email: String,
    pub poc_firstname: Option<String>,
    pub poc_lastname: Option<String>,
    pub cc_holdername: Option<String>,
    pub cc_num: Option<String>,
    pub cc_expdate: Option<String>,
    pub cc_cvv: Option<String>,
    pub cc_zipcode: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "companies"]
pub struct NewCompany {
    pub company_name: String,
    pub address: String,
    pub suite: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub phone: String,
    pub phone_ext: Option<String>,
    pub email: String,
    pub poc_firstname: Option<String>,
    pub poc_lastname: Option<String>,
    pub cc_holdername: Option<String>,
    pub cc_num: Option<String>,
    pub cc_expdate: Option<String>,
    pub cc_cvv: Option<String>,
    pub cc_zipcode: Option<String>,
    pub notes: Option<String>,
}

#[derive(Serialize, Queryable, Identifiable)]
#[table_name = "students"]
pub struct Student {
    pub id: i32,
    pub social: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub suite: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub phone: String,
    pub dob: chrono::NaiveDate,
    pub company_id: Option<i32>,
    pub email: Option<String>,
    pub photo: Option<String>,
}

#[derive(Deserialize, Insertable)]
#[table_name = "students"]
pub struct NewStudent {
    pub social: String,
    pub first_name: String,
    pub last_name: String,
    pub address: String,
    pub suite: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub phone: String,
    pub dob: chrono::NaiveDate,
    pub company_id: Option<i32>,
    pub email: Option<String>,
    pub photo: Option<String>,
}
