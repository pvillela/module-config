use futures::Future;
use std::{pin::Pin, sync::OnceLock};

pub struct Db;

pub struct DbErr;

pub trait DbCfg {
    fn get_db(&self) -> &'static Db;
}

pub struct Tx;

impl<'a> Tx {
    pub async fn get(_db: &'a Db) -> Result<&'a Self, DbErr> {
        // TODO: implement this properly
        static TX: OnceLock<Tx> = OnceLock::new();
        Ok(TX.get_or_init(|| Tx))
    }

    pub async fn begin(&self) -> Result<(), DbErr> {
        // TODO: implement this properly
        println!("Tx.begin() called");
        Ok(())
    }

    pub async fn commit(&self) -> Result<(), DbErr> {
        // TODO: implement this properly
        println!("Tx.commit() called");
        Ok(())
    }

    pub async fn abort(&self) -> Result<(), DbErr> {
        // TODO: implement this properly
        println!("Tx.abort() called");
        Ok(())
    }
}

pub async fn with_transaction<'a, T, AppErr, Fut>(
    db: &'a Db,
    box_block: Box<dyn FnOnce(&'a Tx) -> Fut + Send + Sync>,
) -> Result<T, AppErr>
where
    AppErr: From<DbErr>,
    Fut: Future<Output = Result<T, AppErr>>,
{
    let tx = Tx::get(db).await?;
    tx.begin().await?;
    let res = box_block(tx).await;
    if res.is_ok() {
        tx.commit().await?;
    } else {
        tx.abort().await?;
    }
    res
}

pub fn sfl_with_transaction<S, In, Out, AppErr, Fut>(
    db: &'static Db,
    sfl_c: fn(S, In, &'static Tx) -> Fut,
) -> Box<
    dyn Fn(S, In) -> Pin<Box<dyn Future<Output = Result<Out, AppErr>> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static,
>
where
    S: Send + Sync + 'static,
    In: Send + Sync + 'static,
    Out: Send + Sync + 'static,
    AppErr: From<DbErr> + Send + Sync + 'static,
    Fut: Future<Output = Result<Out, AppErr>> + Send + Sync + 'static,
{
    // Type inferencer annotates `sfl` as `impl FnOnce` but that is obviously incorrect
    //because `Box::new(sfl)` satisfies the return type of Box<dyn Fn>`.
    let sfl_c = move |s, i| {
        let block = move |tx| sfl_c(s, i, tx);
        let box_block = Box::new(block);
        // Convert Pin<Box<impl> to Pin<Box<dyn>>:
        let res: Pin<Box<dyn Future<Output = Result<Out, AppErr>> + Send + Sync + 'static>> =
            Box::pin(with_transaction(db, box_block));
        res
    };
    Box::new(sfl_c)
}
