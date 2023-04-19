mod token;
mod vector;
use actix_web::{delete, get, post, web, App, HttpResponse, HttpServer, Responder};
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

/// Handler for retrieving all messages from the database
#[get("/messages")]
async fn get_messages(db: web::Data<VectorsDb>) -> impl Responder {
    let messages = db.get_messages();
    HttpResponse::Ok().json(messages)
}

/// Handler for deleting a specific message in the database
#[delete("/messages")]
async fn delete_message(db: web::Data<VectorsDb>, message: web::Json<Message>) -> impl Responder {
    match db.delete_message(&message) {
        Ok(_) => HttpResponse::Ok().body("Message deleted!"),
        Err(_) => HttpResponse::NotFound().body("Message not found!"),
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
            .service(get_messages)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
