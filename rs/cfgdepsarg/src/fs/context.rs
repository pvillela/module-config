use common::{
    config::get_pool,
    fwk::{get_connection, AppErr, Tx},
};

pub trait CfgSrc {
    type AppCfg;

    fn cfg_src() -> Self::AppCfg;
}

pub trait AsyncFnTx<CTX, IN, OUT> {
    #[allow(async_fn_in_trait)]
    async fn f(input: IN, tx: &Tx<'_>) -> Result<OUT, AppErr>;

    #[allow(async_fn_in_trait)]
    async fn exec_with_transaction(input: IN) -> Result<OUT, AppErr> {
        let pool = get_pool();
        let mut db = get_connection(pool).await?;
        let tx: Tx = db.transaction().await?;
        let res = Self::f(input, &tx).await;
        if res.is_ok() {
            tx.commit().await?;
        } else {
            tx.rollback().await?;
        }
        res
    }
}
