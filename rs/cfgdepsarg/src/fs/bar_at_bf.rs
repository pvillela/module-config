use common::config::AppCfgInfo;
use common::fs_data::BarAtBfCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_deps_at_boot_free_tx_box, cfg_deps_at_boot_free_tx_lr, AppErr, CfgArcSwapArc, CfgDeps,
    PinBorrowFn2b2Tx, RefreshMode, DummyTx,
};
use std::ops::Deref;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type BarAtBfTxT = PinBorrowFn2b2Tx<u64, Result<String, AppErr>>;

pub type BarAtBfCfg = CfgArcSwapArc<BarAtBfCfgInfo>;

pub type BarAtBfS = CfgDeps<BarAtBfCfg, ()>;

#[instrument(level = "trace", skip(s, tx))]
pub async fn bar_at_bf_c(
    s: impl Deref<Target = BarAtBfS> + Send + Sync,
    sleep_millis: u64,
    tx: &DummyTx<'_>,
) -> Result<String, AppErr> {
    let cfg = s.cfg.get_cfg();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.clone();
    let res = bar_core(u, v) + &tx.dummy("bar_at_bf_c");
    Ok(res)
}

fn bar_at_bf_cfg_adapter(app_cfg: &AppCfgInfo) -> BarAtBfCfgInfo {
    BarAtBfCfgInfo {
        u: app_cfg.y,
        v: app_cfg.x.clone(),
    }
}

/// Returns a boxed bar_at_bf closure with free Tx parameter.
pub fn bar_at_bf_boot_box(
    app_cfg: fn() -> AppCfgInfo,
    refresh_mode: RefreshMode,
) -> Box<BarAtBfTxT> {
    cfg_deps_at_boot_free_tx_box(
        bar_at_bf_c,
        BarAtBfCfg::new_boxed_with_cfg_adapter,
        bar_at_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}

/// Returns a leaked static reference to a bar_at_bf closure with free Tx parameter.
pub fn bar_at_bf_boot_lr(
    app_cfg: fn() -> AppCfgInfo,
    refresh_mode: RefreshMode,
) -> &'static BarAtBfTxT {
    cfg_deps_at_boot_free_tx_lr(
        bar_at_bf_c,
        BarAtBfCfg::new_boxed_with_cfg_adapter,
        bar_at_bf_cfg_adapter,
        app_cfg,
        refresh_mode.clone(),
        (),
    )
}
