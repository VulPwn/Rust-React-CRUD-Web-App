use actix_web::{get, post, delete, put, web, Error, HttpResponse};
use crate::models;
use crate::Pool;

#[get("detailed-address-api-call/{input}")]
async fn detailed_address_api_call(
    pool: web::Data<Pool>,
    input: web::Path<String>,
    ) -> Result<HttpResponse, Error> {
    let input = input.into_inner();

    let address_search = web::block(move || {
        let conn = pool.get()?;
        super::make_detailed_address_api_call(&input)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(address_search) = address_search {
        Ok(HttpResponse::Ok().json(address_search))
    } else {
        let res = HttpResponse::NotFound().body(format!("Error calling address api"));
        Ok(res)
    }
}

#[get("address-api-call/{input}")]
async fn address_api_call(
    pool: web::Data<Pool>,
    input: web::Path<String>,
    ) -> Result<HttpResponse, Error> {
    let input = input.into_inner();

    let address_search = web::block(move || {
        let conn = pool.get()?;
        super::make_address_api_call(&input)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(address_search) = address_search {
        Ok(HttpResponse::Ok().json(address_search))
    } else {
        let res = HttpResponse::NotFound().body(format!("Error calling address api"));
        Ok(res)
    }
}

#[post("student/")]
async fn add_student(
    pool: web::Data<Pool>,
    form: web::Json<models::NewStudent>,
    ) -> Result<HttpResponse, Error> {
    let student = web::block(move || {
        let conn = pool.get()?;
        super::create_student(&conn, 
                              &form.social, 
                              &form.first_name,
                              &form.last_name,
                              &form.address,
                              &form.suite,
                              &form.city,
                              &form.state,
                              &form.zip_code,
                              &form.phone,
                              &form.dob,
                              &form.company_id,
                              &form.email,
                              &form.photo)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(student))
}

#[get("student/{student_id}")]
async fn get_student_by_id(
    pool: web::Data<Pool>,
    student_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let student_id = student_id.into_inner();

    let student = web::block(move || {
        let conn = pool.get()?;
        super::get_student(&conn, student_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        let res = HttpResponse::NotFound().body(format!("No student found with id: {}", student_id));
        Ok(res)
    }
}

#[put("student/{student_id}")]
async fn update_student_by_id(
    pool: web::Data<Pool>,
    student_id: web::Path<i32>,
    form: web::Json<models::NewStudent>,
    ) -> Result<HttpResponse, Error> {
    let student_id = student_id.into_inner();

    let student = web::block(move || {
        let conn = pool.get()?;
        super::update_student(&conn,
                              &student_id,
                              &form.social,
                              &form.first_name,
                              &form.last_name,
                              &form.address,
                              &form.suite,
                              &form.city,
                              &form.state,
                              &form.zip_code,
                              &form.phone,
                              &form.dob,
                              &form.company_id,
                              &form.email,
                              &form.photo)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(student))
}

#[delete("student/{student_id}")]
async fn delete_student_by_id(
    pool: web::Data<Pool>,
    student_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let student_id = student_id.into_inner();

    let student = web::block(move || {
        let conn = pool.get()?;
        super::delete_student(&conn, student_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(student) = student {
        Ok(HttpResponse::Ok().json(student))
    } else {
        let res = HttpResponse::NotFound().body(format!("No student found with id: {}", student_id));
        Ok(res)
    }
}

#[post("company/")]
async fn add_company(
    pool: web::Data<Pool>,
    form: web::Json<models::NewCompany>,
    ) -> Result<HttpResponse, Error> {
    let company = web::block(move || {
        let conn = pool.get()?;
        super::create_company(&conn, 
                              &form.company_name, 
                              &form.address,
                              &form.suite,
                              &form.city,
                              &form.state,
                              &form.zip_code,
                              &form.phone,
                              &form.phone_ext,
                              &form.email,
                              &form.poc_firstname,
                              &form.poc_lastname,
                              &form.cc_holdername,
                              &form.cc_num,
                              &form.cc_expdate,
                              &form.cc_cvv,
                              &form.cc_zipcode,
                              &form.notes)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(company))
}

#[get("company/{company_id}")]
async fn get_company_by_id(
    pool: web::Data<Pool>,
    company_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let company_id = company_id.into_inner();

    let company = web::block(move || {
        let conn = pool.get()?;
        super::get_company(&conn, company_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(company) = company {
        Ok(HttpResponse::Ok().json(company))
    } else {
        let res = HttpResponse::NotFound().body(format!("No company found with id: {}", company_id));
        Ok(res)
    }
}

#[put("company/{company_id}")]
async fn update_company_by_id(
    pool: web::Data<Pool>,
    company_id: web::Path<i32>,
    form: web::Json<models::NewCompany>,
    ) -> Result<HttpResponse, Error> {
    let company_id = company_id.into_inner();

    let company = web::block(move || {
        let conn = pool.get()?;
        super::update_company(&conn,
                              &company_id,
                              &form.company_name,
                              &form.address,
                              &form.suite,
                              &form.city,
                              &form.state,
                              &form.zip_code,
                              &form.phone,
                              &form.phone_ext,
                              &form.email,
                              &form.poc_firstname,
                              &form.poc_lastname,
                              &form.cc_holdername,
                              &form.cc_num,
                              &form.cc_expdate,
                              &form.cc_cvv,
                              &form.cc_zipcode,
                              &form.notes)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(company))
}

#[delete("company/{company_id}")]
async fn delete_company_by_id(
    pool: web::Data<Pool>,
    company_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let company_id = company_id.into_inner();

    let company = web::block(move || {
        let conn = pool.get()?;
        super::delete_company(&conn, company_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(company) = company {
        Ok(HttpResponse::Ok().json(company))
    } else {
        let res = HttpResponse::NotFound().body(format!("No company found with id: {}", company_id));
        Ok(res)
    }
}

#[post("class-type/")]
async fn add_class_type(
    pool: web::Data<Pool>,
    form: web::Json<models::NewClass>,
    ) -> Result<HttpResponse, Error> {
    let class_type = web::block(move || {
        let conn = pool.get()?;
        super::create_class(&conn, 
                            &form.class_title, 
                            &form.class_language,
                            &form.md_approval_num,
                            &form.va_approval_num,
                            &form.dc_approval_num,
                            &form.md_recert_yrs,
                            &form.va_recert_yrs,
                            &form.dc_recert_yrs,
                            &form.hours)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(class_type))
}

#[get("class-type/all")]
async fn get_class_type_all(
    pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
    let class_type_all = web::block(move || {
        let conn = pool.get()?;
        super::get_class_all(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(class_type_all) = class_type_all {
        Ok(HttpResponse::Ok().json(class_type_all))
    } else {
        let res = HttpResponse::NotFound().body(format!("Error retrieving all class types..."));
        Ok(res)
    }
}

#[get("class-type/{class_type_id}")]
async fn get_class_type_by_id(
    pool: web::Data<Pool>,
    class_type_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let class_type_id = class_type_id.into_inner();

    let class_type = web::block(move || {
        let conn = pool.get()?;
        super::get_class(&conn, class_type_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(class_type) = class_type {
        Ok(HttpResponse::Ok().json(class_type))
    } else {
        let res = HttpResponse::NotFound().body(format!("No class type found with id: {}", class_type_id));
        Ok(res)
    }
}

#[put("class-type/{class_type_id}")]
async fn update_class_type_by_id(
    pool: web::Data<Pool>,
    class_type_id: web::Path<i32>,
    form: web::Json<models::NewClass>,
    ) -> Result<HttpResponse, Error> {
    let class_type_id = class_type_id.into_inner();

    let class_type = web::block(move || {
        let conn = pool.get()?;
        super::update_class(&conn,
                            &class_type_id,
                            &form.class_title,
                            &form.class_language,
                            &form.md_approval_num,
                            &form.va_approval_num,
                            &form.dc_approval_num,
                            &form.md_recert_yrs,
                            &form.va_recert_yrs,
                            &form.dc_recert_yrs,
                            &form.hours)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(class_type))
}

#[delete("class-type/{class_type_id}")]
async fn delete_class_type_by_id(
    pool: web::Data<Pool>,
    class_type_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let class_type_id = class_type_id.into_inner();

    let class_type = web::block(move || {
        let conn = pool.get()?;
        super::delete_class(&conn, class_type_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(class_type) = class_type {
        Ok(HttpResponse::Ok().json(class_type))
    } else {
        let res = HttpResponse::NotFound().body(format!("No class type found with id: {}", class_type_id));
        Ok(res)
    }
}

#[post("class/")]
async fn add_class_entry(
    pool: web::Data<Pool>,
    form: web::Json<models::NewClassEntry>,
    ) -> Result<HttpResponse, Error> {
    let class_entry = web::block(move || {
        let conn = pool.get()?;
        super::create_class_entry(&conn, 
                                  &form.class_id, 
                                  &form.class_date,
                                  &form.class_end_date,
                                  &form.md_recert_date,
                                  &form.va_recert_date,
                                  &form.dc_recert_date,
                                  &form.instructor_id,
                                  &form.second_instructor_id,
                                  &form.cancelled)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(class_entry))
}

#[get("class/{class_entry_id}/students")]
async fn get_students_by_class_entry_id(
    pool: web::Data<Pool>,
    class_entry_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let class_entry_id = class_entry_id.into_inner();

    let class_entry = web::block(move || {
        let conn = pool.get()?;
        super::get_studentclass_entries_for_class_entry(&conn, class_entry_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(class_entry) = class_entry {
        Ok(HttpResponse::Ok().json(class_entry))
    } else {
        let res = HttpResponse::NotFound().body(format!("No class found with id: {}", class_entry_id));
        Ok(res)
    }
}

#[get("class/{class_entry_id}")]
async fn get_class_entry_by_id(
    pool: web::Data<Pool>,
    class_entry_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let class_entry_id = class_entry_id.into_inner();

    let class_entry = web::block(move || {
        let conn = pool.get()?;
        super::get_class_entry(&conn, class_entry_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(class_entry) = class_entry {
        Ok(HttpResponse::Ok().json(class_entry))
    } else {
        let res = HttpResponse::NotFound().body(format!("No class found with id: {}", class_entry_id));
        Ok(res)
    }
}

#[put("class/{class_entry_id}")]
async fn update_class_entry_by_id(
    pool: web::Data<Pool>,
    class_entry_id: web::Path<i32>,
    form: web::Json<models::NewClassEntry>,
    ) -> Result<HttpResponse, Error> {
    let class_entry_id = class_entry_id.into_inner();

    let class_entry = web::block(move || {
        let conn = pool.get()?;
        super::update_class_entry(&conn,
                                  &class_entry_id,
                                  &form.class_id, 
                                  &form.class_date,
                                  &form.class_end_date,
                                  &form.md_recert_date,
                                  &form.va_recert_date,
                                  &form.dc_recert_date,
                                  &form.instructor_id,
                                  &form.second_instructor_id,
                                  &form.cancelled)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(class_entry))
}

#[delete("class/{class_entry_id}")]
async fn delete_class_entry_by_id(
    pool: web::Data<Pool>,
    class_entry_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let class_entry_id = class_entry_id.into_inner();

    let class_entry = web::block(move || {
        let conn = pool.get()?;
        super::delete_class_entry(&conn, class_entry_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(class_entry) = class_entry {
        Ok(HttpResponse::Ok().json(class_entry))
    } else {
        let res = HttpResponse::NotFound().body(format!("No class found with id: {}", class_entry_id));
        Ok(res)
    }
}

#[post("instructor/")]
async fn add_instructor(
    pool: web::Data<Pool>,
    form: web::Json<models::NewInstructor>,
    ) -> Result<HttpResponse, Error> {
    let instructor = web::block(move || {
        let conn = pool.get()?;
        super::create_instructor(&conn, 
                                 &form.acronym, 
                                 &form.first_name,
                                 &form.last_name)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(instructor))
}

#[get("instructor/all")]
async fn get_instructor_all(
    pool: web::Data<Pool>,
    ) -> Result<HttpResponse, Error> {
    let instructor_all = web::block(move || {
        let conn = pool.get()?;
        super::get_instructor_all(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(instructor_all) = instructor_all {
        Ok(HttpResponse::Ok().json(instructor_all))
    } else {
        let res = HttpResponse::NotFound().body(format!("Error retrieving all instructors..."));
        Ok(res)
    }
}

#[get("instructor/{instructor_id}")]
async fn get_instructor_by_id(
    pool: web::Data<Pool>,
    instructor_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let instructor_id = instructor_id.into_inner();

    let instructor = web::block(move || {
        let conn = pool.get()?;
        super::get_instructor(&conn, instructor_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(instructor) = instructor {
        Ok(HttpResponse::Ok().json(instructor))
    } else {
        let res = HttpResponse::NotFound().body(format!("No instructor found with id: {}", instructor_id));
        Ok(res)
    }
}

#[put("instructor/{instructor_id}")]
async fn update_instructor_by_id(
    pool: web::Data<Pool>,
    instructor_id: web::Path<i32>,
    form: web::Json<models::NewInstructor>,
    ) -> Result<HttpResponse, Error> {
    let instructor_id = instructor_id.into_inner();

    let instructor = web::block(move || {
        let conn = pool.get()?;
        super::update_instructor(&conn,
                                 &instructor_id,
                                 &form.acronym,
                                 &form.first_name,
                                 &form.last_name)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(instructor))
}

#[delete("instructor/{instructor_id}")]
async fn delete_instructor_by_id(
    pool: web::Data<Pool>,
    instructor_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let instructor_id = instructor_id.into_inner();

    let instructor = web::block(move || {
        let conn = pool.get()?;
        super::delete_instructor(&conn, instructor_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(instructor) = instructor {
        Ok(HttpResponse::Ok().json(instructor))
    } else {
        let res = HttpResponse::NotFound().body(format!("No instructor found with id: {}", instructor_id));
        Ok(res)
    }
}

#[post("student-class/")]
async fn add_studentclass(
    pool: web::Data<Pool>,
    form: web::Json<models::NewStudentClass>,
    ) -> Result<HttpResponse, Error> {
    let studentclass = web::block(move || {
        let conn = pool.get()?;
        super::create_studentclass_entry(&conn, 
                                         &form.class_id, 
                                         &form.student_id, 
                                         &form.certification_num,
                                         &form.test_score,
                                         &form.class_date,
                                         &form.class_end_date,
                                         &form.md_recert_date,
                                         &form.va_recert_date,
                                         &form.dc_recert_date)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(studentclass))
}

#[get("student-class/{studentclass_id}")]
async fn get_studentclass_by_id(
    pool: web::Data<Pool>,
    studentclass_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let studentclass_id = studentclass_id.into_inner();

    let studentclass = web::block(move || {
        let conn = pool.get()?;
        super::get_studentclass_entry(&conn, studentclass_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(studentclass) = studentclass {
        Ok(HttpResponse::Ok().json(studentclass))
    } else {
        let res = HttpResponse::NotFound().body(format!("No student-class entry found with id: {}", studentclass_id));
        Ok(res)
    }
}

#[put("student-class/{studentclass_id}")]
async fn update_studentclass_by_id(
    pool: web::Data<Pool>,
    studentclass_id: web::Path<i32>,
    form: web::Json<models::NewStudentClass>,
    ) -> Result<HttpResponse, Error> {
    let studentclass_id = studentclass_id.into_inner();

    let studentclass = web::block(move || {
        let conn = pool.get()?;
        super::update_studentclass_entry(&conn,
                                         &studentclass_id,
                                         &form.class_id, 
                                         &form.student_id, 
                                         &form.certification_num,
                                         &form.test_score,
                                         &form.class_date,
                                         &form.class_end_date,
                                         &form.md_recert_date,
                                         &form.va_recert_date,
                                         &form.dc_recert_date)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(studentclass))
}

#[delete("student-class/{studentclass_id}")]
async fn delete_studentclass_by_id(
    pool: web::Data<Pool>,
    studentclass_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let studentclass_id = studentclass_id.into_inner();

    let studentclass = web::block(move || {
        let conn = pool.get()?;
        super::delete_studentclass_entry(&conn, studentclass_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(studentclass) = studentclass {
        Ok(HttpResponse::Ok().json(studentclass))
    } else {
        let res = HttpResponse::NotFound().body(format!("No student-class entry found with id: {}", studentclass_id));
        Ok(res)
    }
}

#[get("user/{user_id}")]
async fn get_user(
    pool: web::Data<Pool>,
    user_id: web::Path<i32>,
    ) -> Result<HttpResponse, Error> {
    let user_id = user_id.into_inner();

    let user = web::block(move || {
        let conn = pool.get()?;
        super::get_user(&conn, user_id)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    if let Some(user) = user {
        Ok(HttpResponse::Ok().json(user))
    } else {
        let res = HttpResponse::NotFound().body(format!("No user found with id: {}", user_id));
        Ok(res)
    }
}

