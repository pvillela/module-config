use crate::fs::boot::foo_i_sfl_boot;
use crate::fs::FooISflT;
use common::config::get_app_configuration;

pub fn make_foo_i_sfl() -> Box<FooISflT> {
    foo_i_sfl_boot(get_app_configuration)
}
