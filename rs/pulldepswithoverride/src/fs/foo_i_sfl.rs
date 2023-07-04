use common::{
    config::{get_app_configuration, AppCfgInfo},
    fs_data::FooISflCfgInfo,
    fs_util::foo_core,
    fwk::CfgDepsS,
};
use std::rc::Rc;

use super::{bar_i_bf, BAR_I_BF_CFG};

pub type FooISflT = fn() -> String;

pub struct FooISflDeps {
    pub bar_i_bf: fn() -> String,
}

pub fn foo_i_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _cfg = FOO_I_SFL_CFG_DEPS.get_cfg();

    let cfg = FOO_I_SFL_CFG_TL.with(|c| c.clone());
    let FooISflDeps { bar_i_bf } = FOO_I_SFL_CFG_DEPS.get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}

pub static FOO_I_SFL_CFG_DEPS: CfgDepsS<FooISflCfgInfo, FooISflDeps> = CfgDepsS::lazy_init(
    || foo_i_sfl_cfg_adapter(&get_app_configuration()),
    || {
        BAR_I_BF_CFG.prime(); // optional, just in case we want to force up-front app initialization.
        FooISflDeps {
            // bar_i_bf: || todo!(), // do this before bar_i_bf exists
            bar_i_bf, // replace above with this after bar_i_bf has been created
        }
    },
);

thread_local! {
    pub static FOO_I_SFL_CFG_TL: Rc<FooISflCfgInfo> = Rc::new(FOO_I_SFL_CFG_DEPS.get_cfg().clone());
}

pub fn get_foo_i_sfl_raw(cfg: FooISflCfgInfo, deps: FooISflDeps) -> FooISflT {
    let _ = FOO_I_SFL_CFG_DEPS.set_cfg_lenient(cfg);
    let _ = FOO_I_SFL_CFG_DEPS.set_deps_lenient(deps);
    foo_i_sfl
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
