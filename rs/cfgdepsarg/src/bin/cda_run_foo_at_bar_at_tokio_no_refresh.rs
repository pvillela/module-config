use cfgdepsarg::startup::make_foo_at_sfl_no_refresh;
use common::fs_data::{FooAtIn, FooAtOut};
use common::fwk::{AppErr, BoxPinFn};
use common::tokio_run::{run, RunIn};

fn make_foo_at_sfl() -> BoxPinFn<FooAtIn, Result<FooAtOut, AppErr>> {
    make_foo_at_sfl_no_refresh()
}

#[tokio::main]
async fn main() {
    println!("===== cda_run_foo_at_bar_at_tokio_no_cache =====");

    run(RunIn {
        make_foo_a_sfl: make_foo_at_sfl,
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
