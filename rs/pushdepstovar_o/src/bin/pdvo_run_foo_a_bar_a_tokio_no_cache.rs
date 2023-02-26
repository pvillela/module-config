use pushdepstovar_o::startup::init_a_refreshable;
use pushdepstovar_o::tokio_run_common::run;
use tokio;

#[tokio::main]
async fn main() {
    init_a_refreshable();

    println!("===== pdvo_run_foo_a_bar_a_tokio_no_cache =====");

    println!("*** run(0) -- zero sleep time, zero repeats");
    run(0, 0).await;
    println!("*** run(10) -- total 300 ms sleep time, zero repeats");
    run(10, 0).await;

    println!("*** run(0) -- zero sleep time, 99 repeats");
    run(0, 99).await;
    println!("*** run(10) -- total 300 ms sleep time, 99 repeats");
    run(10, 99).await;
}
