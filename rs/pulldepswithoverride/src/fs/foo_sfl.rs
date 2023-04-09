use std::mem::replace;

use super::bar_bf;
use common::config::{get_app_configuration, AppCfgInfo};
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{
    static_ref, static_ref_with_override, CfgArcSwapArc, CfgOvd, CfgRefCellRc, RefreshMode, Src,
};
use once_cell::sync::{Lazy, OnceCell};

type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub type FooSflCfgOvd = CfgOvd<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf: fn() -> String,
}

pub fn foo_sfl() -> String {
    let cfg = &FOO_SFL_CFG.get_cfg();
    let FooSflDeps { bar_bf } = &FOO_SFL_DEPS as &FooSflDeps;
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_ret = bar_bf();
    foo_core(a, b, bar_ret)
}

pub static FOO_SFL_DEPS: Lazy<&FooSflDeps> = Lazy::new(|| {
    static_ref(FooSflDeps {
        // bar_bf: || todo!(), // do this before bar_bf exists
        bar_bf, // replace above with this after bar_bf has been created
    })
});

pub fn override_lazy<T>(r: &Lazy<T>, ovd_fn: fn() -> T) {
    unsafe {
        let mr = r as *const Lazy<T> as *mut Lazy<T>;
        *mr = Lazy::new(ovd_fn);
    };
}

pub fn foo_sfl_deps_override() -> &'static FooSflDeps {
    static_ref(FooSflDeps {
        bar_bf: || "bar_bf from foo_sfl_deps_override".to_owned(),
    })
}

pub fn foo_sfl_cfg_override() -> FooSflCfg {
    let src = Src::Fn(|| FooSflCfgInfo {
        a: "a from foo_sfl_cfg_override".to_owned(),
        b: 4200,
    });
    FooSflCfg::new(src, RefreshMode::NoRefresh)
}

pub static FOO_SFL_CFG: Lazy<FooSflCfg> = Lazy::new(|| {
    FooSflCfg::new_boxed_with_cfg_adapter(
        get_app_configuration, // use `|| todo!()` before get_app_configuration exists
        foo_sfl_cfg_adapter,   // use `|_| todo!()` before foo_sfl_cfg_adapter exists
        RefreshMode::NoRefresh,
    )
});

pub static FOO_SFL_CFG_OVERRIDE: OnceCell<FooSflCfgOvd> = OnceCell::new();
pub static FOO_SFL_DEPS_OVERRIDE: OnceCell<FooSflDeps> = OnceCell::new();

// This doesn't necessarily exist initially and may be added later, after the
// app configuration source has been created.
fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}
