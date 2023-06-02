use axum::{routing::post, Router};
use common::web::axum_handler::handler_of;
use pushdepstovar::startup::get_foo_a_sfl_no_refresh;

#[tokio::main]
async fn main() {
    let foo_a_sfl = get_foo_a_sfl_no_refresh();

    let foo_a_sfl_hdlr = handler_of(foo_a_sfl);

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let addr = ([127, 0, 0, 1], 8080).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
