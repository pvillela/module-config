use crate::fs;
use crate::fs::{foo_sfl_boot_lr, FooSflT};
use common::config::get_app_configuration;
use common::fwk::{AppCfg, RefreshMode};
use std::sync::OnceLock;
use std::time::Duration;

pub fn make_foo_sfl_no_refresh() -> Box<FooSflT> {
    fs::foo_sfl_boot(AppCfg {
        app_src: get_app_configuration,
        refresh_mode: RefreshMode::NoRefresh,
    })
}

pub fn make_foo_sfl_refreshable() -> Box<FooSflT> {
    fs::foo_sfl_boot(AppCfg {
        app_src: get_app_configuration,
        refresh_mode: RefreshMode::Refreshable(Duration::from_millis(60)),
    })
}

pub fn get_foo_sfl_no_refresh() -> &'static FooSflT {
    static FOO_SFL_NO_REFRESH: OnceLock<&FooSflT> = OnceLock::new();
    FOO_SFL_NO_REFRESH.get_or_init(|| {
        foo_sfl_boot_lr(AppCfg {
            app_src: get_app_configuration,
            refresh_mode: RefreshMode::NoRefresh,
        })
    })
}

pub fn get_foo_sfl_refreshable() -> &'static FooSflT {
    static FOO_SFL_NO_REFRESH: OnceLock<&FooSflT> = OnceLock::new();
    FOO_SFL_NO_REFRESH.get_or_init(|| {
        foo_sfl_boot_lr(AppCfg {
            app_src: get_app_configuration,
            refresh_mode: RefreshMode::Refreshable(Duration::from_millis(60)),
        })
    })
}
