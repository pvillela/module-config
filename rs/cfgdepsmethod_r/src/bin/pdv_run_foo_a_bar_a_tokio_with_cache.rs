use cfgdepsmethod_r::fs::boot::get_foo_a_sfl_s_cached;
use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{arc_pin_async_fn, ArcPinFn};
use common::tokio_run::{run, RunIn};
use tokio;

fn make_foo_a_sfl() -> ArcPinFn<FooAIn, FooAOut> {
    let foo_a_sfl_s = get_foo_a_sfl_s_cached();
    arc_pin_async_fn(|input| foo_a_sfl_s.run(input))
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
