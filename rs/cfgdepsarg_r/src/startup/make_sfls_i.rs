use crate::fs::boot::{foo_i_sfl_boot, foo_i_sfl_boot_lr};
use crate::fs::FooISflT;
use common::config::get_app_configuration;
use std::sync::OnceLock;

pub fn make_foo_i_sfl() -> Box<FooISflT> {
    foo_i_sfl_boot(get_app_configuration)
}

pub fn get_foo_i_sfl() -> &'static FooISflT {
    static FOO_I_SFL: OnceLock<&FooISflT> = OnceLock::new();
    FOO_I_SFL.get_or_init(|| foo_i_sfl_boot_lr(get_app_configuration))
}
