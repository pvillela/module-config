use crate::config::{initialize_app_configuration, refresh_app_configuration};
use crate::fs_data::{FooAIn, FooAOut};
use crate::fwk::ArcPinFn;
use futures::future::join_all;
use std::time::{Duration, Instant};
use tokio;
use tokio::time::sleep;

pub struct RunIn {
    pub foo_a_sfl: ArcPinFn<FooAIn, FooAOut>,
    pub unit_time_millis: u64,
    pub app_cfg_first_refresh_units: u64,
    pub app_cfg_refresh_delta_units: u64,
    pub app_cfg_refresh_count: u64,
    pub batch_initial_sleep_units: u64,
    pub batch_gap_sleep_units: u64,
    pub concurrency: usize,
    pub repeats: usize,
}

pub async fn run(input: RunIn) {
    let RunIn {
        foo_a_sfl,
        unit_time_millis,
        app_cfg_first_refresh_units,
        app_cfg_refresh_delta_units,
        app_cfg_refresh_count,
        batch_initial_sleep_units,
        batch_gap_sleep_units,
        concurrency,
        repeats,
    } = input;

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
        refresh_app_configuration();
    });

    let run_concurrent = |i: usize| {
        let foo_a_sfl = foo_a_sfl.clone();
        tokio::spawn(async move {
            let out = foo_a_sfl(FooAIn { sleep_millis: 0 }).await;
            let res = out.res.len();
            if i == 0 {
                println!(
                    "foo_a executed at {:?} elapsed, res={}, out={:?}",
                    start_time.elapsed(),
                    res,
                    out
                );
            }
            for _ in 0..repeats - 1 {
                foo_a_sfl(FooAIn { sleep_millis: 0 }).await;
            }
            res
        })
    };

    sleep(Duration::from_millis(
        batch_initial_sleep_units * unit_time_millis,
    ))
    .await;
    let handles1 = (0..concurrency).map(run_concurrent).collect::<Vec<_>>();

    sleep(Duration::from_millis(
        (batch_initial_sleep_units + batch_gap_sleep_units) * unit_time_millis,
    ))
    .await;
    let handles2 = (0..concurrency).map(run_concurrent).collect::<Vec<_>>();

    sleep(Duration::from_millis(
        (batch_initial_sleep_units + 2 * batch_gap_sleep_units) * unit_time_millis,
    ))
    .await;
    let handles3 = (0..concurrency).map(run_concurrent).collect::<Vec<_>>();

    let _ = handle_r
        .await
        .ok()
        .expect("app configuration refresh task failed");

    let res1: usize = join_all(handles1)
        .await
        .iter()
        .map(|x| x.as_ref().ok().expect("Failure in first batch of tasks."))
        .sum();

    let res2: usize = join_all(handles2)
        .await
        .iter()
        .map(|x| x.as_ref().ok().expect("Failure in second batch of tasks."))
        .sum();

    let res3: usize = join_all(handles3)
        .await
        .iter()
        .map(|x| x.as_ref().ok().expect("Failure in third batch of tasks."))
        .sum();

    let averages = (
        (res1 as f64) / (concurrency as f64),
        (res2 as f64) / (concurrency as f64),
        (res3 as f64) / (concurrency as f64),
    );

    println!(
        "Ended at {:?}, with execution counts counts={:?}, averages={:?}",
        start_time.elapsed(),
        (
            concurrency * repeats,
            concurrency * repeats,
            concurrency * repeats
        ),
        averages
    );
}
