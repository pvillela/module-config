use common::config::get_app_configuration;
use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{arc_pin_async_fn, ArcPinFn, RefreshMode, Src};
use common::test_support;
use common::tokio_run::{run, RunIn};
use pulldepswithoverride::fs::{
    bar_a_bf_cfg_adapter, foo_a_sfl, foo_a_sfl_cfg_adapter, BarABfCfg, FooASflCfg, BAR_A_BF_CFG,
    FOO_A_SFL_CFG,
};
use tokio;

fn make_foo_a_sfl() -> ArcPinFn<FooAIn, FooAOut> {
    arc_pin_async_fn(foo_a_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdwo_run_foo_a_bar_a_tokio_no_refresh =====");

    unsafe {
        test_support::override_lazy(&FOO_A_SFL_CFG, || {
            let src = Src::Fn(|| foo_a_sfl_cfg_adapter(&get_app_configuration()));
            FooASflCfg::new(src, RefreshMode::NoRefresh)
        });

        test_support::override_lazy(&BAR_A_BF_CFG, || {
            let src = Src::Fn(|| bar_a_bf_cfg_adapter(&get_app_configuration()));
            BarABfCfg::new(src, RefreshMode::NoRefresh)
        });
    }

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
