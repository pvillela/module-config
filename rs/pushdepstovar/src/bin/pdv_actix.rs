use actix_web::{web, App, HttpServer};
use pushdepstovar::{fwk::foo_handler, startup::init_a_no_cache};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_a_no_cache();

    HttpServer::new(|| App::new().route("/", web::post().to(foo_handler)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
