use axum::{routing::post, Router};
use cfgdepsarg_r::startup::make_foo_a_sfl_refreshable;
use common::config::refresh_app_configuration;
use common::web::axum_handler::handler_of_boxpin;
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    // Need to convert Box to Arc in order for Axum to work because Axum requires handler closures
    // to be Clone.
    let foo_a_sfl = make_foo_a_sfl_refreshable();

    let foo_a_sfl_hdlr = handler_of_boxpin(foo_a_sfl);

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let addr = ([127, 0, 0, 1], 8080).into();

    let _ = thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));
        refresh_app_configuration();
    });

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
