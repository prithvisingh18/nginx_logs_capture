use actix_web::{web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Input {
    data: u32
}

async fn handle_post(data: web::Json<Input>) -> impl Responder {
    println!("Received number: {:?}", data);
    "OK"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the server");
    HttpServer::new(|| {
        App::new()
            .route("/post", web::post().to(handle_post))
    })
    .bind(("0.0.0.0", 8181))?
    .run()
    .await
}
