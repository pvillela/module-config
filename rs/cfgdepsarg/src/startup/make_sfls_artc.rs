use crate::fs::{foo_artc_sfl_boot_arc, CfgSrc, FooAtSflT};
use common::config::{get_app_configuration, get_pool, AppCfgInfo};
use common::fwk::fn2_arc_with_transaction;

struct Ctx;

impl CfgSrc for Ctx {
    type AppCfg = AppCfgInfo;

    fn cfg_src() -> Self::AppCfg {
        get_app_configuration()
    }
}

pub fn make_foo_artc_sfl() -> Box<FooAtSflT> {
    let f_free = foo_artc_sfl_boot_arc::<Ctx>();
    Box::new(fn2_arc_with_transaction(get_pool(), f_free))
}

// pub fn get_foo_artc_sfl() -> &'static FooAtSflT {
//     static FOO_ART_SFL: OnceLock<&FooAtSflT> = OnceLock::new();
//     FOO_ART_SFL.get_or_init(|| {
//         let f_free = foo_artc_sfl_boot_lr(get_app_configuration);
//         Box::leak(Box::new(fn2_static_ref_with_transaction(
//             get_pool(),
//             f_free,
//         )))
//     })
// }
