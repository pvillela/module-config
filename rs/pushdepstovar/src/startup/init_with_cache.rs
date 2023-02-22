use crate::config::get_app_configuration;
use crate::fs::boot::{bar_bf_init_cached, foo_sfl_init_cached};

pub fn init_with_cache() {
    println!("init_with_cache() has been called");
    let c = get_app_configuration;

    foo_sfl_init_cached(c);
    bar_bf_init_cached(c);
}
