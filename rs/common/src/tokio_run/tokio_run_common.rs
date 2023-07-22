use crate::config::{initialize_app_configuration, refresh_app_configuration};
use crate::fs_data::FooAIn;
use crate::fwk::BoxPinFn;
use futures::future::join_all;
use std::fmt::Debug;
use std::time::{Duration, Instant};
use tokio;
use tokio::time::sleep;

pub struct RunIn<T: Debug> {
    pub make_foo_a_sfl: fn() -> BoxPinFn<FooAIn, T>,
    pub unit_time_millis: u64,
    pub app_cfg_first_refresh_units: u64,
    pub app_cfg_refresh_delta_units: u64,
    pub app_cfg_refresh_count: u64,
    pub per_call_sleep_units: u64,
    pub increment_to_print: usize,
    pub concurrency: usize,
    pub repeats: usize,
}

pub async fn run<T: Debug + 'static>(input: RunIn<T>) {
    let RunIn {
        make_foo_a_sfl,
        unit_time_millis,
        app_cfg_first_refresh_units,
        app_cfg_refresh_delta_units,
        app_cfg_refresh_count,
        per_call_sleep_units,
        increment_to_print,
        concurrency,
        repeats,
    } = input;

    println!(
        "\n*** run -- {} ms sleep per call, {} concurrency, {} repeats",
        per_call_sleep_units * unit_time_millis,
        concurrency,
        repeats
    );

    let start_time = Instant::now();
    println!("Started at {:?}", start_time);

    initialize_app_configuration();

    let handle_r = tokio::spawn(async move {
        sleep(Duration::from_millis(
            app_cfg_first_refresh_units * unit_time_millis,
        ))
        .await;
        for _ in 0..app_cfg_refresh_count {
            sleep(Duration::from_millis(
                app_cfg_refresh_delta_units * unit_time_millis,
            ))
            .await;
            refresh_app_configuration();
            println!(
                "App configuration refreshed at elapsed time {:?}.",
                start_time.elapsed()
            );
        }
    });

    let run_concurrent = |i: usize| {
        let foo_a_sfl = make_foo_a_sfl();
        tokio::spawn(async move {
            let mut res: usize = 0;
            for j in 0..repeats - 1 {
                let out = foo_a_sfl(FooAIn {
                    sleep_millis: per_call_sleep_units * unit_time_millis,
                })
                .await;
                res = format!("{:?}", out).len();
                if i == 0 && j % increment_to_print == 0 {
                    println!(
                        "foo_a executed at {:?} elapsed, res={}, out={:?}",
                        start_time.elapsed(),
                        res,
                        out
                    );
                }
            }
            res
        })
    };

    let handles1 = (0..concurrency).map(run_concurrent).collect::<Vec<_>>();

    let _ = handle_r
        .await
        .ok()
        .expect("app configuration refresh task failed");

    let res1: usize = join_all(handles1)
        .await
        .iter()
        .map(|x| x.as_ref().ok().expect("Failure in first batch of tasks."))
        .sum();

    let average = (res1 as f64) / (concurrency as f64);

    println!(
        "Ended at {:?}, with count={:?}, average={:?}",
        start_time.elapsed(),
        concurrency * repeats,
        average
    );
}
