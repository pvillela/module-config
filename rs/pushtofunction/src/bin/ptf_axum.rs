use axum::{routing::post, Router};
use common::{
    fs_data::{BarBfCfgInfo, FooSflCfgInfo},
    fwk::{RefreshMode, Src},
    web::axum_handler::handler_of_boxpin,
};
use pushtofunction::fs::{bar_a_bf_c, foo_a_sfl_c, BarABfCfg, FooASflCfg, FooASflDeps};

#[tokio::main]
async fn main() {
    let foo_a_sfl_cfg_info = FooSflCfgInfo {
        a: "foo_aw_test1".to_owned(),
        b: 1,
    };
    let bar_a_bf_cfg_info = BarBfCfgInfo {
        u: 11,
        v: "bar_a_test1".to_owned(),
    };

    let bar_a_cfg = BarABfCfg::new(
        Src::new_boxed(move || bar_a_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let bar_a_bf = bar_a_bf_c(bar_a_cfg);

    let foo_a_cfg = FooASflCfg::new(
        Src::new_boxed(move || foo_a_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let foo_a_deps = FooASflDeps { bar_a_bf };

    // let foo_a_sfl_hdlr = move |Json(payload): Json<FooAwIn>| async move {
    //     let res = foo_a_sfl(payload).await;
    //     Json(res)
    //     // (StatusCode::OK, Json(res))
    // };

    let foo_a_sfl = foo_a_sfl_c(foo_a_cfg, foo_a_deps);
    let foo_a_sfl_hdlr = handler_of_boxpin(foo_a_sfl);

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
