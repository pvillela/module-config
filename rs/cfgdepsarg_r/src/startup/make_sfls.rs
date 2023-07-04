use crate::fs::boot::{
    foo_a_sfl_boot, foo_ai_sfl_boot, foo_aw_sfl_boot, foo_i_sfl_boot, foo_sfl_boot,
};
use crate::fs::{FooASflT, FooAiSflT, FooAwSflT, FooISflT, FooSflT};
use common::config::get_app_configuration;
use common::fwk::RefreshMode;
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

pub fn make_foo_a_sfl_refreshable() -> FooASflT {
    foo_a_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_a_sfl_no_refresh() -> FooASflT {
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
