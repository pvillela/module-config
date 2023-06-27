use common::{
    config::{get_app_configuration, AppCfgInfo},
    fs_data::FooISflCfgInfo,
    fs_util::foo_core,
};
use std::{rc::Rc, sync::OnceLock};

use super::bar_i_bf;

pub type FooISflT = fn() -> String;

pub struct FooISflDeps {
    pub bar_i_bf: fn() -> String,
}

pub fn foo_i_sfl() -> String {
    // This is to demonstrate using the global config instead of thread-local.
    let _cfg = get_cfg();

    let cfg = FOO_I_SFL_CFG_TL.with(|c| c.clone());
    let FooISflDeps { bar_i_bf } = get_deps();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf();
    foo_core(a, b, bar_res)
}

pub static FOO_I_SFL_CFG: OnceLock<FooISflCfgInfo> = OnceLock::new();

fn get_cfg() -> &'static FooISflCfgInfo {
    FOO_I_SFL_CFG.get_or_init(|| foo_i_sfl_cfg_adapter(&get_app_configuration()))
}

thread_local! {
    pub static FOO_I_SFL_CFG_TL: Rc<FooISflCfgInfo> = Rc::new(get_cfg().clone());
}

pub static FOO_I_SFL_DEPS: OnceLock<FooISflDeps> = OnceLock::new();

fn get_deps() -> &'static FooISflDeps {
    FOO_I_SFL_DEPS.get_or_init(|| {
        FooISflDeps {
            // bar_i_bf: || todo!(), // do this before bar_i_bf exists
            bar_i_bf, // replace above with this after bar_i_bf has been created
        }
    })
}

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
pub fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
