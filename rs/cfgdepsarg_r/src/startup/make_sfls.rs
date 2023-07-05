use crate::fs::boot::{
    foo_a_sfl_boot, foo_ai_sfl_boot, foo_aw_sfl_boot, foo_i_sfl_boot, foo_sfl_boot,
};
use crate::fs::{FooASflT, FooAiSflT, FooAwSflT, FooISflT, FooSflT};
use common::config::get_app_configuration;
use common::fwk::RefreshMode;
use std::sync::OnceLock;
use std::time::Duration;

pub fn make_foo_sfl_refreshable() -> FooSflT {
    foo_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_sfl_no_refresh() -> FooSflT {
    foo_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}

pub fn make_foo_i_sfl() -> FooISflT {
    foo_i_sfl_boot(get_app_configuration)
}

pub fn make_foo_a_sfl_refreshable() -> Box<FooASflT> {
    foo_a_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_a_sfl_no_refresh() -> Box<FooASflT> {
    foo_a_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}

pub fn make_foo_ai_sfl() -> FooAiSflT {
    foo_ai_sfl_boot(get_app_configuration)
}

pub fn make_foo_aw_sfl_refreshable() -> FooAwSflT {
    foo_aw_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_aw_sfl_no_refresh() -> FooAwSflT {
    foo_aw_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
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
