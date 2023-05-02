use crate::fs::boot::{bar_a_bf_init_refreshable, foo_a_sfl_init_refreshable};
use common::config::get_app_configuration;

/// Should only initialize service flows and let stereotypes initialize their dependencies,
/// but here we initialize bar_a_bf redundantly to show it's OK to do so.
pub fn init_a_refreshable(refresh_millis: u64) {
    println!("init_a_no_cache() has been called");
    let c = get_app_configuration;

    foo_a_sfl_init_refreshable(c, refresh_millis);
    println!("Redundant init of foo_a_sfl from init_no_cache(), with no effect:");
    foo_a_sfl_init_refreshable(c, refresh_millis);
    println!("Redundant init of bar_a_bf from init_no_cache(), with no effect:");
    bar_a_bf_init_refreshable(c, refresh_millis);
}
