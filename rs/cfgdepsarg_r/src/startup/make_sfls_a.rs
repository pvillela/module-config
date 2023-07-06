use crate::fs::boot::foo_a_sfl_boot;
use crate::fs::FooASflT;
use common::config::get_app_configuration;
use common::fwk::RefreshMode;
use std::sync::OnceLock;
use std::time::Duration;

pub fn make_foo_a_sfl_refreshable() -> Box<FooASflT> {
    foo_a_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_a_sfl_no_refresh() -> Box<FooASflT> {
    foo_a_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}

pub fn get_foo_a_sfl_no_refresh() -> &'static FooASflT {
    static FOO_A_SFL_NO_REFRESH: OnceLock<&FooASflT> = OnceLock::new();
    FOO_A_SFL_NO_REFRESH.get_or_init(|| {
        Box::leak(foo_a_sfl_boot(
            get_app_configuration,
            RefreshMode::NoRefresh,
        ))
    })
}

pub fn get_foo_a_sfl_refreshable() -> &'static FooASflT {
    static FOO_A_SFL_REFRESHABLE: OnceLock<&FooASflT> = OnceLock::new();
    FOO_A_SFL_REFRESHABLE.get_or_init(|| {
        Box::leak(foo_a_sfl_boot(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(60)),
        ))
    })
}
