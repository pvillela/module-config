use crate::fs::boot::foo_at_sfl_boot_lr;
use crate::fs::FooAtSflT;
use common::config::{get_app_configuration, get_pool};
use common::fwk::{fn2_static_ref_with_transaction, RefreshMode};
use std::sync::OnceLock;
use std::time::Duration;

// pub fn make_foo_at_sfl_no_refresh() -> Box<FooAtSflT> {
//     let f_free = foo_at_sfl_boot(get_app_configuration, RefreshMode::NoRefresh);
//     Box::new(fn2_with_transaction(get_pool(), f_free))
// }

// pub fn make_foo_at_sfl_refreshable() -> Box<FooAtSflT> {
//     foo_at_sfl_boot(
//         get_app_configuration,
//         RefreshMode::Refreshable(Duration::from_millis(60)),
//     )
// }

pub fn get_foo_at_sfl_no_refresh() -> &'static FooAtSflT {
    static FOO_AT_SFL_NO_REFRESH: OnceLock<&FooAtSflT> = OnceLock::new();
    FOO_AT_SFL_NO_REFRESH.get_or_init(|| {
        let f_free = foo_at_sfl_boot_lr(get_app_configuration, RefreshMode::NoRefresh);
        Box::leak(Box::new(fn2_static_ref_with_transaction(
            get_pool(),
            f_free,
        )))
    })
}

pub fn get_foo_at_sfl_refreshable() -> &'static FooAtSflT {
    static FOO_AT_SFL_REFRESHABLE: OnceLock<&FooAtSflT> = OnceLock::new();
    FOO_AT_SFL_REFRESHABLE.get_or_init(|| {
        let f_free = foo_at_sfl_boot_lr(
            get_app_configuration,
            RefreshMode::Refreshable(Duration::from_millis(60)),
        );
        Box::leak(Box::new(fn2_static_ref_with_transaction(
            get_pool(),
            f_free,
        )))
    })
}
