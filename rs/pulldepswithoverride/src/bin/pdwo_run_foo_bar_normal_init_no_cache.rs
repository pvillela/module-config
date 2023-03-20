use common::config::refresh_app_configuration;
use common::fwk::{CfgOvd, RefreshMode};
use pulldepswithoverride::fs::{
    foo_sfl, BAR_BF_CFG_OVERRIDE, FOO_SFL_CFG_OVERRIDE, FOO_SFL_DEPS_OVERRIDE,
};
use std::thread;
use std::time::Duration;

fn main() {
    let _ = CfgOvd::set_once_cell(
        &FOO_SFL_CFG_OVERRIDE,
        None,
        Some(RefreshMode::Refreshable(Duration::from_millis(0))),
    );

    // Below can be deleted; included only to prove it compiles.
    let _ = FOO_SFL_DEPS_OVERRIDE.set(pulldepswithoverride::fs::FooSflDeps {
        bar_bf: pulldepswithoverride::fs::bar_bf,
    });

    let _ = CfgOvd::set_once_cell(
        &BAR_BF_CFG_OVERRIDE,
        None,
        Some(RefreshMode::Refreshable(Duration::from_millis(0))),
    );

    let handle = thread::spawn(move || {
        let res = foo_sfl();
        println!("{}", res);

        thread::sleep(Duration::from_millis(30));

        refresh_app_configuration();
        println!("App configuration refreshed -- output should be different.");

        let res = foo_sfl();
        println!("{}", res);
    });
    let _ = handle.join().unwrap();
}
