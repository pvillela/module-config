use actix_web::{web, App, HttpServer};
use cfgdepsarg_r::fs::{
    bar_aw_bf_c, foo_aw_sfl_c, BarAwBfCfg, BarAwBfS, FooAwSflCfg, FooAwSflDeps, FooAwSflS,
};
use common::{
    fs_data::{BarAwBfCfgInfo, FooAwSflCfgInfo},
    fwk::{box_pin_async_fn_wss, RefreshMode, Src},
    web::actix_handler::handler_of_web,
};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let bar_aw_bf = |sleep_millis| {
            let bar_aw_bf_cfg_info = BarAwBfCfgInfo {
                u: 11,
                v: "bar_aw_test1".to_owned(),
            };

            let bar_aw_cfg = BarAwBfCfg::new(
                Src::new_boxed(move || bar_aw_bf_cfg_info.clone()),
                RefreshMode::NoRefresh,
            );
            let bar_aw_s = Arc::new(BarAwBfS {
                cfg: bar_aw_cfg,
                deps: (),
            });

            bar_aw_bf_c(bar_aw_s, sleep_millis)
        };

        let foo_aw_sfl = move |input| {
            let foo_aw_sfl_cfg_info = FooAwSflCfgInfo {
                a: "foo_aw_test1".to_owned(),
                b: 1,
            };

            let foo_aw_cfg = FooAwSflCfg::new(
                Src::new_boxed(move || foo_aw_sfl_cfg_info.clone()),
                RefreshMode::NoRefresh,
            );

            let foo_aw_deps = FooAwSflDeps {
                bar_aw_bf: box_pin_async_fn_wss(bar_aw_bf),
            };

            let foo_aw_s = Arc::new(FooAwSflS {
                cfg: foo_aw_cfg,
                deps: foo_aw_deps,
            });

            foo_aw_sfl_c(foo_aw_s, input)
        };

        let f = handler_of_web(foo_aw_sfl);
        App::new().route("/", web::post().to(f))
    })
    .bind(("127.0.0.1", 8080))?
    .workers(4) // default is the numbe of physical CPUs
    .run()
    .await
}
