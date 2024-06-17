use common::config::AppCfgInfo;
use common::fs_data::{FooArtIn, FooArtOut};
use common::fs_util::foo_core;
use common::fwk::{
    cfg_deps_art_boot_free_tx_lr, cfg_deps_art_partial_apply_free_tx_arc, AppErr, Make,
    PinBorrowFn2b2Tx, PinFn, RefInto, Tx,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

use super::{bar_art_bf_boot_box, bar_art_bf_boot_lr, BarArtBfCfgInfo, BarArtBfTxT};

pub struct FooArtSflCfgInfo<'a> {
    pub a: &'a str,
    pub b: i32,
}

impl<'a> RefInto<'a, FooArtSflCfgInfo<'a>> for AppCfgInfo {
    fn ref_into(&'a self) -> FooArtSflCfgInfo<'a> {
        FooArtSflCfgInfo {
            a: &self.x,
            b: self.y,
        }
    }
}

pub type FooArtSflTxT = PinBorrowFn2b2Tx<FooArtIn, Result<FooArtOut, AppErr>>;

pub type FooArtSflT = PinFn<FooArtIn, Result<FooArtOut, AppErr>>;

pub struct FooArtSflDeps {
    pub bar_art_bf: Box<BarArtBfTxT>,
}

#[instrument(level = "trace", skip_all)]
pub async fn foo_art_sfl_c<ACFG>(
    cfg_src: impl Make<ACFG>,
    d: impl Deref<Target = FooArtSflDeps>,
    input: FooArtIn,
    tx: &Tx<'_>,
) -> Result<FooArtOut, AppErr>
where
    ACFG: for<'a> RefInto<'a, FooArtSflCfgInfo<'a>>,
{
    let app_cfg_info = cfg_src.make();
    let cfg = app_cfg_info.ref_into();
    let FooArtIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = cfg.a.to_owned();
    let b = cfg.b;
    let bar_res = (d.bar_art_bf)(0, tx).await.unwrap();
    let res = foo_core(a, b, bar_res) + &tx.dummy("foo_art_sfl_c");
    Ok(FooArtOut { res })
}

/// Returns an arced foo_art_sfl closure with free Tx parameter.
pub fn foo_art_sfl_boot_arc<ACFG>(
    c: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> Arc<FooArtSflTxT>
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> RefInto<'a, BarArtBfCfgInfo<'a>>,
    ACFG: for<'a> RefInto<'a, FooArtSflCfgInfo<'a>>,
{
    let b = bar_art_bf_boot_box(c.clone());
    let deps = Arc::new(FooArtSflDeps { bar_art_bf: b });
    cfg_deps_art_partial_apply_free_tx_arc(foo_art_sfl_c, c, deps)
}

/// Returns a leaked static reference to a foo_at_sfl closure with free Tx parameter.
pub fn foo_art_sfl_boot_lr<ACFG>(
    c: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> &'static FooArtSflTxT
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> RefInto<'a, BarArtBfCfgInfo<'a>>,
    ACFG: for<'a> RefInto<'a, FooArtSflCfgInfo<'a>>,
{
    let b = Box::new(bar_art_bf_boot_lr(c.clone()));
    let deps = FooArtSflDeps { bar_art_bf: b };
    cfg_deps_art_boot_free_tx_lr(foo_art_sfl_c, c, deps)
}
