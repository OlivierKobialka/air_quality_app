use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    counter: i64,
}

#[get("/")]
async fn index() -> String {
    "Hello world!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(Mutex::new(AppState { counter: 0 }));

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}