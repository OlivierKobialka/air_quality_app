use actix_web::{http, HttpRequest, HttpResponse, Responder};
use serde_json::json;

pub async fn not_found(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .status(http::StatusCode::NOT_FOUND)
        .content_type("application/json")
        .body(json!({ "url": &req.uri().to_string() }).to_string())
}
