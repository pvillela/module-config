use actix_web::{web, App, HttpServer};
use common::{config::get_app_configuration, fwk::RefreshMode, web::actix_handler::handler_of};
use pushdepstovar::fs::boot::get_foo_a_sfl;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let foo_a_sfl = get_foo_a_sfl(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(0)),
    );

    HttpServer::new(move || {
        let f = handler_of(foo_a_sfl);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
