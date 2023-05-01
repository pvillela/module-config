use crate::fs::boot::foo_i_sfl_init;
use common::config::get_app_configuration;

/// Initialize service flows, let stereotypes initialize their dependencies.
pub fn init_i() {
    println!("init_i() has been called");
    let c = get_app_configuration;

    foo_i_sfl_init(c);
}
