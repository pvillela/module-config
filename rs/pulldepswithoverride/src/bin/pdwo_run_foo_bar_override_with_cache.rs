use common::config::{get_app_configuration, refresh_app_configuration};
use common::fwk::{RefreshMode, Src};
use pulldepswithoverride::fs::{
    bar_bf_cfg_adapter, foo_sfl, foo_sfl_cfg_adapter, BarBfCfg, FooSflCfg, BAR_BF_CFG,
    FOO_SFL_CFG_DEPS,
};
use std::thread;
use std::time::Duration;

fn main() {
    FOO_SFL_CFG_DEPS.set_cfg_strict({
        let src = Src::Fn(|| foo_sfl_cfg_adapter(&get_app_configuration()));
        FooSflCfg::new(src, RefreshMode::Refreshable(Duration::from_millis(60)))
    });

    BAR_BF_CFG.set_cfg_strict({
        let src = Src::Fn(|| bar_bf_cfg_adapter(&get_app_configuration()));
        BarBfCfg::new(src, RefreshMode::Refreshable(Duration::from_millis(60)))
    });

    let handle = thread::spawn(move || {
        let res = foo_sfl();
        println!("{}", res);

        thread::sleep(Duration::from_millis(30));

        refresh_app_configuration();
        println!("App configuration refreshed -- output should be the same.");

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
