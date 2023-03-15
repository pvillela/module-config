use common::config::refresh_app_configuration;
use common::fs_data::{BarBfCfgInfo, FooSflCfgInfo};
use common::fwk::{nil_app_cfg, RefreshMode};
use pushtofunction::fs::boot::{foo_sfl_boot, BAR_BF_CFG_INFO_OVERRIDE, FOO_SFL_CFG_INFO_OVERRIDE};
use pushtofunction::startup::make_foo_sfl_refreshable;
use std::thread;
use std::time::Duration;

fn main() {
    FOO_SFL_CFG_INFO_OVERRIDE
        .set(FooSflCfgInfo {
            a: "foo_override".to_owned(),
            b: 11,
        })
        .unwrap();

    BAR_BF_CFG_INFO_OVERRIDE
        .set(BarBfCfgInfo {
            u: 33,
            v: "bar_override".to_owned(),
        })
        .unwrap();

    {
        println!("Running foo_sfl with config info override in 2 threads, using nil_app_cfg.");

        let handle = thread::spawn(move || {
            foo_sfl_boot(
                nil_app_cfg,
                RefreshMode::Refreshable(Duration::from_millis(0)),
            )()
        });
        let res = handle.join().unwrap();
        println!("{}", res);

        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let handle = thread::spawn(move || {
            foo_sfl_boot(
                nil_app_cfg,
                RefreshMode::Refreshable(Duration::from_millis(0)),
            )()
        });
        let res = handle.join().unwrap();
        println!("{}", res);
    }

    {
        println!("Running foo_sfl with config info override in 2 threads, using get_app_configuration. There should be no change from nil_app_cfg.");

        let handle = thread::spawn(move || make_foo_sfl_refreshable()());
        let res = handle.join().unwrap();
        println!("{}", res);

        refresh_app_configuration();
        thread::sleep(Duration::from_millis(70));
        println!("App configuration refreshed -- there should be no difference in output.");

        let handle = thread::spawn(move || make_foo_sfl_refreshable()());
        let res = handle.join().unwrap();
        println!("{}", res);
    }
}
