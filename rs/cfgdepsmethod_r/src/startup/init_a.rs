//! These functions are defined here only for demonstration purposes, but they are not
//! used in any of the binaries or tests. Instead, [get_foo_a_sfl_s_no_refresh] and
//! [get_foo_a_sfl_s_cached] are used directly.

use crate::fs::boot::{get_foo_a_sfl_s_cached, get_foo_a_sfl_s_no_refresh};
use common::fs_data::{FooAIn, FooAOut};

#[allow(unused)]
async fn foo_a_sfl_no_refresh(input: FooAIn) -> FooAOut {
    let foo_a_sfl_s = get_foo_a_sfl_s_no_refresh();
    foo_a_sfl_s.run(input).await
}

#[allow(unused)]
async fn foo_a_sfl_cached(input: FooAIn) -> FooAOut {
    let foo_a_sfl_s = get_foo_a_sfl_s_cached();
    foo_a_sfl_s.run(input).await
}
