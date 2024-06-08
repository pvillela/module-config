use crate::fs;
use crate::fs::{FooASflT, FooAwSflT, FooSflT};
use common::config::get_app_configuration;
use common::fwk::RefreshMode;
use std::time::Duration;

pub fn make_foo_sfl_refreshable() -> FooSflT {
    fs::foo_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_sfl_no_refresh() -> FooSflT {
    fs::foo_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}

pub fn make_foo_a_sfl_refreshable() -> FooASflT {
    fs::foo_a_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_a_sfl_no_refresh() -> FooASflT {
    fs::foo_a_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}

pub fn make_foo_aw_sfl_refreshable() -> FooAwSflT {
    fs::foo_aw_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_aw_sfl_no_refresh() -> FooAwSflT {
    fs::foo_aw_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}
