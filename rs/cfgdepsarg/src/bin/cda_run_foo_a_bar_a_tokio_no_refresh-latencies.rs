mod print_latency_stats;

use cfgdepsarg::fs::FooASflT;
use cfgdepsarg::startup::make_foo_a_sfl_no_refresh;
use common::tokio_run::{run, RunIn};
use latency_trace::measure_latencies_tokio;
use print_latency_stats::print_parents_means_medians;

fn make_foo_a_sfl() -> Box<FooASflT> {
    make_foo_a_sfl_no_refresh()
}

fn main() {
    // Set below value to "trace" to enable full library tracing.
    // set_var("RUST_LOG", "info");

    let latencies = measure_latencies_tokio(|| async {
        // Set env_logger only if `tracing_subsriber` hasn't pulled in `tracing_log` and already set a logger.
        // Otherwise, setting a second logger would panic.
        // _ = env_logger::try_init();

        println!("===== cda_run_foo_a_bar_a_tokio_no_cache =====");

        run(RunIn {
            make_foo_a_sfl: make_foo_a_sfl,
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

    print_parents_means_medians(&latencies);
}
