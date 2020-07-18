#[macro_use]
extern crate log;

use actix_web::body::Body;
use actix_web::dev;
use actix_web::error;
use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::{http, middleware, App, HttpServer};

mod models;
mod routes;

fn handle_bad_request<B>(
    mut res: dev::ServiceResponse<B>,
) -> error::Result<ErrorHandlerResponse<Body>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("application/json"),
    );
    let error_msg: String = match res.response().error() {
        Some(e) => format!("{:?}", e.as_response_error()),
        None => String::from("Unknown error"),
    };
    let new_res: dev::ServiceResponse<Body> =
        res.map_body(|_head, _body| dev::ResponseBody::Other(Body::from_message(error_msg)));
    Ok(ErrorHandlerResponse::Response(new_res))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "error,actix_web=debug");
    env_logger::init();
    dotenv::dotenv().ok();

    let bind = "127.0.0.1:8888";
    println!("Starting server at {}", &bind);

    HttpServer::new(|| {
        App::new()
            .wrap(ErrorHandlers::new().handler(http::StatusCode::BAD_REQUEST, handle_bad_request))
            .wrap(middleware::Logger::default())
            .service(routes::index)
            .service(routes::event)
    })
    .bind(&bind)?
    .run()
    .await
}
