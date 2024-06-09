use super::{bar_bf_boot_lr, BarBfT};
use crate::fs;
use common::config::{AppCfg, AppCfgInfo};
use common::fs_data::FooSflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{cfg_deps_boot, cfg_deps_boot_lr, CfgArcSwapArc, CfgDeps};

pub type FooSflT = dyn Fn(()) -> String + Send + Sync;

pub type FooSflCfg = CfgArcSwapArc<FooSflCfgInfo>;

pub struct FooSflDeps {
    pub bar_bf: Box<BarBfT>,
}

pub type FooSflS = CfgDeps<FooSflCfg, FooSflDeps>;

pub fn foo_sfl_c(s: &FooSflS, _: ()) -> String {
    let FooSflDeps { bar_bf } = &s.deps;
    let cfg = s.cfg.get_cfg();
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_bf(());
    foo_core(a, b, bar_res)
}

fn foo_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooSflCfgInfo {
    FooSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Coded without use of [cfg_deps_boot].
/// Returns a boxed foo_sfl closure.
pub fn foo_sfl_boot_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> Box<FooSflT> {
    let app_cfg1 = app_cfg.clone();
    let cfg = FooSflCfg::new_boxed_with_cfg_adapter(
        app_cfg1.app_src,
        foo_sfl_cfg_adapter,
        app_cfg1.refresh_mode,
    );
    let deps = FooSflDeps {
        bar_bf: fs::bar_bf_boot(app_cfg),
    };
    let foo_sfl_s = FooSflS { cfg, deps };
    let f = move |_| foo_sfl_c(&foo_sfl_s, ());
    Box::new(f)
}

/// Returns a boxed foo_sfl closure.
pub fn foo_sfl_boot(app_cfg: AppCfg<AppCfgInfo>) -> Box<FooSflT> {
    let cfg_factory = FooSflCfg::new_boxed_with_cfg_adapter;
    let deps = FooSflDeps {
        bar_bf: fs::bar_bf_boot(app_cfg.clone()),
    };
    cfg_deps_boot(foo_sfl_c, cfg_factory, foo_sfl_cfg_adapter, app_cfg, deps)
}

/// Coded without use of [cfg_deps_boot_lr].
/// Returns a leaked static reference to a foo_sfl closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn foo_sfl_boot_lr_by_hand(app_cfg: AppCfg<AppCfgInfo>) -> &'static FooSflT {
    let app_cfg1 = app_cfg.clone();
    let cfg = FooSflCfg::new_boxed_with_cfg_adapter(
        app_cfg1.app_src,
        foo_sfl_cfg_adapter,
        app_cfg1.refresh_mode,
    );
    let deps = FooSflDeps {
        bar_bf: Box::new(bar_bf_boot_lr(app_cfg)),
    };
    let foo_sfl_s: &FooSflS = Box::leak(Box::new(FooSflS { cfg, deps }));
    let f = move |_| foo_sfl_c(foo_sfl_s, ());
    Box::leak(Box::new(f))
}

/// Returns a leaked static reference to a foo_sfl closure.
/// Since bar_bf has no dependencies, there is no benefit over _boot.
pub fn foo_sfl_boot_lr(app_cfg: AppCfg<AppCfgInfo>) -> &'static FooSflT {
    let cfg_factory = FooSflCfg::new_boxed_with_cfg_adapter;
    let deps = FooSflDeps {
        bar_bf: Box::new(bar_bf_boot_lr(app_cfg.clone())),
    };
    cfg_deps_boot_lr(foo_sfl_c, cfg_factory, foo_sfl_cfg_adapter, app_cfg, deps)
}
