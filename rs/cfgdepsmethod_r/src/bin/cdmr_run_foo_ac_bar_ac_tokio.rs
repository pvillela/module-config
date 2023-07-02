use cfgdepsmethods::fs::foo_ac_sfl;
use cfgdepsmethods::fs::FooAcIn;
use futures::future::join_all;
use std::time::Instant;
use tokio;

#[tokio::main]
async fn main() {
    println!("===== pdv_pdv_run_foo_ac_bar_ac_tokio =====");

    println!("*** run(0) -- zero sleep time, zero repeats");
    run(0, 0).await;
    println!("*** run(10) -- total 300 ms sleep time, zero repeats");
    run(10, 0).await;

    println!("*** run(0) -- zero sleep time, 99 repeats");
    run(0, 99).await;
    println!("*** run(10) -- total 300 ms sleep time, 99 repeats");
    run(10, 99).await;
}

async fn run(sleep_factor: u64, repeats: usize) {
    const N: usize = 10_000;

    let start_time = Instant::now();
    println!("Started at {:?}", start_time);

    let handles1 = (0..N / 2)
        .map(|_| {
            tokio::spawn(async move {
                let res = foo_ac_sfl(FooAcIn {
                    sleep_millis: 20u64 * sleep_factor,
                })
                .await
                .res
                .len();
                for _ in 0..repeats {
                    foo_ac_sfl(FooAcIn { sleep_millis: 0 }).await;
                }
                res
            })
        })
        .collect::<Vec<_>>();

    let handles2: Vec<_> = (0..N / 2)
        .map(|_| {
            tokio::spawn(async move {
                let res = foo_ac_sfl(FooAcIn {
                    sleep_millis: 30u64 * sleep_factor,
                })
                .await
                .res
                .len();
                for _ in 0..repeats {
                    foo_ac_sfl(FooAcIn { sleep_millis: 0 }).await;
                }
                res
            })
        })
        .collect();

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
        "Ended at {:?}, with averages {:?} (expected averages = (91, 91))",
        start_time.elapsed(),
        averages
    );
}
