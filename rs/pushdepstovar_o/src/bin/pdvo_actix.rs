use actix_web::{web, App, HttpServer};
use pushdepstovar_o::{fs::foo_a_sfl, fwk::handler_of, startup::init_a_refreshable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_a_refreshable();

    HttpServer::new(move || {
        let arc_f = handler_of(foo_a_sfl);
        let f = move |i| arc_f(i);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
