use crate::fs::foo_sfl_init_no_refresh;

/// Initialize service flows, let stereotypes initialize their dependencies.
pub fn init_no_refresh() {
    println!("init_with_cache() has been called");

    foo_sfl_init_no_refresh();
}
