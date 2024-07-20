use crate::{config::get_pool, fwk::AsyncBorrowFn2b2};
use futures::Future;
use std::{
    error::Error,
    fmt::{Debug, Display},
    pin::Pin,
    sync::Arc,
};

use super::AppErr;

pub trait Transaction {
    type Tx<'a>;
    type DbErr: Error + Into<AppErr> + Send;

    #[allow(async_fn_in_trait)]
    fn transaction<'a>(
        &'a mut self,
    ) -> impl Future<Output = Result<DummyTx<'a>, Self::DbErr>> + Send;
}

pub trait Db {
    type Db: Transaction + Send;

    #[allow(async_fn_in_trait)]
    fn db_client() -> impl Future<Output = Result<Self::Db, <Self::Db as Transaction>::DbErr>> + Send;
}

pub trait DbCtx {
    type DbClient: Db;
}

pub trait DbClientDefault {}

impl<T> Db for T
where
    T: DbClientDefault,
{
    type Db = DummyDbClient;

    #[allow(async_fn_in_trait)]
    async fn db_client() -> Result<DummyDbClient, DbErr> {
        let pool = get_pool();
        get_connection(pool).await
    }
}

pub struct DummyDbClient;

pub struct DummyDbPool;

#[derive(Debug)]
pub struct DbErr;

impl Display for DbErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for DbErr {}

pub trait DbCfg {
    fn get_pool(&self) -> &DummyDbPool;
}

pub async fn get_connection(_pool: &DummyDbPool) -> Result<DummyDbClient, DbErr> {
    // TODO: implement this properly
    Ok(DummyDbClient)
}

pub struct DummyTx<'a> {
    #[allow(unused)]
    db: &'a mut DummyDbClient,
}

impl DummyDbClient {
    pub async fn transaction<'a>(&'a mut self) -> Result<DummyTx<'a>, DbErr> {
        // TODO: implement this properly
        // println!("Db.transaction() called");
        Ok(DummyTx { db: self })
    }
}

impl Transaction for DummyDbClient {
    type Tx<'a> = DummyTx<'a>;
    type DbErr = DbErr;

    async fn transaction<'a>(&'a mut self) -> Result<DummyTx<'a>, Self::DbErr> {
        self.transaction().await
    }
}

impl<'a> DummyTx<'a> {
    pub async fn commit(self) -> Result<(), DbErr> {
        // TODO: implement this properly
        // println!("Tx.commit() called");
        Ok(())
    }

    pub async fn rollback(self) -> Result<(), DbErr> {
        // TODO: implement this properly
        // println!("Tx.rollback() called");
        Ok(())
    }

    /// Dummy method to demonstrate use of transaction reference.
    pub fn dummy(&self, src: &str) -> String {
        format!("-Tx.dummy() called from {}", src)
    }
}

async fn exec_fn2_with_transaction<'p, A, T, AppErr>(
    pool: &'p DummyDbPool,
    f: impl for<'a> FnOnce(
            A,
            &'a DummyTx<'a>,
        )
            -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'a>>
        + Send
        + Sync,
    input: A,
) -> Result<T, AppErr>
where
    AppErr: From<DbErr>,
{
    let mut db = get_connection(pool).await?;
    let tx: DummyTx = db.transaction().await?;
    let res = f(input, &tx).await;
    if res.is_ok() {
        tx.commit().await?;
    } else {
        tx.rollback().await?;
    }
    res
}

async fn exec_fn2_arc_with_transaction<'p, A, T, AppErr>(
    pool: &'p DummyDbPool,
    f: Arc<
        dyn for<'a> Fn(
                A,
                &'a DummyTx<'a>,
            )
                -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'a>>
            + Send
            + Sync,
    >,
    input: A,
) -> Result<T, AppErr>
where
    AppErr: From<DbErr>,
{
    let mut db = get_connection(pool).await?;
    let tx: DummyTx = db.transaction().await?;
    let res = f(input, &tx).await;
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
pub fn fn2_with_transaction<'p, A, T, AppErr>(
    pool: &'p DummyDbPool,
    f: impl for<'a> Fn(
            A,
            &'a DummyTx<'a>,
        ) -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'a>>
        + Send
        + Sync
        + Clone
        + 'p,
) -> impl Fn(A) -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'p>> + Send + Sync
where
    A: Send + Sync + 'static,
    T: Send + Sync + 'static,
    AppErr: From<DbErr> + Send + Sync + 'static,
{
    move |input| {
        let res = Box::pin(exec_fn2_with_transaction(pool, f.clone(), input));
        res
    }
}

/// Takes a pool source and a closure `f` with a free `&'a Tx` parameter,
/// returns a closure which, for each input,
/// returns the result of executing `f` with the input and a `&Tx` in a transactional context.
pub fn fn2_arc_with_transaction<'p, A, T, AppErr>(
    pool: &'p DummyDbPool,
    f: Arc<
        dyn for<'a> Fn(
                A,
                &'a DummyTx<'a>,
            )
                -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'a>>
            + Send
            + Sync,
    >,
) -> impl Fn(A) -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'p>> + Send + Sync
where
    A: Send + Sync + 'static,
    T: Send + Sync + 'static,
    AppErr: From<DbErr> + Send + Sync + 'static,
{
    move |input| {
        let res = Box::pin(exec_fn2_arc_with_transaction(pool, f.clone(), input));
        res
    }
}

/// Takes a pool source and a static reference to a  closure `f` with a free `&'a Tx` parameter,
/// returns a closure which, for each input,
/// returns the result of executing `f` with the input and a `&Tx` in a transactional context.
pub fn fn2_static_ref_with_transaction<'p, A, T, AppErr>(
    pool: &'p DummyDbPool,
    f: &'static (dyn for<'a> Fn(
        A,
        &'a DummyTx<'a>,
    )
        -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'a>>
                  + Send
                  + Sync),
) -> impl Fn(A) -> Pin<Box<dyn Future<Output = Result<T, AppErr>> + Send + Sync + 'p>> + Send + Sync
where
    A: Send + Sync + 'static,
    T: Send + Sync + 'static,
    AppErr: From<DbErr> + Send + Sync + 'static,
{
    move |input| {
        let res = Box::pin(exec_fn2_with_transaction(pool, f, input));
        res
    }
}

pub type PinBorrowFn2b2Tx<S1, T> = dyn for<'a> Fn(S1, &'a DummyTx<'a>) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>
    + Send
    + Sync;

/// Transforms an async closure with a `Tx` reference argument into a closure that returns a pinned-boxed future.
pub fn pin_async_borrow_fn_2b2_tx<S, T>(
    f: impl for<'a> AsyncBorrowFn2b2<'a, S, DummyTx<'a>, T>,
) -> impl for<'a> Fn(S, &'a DummyTx<'a>) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> {
    move |s, tx| {
        let x = f(s, tx);
        Box::pin(x)
    }
}
