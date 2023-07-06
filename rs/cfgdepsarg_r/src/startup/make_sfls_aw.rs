use crate::fs::boot::foo_aw_sfl_boot;
use crate::fs::FooAwSflT;
use common::config::get_app_configuration;
use common::fwk::RefreshMode;

pub fn make_foo_aw_sfl_no_refresh() -> FooAwSflT {
    foo_aw_sfl_boot(get_app_configuration, RefreshMode::NoRefresh)
}
