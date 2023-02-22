use crate::config::get_app_configuration;
use crate::fs::boot::{bar_bf_init_refreshable, foo_sfl_init_refreshable};

pub fn init_no_cache() {
    println!("init_no_cache() has been called");
    let c = get_app_configuration;

    foo_sfl_init_refreshable(c);
    bar_bf_init_refreshable(c);
}
