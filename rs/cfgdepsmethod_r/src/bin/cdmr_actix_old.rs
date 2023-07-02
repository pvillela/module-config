use actix_web::{web, App, HttpServer};
use cfgdepsmethods::fs::boot::get_foo_a_sfl_s_no_refresh;
use common::web::actix_handler::handler_arc_of;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let foo_a_sfl_s = get_foo_a_sfl_s_no_refresh();
    let foo_a_sfl = |input| foo_a_sfl_s.run(input);

    HttpServer::new(move || {
        let arc_f = handler_arc_of(foo_a_sfl);
        let f = move |i| arc_f(i);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
