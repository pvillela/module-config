use crate::fs;
use crate::fs::{foo_a_sfl_boot_lr, FooASflT};
use common::config::get_app_configuration;
use common::fwk::{AppCfg, RefreshMode};
use std::sync::OnceLock;
use std::time::Duration;

pub fn make_foo_a_sfl_no_refresh() -> Box<FooASflT> {
    fs::foo_a_sfl_boot(AppCfg {
        app_src: get_app_configuration,
        refresh_mode: RefreshMode::NoRefresh,
    })
}

pub fn make_foo_a_sfl_refreshable() -> Box<FooASflT> {
    fs::foo_a_sfl_boot(AppCfg {
        app_src: get_app_configuration,
        refresh_mode: RefreshMode::Refreshable(Duration::from_millis(60)),
    })
}

pub fn get_foo_a_sfl_no_refresh() -> &'static FooASflT {
    static FOO_A_SFL_NO_REFRESH: OnceLock<&FooASflT> = OnceLock::new();
    FOO_A_SFL_NO_REFRESH.get_or_init(|| {
        foo_a_sfl_boot_lr(AppCfg {
            app_src: get_app_configuration,
            refresh_mode: RefreshMode::NoRefresh,
        })
    })
}

pub fn get_foo_a_sfl_refreshable() -> &'static FooASflT {
    static FOO_A_SFL_REFRESHABLE: OnceLock<&FooASflT> = OnceLock::new();
    FOO_A_SFL_REFRESHABLE.get_or_init(|| {
        foo_a_sfl_boot_lr(AppCfg {
            app_src: get_app_configuration,
            refresh_mode: RefreshMode::Refreshable(Duration::from_millis(60)),
        })
    })
}
