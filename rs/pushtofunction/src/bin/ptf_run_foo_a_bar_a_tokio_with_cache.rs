use std::time::Duration;

use common::config::get_app_configuration;
use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{ArcPinFn, RefreshMode};
use common::tokio_run::{run, RunIn};
use pushtofunction::fs::boot::foo_a_sfl_boot;
use tokio;

fn make_foo_a_sfl() -> ArcPinFn<FooAIn, FooAOut> {
    foo_a_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(100)),
    )
}

#[tokio::main]
async fn main() {
    println!("===== ptf_run_foo_a_bar_a_tokio_with_cache =====");

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
