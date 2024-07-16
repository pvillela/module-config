use common::config::AppCfgInfo;
use common::fs_util::bar_core;
use common::fwk::{AppErr, PinBorrowFn2b2Tx, RefInto, Tx};
use std::marker::PhantomData;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

use crate::fs::context::CfgSrc;

pub type BarArtctBfTxT = PinBorrowFn2b2Tx<u64, Result<String, AppErr>>;

pub struct BarArtctBfCfgInfo<'a> {
    pub u: i32,
    pub v: &'a str,
}

impl<'a> RefInto<'a, BarArtctBfCfgInfo<'a>> for AppCfgInfo {
    fn ref_into(&'a self) -> BarArtctBfCfgInfo<'a> {
        BarArtctBfCfgInfo {
            u: self.y,
            v: &self.x,
        }
    }
}

pub trait BarArtctBf<CTX> {
    #[allow(async_fn_in_trait)]
    async fn bar_artct_bf(sleep_millis: u64, tx: &Tx<'_>) -> Result<String, AppErr>;
}

pub trait BarArtctBfBoot<CTX>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
{
    #[instrument(level = "trace", skip_all)]
    #[allow(async_fn_in_trait)]
    async fn bar_artct_bf_boot(sleep_millis: u64, tx: &Tx<'_>) -> Result<String, AppErr> {
        let app_cfg_info = CTX::cfg_src();
        let cfg = app_cfg_info.ref_into();
        sleep(Duration::from_millis(sleep_millis)).await;
        let u = cfg.u;
        let v = cfg.v.to_owned();
        let res = bar_core(u, v) + &tx.dummy("bar_artct_bf_c");
        Ok(res)
    }
}

impl<CTX, T> BarArtctBf<CTX> for T
where
    T: BarArtctBfBoot<CTX>,
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
{
    async fn bar_artct_bf(sleep_millis: u64, tx: &Tx<'_>) -> Result<String, AppErr> {
        Self::bar_artct_bf_boot(sleep_millis, tx).await
    }
}

pub struct BarArtctBfBootI<CTX>(PhantomData<CTX>);

impl<CTX> BarArtctBfBoot<CTX> for BarArtctBfBootI<CTX>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
{
}
