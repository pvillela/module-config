use crate::fs::{boot::get_foo_a_sfl_wtih_app_cfg, FooASflT};
use common::{config::get_app_configuration, fwk::RefreshMode};
use std::time::Duration;

pub fn get_foo_a_sfl_no_refresh() -> FooASflT {
    get_foo_a_sfl_wtih_app_cfg(get_app_configuration, RefreshMode::NoRefresh)
}

pub fn get_foo_a_sfl_with_cache() -> FooASflT {
    get_foo_a_sfl_wtih_app_cfg(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(150)),
    )
}