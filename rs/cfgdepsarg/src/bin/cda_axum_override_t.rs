use axum::{routing::post, Router};
use cfgdepsarg::fs::{
    bar_at_bf_c, foo_at_sfl_c, BarAtBfCfg, BarAtBfS, FooAtSflCfg, FooAtSflDeps, FooAtSflS,
};
use common::{
    config::{get_pool, refresh_app_configuration},
    fs_data::{BarAtBfCfgInfo, FooAtSflCfgInfo},
    fwk::{
        cfg_deps_at_partial_apply_free_tx_arc, cfg_deps_at_partial_apply_free_tx_box,
        fn2_arc_with_transaction, RefreshMode, Src,
    },
    web::axum_handler::handler_of_pin,
};
use std::{sync::Arc, thread, time::Duration};

#[tokio::main]
async fn main() {
    let bar_at_s = {
        let bar_at_bf_cfg_info = BarAtBfCfgInfo {
            u: 11,
            v: "bar_at_test1".to_owned(),
        };

        let bar_at_cfg = BarAtBfCfg::new(
            Src::new_boxed(move || bar_at_bf_cfg_info.clone()),
            RefreshMode::NoRefresh,
        );
        Arc::new(BarAtBfS {
            cfg: bar_at_cfg,
            deps: (),
        })
    };

    let bar_at_bf_tx = cfg_deps_at_partial_apply_free_tx_box(bar_at_bf_c, bar_at_s);

    let foo_at_s = {
        let foo_at_sfl_cfg_info = FooAtSflCfgInfo {
            a: "foo_at_test1".to_owned(),
            b: 1,
        };

        let foo_at_cfg = FooAtSflCfg::new(
            Src::new_boxed(move || foo_at_sfl_cfg_info.clone()),
            RefreshMode::NoRefresh,
        );

        let foo_at_deps = FooAtSflDeps {
            bar_at_bf: bar_at_bf_tx,
        };

        Arc::new(FooAtSflS {
            cfg: foo_at_cfg,
            deps: foo_at_deps,
        })
    };

    let foo_at_sfl_tx_arc = cfg_deps_at_partial_apply_free_tx_arc(foo_at_sfl_c, foo_at_s);
    let foo_at_sfl = fn2_arc_with_transaction(get_pool(), foo_at_sfl_tx_arc);

    let foo_at_sfl_hdlr = handler_of_pin(foo_at_sfl);

    let app = Router::new().route("/", post(foo_at_sfl_hdlr));

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
