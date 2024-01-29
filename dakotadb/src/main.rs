use actix_web::{dev::ServiceRequest, middleware, web, App, Error, HttpServer};
use actix_cors::Cors;
use dakotadb::establish_connection;
use dakotadb::handlers;
use std::env;
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug, actix_server=info");
    env_logger::init();

    let pool = establish_connection().unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(handlers::detailed_address_api_call)
            .service(handlers::address_api_call)
            .service(handlers::add_student)
            .service(handlers::get_student_by_id)
            .service(handlers::update_student_by_id)
            .service(handlers::delete_student_by_id)
            .service(handlers::add_company)
            .service(handlers::get_company_by_id)
            .service(handlers::update_company_by_id)
            .service(handlers::delete_company_by_id)
            .service(handlers::add_class_type)
            .service(handlers::get_class_type_all)
            .service(handlers::get_class_type_by_id)
            .service(handlers::update_class_type_by_id)
            .service(handlers::delete_class_type_by_id)
            .service(handlers::add_class_entry)
            .service(handlers::get_students_by_class_entry_id)
            .service(handlers::get_class_entry_by_id)
            .service(handlers::update_class_entry_by_id)
            .service(handlers::delete_class_entry_by_id)
            .service(handlers::add_instructor)
            .service(handlers::get_instructor_all)
            .service(handlers::get_instructor_by_id)
            .service(handlers::update_instructor_by_id)
            .service(handlers::delete_instructor_by_id)
            .service(handlers::add_studentclass)
            .service(handlers::get_studentclass_by_id)
            .service(handlers::update_studentclass_by_id)
            .service(handlers::delete_studentclass_by_id)
            .service(handlers::get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
