use super::{AsyncBorrowFn1a1, AsyncBorrowFn2a2};
use futures::Future;
use std::pin::Pin;

/// Partial application for async function, where the resulting closure returns a box-pinned future.
pub fn partial_apply_boxpin<A1, A2, T>(
    f: impl for<'a> AsyncBorrowFn2a2<'a, A1, A2, Out = T> + 'static,
    a1: A1,
) -> impl for<'a> Fn(&'a A2) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    A1: Copy + Send + Sync,
    A2: ?Sized, // optional Sized relaxation
{
    move |a2| {
        let y = f(a1.clone(), a2);
        Box::pin(y)
    }
}

/// Partial application for async function, where the result is an AsyncBorrowFn1a1.
pub fn partial_apply<'a, A1, A2, T>(
    f: impl AsyncBorrowFn2a2<'a, A1, A2, Out = T> + 'static,
    a1: A1,
) -> impl AsyncBorrowFn1a1<'a, A2, Out = T>
where
    A1: Copy + Send + Sync + 'static,
    A2: ?Sized + 'static,
{
    move |a2| f(a1.clone(), a2)
}
