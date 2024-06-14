use common::config::AppCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_deps_ast_boot_free_tx_box, cfg_deps_ast_boot_free_tx_lr, AppErr, CfgDeps, PinBorrowFn2b2Tx,
    Tx,
};
use std::ops::Deref;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type BarAstBfTxT = PinBorrowFn2b2Tx<u64, Result<String, AppErr>>;

pub type BarAstBfS = CfgDeps<AppCfgInfo, ()>;

#[instrument(level = "trace", skip(s, tx))]
pub async fn bar_ast_bf_c(
    s: impl Deref<Target = BarAstBfS>,
    sleep_millis: u64,
    tx: &Tx<'_>,
) -> Result<String, AppErr> {
    let cfg = &s.cfg;
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.y;
    let v = cfg.x.clone();
    let res = bar_core(u, v) + &tx.dummy("bar_ast_bf_c");
    Ok(res)
}

/// Returns a boxed bar_ast_bf closure with free Tx parameter.
pub fn bar_ast_bf_boot(app_cfg: AppCfgInfo) -> Box<BarAstBfTxT> {
    cfg_deps_ast_boot_free_tx_box(bar_ast_bf_c, app_cfg, ())
}

/// Returns a leaked static reference to a bar_ast_bf closure with free Tx parameter.
pub fn bar_ast_bf_boot_lr(app_cfg: AppCfgInfo) -> &'static BarAstBfTxT {
    cfg_deps_ast_boot_free_tx_lr(bar_ast_bf_c, app_cfg, ())
}
