use crate::fs::boot::{foo_sfl_boot, foo_sfl_boot_lr};
use crate::fs::FooSflT;
use common::config::get_app_configuration;
use common::fwk::RefreshMode;
use std::sync::OnceLock;
use std::time::Duration;

pub fn make_foo_sfl_no_refresh() -> Box<FooSflT> {
    foo_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}

pub fn make_foo_sfl_refreshable() -> Box<FooSflT> {
    foo_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn get_foo_sfl_no_refresh() -> &'static FooSflT {
    static FOO_SFL_NO_REFRESH: OnceLock<&FooSflT> = OnceLock::new();
    FOO_SFL_NO_REFRESH
        .get_or_init(|| foo_sfl_boot_lr(get_app_configuration, RefreshMode::NoRefresh))
}

pub fn get_foo_sfl_refreshable() -> &'static FooSflT {
    static FOO_SFL_NO_REFRESH: OnceLock<&FooSflT> = OnceLock::new();
    FOO_SFL_NO_REFRESH.get_or_init(|| {
        foo_sfl_boot_lr(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(60)),
        )
    })
}
