use std::ops::Deref;

use actix_web::{web, App, HttpServer};
use pushdepstovar::{
    fs::{foo_a_sfl, FooAIn},
    fwk::{common_handler, foo_handler},
    startup::init_a_no_cache,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_a_no_cache();

    HttpServer::new(move || {
        let arc_f = common_handler(foo_a_sfl);
        let f = move |i| arc_f(i);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
