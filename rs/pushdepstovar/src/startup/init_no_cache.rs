use crate::config::get_app_configuration;
use crate::fs::boot::{bar_bf_init_refreshable, foo_sfl_init_refreshable};

/// Sould only initialize service flows and let stereotypes initialize their dependencies,
/// but here we initialize bar_bf redundantly to show it's OK to do so.
pub fn init_no_cache() {
    println!("init_no_cache() has been called");
    let c = get_app_configuration;

    foo_sfl_init_refreshable(c);
    println!("Redundant init of foo_sfl from init_no_cache(), with no effect:");
    foo_sfl_init_refreshable(c);
    println!("Redundant init of bar_bf from init_no_cache(), with no effect:");
    bar_bf_init_refreshable(c);
}
