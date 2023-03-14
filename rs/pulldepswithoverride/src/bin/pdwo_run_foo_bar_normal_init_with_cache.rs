use common::config::refresh_app_configuration;
use common::fwk::RefreshMode;
use pulldepswithoverride::fs::{foo_sfl, BAR_BF_CFG_DEPS, FOO_SFL_CFG_DEPS};
use std::{thread, time::Duration};

fn main() {
    FOO_SFL_CFG_DEPS
        .with(|c| c.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(60))));
    BAR_BF_CFG_DEPS
        .with(|c| c.update_refresh_mode(RefreshMode::Refreshable(Duration::from_millis(60))));

    // Everything going on in one thread because the static cfg deps variables are thread-local.
    let handle = thread::spawn(move || {
        let res = foo_sfl();
        println!("{}", res);

        thread::sleep(Duration::from_millis(30));

        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let res = foo_sfl();
        println!("{}", res);
    });
    let _ = handle.join().unwrap();
}
