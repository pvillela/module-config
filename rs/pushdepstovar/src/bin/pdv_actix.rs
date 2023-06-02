use actix_web::{web, App, HttpServer};
use common::web::actix_handler::handler_of;
use pushdepstovar::startup::get_foo_a_sfl_no_refresh;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let foo_a_sfl = get_foo_a_sfl_no_refresh();

    HttpServer::new(move || {
        let f = handler_of(foo_a_sfl);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
