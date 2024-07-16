use crate::fs::CfgSrc;
use common::config::AppCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{cfg_deps_artc_partial_apply_free_tx_box, AppErr, PinBorrowFn2b2Tx, RefInto, Tx};
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

pub type BarArtcBfTxT = PinBorrowFn2b2Tx<u64, Result<String, AppErr>>;

pub struct BarArtcBfCfgInfo<'a> {
    pub u: i32,
    pub v: &'a str,
}

impl<'a> RefInto<'a, BarArtcBfCfgInfo<'a>> for AppCfgInfo {
    fn ref_into(&'a self) -> BarArtcBfCfgInfo<'a> {
        BarArtcBfCfgInfo {
            u: self.y,
            v: &self.x,
        }
    }
}

#[instrument(level = "trace", skip_all)]
pub async fn bar_artc_bf_c<CTX, DUMMY>(
    _: DUMMY,
    sleep_millis: u64,
    tx: &Tx<'_>,
) -> Result<String, AppErr>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtcBfCfgInfo<'a>>,
{
    let app_cfg_info = CTX::cfg_src();
    let cfg = app_cfg_info.ref_into();
    sleep(Duration::from_millis(sleep_millis)).await;
    let u = cfg.u;
    let v = cfg.v.to_owned();
    let res = bar_core(u, v) + &tx.dummy("bar_artc_bf_c");
    Ok(res)
}

/// Returns a boxed bar_artc_bf closure with free Tx parameter.
pub fn bar_artc_bf_boot_box<CTX>() -> Box<BarArtcBfTxT>
where
    CTX: CfgSrc + 'static,
    CTX::AppCfg: Send + Sync + 'static,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtcBfCfgInfo<'a>>,
{
    cfg_deps_artc_partial_apply_free_tx_box(bar_artc_bf_c::<CTX, ()>, ())
}

// /// Returns a leaked static reference to a bar_artc_bf closure with free Tx parameter.
// pub fn bar_artc_bf_boot_lr<ACFG>(
//     c: impl Make<ACFG> + Send + Sync + Clone + 'static,
// ) -> &'static BarArtcBfTxT
// where
//     ACFG: Send + Sync + 'static,
//     ACFG: for<'a> RefInto<'a, BarArtcBfCfgInfo<'a>>,
// {
//     cfg_deps_art_boot_free_tx_lr(bar_artc_bf_c, c, ())
// }
