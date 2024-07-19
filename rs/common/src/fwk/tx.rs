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
    fn transaction<'a>(&'a mut self) -> impl Future<Output = Result<Tx<'a>, Self::DbErr>> + Send;
}

pub trait DbClient {
    type DbClient: Transaction + Send;
    type Tx<'a>;

    #[allow(async_fn_in_trait)]
    fn db_client(
    ) -> impl Future<Output = Result<Self::DbClient, <DummyDbClient as Transaction>::DbErr>> + Send;
}

pub trait DbClientParam {
    type DbClient: DbClient;
}

pub trait DbClientDefault {}

impl<T> DbClient for T
where
    T: DbClientDefault,
{
    type DbClient = DummyDbClient;
    type Tx<'a> = Tx<'a>;

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

pub struct Tx<'a> {
    #[allow(unused)]
    db: &'a mut DummyDbClient,
}

impl DummyDbClient {
    pub async fn transaction<'a>(&'a mut self) -> Result<Tx<'a>, DbErr> {
        // TODO: implement this properly
        // println!("Db.transaction() called");
        Ok(Tx { db: self })
    }
}

impl Transaction for DummyDbClient {
    type Tx<'a> = Tx<'a>;
    type DbErr = DbErr;

    async fn transaction<'a>(&'a mut self) -> Result<Tx<'a>, Self::DbErr> {
        self.transaction().await
    }
}

impl<'a> Tx<'a> {
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
            &'a Tx<'a>,
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
    let tx: Tx = db.transaction().await?;
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
                &'a Tx<'a>,
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
    let tx: Tx = db.transaction().await?;
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
            &'a Tx<'a>,
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
                &'a Tx<'a>,
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
        &'a Tx<'a>,
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

pub type PinBorrowFn2b2Tx<S1, T> = dyn for<'a> Fn(S1, &'a Tx<'a>) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>
    + Send
    + Sync;

/// Transforms an async closure with a `Tx` reference argument into a closure that returns a pinned-boxed future.
pub fn pin_async_borrow_fn_2b2_tx<S, T>(
    f: impl for<'a> AsyncBorrowFn2b2<'a, S, Tx<'a>, T>,
) -> impl for<'a> Fn(S, &'a Tx<'a>) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> {
    move |s, tx| {
        let x = f(s, tx);
        Box::pin(x)
    }
}
