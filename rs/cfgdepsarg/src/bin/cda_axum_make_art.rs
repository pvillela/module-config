use axum::{routing::post, Router};
use cfgdepsarg::startup::make_foo_art_sfl;
use common::config::refresh_app_configuration;
use common::web::axum_handler::handler_of_boxpin;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    // Need to convert Box to Arc in order for Axum to work because Axum requires handler closures
    // to be Clone.
    let foo_art_sfl = make_foo_art_sfl();

    let foo_art_sfl_hdlr = handler_of_boxpin(foo_art_sfl);

    let app = Router::new().route("/", post(foo_art_sfl_hdlr));

    let _ = thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));
        refresh_app_configuration();
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
