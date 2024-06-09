use super::{bar_i_bf_boot_lr, BarIBfT};
use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::FooISflCfgInfo;
use common::fs_util::foo_core;
use common::fwk::{cfg_deps_i_boot, cfg_deps_i_boot_lr, CfgDeps};

pub type FooISflT = dyn Fn(()) -> String + Send + Sync;

pub struct FooISflDeps {
    pub bar_i_bf: Box<BarIBfT>,
}

pub type FooISflS = CfgDeps<FooISflCfgInfo, FooISflDeps>;

pub fn foo_i_sfl_c(s: &FooISflS, _: ()) -> String {
    let FooISflDeps { bar_i_bf } = &s.deps;
    let cfg = &s.cfg;
    let a = cfg.a.clone();
    let b = cfg.b;
    let bar_res = bar_i_bf(());
    foo_core(a, b, bar_res)
}

fn foo_i_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooISflCfgInfo {
    FooISflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Coded without use of [cfg_deps_boot_i].
/// Returns a boxed foo_i_sfl closure.
pub fn foo_i_sfl_boot_by_hand(app_cfg: &AppCfgInfo) -> Box<FooISflT> {
    let cfg = foo_i_sfl_cfg_adapter(app_cfg);
    let deps = FooISflDeps {
        bar_i_bf: fs::bar_i_bf_boot(app_cfg),
    };
    let foo_i_sfl_s = FooISflS { cfg: cfg, deps };
    let f = move |_| foo_i_sfl_c(&foo_i_sfl_s, ());
    Box::new(f)
}

/// Returns a boxed foo_i_sfl closure.
pub fn foo_i_sfl_boot(app_cfg: &AppCfgInfo) -> Box<FooISflT> {
    let deps = FooISflDeps {
        bar_i_bf: fs::bar_i_bf_boot(app_cfg),
    };
    cfg_deps_i_boot(foo_i_sfl_c, foo_i_sfl_cfg_adapter, app_cfg, deps)
}

/// Coded without use of [cfg_deps_boot_i_lr].
/// Returns a leaked static reference to a foo_i_sfl closure.
/// Since bar_i_bf has no dependencies, there is no benefit over _boot.
pub fn foo_i_sfl_boot_lr_by_hand(app_cfg: &AppCfgInfo) -> &'static FooISflT {
    let cfg = foo_i_sfl_cfg_adapter(app_cfg);
    let deps = FooISflDeps {
        bar_i_bf: Box::new(bar_i_bf_boot_lr(app_cfg)),
    };
    let foo_i_sfl_s: &FooISflS = Box::leak(Box::new(FooISflS { cfg, deps }));
    let f = move |_| foo_i_sfl_c(foo_i_sfl_s, ());
    Box::leak(Box::new(f))
}

/// Returns a leaked static reference to a foo_i_sfl closure.
/// Since bar_i_bf has no dependencies, there is no benefit over _boot.
pub fn foo_i_sfl_boot_lr(app_cfg: &AppCfgInfo) -> &'static FooISflT {
    let deps = FooISflDeps {
        bar_i_bf: Box::new(bar_i_bf_boot_lr(app_cfg)),
    };
    cfg_deps_i_boot_lr(foo_i_sfl_c, foo_i_sfl_cfg_adapter, app_cfg, deps)
}
