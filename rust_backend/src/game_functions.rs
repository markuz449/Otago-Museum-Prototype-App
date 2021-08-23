use rand::Rng;
use diesel::prelude::*;
use diesel::query_dsl::QueryDsl;
use diesel::query_dsl::RunQueryDsl;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use actix_web::{HttpResponse, Result, web};
use crate::models::Question;
use crate::schema::questions::dsl::*;

/* /api/check-answer/{id} - GET */
pub async fn get_question() -> Result<HttpResponse> {
    let connection = establish_connection();
    let question_list = questions.load::<Question>(&connection)
        .expect("Error loading questions");
    let mut rng = rand::thread_rng();
    let select = rng.gen_range(0..question_list.len());
    
    Ok(HttpResponse::Ok().json(question_list.get(select)))
}

/* /api/check-answer/{id} - POST */
pub async fn check_answer(path_id: web::Path<i32>, answer: web::Json<String>) -> Result<HttpResponse> {
    let search_id: i32 = path_id.0;
    let connection = establish_connection();
    let get_question: Question = questions.find(search_id)
        .first(&connection)
        .expect("Failed to get question");

    let mut response = "incorrect";
    if answer.0.eq(&get_question.correct_answer){
        response = "correct";
    }
    Ok(HttpResponse::Ok().json(response)) // <- send response
}

// Connects to the Postgres Server
pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}