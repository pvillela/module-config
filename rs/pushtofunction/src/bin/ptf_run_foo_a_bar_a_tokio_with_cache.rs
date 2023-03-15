use std::time::Duration;

use common::config::get_app_configuration;
use common::fwk::RefreshMode;
use common::tokio_run::{run, RunIn};
use pushtofunction::fs::boot::foo_a_sfl_boot;
use tokio;

#[tokio::main]
async fn main() {
    println!("===== ptf_run_foo_a_bar_a_tokio_with_cache =====");

    println!("\n*** run -- total 0 ms sleep time, 10_000 concurrency, 100 repeats");
    let foo_a_sfl = foo_a_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(100)),
    );
    run(RunIn {
        foo_a_sfl,
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
    let foo_a_sfl = foo_a_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(100)),
    );
    run(RunIn {
        foo_a_sfl,
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
