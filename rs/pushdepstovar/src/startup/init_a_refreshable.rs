use std::time::Duration;

use crate::fs::boot::{bar_a_bf_init_refreshable, foo_a_sfl_init_refreshable};

/// Sould only initialize service flows and let stereotypes initialize their dependencies,
/// but here we initialize bar_a_bf redundantly to show it's OK to do so.
pub fn init_a_refreshable() {
    println!("init_a_no_cache() has been called");

    const CACHE_TTL: Duration = Duration::from_millis(275);

    foo_a_sfl_init_refreshable(CACHE_TTL);
    println!("Redundant init of foo_a_sfl from init_no_cache(), with no effect:");
    foo_a_sfl_init_refreshable(CACHE_TTL);
    println!("Redundant init of bar_a_bf from init_no_cache(), with no effect:");
    bar_a_bf_init_refreshable(CACHE_TTL);
}
