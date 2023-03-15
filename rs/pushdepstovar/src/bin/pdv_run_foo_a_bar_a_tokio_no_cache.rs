use common::fwk::arc_pin_async_fn;
use common::tokio_run::{run, RunIn};
use pushdepstovar::fs::foo_a_sfl;
use pushdepstovar::startup::init_a_refreshable;
use tokio;

#[tokio::main]
async fn main() {
    println!("===== pdv_run_foo_a_bar_a_tokio_no_cache =====");

    init_a_refreshable();

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
