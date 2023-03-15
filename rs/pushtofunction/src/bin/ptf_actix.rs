use actix_web::{web, App, HttpServer};
use common::{
    fs_data::{BarBfCfgInfo, FooSflCfgInfo},
    fwk::RefreshMode,
    web::handler_of_boxed_web,
};
use pushtofunction::fs::{bar_a_bf_c, foo_aw_sfl_c, BarABfCfgDeps, FooAwSflCfgDeps, FooAwSflDeps};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let foo_sfl_cfg_info = FooSflCfgInfo {
            a: "foo_aw_test1".to_owned(),
            b: 1,
        };
        let bar_bf_cfg_info = BarBfCfgInfo {
            u: 11,
            v: "bar_a_test1".to_owned(),
        };

        let bar_cfg_deps = BarABfCfgDeps::new(
            move || bar_bf_cfg_info.clone().into(),
            RefreshMode::NoRefresh,
            (),
        );

        let bar_a_bf = bar_a_bf_c(bar_cfg_deps);

        let foo_cfg_deps = FooAwSflCfgDeps::new(
            move || foo_sfl_cfg_info.clone().into(),
            RefreshMode::NoRefresh,
            FooAwSflDeps { bar_a_bf },
        );

        let foo_aw_sfl = foo_aw_sfl_c(foo_cfg_deps);
        let arc_f = handler_of_boxed_web(foo_aw_sfl);
        let f = move |i| arc_f(i);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
