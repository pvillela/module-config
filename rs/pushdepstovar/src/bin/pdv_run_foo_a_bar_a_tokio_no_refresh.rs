use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{arc_pin_async_fn, ArcPinFn};
use common::tokio_run::{run, RunIn};
use pushdepstovar::fs::foo_a_sfl;
use pushdepstovar::startup::init_a_no_refresh;
use tokio;

fn make_foo_a_sfl() -> ArcPinFn<FooAIn, FooAOut> {
    arc_pin_async_fn(foo_a_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdv_run_foo_a_bar_a_tokio_no_refresh =====");

    init_a_no_refresh();

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
