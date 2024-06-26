use axum::{routing::post, Router};
use common::web::axum_handler::handler_of;
use pulldepswithoverride::fs::foo_a_sfl;

#[tokio::main]
async fn main() {
    let foo_a_sfl_hdlr = handler_of(foo_a_sfl);

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
