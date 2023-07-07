use common::config::get_app_configuration;
use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{BoxPinFn, RefreshMode};
use common::tokio_run::{run, RunIn};
use pushdepstovar::fs::boot::get_foo_a_sfl_with_app_cfg;
use std::time::Duration;
use tokio;

fn make_foo_a_sfl() -> BoxPinFn<FooAIn, FooAOut> {
    let foo_a_sfl = get_foo_a_sfl_with_app_cfg(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(150)),
    );
    Box::new(foo_a_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdv_run_foo_a_bar_a_tokio__cache =====");

    run(RunIn {
        make_foo_a_sfl,
        unit_time_millis: 1,
        app_cfg_first_refresh_units: 10,
        app_cfg_refresh_delta_units: 10,
        app_cfg_refresh_count: 10,
        per_call_sleep_units: 1,
        increment_to_print: 33,
        concurrency: 1_000,
        repeats: 100,
    })
    .await;
}
