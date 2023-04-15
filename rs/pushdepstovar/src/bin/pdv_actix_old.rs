use actix_web::{web, App, HttpServer};
use common::web::actix_handler::handler_arc_of;
use pushdepstovar::{fs::foo_a_sfl, startup::init_a_refreshable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_a_refreshable(0);

    HttpServer::new(move || {
        let arc_f = handler_arc_of(foo_a_sfl);
        let f = move |i| arc_f(i);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
