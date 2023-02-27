use crate::config::get_app_configuration;
use crate::fs::boot::foo_a_sfl_init_no_refresh;

/// Initialize service flows, let stereotypes initialize their dependencies.
pub fn init_a_no_refresh() {
    println!("init_a_no_refresh() has been called");
    let c = get_app_configuration;

    foo_a_sfl_init_no_refresh(c);
}
