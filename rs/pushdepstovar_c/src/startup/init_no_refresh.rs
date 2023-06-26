use crate::fs::boot::foo_sfl_init_cached;
use common::config::get_app_configuration;

/// Initialize service flows, let stereotypes initialize their dependencies.
pub fn init_no_refresh() {
    println!("init_with_cache() has been called");
    let c = get_app_configuration;

    foo_sfl_init_cached(c);
}
