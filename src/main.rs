mod token;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//use token::tokenize;

/// Health checkpoint to see if the api is online
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Handler for storing a message in the database
#[post("/messages")]
async fn post_message(message: String) -> impl Responder {
    // tokenize the message
    //let token = tokenize(message);
    // store message in the database
    HttpResponse::Ok().body("Message posted!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index).service(post_message))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
