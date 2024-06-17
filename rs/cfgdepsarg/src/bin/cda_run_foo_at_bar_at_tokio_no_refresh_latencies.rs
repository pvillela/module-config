use cfgdepsarg::startup::make_foo_at_sfl_no_refresh;
use common::fs_data::{FooAtIn, FooAtOut};
use common::fwk::{AppErr, BoxPinFn};
use common::tokio_run::{run, RunIn};
use latency_trace::LatencyTrace;

fn make_foo_at_sfl() -> BoxPinFn<FooAtIn, Result<FooAtOut, AppErr>> {
    make_foo_at_sfl_no_refresh()
}

fn main() {
    // Set below value to "trace" to enable full library tracing.
    // set_var("RUST_LOG", "info");

    let latencies = LatencyTrace::default().measure_latencies_tokio(|| async {
        // Set env_logger only if `tracing_subsriber` hasn't pulled in `tracing_log` and already set a logger.
        // Otherwise, setting a second logger would panic.
        // _ = env_logger::try_init();

        println!("===== cda_run_foo_at_bar_at_tokio_no_refresh_latencies =====");

        run(RunIn {
            make_foo_a_sfl: make_foo_at_sfl,
            unit_time_millis: 1,
            app_cfg_first_refresh_units: 10,
            app_cfg_refresh_delta_units: 10,
            app_cfg_refresh_count: 10,
            per_call_sleep_units: 10,
            increment_to_print: 33,
            concurrency: 5,
            repeats: 100,
        })
        .await;
    });

    println!("\nLatency stats below are in microseconds");
    for (span_group, stats) in latencies.summary_stats() {
        println!("  * {:?}, {:?}", span_group, stats);
    }
}
