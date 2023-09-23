//! The trait defined here was recommended by https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952
//! in response to my issue https://github.com/rust-lang/rust/issues/113495.

use std::{future::Future, pin::Pin};

/// Represents an async function with single argument that is a reference.
pub trait AsyncBorrowFn1b1<'a, A: ?Sized + 'a>: Fn(&'a A) -> Self::Fut + Send + Sync {
    type Out;
    type Fut: Future<Output = Self::Out> + Send + Sync + 'a;
}

impl<'a, A, F, Fut> AsyncBorrowFn1b1<'a, A> for F
where
    A: ?Sized + 'a,
    F: Fn(&'a A) -> Fut + Send + Sync + 'a,
    Fut: Future + Send + Sync + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

/// Represents an async function with 2 arguments; the first is not a reference, the last is a reference.
pub trait AsyncBorrowFn2b2<'a, A1, A2: ?Sized + 'a>:
    Fn(A1, &'a A2) -> Self::Fut + Send + Sync
{
    type Out;
    type Fut: Future<Output = Self::Out> + Send + Sync + 'a;
}

impl<'a, A1, A2, F, Fut> AsyncBorrowFn2b2<'a, A1, A2> for F
where
    A2: ?Sized + 'a,
    F: Fn(A1, &'a A2) -> Fut + Send + Sync + 'a,
    Fut: Future + Send + Sync + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

/// Represents an async function with 3 arguments; the first 2 are not references, the last is a reference.
pub trait AsyncBorrowFn3b3<'a, A1, A2, A3: ?Sized + 'a>:
    Fn(A1, A2, &'a A3) -> Self::Fut + Send + Sync
{
    type Out;
    type Fut: Future<Output = Self::Out> + Send + Sync + 'a;
}

impl<'a, A1, A2, A3, F, Fut> AsyncBorrowFn3b3<'a, A1, A2, A3> for F
where
    A3: ?Sized + 'a,
    F: Fn(A1, A2, &'a A3) -> Fut + Send + Sync + 'a,
    Fut: Future + Send + Sync + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

/// Partial application for async function, where the resulting closure returns a box-pinned future.
pub fn partial_apply_async_borrow_fn_2b2_boxpin<A1, A2, T>(
    f: impl for<'a> AsyncBorrowFn2b2<'a, A1, A2, Out = T>,
    a1: A1,
) -> impl for<'a> Fn(&'a A2) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
where
    A1: Clone + Send + Sync,
    A2: ?Sized, // optional Sized relaxation
{
    move |a2| {
        let y = f(a1.clone(), a2);
        Box::pin(y)
    }
}

// Code below doesn't compile
//
///Partial application for async function, where the result is an AsyncBorrowFn1a1.
// pub fn partial_apply<A1, A2, T>(
//     f: impl for<'a> AsyncBorrowFn2b2<'a, A1, &'a A2, Out = T> + 'static,
//     a1: A1,
// ) -> impl for<'a> AsyncBorrowFn1b1<'a, &'a A2, Out = T>
// where
//     A1: Clone + Send + Sync + 'static,
//     A2: ?Sized + 'static,
// {
//     move |a2| f(a1.clone(), a2)
// }

#[cfg(test)]
mod tests {
    use super::*;
    use std::pin::Pin;

    trait Trt {
        fn value(&self) -> u32;
    }

    impl Trt for u32 {
        fn value(&self) -> u32 {
            *self
        }
    }

    async fn higher_order_dyn_trt(
        f: impl for<'a> AsyncBorrowFn1b1<'a, dyn Trt + Send + Sync + 'a, Out = ()>,
    ) {
        f(&12u32).await;
    }

    async fn f_tx(_input: &(dyn Trt + Send + Sync)) {}

    fn higher_order_dyn_trt_2(
        f: impl for<'a> AsyncBorrowFn2b2<'a, u32, dyn Trt + Send + Sync + 'a, Out = u32>,
        i: u32,
    ) -> impl for<'a> Fn(
        &'a (dyn Trt + Send + Sync),
    ) -> Pin<Box<dyn Future<Output = u32> + Send + Sync + 'a>> {
        move |x| {
            let y = f(i, x);
            Box::pin(y)
        }
    }

    fn higher_order_dyn_trt_2_somewhat_generic<A1, T>(
        f: impl for<'a> AsyncBorrowFn2b2<'a, A1, dyn Trt + Send + Sync + 'a, Out = T>,
        i: A1,
    ) -> impl for<'a> Fn(
        &'a (dyn Trt + Send + Sync),
    ) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>
    where
        A1: Clone,
    {
        move |x| {
            let y = f(i.clone(), x);
            Box::pin(y)
        }
    }

    #[allow(unused)]
    fn higher_order_2_generic<A1, A2, T>(
        f: impl for<'a> AsyncBorrowFn2b2<'a, A1, A2, Out = T>,
        i: A1,
    ) -> impl for<'a> Fn(&'a A2) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>
    where
        A1: Clone,
        A2: ?Sized,
    {
        move |x| {
            let y = f(i.clone(), x);
            Box::pin(y)
        }
    }

    async fn f_tx2(i: u32, tx: &(dyn Trt + Send + Sync)) -> u32 {
        i + tx.value()
    }

    #[test]
    fn test_all() {
        _ = higher_order_dyn_trt(f_tx);
        _ = higher_order_dyn_trt_2(f_tx2, 1);
        _ = higher_order_dyn_trt_2_somewhat_generic(f_tx2, 1);
        // _ = higher_order_2_generic(f_tx2, 1); // doesn't compile
    }
}
