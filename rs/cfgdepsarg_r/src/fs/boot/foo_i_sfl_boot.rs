use super::bar_i_bf_boot;
use crate::fs::{foo_i_sfl_c, FooISflDeps, FooISflS, FooISflT};
use bar_i_bf_boot::bar_i_bf_boot_xr;
use common::config::AppCfgInfo;
use common::fs_data::FooISflCfgInfo;
use std::sync::{Arc, OnceLock};

fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

pub fn foo_i_sfl_boot(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<FooISflT> {
    let cfg = foo_i_sfl_cfg_adapter(&app_cfg());
    let deps = FooISflDeps {
        bar_i_bf: bar_i_bf_boot(app_cfg),
    };

    // We are forced to use Arc below instead of Rc to conform to Send/Sync type bounds
    // needed for the _r variant below.
    let foo_i_sfl_s = Arc::new(FooISflS { cfg: cfg, deps });

    let f = move || foo_i_sfl_c(&foo_i_sfl_s.clone());
    Box::new(f)
}

// The only benefit of this version over the above is that it saves an Arc clone for each call to the returned function.
pub fn foo_i_sfl_boot_r(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<FooISflT> {
    static FOO_I_SFL_S: OnceLock<FooISflS> = OnceLock::new();
    let foo_i_sfl_s = FOO_I_SFL_S.get_or_init(|| {
        let cfg = foo_i_sfl_cfg_adapter(&app_cfg());
        let deps = FooISflDeps {
            bar_i_bf: bar_i_bf_boot(app_cfg),
        };
        FooISflS { cfg: cfg, deps }
    });
    let f = move || foo_i_sfl_c(&foo_i_sfl_s);
    Box::new(f)
}

// The only benefit of this version over the above is that it saves an Arc clone for this and its dependencies
// for each call to the returned function.
pub fn foo_i_sfl_boot_xr(app_cfg: fn() -> Arc<AppCfgInfo>) -> Box<FooISflT> {
    static FOO_I_SFL_S_X: OnceLock<FooISflS> = OnceLock::new();
    let foo_i_sfl_s = FOO_I_SFL_S_X.get_or_init(|| {
        let cfg = foo_i_sfl_cfg_adapter(&app_cfg());
        let deps = FooISflDeps {
            bar_i_bf: bar_i_bf_boot_xr(app_cfg),
        };
        FooISflS { cfg: cfg, deps }
    });
    let f = move || foo_i_sfl_c(&foo_i_sfl_s);
    Box::new(f)
}
