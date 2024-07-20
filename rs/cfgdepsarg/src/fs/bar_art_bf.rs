use common::config::AppCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{
    cfg_deps_art_boot_free_tx_lr, cfg_deps_art_partial_apply_free_tx_box, AppErr, Make,
    PinBorrowFn2b2Tx, RefInto, DummyTx,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type BarArtBfTxT = PinBorrowFn2b2Tx<u64, Result<String, AppErr>>;

pub struct BarArtBfCfgInfo<'a> {
    pub u: i32,
    pub v: &'a str,
}

impl<'a> RefInto<'a, BarArtBfCfgInfo<'a>> for AppCfgInfo {
    fn ref_into(&'a self) -> BarArtBfCfgInfo<'a> {
        BarArtBfCfgInfo {
            u: self.y,
            v: &self.x,
        }
    }
}

#[instrument(level = "trace", skip_all)]
pub async fn bar_art_bf_c<ACFG, DUMMY>(
    cfg_src: impl Make<ACFG>,
    _: DUMMY,
    sleep_millis: u64,
    tx: &DummyTx<'_>,
) -> Result<String, AppErr>
where
    ACFG: for<'a> RefInto<'a, BarArtBfCfgInfo<'a>>,
{
    let app_cfg_info = cfg_src.make();
    let cfg = app_cfg_info.ref_into();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.to_owned();
    let res = bar_core(u, v) + &tx.dummy("bar_art_bf_c");
    Ok(res)
}

/// Returns a boxed bar_art_bf closure with free Tx parameter.
pub fn bar_art_bf_boot_box<ACFG>(
    c: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> Box<BarArtBfTxT>
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> RefInto<'a, BarArtBfCfgInfo<'a>>,
{
    cfg_deps_art_partial_apply_free_tx_box(bar_art_bf_c, c, ())
}

/// Returns a leaked static reference to a bar_art_bf closure with free Tx parameter.
pub fn bar_art_bf_boot_lr<ACFG>(
    c: impl Make<ACFG> + Send + Sync + Clone + 'static,
) -> &'static BarArtBfTxT
where
    ACFG: Send + Sync + 'static,
    ACFG: for<'a> RefInto<'a, BarArtBfCfgInfo<'a>>,
{
    cfg_deps_art_boot_free_tx_lr(bar_art_bf_c, c, ())
}
