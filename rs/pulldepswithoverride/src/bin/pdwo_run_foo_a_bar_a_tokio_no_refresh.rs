use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{arc_pin_async_fn, ArcPinFn, CfgOvd, RefreshMode};
use common::tokio_run::{run, RunIn};
use pulldepswithoverride::fs::{
    foo_a_sfl, BAR_A_BF_CFG_OVERRIDE, FOO_A_SFL_CFG_OVERRIDE, FOO_A_SFL_DEPS_OVERRIDE,
};
use tokio;

fn make_foo_a_sfl() -> ArcPinFn<FooAIn, FooAOut> {
    arc_pin_async_fn(foo_a_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdwo_run_foo_a_bar_a_tokio_with_cache =====");

    let _ = CfgOvd::set_once_cell(&FOO_A_SFL_CFG_OVERRIDE, None, Some(RefreshMode::NoRefresh));

    // Below can be deleted; included only to prove it compiles.
    let _ = FOO_A_SFL_DEPS_OVERRIDE.set(pulldepswithoverride::fs::FooASflDeps {
        bar_a_bf: arc_pin_async_fn(pulldepswithoverride::fs::bar_a_bf),
    });

    let _ = CfgOvd::set_once_cell(&BAR_A_BF_CFG_OVERRIDE, None, Some(RefreshMode::NoRefresh));

    println!("\n*** run -- total 0 ms sleep time, 10_000 concurrency, 100 repeats");
    run(RunIn {
        make_foo_a_sfl,
        unit_time_millis: 0,
        app_cfg_first_refresh_units: 1,
        app_cfg_refresh_delta_units: 1,
        app_cfg_refresh_count: 0,
        batch_initial_sleep_units: 0,
        batch_gap_sleep_units: 4,
        concurrency: 10_000,
        repeats: 100,
    })
    .await;

    println!("\n*** run -- total 80 ms sleep time, 10_000 concurrency, 100 repeats");
    run(RunIn {
        make_foo_a_sfl,
        unit_time_millis: 10,
        app_cfg_first_refresh_units: 1,
        app_cfg_refresh_delta_units: 1,
        app_cfg_refresh_count: 10,
        batch_initial_sleep_units: 0,
        batch_gap_sleep_units: 4,
        concurrency: 10_000,
        repeats: 100,
    })
    .await;
}
