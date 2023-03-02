use common::fwk::{CfgDepsInnerMut, RefreshMode};
use pulldepswithoverride::{
    fs::{BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS},
    tokio_run_common::run,
};
use tokio;

#[tokio::main]
async fn main() {
    CfgDepsInnerMut::update_refresh_mode(&FOO_SFL_CFG_DEPS, RefreshMode::NoRefresh);
    CfgDepsInnerMut::update_refresh_mode(&BAR_BF_CFG_DEPS, RefreshMode::NoRefresh);

    println!("===== pdv_run_foo_a_bar_a_tokio_no_cache =====");

    println!("*** run(0) -- zero sleep time, zero repeats");
    run(0, 0).await;
    println!("*** run(10) -- total 300 ms sleep time, zero repeats");
    run(10, 0).await;

    println!("*** run(0) -- zero sleep time, 99 repeats");
    run(0, 99).await;
    println!("*** run(10) -- total 300 ms sleep time, 99 repeats");
    run(10, 99).await;
}