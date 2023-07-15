use futures::Future;
use std::pin::Pin;

pub struct Db;

pub struct DbErr;

pub trait DbCfg {
    fn get_db(&self) -> Result<&'static Db, DbErr>;
}

pub struct Tx;

impl<'a> Tx {
    pub async fn get(_db: &'a Db) -> Result<&'a Self, DbErr> {
        todo!()
    }

    pub async fn begin(&self) -> Result<(), DbErr> {
        todo!()
    }

    pub async fn commit(&self) -> Result<(), DbErr> {
        todo!()
    }

    pub async fn abort(&self) -> Result<(), DbErr> {
        todo!()
    }
}

pub async fn with_transaction<'a, T, AppErr, Fut>(
    db: &'a Db,
    box_block: Box<dyn FnOnce(&'a Tx) -> Fut>,
) -> Result<T, AppErr>
where
    AppErr: From<DbErr>,
    Fut: Future<Output = Result<T, AppErr>>,
{
    let tx = Tx::get(db).await?;
    tx.begin().await?;
    let res = box_block(tx).await?;
    tx.commit().await?;
    Ok(res)
}

pub fn sfl_with_transaction<S, In, Out, AppErr, Fut>(
    db: &'static Db,
    sfl: fn(S, In, &'static Tx) -> Fut,
) -> Box<dyn Fn(S, In) -> Pin<Box<dyn Future<Output = Result<Out, AppErr>> + 'static>> + 'static>
where
    S: 'static,
    Out: 'static,
    In: 'static,
    AppErr: From<DbErr> + 'static,
    Fut: Future<Output = Result<Out, AppErr>> + 'static,
{
    // Type inferencer annotates `sfl` as `impl FnOnce` but that is obviously incorrect
    //because `Box::new(sfl)` satisfies the return type of Box<dyn Fn>`.
    let sfl = move |s, i| {
        let block = move |tx| sfl(s, i, tx);
        let box_block = Box::new(block);
        // Convert Pin<Box<impl> to Pin<Box<dyn>>:
        let res: Pin<Box<dyn Future<Output = Result<Out, AppErr>> + 'static>> =
            Box::pin(with_transaction(db, box_block));
        res
    };
    Box::new(sfl)
}
