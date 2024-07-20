use super::{bar_at_bf_boot_lr, BarAtBfTxT};
use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAtIn, FooAtOut, FooAtSflCfgInfo};
use common::fs_util::foo_core;
use common::fwk::{
    cfg_deps_at_boot_free_tx_arc, cfg_deps_at_boot_free_tx_lr, AppErr, CfgArcSwapArc, CfgDeps,
    PinBorrowFn2b2Tx, PinFn, RefreshMode, DummyTx,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type FooAtSflTxT = PinBorrowFn2b2Tx<FooAtIn, Result<FooAtOut, AppErr>>;

pub type FooAtSflT = PinFn<FooAtIn, Result<FooAtOut, AppErr>>;

pub type FooAtSflCfg = CfgArcSwapArc<FooAtSflCfgInfo>;

pub struct FooAtSflDeps {
    pub bar_at_bf: Box<BarAtBfTxT>,
}

pub type FooAtSflS = CfgDeps<FooAtSflCfg, FooAtSflDeps>;

#[instrument(level = "trace", skip(s, tx))]
pub async fn foo_at_sfl_c(
    s: impl Deref<Target = FooAtSflS> + Send + Sync,
    input: FooAtIn,
    tx: &DummyTx<'_>,
) -> Result<FooAtOut, AppErr> {
    let c = s.cfg.get_cfg();
    let d = &s.deps;
    let FooAtIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.a.clone();
    let b = c.b;
    let bar_res = (d.bar_at_bf)(0, tx).await.unwrap();
    let res = foo_core(a, b, bar_res) + &tx.dummy("foo_at_sfl_c");
    Ok(FooAtOut { res })
}

fn foo_at_sfl_cfg_atdapter(app_cfg: &AppCfgInfo) -> FooAtSflCfgInfo {
    FooAtSflCfgInfo {
        a: app_cfg.x.clone(),
        b: app_cfg.y,
    }
}

/// Returns an arced foo_at_sfl closure with free Tx parameter.
pub fn foo_at_sfl_boot_arc(
    app_cfg: fn() -> AppCfgInfo,
    refresh_mode: RefreshMode,
) -> Arc<FooAtSflTxT> {
    let b = fs::bar_at_bf_boot_box(app_cfg, refresh_mode.clone());
    let deps = FooAtSflDeps { bar_at_bf: b };

    cfg_deps_at_boot_free_tx_arc(
        foo_at_sfl_c,
        FooAtSflCfg::new_boxed_with_cfg_adapter,
        foo_at_sfl_cfg_atdapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}

/// Returns a leaked static reference to a foo_at_sfl closure with free Tx parameter.
pub fn foo_at_sfl_boot_lr(
    app_cfg: fn() -> AppCfgInfo,
    refresh_mode: RefreshMode,
) -> &'static FooAtSflTxT {
    let b = Box::new(bar_at_bf_boot_lr(app_cfg, refresh_mode.clone()));
    let deps = FooAtSflDeps { bar_at_bf: b };

    cfg_deps_at_boot_free_tx_lr(
        foo_at_sfl_c,
        FooAtSflCfg::new_boxed_with_cfg_adapter,
        foo_at_sfl_cfg_atdapter,
        app_cfg,
        refresh_mode.clone(),
        deps,
    )
}
