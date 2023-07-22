use axum::{routing::post, Router};
use cfgdepsarg::fs::{
    bar_at_bf_c, foo_at_sfl_c, BarAtBfCfg, BarAtBfS, FooAtSflCfg, FooAtSflDeps, FooAtSflS,
};
use common::{
    config::{get_pool, refresh_app_configuration},
    fs_data::{BarAtBfCfgInfo, FooAtIn, FooAtOut, FooAtSflCfgInfo},
    fwk::{fn2_arc_with_transaction, pin_async_borrow_fn_2b2_tx, AppErr, RefreshMode, Src, Tx},
    web::axum_handler::handler_of_boxpin,
};
use std::{sync::Arc, thread, time::Duration};

async fn bar_at_bf_tx(sleep_millis: u64, tx: &Tx<'_>) -> Result<String, AppErr> {
    let bar_at_bf_cfg_info = BarAtBfCfgInfo {
        u: 11,
        v: "bar_at_test1".to_owned(),
    };

    let bar_at_cfg = BarAtBfCfg::new(
        Src::new_boxed(move || bar_at_bf_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );
    let bar_at_s = Arc::new(BarAtBfS {
        cfg: bar_at_cfg,
        deps: (),
    });

    bar_at_bf_c(bar_at_s, sleep_millis, tx).await
}

async fn foo_at_sfl_tx(input: FooAtIn, tx: &Tx<'_>) -> Result<FooAtOut, AppErr> {
    let foo_at_sfl_cfg_info = FooAtSflCfgInfo {
        a: "foo_at_test1".to_owned(),
        b: 1,
    };

    let foo_at_cfg = FooAtSflCfg::new(
        Src::new_boxed(move || foo_at_sfl_cfg_info.clone()),
        RefreshMode::NoRefresh,
    );

    let foo_at_deps = FooAtSflDeps {
        bar_at_bf: Box::new(move |s1, s2| {
            let x = bar_at_bf_tx(s1, s2);
            Box::pin(x)
        }),
    };

    let foo_at_s = Arc::new(FooAtSflS {
        cfg: foo_at_cfg,
        deps: foo_at_deps,
    });

    foo_at_sfl_c(foo_at_s, input, tx).await
}

// Below is the by-hand implementation of `foo_at_sfl_tx_pin` (see first line in `main`)
//
// fn foo_at_sfl_tx_pin<'a>(
//     input: FooAtIn,
//     tx: &'a Tx<'a>,
// ) -> Pin<Box<dyn Future<Output = Result<FooAtOut, AppErr>> + Send + Sync + 'a>> {
//     Box::pin(foo_at_sfl_tx(input, tx))
// }

#[tokio::main]
async fn main() {
    let foo_at_sfl_tx_pin = pin_async_borrow_fn_2b2_tx(foo_at_sfl_tx);
    let foo_at_sfl_tx_arc = Arc::new(foo_at_sfl_tx_pin);
    let foo_at_sfl = fn2_arc_with_transaction(get_pool(), foo_at_sfl_tx_arc);
    let foo_at_sfl_boxed = Box::new(foo_at_sfl);

    let foo_at_sfl_hdlr = handler_of_boxpin(foo_at_sfl_boxed);

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
