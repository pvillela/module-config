use actix_web::{web, App, HttpServer};
use common::{
    fs_data::{BarBfCfgInfo, FooSflCfgInfo},
    fwk::{RefreshMode, Src},
    web::actix_handler::handler_of_web,
};
use pushtofunction::fs::{
    bar_aw_bf_c, foo_aw_sfl_c, BarAwBfCfgDeps, FooAwSflCfgDeps, FooAwSflDeps,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let foo_aw_sfl_cfg_info = FooSflCfgInfo {
            a: "foo_aw_test1".to_owned(),
            b: 1,
        };
        let bar_aw_bf_cfg_info = BarBfCfgInfo {
            u: 11,
            v: "bar_a_test1".to_owned(),
        };

        let bar_aw_cfg_deps = BarAwBfCfgDeps::new(
            Src::new_boxed(move || bar_aw_bf_cfg_info.clone()),
            RefreshMode::NoRefresh,
            (),
        );

        let bar_aw_bf = bar_aw_bf_c(bar_aw_cfg_deps);

        let foo_aw_cfg_deps = FooAwSflCfgDeps::new(
            Src::new_boxed(move || foo_aw_sfl_cfg_info.clone()),
            RefreshMode::NoRefresh,
            FooAwSflDeps { bar_aw_bf },
        );

        let foo_aw_sfl = foo_aw_sfl_c(foo_aw_cfg_deps);
        let arc_f = handler_of_web(foo_aw_sfl);
        let f = move |i| arc_f(i);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4) // default is the numbe of physical CPUs
    .run()
    .await
}
