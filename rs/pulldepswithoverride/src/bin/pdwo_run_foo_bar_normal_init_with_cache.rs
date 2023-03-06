use common::config::refresh_app_configuration;
use common::fwk::RefreshMode;
use pulldepswithoverride::fs::{foo_sfl, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS};
use std::{thread, time::Duration};

fn main() {
    FOO_SFL_CFG_DEPS
        .with(|c| c.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(60))));
    BAR_BF_CFG_DEPS
        .with(|c| c.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(60))));

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);

    refresh_app_configuration();
    println!("App configuration refreshed -- there should be no difference in output.");

    let handle = thread::spawn(move || foo_sfl());
    let res = handle.join().unwrap();
    println!("{}", res);
}
