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

    /// Dummy method to demonstrate use of transaction reference.
    pub fn dummy(&self, src: &str) -> String {
        format!("-Tx.dummy() called from {}", src)
    }
}

pub async fn with_transaction<'a, T, AppErr, Fut>(
    db: &'a Db,
    block: impl FnOnce(&'a Tx) -> Fut + Send + Sync,
) -> Result<T, AppErr>
where
    AppErr: From<DbErr>,
    Fut: Future<Output = Result<T, AppErr>>,
{
    let tx = Tx::get(db).await?;
    tx.begin().await?;
    let res = block(tx).await;
    if res.is_ok() {
        tx.commit().await?;
    } else {
        tx.abort().await?;
    }
    res
}

pub fn fn2_with_transaction<S1, S2, T, AppErr, Fut>(
    db: &'static Db,
    f: impl Fn(S1, S2, &'static Tx) -> Fut + Clone + Send + Sync + 'static,
) -> impl Fn(S1, S2) -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'static>>
       + Send
       + Sync
       + 'static
where
    S1: Send + Sync + 'static,
    S2: Send + Sync + 'static,
    T: Send + Sync + 'static,
    AppErr: From<DbErr> + Send + Sync + 'static,
    Fut: Future<Output = Result<T, AppErr>> + Send + Sync + 'static,
{
    // Type inferencer annotates `f_t` as `impl FnOnce` but that is obviously incorrect
    // because the overall return type is satisfied.
    let f_t = move |s, i| {
        let f = f.clone();
        let block = move |tx| f(s, i, tx);
        // Convert Pin<Box<impl> to Pin<Box<dyn>>:
        let res: Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'static>> =
            Box::pin(with_transaction(db, block));
        res
    };
    f_t
}
