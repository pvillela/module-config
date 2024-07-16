use common::config::AppCfgInfo;
use common::fs_data::{FooArtIn, FooArtOut};
use common::fs_util::foo_core;
use common::fwk::{AppErr, PinBorrowFn2b2Tx, PinFn, RefInto, Tx};
use std::marker::PhantomData;
use std::time::Duration;
use tokio::time::sleep;
use tracing::instrument;

use super::{AsyncFnTx, BarArtctBf, BarArtctBfBoot, BarArtctBfCfgInfo, CfgSrc};

pub type FooArtctIn = FooArtIn;
pub type FooArtctOut = FooArtOut;

pub struct FooArtctSflCfgInfo<'a> {
    pub a: &'a str,
    pub b: i32,
}

impl<'a> RefInto<'a, FooArtctSflCfgInfo<'a>> for AppCfgInfo {
    fn ref_into(&'a self) -> FooArtctSflCfgInfo<'a> {
        FooArtctSflCfgInfo {
            a: &self.x,
            b: self.y,
        }
    }
}

pub type FooArtctSflTxT = PinBorrowFn2b2Tx<FooArtIn, Result<FooArtOut, AppErr>>;

pub type FooArtctSflT = PinFn<FooArtIn, Result<FooArtOut, AppErr>>;

pub trait FooArtctSfl<CTX> {
    #[allow(async_fn_in_trait)]
    async fn foo_artct_sfl(input: FooArtctIn, tx: &Tx<'_>) -> Result<FooArtctOut, AppErr>;
}

pub trait FooArtctSflC<CTX>: BarArtctBf<CTX>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, FooArtctSflCfgInfo<'a>>,
{
    #[instrument(level = "trace", skip_all)]
    #[allow(async_fn_in_trait)]
    async fn foo_artct_sfl_c(input: FooArtctIn, tx: &Tx<'_>) -> Result<FooArtctOut, AppErr> {
        let app_cfg_info = CTX::cfg_src();
        let cfg = app_cfg_info.ref_into();
        let FooArtctIn { sleep_millis } = input;
        sleep(Duration::from_millis(sleep_millis)).await;
        let a = cfg.a.to_owned();
        let b = cfg.b;
        let bar_res = (Self::bar_artct_bf)(0, tx).await.unwrap();
        let res = foo_core(a, b, bar_res) + &tx.dummy("foo_artct_sfl_c");
        Ok(FooArtctOut { res })
    }
}

pub struct FooArtctSflBootI<CTX>(PhantomData<CTX>);

impl<CTX> BarArtctBfBoot<CTX> for FooArtctSflBootI<CTX>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
{
}

impl<CTX> FooArtctSflC<CTX> for FooArtctSflBootI<CTX>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
    CTX::AppCfg: for<'a> RefInto<'a, FooArtctSflCfgInfo<'a>>,
{
}

impl<CTX> FooArtctSflBoot<CTX> for FooArtctSflBootI<CTX>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
    CTX::AppCfg: for<'a> RefInto<'a, FooArtctSflCfgInfo<'a>>,
{
}

pub trait FooArtctSflBoot<CTX>
where
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
    CTX::AppCfg: for<'a> RefInto<'a, FooArtctSflCfgInfo<'a>>,
{
    #[allow(async_fn_in_trait)]
    async fn foo_artct_sfl_boot(input: FooArtctIn, tx: &Tx<'_>) -> Result<FooArtctOut, AppErr> {
        FooArtctSflBootI::<CTX>::foo_artct_sfl_c(input, tx).await
    }
}

impl<T, CTX> FooArtctSfl<CTX> for T
where
    T: FooArtctSflBoot<CTX>,
    CTX: CfgSrc,
    CTX::AppCfg: for<'a> RefInto<'a, BarArtctBfCfgInfo<'a>>,
    CTX::AppCfg: for<'a> RefInto<'a, FooArtctSflCfgInfo<'a>>,
{
    async fn foo_artct_sfl(input: FooArtctIn, tx: &Tx<'_>) -> Result<FooArtctOut, AppErr> {
        T::foo_artct_sfl_boot(input, tx).await
    }
}

impl<T, CTX> AsyncFnTx<CTX, FooArtctIn, FooArtctOut> for T
where
    T: FooArtctSfl<CTX>,
{
    async fn f(input: FooArtctIn, tx: &Tx<'_>) -> Result<FooArtctOut, AppErr> {
        T::foo_artct_sfl(input, tx).await
    }
}
