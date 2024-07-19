use crate::fs::{AsyncFnTx, CfgSrc, FooArtctIn, FooArtctOut, FooArtctSflI};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fwk::{AppErr, TxParamDefault};

struct Ctx;
impl TxParamDefault for Ctx {}

impl CfgSrc for Ctx {
    type CfgInfo = AppCfgInfo;

    fn cfg_src() -> Self::CfgInfo {
        get_app_configuration()
    }
}

pub async fn foo_artct_sfl(input: FooArtctIn) -> Result<FooArtctOut, AppErr> {
    FooArtctSflI::<Ctx>::exec_with_transaction(input).await
}
