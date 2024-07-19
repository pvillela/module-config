use crate::fs::{AsyncFnTx, Cfg, CfgParam, FooArtctIn, FooArtctOut, FooArtctSflI};
use common::config::{get_app_configuration, AppCfgInfo};
use common::fwk::{AppErr, DbClientDefault, DbClientParam};

struct Ctx;

struct CtxCfg;

impl Cfg for CtxCfg {
    type Info = AppCfgInfo;

    fn cfg() -> Self::Info {
        get_app_configuration()
    }
}

impl CfgParam for Ctx {
    type Cfg = CtxCfg;
}

struct CtxDbClient;

impl DbClientDefault for CtxDbClient {}

impl DbClientParam for Ctx {
    type DbClient = CtxDbClient;
}

pub async fn foo_artct_sfl(input: FooArtctIn) -> Result<FooArtctOut, AppErr> {
    FooArtctSflI::<Ctx>::exec_with_transaction(input).await
}
