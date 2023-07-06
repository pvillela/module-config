use crate::fs::boot::foo_sfl_boot;
use crate::fs::FooSflT;
use common::config::get_app_configuration;
use common::fwk::RefreshMode;
use std::time::Duration;

pub fn make_foo_sfl_refreshable() -> Box<FooSflT> {
    foo_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_sfl_no_refresh() -> Box<FooSflT> {
    foo_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}
