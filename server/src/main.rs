use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    web, App, HttpResponse, HttpServer,
};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

struct AppState {
    counter: i64,
}

// ========================== GET ==========================
#[get("/")]
async fn index() -> String {
    "Hello world!".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data: web::Data<Mutex<AppState>> = web::Data::new(Mutex::new(AppState { counter: 0 }));
    println!("[server] running at http://127.0.0.1:8080");

    HttpServer::new(move || App::new().app_data(app_data.clone()).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
