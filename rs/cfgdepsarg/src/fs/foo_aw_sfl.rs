use super::BarAwBfT;
use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAwIn, FooAwOut, FooAwSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    box_pin_async_fn_wss, cfg_deps_aw_boot, CfgDeps, CfgRefCellId, PinFnWss, RefreshMode,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

pub type FooAwSflT = PinFnWss<FooAwIn, FooAwOut>;

pub type FooAwSflCfg = CfgRefCellId<FooAwSflCfgInfo>;

// #[derive(Clone)]
pub struct FooAwSflDeps {
    pub bar_aw_bf: Box<BarAwBfT>,
}

pub type FooAwSflS = CfgDeps<FooAwSflCfg, FooAwSflDeps>;

pub async fn foo_aw_sfl_c(s: impl Deref<Target = FooAwSflS>, input: FooAwIn) -> FooAwOut {
    let c = s.cfg.get_cfg();
    let d = &s.deps;
    let FooAwIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_aw_bf)(0).await;
    let res = foo_core(a, b, bar_res);
    FooAwOut { res }
}

fn foo_aw_sfl_cfg_adapter(app_cfg: &AppCfgInfo) -> FooAwSflCfgInfo {
    FooAwSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Coded without use of [cfg_deps_boot_aw].
/// Returns a foo_aw_sfl stereotype instance.
pub fn foo_aw_sfl_boot_by_hand(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooAwSflT> {
    let cfg = FooAwSflCfg::new_boxed_with_cfg_adapter(
        app_cfg,
        foo_aw_sfl_cfg_adapter,
        refresh_mode.clone(),
    );
    let deps = FooAwSflDeps {
        bar_aw_bf: fs::bar_aw_bf_boot(app_cfg, refresh_mode.clone()),
    };
    let foo_aw_sfl_s = Arc::new(FooAwSflS { cfg, deps });
    let f = move |input| foo_aw_sfl_c(foo_aw_sfl_s.clone(), input);
    box_pin_async_fn_wss(f)
}

/// Returns a foo_aw_sfl stereotype instance.
pub fn foo_aw_sfl_boot(
    app_cfg: fn() -> Arc<AppCfgInfo>,
    refresh_mode: RefreshMode,
) -> Box<FooAwSflT> {
    let deps = FooAwSflDeps {
        bar_aw_bf: fs::bar_aw_bf_boot(app_cfg, refresh_mode.clone()),
    };
    cfg_deps_aw_boot(
        foo_aw_sfl_c,
        FooAwSflCfg::new_boxed_with_cfg_adapter,
        foo_aw_sfl_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}
