use crate::fs::{AsyncFnTx, CfgSrc, FooArtctIn, FooArtctOut, FooArtctSflBootI};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fwk::AppErr;

struct Ctx;

impl CfgSrc for Ctx {
    type AppCfg = AppCfgInfo;

    fn cfg_src() -> Self::AppCfg {
        get_app_configuration()
    }
}

pub async fn foo_artct_sfl(input: FooArtctIn) -> Result<FooArtctOut, AppErr> {
    FooArtctSflBootI::<Ctx>::exec_with_transaction(input).await
}
