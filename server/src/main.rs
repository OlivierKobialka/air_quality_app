use actix_web::{
    error, get,
    http::{header::ContentType, StatusCode},
    web, App, HttpResponse, HttpServer, Responder,
};
use derive_more::{Display, Error};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Mutex;

mod not_found;

struct AppState {
    counter: i64,
}

const HOST: &str = "127.0.0.1";
const PORT: u16 = 8080;
const PV_KEY: &str = "f22d8827-b111-445f-bf47-68299ced2d2f";

#[derive(Debug, Display, Error)]
enum UserError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
}

impl error::ResponseError for UserError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            UserError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

// ========================== GET ==========================
#[get("/")]
async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data: web::Data<Mutex<AppState>> = web::Data::new(Mutex::new(AppState { counter: 0 }));
    println!("[server] running at http://{}:{}", HOST, PORT);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            // .route("/hey", web::get().to(manual_hello))
            // .service(
            //     web::scope("/operation")
            //         .service(operation::multiply)
            //         .service(operation::add),
            // )
            .default_service(web::route().to(not_found::not_found))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
