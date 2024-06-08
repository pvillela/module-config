use crate::fs::foo_a_sfl_init_no_refresh;

/// Initialize service flows, let stereotypes initialize their dependencies.
pub fn init_a_no_refresh() {
    println!("init_a_no_refresh() has been called");

    foo_a_sfl_init_no_refresh();
}
