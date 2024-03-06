use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello from Rust server!")
}

#[actix_web::main] // Marks the async main function to be run by the actix runtime
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet)) // Define a route and assign the `greet` function to handle GET requests
    })
    .bind("127.0.0.1:8080")? // Specify the address and port to serve on
    .run() // Start the server
    .await
}