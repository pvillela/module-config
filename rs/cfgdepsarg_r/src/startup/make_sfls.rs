use crate::fs::boot::{foo_a_sfl_boot, foo_aw_sfl_boot, foo_sfl_boot};
use crate::fs::{FooASflT, FooAwSflT, FooSflT, FooSflTr};
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

pub fn get_foo_sfl_refreshable() -> FooSflTr {
    static FOO_SFL_REFRESHABLE: OnceLock<FooSflT> = OnceLock::new();
    FOO_SFL_REFRESHABLE
        .get_or_init(|| {
            foo_sfl_boot(
                get_app_configuration,
                RefreshMode::Refreshable(Duration::from_millis(60)),
            )
        })
        .as_ref()
}

pub fn make_foo_sfl_no_refresh() -> FooSflT {
    foo_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
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

pub fn make_foo_aw_sfl_refreshable() -> FooAwSflT {
    foo_aw_sfl_boot(
        get_app_configuration,
        RefreshMode::Refreshable(Duration::from_millis(60)),
    )
}

pub fn make_foo_aw_sfl_no_refresh() -> FooAwSflT {
    foo_aw_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}
