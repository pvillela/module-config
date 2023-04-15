use axum::{routing::post, Router};
use common::web::axum_handler::handler_of;
use pushdepstovar::{fs::foo_a_sfl, startup::init_a_refreshable};

#[tokio::main]
async fn main() {
    init_a_refreshable(0);

    let foo_a_sfl_hdlr = handler_of(foo_a_sfl);

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let addr = ([127, 0, 0, 1], 8080).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
