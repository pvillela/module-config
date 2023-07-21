use futures::Future;
use std::{pin::Pin, sync::OnceLock};

pub struct DbClient;

pub struct DbPool;

#[derive(Debug)]
pub struct DbErr;

pub trait DbCfg {
    fn get_db(&self) -> &'static DbPool;
}

pub async fn get_connection(pool: &DbPool) -> Result<DbClient, DbErr> {
    // TODO: implement this properly
    Ok(DbClient)
}

pub struct Tx<'a> {
    db: &'a mut DbClient,
}

impl DbClient {
    pub async fn transaction<'a>(&'a mut self) -> Result<Tx<'a>, DbErr> {
        // TODO: implement this properly
        println!("Db.transaction() called");
        Ok(Tx { db: self })
    }
}

impl<'a> Tx<'a> {
    pub async fn commit(self) -> Result<(), DbErr> {
        // TODO: implement this properly
        println!("Tx.commit() called");
        Ok(())
    }

    pub async fn rollback(self) -> Result<(), DbErr> {
        // TODO: implement this properly
        println!("Tx.rollback() called");
        Ok(())
    }

    /// Dummy method to demonstrate use of transaction reference.
    pub fn dummy(&self, src: &str) -> String {
        format!("-Tx.dummy() called from {}", src)
    }
}

pub async fn with_transaction<'a, T, AppErr, Fut>(
    get_pool: fn() -> &'static DbPool,
    block: impl FnOnce(&Tx) -> Fut + Send + Sync,
) -> Result<T, AppErr>
where
    AppErr: From<DbErr>,
    Fut: Future<Output = Result<T, AppErr>>,
{
    let pool = get_pool();
    let mut db = get_connection(pool).await?;
    let tx = db.transaction().await?;
    let res = block(&tx).await;
    if res.is_ok() {
        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }
    res
}

/// Takes a pool source and a closure `f` with a free `&'a Tx` parameter,
/// returns a closure which, for each input,
/// returns the result of executing `f` with the input and a `&Tx` in a transactional context.
pub fn pin_fn2_with_transaction<A, T, AppErr>(
    get_pool: fn() -> &'static DbPool,
    f: impl for<'a> Fn(A, &'a Tx) -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'a>>
        + Send
        + Sync
        + Clone
        + 'static,
) -> impl Fn(A) -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync>> + Send + Sync
where
    A: Send + Sync + 'static,
    T: Send + Sync + 'static,
    AppErr: From<DbErr> + Send + Sync + 'static,
{
    let f_t = move |a| {
        let f = f.clone();
        let block = move |tx| f(a, tx);
        // Convert Pin<Box<impl> to Pin<Box<dyn>>:
        let res: Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync>> =
            Box::pin(with_transaction(get_pool, block));
        res
    };
    f_t
}
