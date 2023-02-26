use crate::config::refresh_app_configuration;
use crate::fs::foo_a_sfl;
use crate::fs::FooAIn;
use futures::future::join_all;
use std::time::Duration;
use std::time::SystemTime;
use tokio;
use tokio::time::sleep;

pub async fn run(sleep_factor: u64, repeats: usize) {
    const N: usize = 10_000;

    let start_time = SystemTime::now();
    println!("Started at {:?}", start_time);

    let handle_r = tokio::spawn(async move {
        sleep(Duration::from_millis(25u64 * sleep_factor)).await;
        refresh_app_configuration();
        println!(
            "App configuration refreshed at elapsed time {:?}.",
            start_time.elapsed()
        );
    });

    let handles1 = (0..N / 2)
        .map(|_| {
            tokio::spawn(async move {
                let res = foo_a_sfl(FooAIn {
                    sleep_millis: 20u64 * sleep_factor,
                })
                .await
                .res
                .len();
                for _ in 0..repeats {
                    foo_a_sfl(FooAIn { sleep_millis: 0 }).await;
                }
                res
            })
        })
        .collect::<Vec<_>>();

    let handles2: Vec<_> = (0..N / 2)
        .map(|_| {
            tokio::spawn(async move {
                let res = foo_a_sfl(FooAIn {
                    sleep_millis: 30u64 * sleep_factor,
                })
                .await
                .res
                .len();
                for _ in 0..repeats {
                    foo_a_sfl(FooAIn { sleep_millis: 0 }).await;
                }
                res
            })
        })
        .collect();

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

    let averages = (
        (res1 as f64) / (N as f64) * 2.0,
        (res2 as f64) / (N as f64) * 2.0,
    );

    println!(
        "Ended at {:?}, with averages {:?} (expected averages = (65, 73) for non-zero sleep)",
        start_time.elapsed(),
        averages
    );
}
