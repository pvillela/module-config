use crate::fs::{foo_art_sfl_boot_arc, foo_art_sfl_boot_lr, FooAtSflT};
use common::config::{get_app_configuration, get_pool};
use common::fwk::{fn2_arc_with_transaction, fn2_static_ref_with_transaction};
use std::sync::OnceLock;

pub fn make_foo_art_sfl() -> Box<FooAtSflT> {
    let f_free = foo_art_sfl_boot_arc(get_app_configuration);
    Box::new(fn2_arc_with_transaction(get_pool(), f_free))
}

pub fn get_foo_art_sfl() -> &'static FooAtSflT {
    static FOO_ART_SFL: OnceLock<&FooAtSflT> = OnceLock::new();
    FOO_ART_SFL.get_or_init(|| {
        let f_free = foo_art_sfl_boot_lr(get_app_configuration);
        Box::leak(Box::new(fn2_static_ref_with_transaction(
            get_pool(),
            f_free,
        )))
    })
}
