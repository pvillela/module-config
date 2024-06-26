use common::config::get_app_configuration;

use crate::fs::{get_foo_i_sfl_with_app_cfg, FooISflT};

/// Initialize service flows, let stereotypes initialize their dependencies.
pub fn get_foo_i_sfl() -> FooISflT {
    get_foo_i_sfl_with_app_cfg(&get_app_configuration())
}

pub fn init_foo_i_sfl() {
    get_foo_i_sfl();
}
