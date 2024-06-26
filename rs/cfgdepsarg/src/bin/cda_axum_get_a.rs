use axum::{routing::post, Router};
use cfgdepsarg::startup::get_foo_a_sfl_refreshable;
use common::{config::refresh_app_configuration, web::axum_handler::handler_of};
use std::{thread, time::Duration};

#[tokio::main]
async fn main() {
    let foo_a_sfl = get_foo_a_sfl_refreshable();

    let foo_a_sfl_hdlr = handler_of(foo_a_sfl);

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let _ = thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));
        refresh_app_configuration();
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
