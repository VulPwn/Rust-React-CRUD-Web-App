extern crate dakotadb;
extern crate diesel;

use self::dakotadb::*;
use std::io::stdin;
use std::fs;

fn main() {
    let connection = establish_connection();
 
    let file = fs::read_to_string("/home/vulpwn/docs/dakotadb_csv/company_poc_rearranged.tsv").unwrap();
    let lines = file.split("\n");
    
    
    println!("Enter how many classes you plan to add: ");
    let mut iter = String::new();
    stdin().read_line(&mut iter).unwrap();
    let mut iter = &iter[..(iter.len() - 1)];
    let iter = iter.to_string().parse::<i32>().unwrap();

    for i in 0..iter {
        println!("Enter a class_title: ");
        let mut class_title = String::new();
        stdin().read_line(&mut class_title).unwrap();
        let class_title = &class_title[..(class_title.len() - 1)];
        println!("Enter a class_language: ");
        let mut class_language = String::new();
        stdin().read_line(&mut class_language).unwrap();
        let class_language = &class_language[..(class_language.len() - 1)];
        println!("\nEnter a md_approval_num: ");
        let mut md_num = String::new();
        stdin().read_line(&mut md_num).unwrap();
        let md_num = &md_num[..(md_num.len() - 1)];
        println!("\nEnter a va_approval_num: ");
        let mut va_num = String::new();
        stdin().read_line(&mut va_num).unwrap();
        let va_num = &va_num[..(va_num.len() - 1)];
        println!("\nEnter a dc_approval_num: ");
        let mut dc_num = String::new();
        stdin().read_line(&mut dc_num).unwrap();
        let dc_num = &dc_num[..(dc_num.len() - 1)];
        println!("\nEnter a md_recert_yrs: ");
        let mut md_yrs = String::new();
        stdin().read_line(&mut md_yrs).unwrap();
        let mut md_yrs = &md_yrs[..(md_yrs.len() - 1)];
        let md_yrs = md_yrs.to_string().parse::<i32>().unwrap();
        println!("\nEnter a va_recert_yrs: ");
        let mut va_yrs = String::new();
        stdin().read_line(&mut va_yrs).unwrap();
        let mut va_yrs = &va_yrs[..(va_yrs.len() - 1)];
        let va_yrs = va_yrs.to_string().parse::<i32>().unwrap();
        println!("\nEnter a dc_recert_yrs: ");
        let mut dc_yrs = String::new();
        stdin().read_line(&mut dc_yrs).unwrap();
        let mut dc_yrs = &dc_yrs[..(dc_yrs.len() - 1)];
        let dc_yrs = dc_yrs.to_string().parse::<i32>().unwrap();

        let _class = create_class(&connection, 
                                    class_title, 
                                    class_language,
                                    Some(md_num),
                                    Some(va_num),
                                    Some(dc_num),
                                    Some(&md_yrs),
                                    Some(&va_yrs),
                                    Some(&dc_yrs));
    };
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

