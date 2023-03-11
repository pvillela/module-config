use common::fwk::box_pin_async_fn;
use common::tokio_run::{run, RunIn};
use pushdepstovar::fs::foo_a_sfl;
use pushdepstovar::startup::init_a_no_refresh;
use tokio;

#[tokio::main]
async fn main() {
    println!("===== pdv_run_foo_a_bar_a_tokio_with_cache =====");

    init_a_no_refresh();

    println!("\n*** run -- total 80 ms sleep time, 10_000 concurrency, 100 repeats");
    run(RunIn {
        foo_a_sfl: box_pin_async_fn(foo_a_sfl),
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
