use common::config::AppCfgInfo;
use common::fs_data::{FooArtIn, FooArtOut};
use common::fs_util::foo_core;
use common::fwk::{
    cfg_deps_artc_partial_apply_free_tx_arc, AppErr, PinBorrowFn2b2Tx, PinFn, RefInto, Tx,
};
use std::ops::Deref;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

use super::{bar_artc_bf_boot_box, BarArtcBfCfgInfo, BarArtcBfTxT, CfgSrc};

pub type FooArtcIn = FooArtIn;
type FooArtcOut = FooArtOut;

pub struct FooArtcSflCfgInfo<'a> {
    pub a: &'a str,
    pub b: i32,
}

impl<'a> RefInto<'a, FooArtcSflCfgInfo<'a>> for AppCfgInfo {
    fn ref_into(&'a self) -> FooArtcSflCfgInfo<'a> {
        FooArtcSflCfgInfo {
            a: &self.x,
            b: self.y,
        }
    }
}

pub type FooArtcSflTxT = PinBorrowFn2b2Tx<FooArtIn, Result<FooArtOut, AppErr>>;

pub type FooArtcSflT = PinFn<FooArtIn, Result<FooArtOut, AppErr>>;

pub struct FooArtcSflDeps {
    pub bar_artc_bf: Box<BarArtcBfTxT>,
}

#[instrument(level = "trace", skip_all)]
pub async fn foo_artc_sfl_c<CTX>(
    d: impl Deref<Target = FooArtcSflDeps>,
    input: FooArtcIn,
    tx: &Tx<'_>,
) -> Result<FooArtcOut, AppErr>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, FooArtcSflCfgInfo<'a>>,
{
    let app_cfg_info = CTX::cfg_src();
    let cfg = app_cfg_info.ref_into();
    let FooArtcIn { sleep_millis } = input;
    sleep(Duration::from_millis(sleep_millis)).await;
    let a = cfg.a.to_owned();
    let b = cfg.b;
    let bar_res = (d.bar_artc_bf)(0, tx).await.unwrap();
    let res = foo_core(a, b, bar_res) + &tx.dummy("foo_artc_sfl_c");
    Ok(FooArtcOut { res })
}

/// Returns an arced foo_artc_sfl closure with free Tx parameter.
pub fn foo_artc_sfl_boot_arc<CTX>() -> Arc<FooArtcSflTxT>
where
    CTX: CfgSrc + 'static,
    CTX::AppCfg: Send + Sync + 'static,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtcBfCfgInfo<'a>>,
    CTX::AppCfg: for<'a> RefInto<'a, FooArtcSflCfgInfo<'a>>,
{
    let b = bar_artc_bf_boot_box::<CTX>();
    let deps = Arc::new(FooArtcSflDeps { bar_artc_bf: b });
    cfg_deps_artc_partial_apply_free_tx_arc(foo_artc_sfl_c::<CTX>, deps)
}

// /// Returns a leaked static reference to a foo_at_sfl closure with free Tx parameter.
// pub fn foo_artc_sfl_boot_lr<ACFG>(
//     c: impl Make<ACFG> + Send + Sync + Clone + 'static,
// ) -> &'static FooArtcSflTxT
// where
//     ACFG: Send + Sync + 'static,
//     ACFG: for<'a> RefInto<'a, BarArtcBfCfgInfo<'a>>,
//     ACFG: for<'a> RefInto<'a, FooArtcSflCfgInfo<'a>>,
// {
//     let b = Box::new(bar_artc_bf_boot_lr(c.clone()));
//     let deps = FooArtcSflDeps { bar_artc_bf: b };
//     cfg_deps_artc_boot_free_tx_lr(foo_artc_sfl_c, c, deps)
// }
