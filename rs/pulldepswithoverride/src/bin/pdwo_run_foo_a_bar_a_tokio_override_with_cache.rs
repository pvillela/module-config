use common::config::get_app_configuration;
use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{box_pin_async_fn, BoxPinFn, RefreshMode, Src};
use common::tokio_run::{run, RunIn};
use pulldepswithoverride::fs::{
    bar_a_bf_cfg_adapter, foo_a_sfl, foo_a_sfl_cfg_adapter, BarABfCfg, FooASflCfg, BAR_A_BF_CFG,
    FOO_A_SFL_CFG_DEPS,
};
use std::time::Duration;
use tokio;

fn make_foo_a_sfl() -> BoxPinFn<FooAIn, FooAOut> {
    box_pin_async_fn(foo_a_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdwo_run_foo_a_bar_a_tokio_with_cache =====");

    const CACHE_TTL: Duration = Duration::from_millis(200);

    FOO_A_SFL_CFG_DEPS.set_cfg_strict({
        let src = Src::Fn(|| foo_a_sfl_cfg_adapter(&get_app_configuration()));
        FooASflCfg::new(src, RefreshMode::Refreshable(CACHE_TTL))
    });

    BAR_A_BF_CFG.set_cfg_strict({
        let src = Src::Fn(|| bar_a_bf_cfg_adapter(&get_app_configuration()));
        BarABfCfg::new(src, RefreshMode::Refreshable(CACHE_TTL))
    });

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
