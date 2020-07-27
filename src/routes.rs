use actix_web::{web, HttpResponse, Responder};
use actix_web_codegen::{get, post};
use influx_db_client::{Client, Point, Points, Value, Precision};


use crate::models;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[post("/event")]
async fn event(json: web::Json<models::Event>) -> impl Responder {
    let client = Client::default().set_authentication("root", "root");

    error!(
        "Parsed data: user: {}, event name: {}",
        json.user, json.event_name
    );
    HttpResponse::Ok().body("POST")
}
