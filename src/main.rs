mod token;
mod vector;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use token::tokenize;
use vector::{Message, VectorsDb};

/// Health checkpoint to see if the api is online
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Handler for storing a message in the database
#[post("/messages")]
async fn post_message(message: String, db: web::Data<VectorsDb>) -> impl Responder {
    // tokenize the message
    let token_result = tokenize(message);
    match token_result {
        Ok(tokens) => {
            // store message in the database
            let message = Message { tokens };
            db.add_message(message.clone());
            HttpResponse::Ok().body("Message posted!")
        }
        Err(error) => {
            // return HTTP response indicating that tokenization failed
            HttpResponse::BadRequest().body(error.to_string())
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(VectorsDb::new());
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(index)
            .service(post_message)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
