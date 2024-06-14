use super::{bar_ast_bf_boot_lr, BarAtBfTxT};
use crate::fs;
use common::config::AppCfgInfo;
use common::fs_data::{FooAtIn, FooAtOut};
use common::fs_util::foo_core;
use common::fwk::{
    cfg_deps_ast_boot_free_tx_arc, cfg_deps_ast_boot_free_tx_lr, AppErr, CfgArcSwapArc, CfgDeps,
    PinBorrowFn2b2Tx, PinFn, Tx,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type FooAstSflTxT = PinBorrowFn2b2Tx<FooAtIn, Result<FooAtOut, AppErr>>;

pub type FooAstSflT = PinFn<FooAtIn, Result<FooAtOut, AppErr>>;

pub type FooAstSflCfg = CfgArcSwapArc<AppCfgInfo>;

pub struct FooAstSflDeps {
    pub bar_ast_bf: Box<BarAtBfTxT>,
}

pub type FooAstSflS = CfgDeps<AppCfgInfo, FooAstSflDeps>;

#[instrument(level = "trace", skip(s, tx))]
pub async fn foo_ast_sfl_c(
    s: impl Deref<Target = FooAstSflS> + Send + Sync,
    input: FooAtIn,
    tx: &Tx<'_>,
) -> Result<FooAtOut, AppErr> {
    let c = &s.cfg;
    let d = &s.deps;
    let FooAtIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = c.x.clone();
    let b = c.y;
    let bar_res = (d.bar_ast_bf)(0, tx).await.unwrap();
    let res = foo_core(a, b, bar_res) + &tx.dummy("foo_ast_sfl_c");
    Ok(FooAtOut { res })
}

/// Returns an arced foo_ast_sfl closure with free Tx parameter.
pub fn foo_ast_sfl_boot_arc(app_cfg: AppCfgInfo) -> Arc<FooAstSflTxT> {
    let b = fs::bar_ast_bf_boot_box(app_cfg.clone());
    let deps = FooAstSflDeps { bar_ast_bf: b };

    cfg_deps_ast_boot_free_tx_arc(foo_ast_sfl_c, app_cfg, deps)
}

/// Returns a leaked static reference to a foo_ast_sfl closure with free Tx parameter.
pub fn foo_ast_sfl_boot_lr(app_cfg: AppCfgInfo) -> &'static FooAstSflTxT {
    let b = Box::new(bar_ast_bf_boot_lr(app_cfg.clone()));
    let deps = FooAstSflDeps { bar_ast_bf: b };

    cfg_deps_ast_boot_free_tx_lr(foo_ast_sfl_c, app_cfg, deps)
}
