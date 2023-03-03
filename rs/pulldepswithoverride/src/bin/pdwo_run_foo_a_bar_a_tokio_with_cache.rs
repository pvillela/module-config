use std::time::Duration;

use common::fwk::RefreshMode;
use pulldepswithoverride::{
    fs::{BAR_A_BF_CFG_DEPS, FOO_A_SFL_CFG_DEPS},
    tokio_run_common::run,
};
use tokio;

#[tokio::main]
async fn main() {
    FOO_A_SFL_CFG_DEPS.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(60)));
    BAR_A_BF_CFG_DEPS.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(60)));

    println!("===== pdv_run_foo_a_bar_a_tokio_no_cache =====");

    // println!("*** run(0) -- zero sleep time, zero repeats");
    // run(0, 0).await;
    println!("*** run(10) -- total 300 ms sleep time, zero repeats");
    run(10, 0).await;

    // println!("*** run(0) -- zero sleep time, 99 repeats");
    // run(0, 99).await;
    println!("*** run(10) -- total 300 ms sleep time, 99 repeats");
    run(10, 99).await;
}
