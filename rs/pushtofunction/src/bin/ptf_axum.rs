use axum::{routing::post, Json, Router};
use common::{
    fs_data::{BarBfCfgInfo, FooAwIn, FooSflCfgInfo},
    fwk::{RefreshMode, Src},
    web::axum_handler,
};
use pushtofunction::fs::{bar_a_bf_c, foo_a_sfl_c, BarABfCfgDeps, FooASflCfgDeps, FooASflDeps};

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

    let bar_a_cfg_deps = BarABfCfgDeps::new(
        Src::new_boxed(move || bar_a_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
        (),
    );

    let bar_a_bf = bar_a_bf_c(bar_a_cfg_deps);

    let foo_a_cfg_deps = FooASflCfgDeps::new(
        Src::new_boxed(move || foo_a_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
        FooASflDeps { bar_a_bf },
    );

    let foo_a_sfl = foo_a_sfl_c(foo_a_cfg_deps);
    let arc_f = axum_handler::handler_of(foo_a_sfl);
    let arc_f = handler_of_web(foo_aw_sfl);
    let f = move |i| arc_f(i);

    // let foo_a_sfl_hdlr = move |Json(payload): Json<FooAwIn>| async move {
    //     let res = foo_a_sfl(payload).await;
    //     Json(res)
    //     // (StatusCode::OK, Json(res))
    // };

    let app = Router::new().route("/", post(foo_a_sfl_hdlr));

    let addr = ([127, 0, 0, 1], 8080).into();
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
