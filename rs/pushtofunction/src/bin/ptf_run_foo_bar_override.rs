use pushtofunction::config::{get_app_configuration, refresh_app_configuration};
use pushtofunction::fs::boot::{foo_sfl_boot, BAR_BF_CFG_INFO_OVERRIDE, FOO_SFL_CFG_INFO_OVERRIDE};
use pushtofunction::fs::{BarBfCfgInfo, FooSflCfgInfo};
use pushtofunction::fwk::nil_app_cfg;
use std::thread;

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
        let foo_sfl = foo_sfl_boot(nil_app_cfg);
        let foo_sfl_clone = foo_sfl.clone();

        let handle = thread::spawn(move || foo_sfl());
        let res = handle.join().unwrap();
        println!("{}", res);

        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let handle = thread::spawn(move || foo_sfl_clone());
        let res = handle.join().unwrap();
        println!("{}", res);
    }

    {
        println!("Running foo_sfl with config info override in 2 threads, using get_app_configuration. There should be no change from nil_app_cfg.");
        let foo_sfl = foo_sfl_boot(get_app_configuration);
        let foo_sfl_clone = foo_sfl.clone();

        let handle = thread::spawn(move || foo_sfl());
        let res = handle.join().unwrap();
        println!("{}", res);

        refresh_app_configuration();
        println!("App configuration refreshed -- there should be no difference in output.");

        let handle = thread::spawn(move || foo_sfl_clone());
        let res = handle.join().unwrap();
        println!("{}", res);
    }
}
