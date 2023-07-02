use cfgdepsmethods::fs::boot::get_foo_sfl_with_app_cfg;
use common::{
    config::{get_app_configuration, refresh_app_configuration},
    fwk::RefreshMode,
};
use std::{thread, time::Duration};

fn main() {
    let foo_sfl = get_foo_sfl_with_app_cfg(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(50)),
    );

    let handle = thread::spawn(move || {
        let res = foo_sfl();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(30));
        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let res = foo_sfl();
        println!("{}", res);

        thread::sleep(std::time::Duration::from_millis(30));
        refresh_app_configuration();
        println!("App configuration refreshed -- output should be different.");

        let res = foo_sfl();
        println!("{}", res);
    });
    let _ = handle.join().unwrap();
}
