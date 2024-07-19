use common::fwk::{AppErr, DbClient, DbClientParam, Transaction, Tx};

pub trait Cfg {
    type Info;

    fn cfg() -> Self::Info;
}

pub trait CfgParam {
    type Cfg: Cfg;
}

pub trait AsyncFnTx<CTX, IN, OUT>
where
    CTX: DbClientParam,
{
    #[allow(async_fn_in_trait)]
    async fn f(input: IN, tx: &Tx<'_>) -> Result<OUT, AppErr>;

    #[allow(async_fn_in_trait)]
    async fn exec_with_transaction(input: IN) -> Result<OUT, AppErr> {
        // let pool = get_pool();
        // let mut db = get_connection(pool).await?;
        let mut db = CTX::DbClient::db_client().await?;
        let tx: Tx = db.transaction().await.map_err(|err| err.into())?;

        let res = Self::f(input, &tx).await;
        if res.is_ok() {
            tx.commit().await?;
        } else {
            tx.rollback().await?;
        }
        res
    }
}
