use crate::config::get_app_configuration;
use crate::fs::boot::foo_sfl_init_cached;

/// Initialize service flows, let stereotypes initialize their dependencies.
pub fn init_with_cache() {
    println!("init_with_cache() has been called");
    let c = get_app_configuration;

    foo_sfl_init_cached(c);
}
