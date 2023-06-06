use common::config::get_app_configuration;
use common::fs_data::{FooAIn, FooAOut};
use common::fwk::{arc_pin_async_fn, ArcPinFn, RefreshMode, Src};
use common::test_support;
use common::tokio_run::{run, RunIn};
use pulldepswithoverride::fs::{
    bar_a_bf_cfg_adapter, foo_a_sfl, foo_a_sfl_cfg_adapter, BarABfCfg, FooASflCfg, BAR_A_BF_CFG,
    FOO_A_SFL_CFG,
};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tokio;

static READY: AtomicBool = AtomicBool::new(false);

fn ensure_happens_before(ready: &AtomicBool, max_tries: usize, each_sleep_millis: u64) {
    if !ready.load(Ordering::Acquire) {
        for i in 0.. {
            if i >= max_tries {
                panic!("Atomic never ready.")
            }
            thread::sleep(Duration::from_millis(each_sleep_millis));
            if ready.load(Ordering::Acquire) {
                break;
            }
        }
    }
}

fn make_foo_a_sfl() -> ArcPinFn<FooAIn, FooAOut> {
    ensure_happens_before(&READY, 100, 1);
    arc_pin_async_fn(foo_a_sfl)
}

#[tokio::main]
async fn main() {
    println!("===== pdwo_run_foo_a_bar_a_tokio_with_cache =====");

    const CACHE_TTL: Duration = Duration::from_millis(200);

    // Safety: This HAPPENS BEFORE statics are accessed because `make_foo_a_sfl` is called before
    // statics are accessed and thre is a happens before relationship established between the
    // Release at the end of this block and the Acquire in `make_foo_a_sfl`.
    unsafe {
        test_support::override_lazy(&FOO_A_SFL_CFG, || {
            let src = Src::Fn(|| foo_a_sfl_cfg_adapter(&get_app_configuration()));
            FooASflCfg::new(src, RefreshMode::Refreshable(CACHE_TTL))
        });

        test_support::override_lazy(&BAR_A_BF_CFG, || {
            let src = Src::Fn(|| bar_a_bf_cfg_adapter(&get_app_configuration()));
            BarABfCfg::new(src, RefreshMode::Refreshable(CACHE_TTL))
        });

        READY.store(true, Ordering::Release);
    }

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
