use common::fs_data::{FooAIn, FooAOut};
use common::fwk::BoxPinFn;
use common::tokio_run::{run, RunIn};
use pushdepstovar::startup::get_foo_a_sfl_no_refresh;
use tokio;

fn make_foo_a_sfl() -> BoxPinFn<FooAIn, FooAOut> {
    let foo_a_sfl = get_foo_a_sfl_no_refresh();
    Box::new(foo_a_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdv_run_foo_a_bar_a_tokio_no_refresh =====");

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
