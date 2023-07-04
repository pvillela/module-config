use axum::{routing::post, Router};
use cfgdepsarg_r::{
    fs::{bar_a_bf_c, foo_a_sfl_c, BarABfCfg, BarABfS, FooASflCfg, FooASflDeps, FooASflS},
    startup::{make_foo_a_sfl_no_refresh, make_foo_a_sfl_refreshable},
};
use common::{
    config::refresh_app_configuration,
    fs_data::{BarBfCfgInfo, FooSflCfgInfo},
    fwk::{RefreshMode, Src},
    ref_pin_async_fn,
    web::axum_handler::handler_of,
};
use std::{sync::Arc, thread, time::Duration};

#[tokio::main]
async fn main() {
    let bar_a_bf = |sleep_millis| {
        let bar_a_bf_cfg_info = BarBfCfgInfo {
            u: 11,
            v: "bar_a_test1".to_owned(),
        };

        let bar_a_cfg = BarABfCfg::new(
            Src::new_boxed(move || bar_a_bf_cfg_info.clone()),
            RefreshMode::NoRefresh,
        );
        let bar_a_s = Arc::new(BarABfS {
            cfg: bar_a_cfg,
            deps: (),
        });

        bar_a_bf_c(bar_a_s, sleep_millis)
    };

    let foo_a_sfl = move |input| {
        let foo_a_sfl_cfg_info = FooSflCfgInfo {
            a: "foo_aw_test1".to_owned(),
            b: 1,
        };

        let foo_a_cfg = FooASflCfg::new(
            Src::new_boxed(move || foo_a_sfl_cfg_info.clone()),
            RefreshMode::NoRefresh,
        );

        let foo_a_deps = FooASflDeps {
            bar_a_bf: ref_pin_async_fn!(bar_a_bf),
        };

        let foo_a_s = Arc::new(FooASflS {
            cfg: foo_a_cfg,
            deps: foo_a_deps,
        });

        foo_a_sfl_c(foo_a_s, input)
    };

    // let foo_a_sfl_hdlr = move |Json(payload): Json<FooAwIn>| async move {
    //     let res = foo_a_sfl(payload).await;
    //     Json(res)
    //     // (StatusCode::OK, Json(res))
    // };

    let foo_a_sfl = make_foo_a_sfl_refreshable();

    let foo_a_sfl_hdlr = handler_of(foo_a_sfl);

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let addr = ([127, 0, 0, 1], 8080).into();

    let _ = thread::spawn(|| loop {
        thread::sleep(Duration::from_millis(500));
        refresh_app_configuration();
    });

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
