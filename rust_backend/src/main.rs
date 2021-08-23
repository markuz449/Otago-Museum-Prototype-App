#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;
mod game_functions;

use actix_web::{App, HttpServer, web};
use actix_cors::Cors;
use game_functions::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("{}", "Starting Rust API");
    HttpServer::new(|| {
        // Original Cors
        /* let cors = Cors::default()
             .allowed_origin("http://localhost:6969")
             .allowed_methods(vec!["POST", "PUT", "PATCH", "GET", "OPTIONS", "HEAD"]);*/
        // Localhost Cors
        let cors = Cors::permissive();
        App::new()
        .wrap(cors)
        .service(
            web::scope("/api")
                .route("/question", web::get().to(get_question))
                .route("/check-answer/{id}", web::post().to(check_answer))
        )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}