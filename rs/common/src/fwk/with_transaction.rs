use futures::Future;
use std::pin::Pin;

pub struct DbClient;

pub struct DbPool;

#[derive(Debug)]
pub struct DbErr;

pub trait DbCfg {
    fn get_pool(&self) -> &DbPool;
}

pub async fn get_connection(_pool: &DbPool) -> Result<DbClient, DbErr> {
    // TODO: implement this properly
    Ok(DbClient)
}

pub struct Tx<'a> {
    #[allow(unused)]
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

async fn exec_fn2_with_transaction<'p, A, T, AppErr>(
    pool: &'p DbPool,
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

// TODO: This function is useless due to the Clone constraint on `f`.
//
/// Takes a pool source and a closure `f` with a free `&'a Tx` parameter,
/// returns a closure which, for each input,
/// returns the result of executing `f` with the input and a `&Tx` in a transactional context.
pub fn fn2_with_transaction<'p, A, T, AppErr>(
    pool: &'p DbPool,
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

/// Takes a pool source and a static reference to a  closure `f` with a free `&'a Tx` parameter,
/// returns a closure which, for each input,
/// returns the result of executing `f` with the input and a `&Tx` in a transactional context.
pub fn fn2_static_ref_with_transaction<'p, A, T, AppErr>(
    pool: &'p DbPool,
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
        // let f = f.clone();
        let res = Box::pin(exec_fn2_with_transaction(pool, f, input));
        res
    }
}
