use std::time::Duration;

use common::fwk::{arc_pin_async_fn, RefreshMode};
use common::tokio_run::{run, RunIn};
use pulldepswithoverride::fs::{
    foo_a_sfl, BarABfCfgDepsOvr, FooASflCfgDepsOvr, BAR_A_BF_CFG_DEPS_OVERRIDE,
    FOO_A_SFL_CFG_DEPS_OVERRIDE,
};
use tokio;

#[tokio::main]
async fn main() {
    println!("===== pdwo_run_foo_a_bar_a_tokio_with_cache =====");

    let _ = FOO_A_SFL_CFG_DEPS_OVERRIDE
        .set(FooASflCfgDepsOvr {
            cfg_src: None,
            refresh_mode: Some(RefreshMode::Refreshable(Duration::from_millis(100))),
            deps: None,
        })
        .ok()
        .expect("FOO_A_SFL_CFG_DEPS_OVERRIDE already initialized");

    let _ = BAR_A_BF_CFG_DEPS_OVERRIDE
        .set(BarABfCfgDepsOvr {
            cfg_src: None,
            refresh_mode: Some(RefreshMode::Refreshable(Duration::from_millis(100))),
            deps: None,
        })
        .ok()
        .expect("BAR_A_BF_CFG_DEPS_OVERRIDE already initialized");

    println!("\n*** run -- total 0 ms sleep time, 10_000 concurrency, 100 repeats");
    run(RunIn {
        foo_a_sfl: arc_pin_async_fn(foo_a_sfl),
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
        foo_a_sfl: arc_pin_async_fn(foo_a_sfl),
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
