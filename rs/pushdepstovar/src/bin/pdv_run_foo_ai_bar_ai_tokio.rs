use common::fs_data::{FooAiIn, FooAiOut};
use common::fwk::BoxPinFn;
use common::tokio_run::{run, RunIn};
use pushdepstovar::startup::get_foo_ai_sfl;
use tokio;

fn make_foo_ai_sfl() -> BoxPinFn<FooAiIn, FooAiOut> {
    let foo_ai_sfl = get_foo_ai_sfl();
    Box::new(foo_ai_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdv_run_foo_ai_bar_ai_tokio =====");

    run(RunIn {
        make_foo_a_sfl: make_foo_ai_sfl,
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
