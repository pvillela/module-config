use axum::{routing::post, Router};
use cfgdepsarg::startup::get_foo_at_sfl_refreshable;
use common::{config::refresh_app_configuration, web::axum_handler::handler_of};
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    let foo_at_sfl = get_foo_at_sfl_refreshable();

    let foo_at_sfl_hdlr = handler_of(foo_at_sfl);

    let app = Router::new().route("/", post(foo_at_sfl_hdlr));

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
