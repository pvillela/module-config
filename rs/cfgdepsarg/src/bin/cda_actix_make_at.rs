use actix_web::{web, App, HttpServer};
use cfgdepsarg::startup::make_foo_at_sfl_no_refresh;
use common::web::actix_handler::handler_of_boxpin;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let foo_at_sfl = make_foo_at_sfl_no_refresh();
        let f = handler_of_boxpin(foo_at_sfl);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4) // default is the numbe of physical CPUs
    .run()
    .await
}
