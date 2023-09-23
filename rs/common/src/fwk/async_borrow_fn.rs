//! The trait defined here was recommended by https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952
//! in response to my issue https://github.com/rust-lang/rust/issues/113495;
//! Enhanced by https://github.com/rust-lang/rust/issues/113495#issuecomment-1728150795.

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

///Partial application for async function, where the result is an AsyncBorrowFn1a1.
///
/// The code below doesn't compile, thus the need for `nudge_inference`
/// (see https://github.com/rust-lang/rust/issues/113495#issuecomment-1728150795)
/// in this function.
/// ```
/// pub fn partial_apply<A1, A2, T>(
///     f: impl for<'a> AsyncBorrowFn2b2<'a, A1, &'a A2, Out = T> + 'static,
///     a1: A1,
/// ) -> impl for<'a> AsyncBorrowFn1b1<'a, &'a A2, Out = T>
/// where
///     A1: Clone + Send + Sync + 'static,
///     A2: ?Sized + 'static,
/// {
///     move |a2| f(a1.clone(), a2)
/// }
/// ```
pub fn partial_apply_async_borrow_fn_2b2<A1, A2, F, T>(
    f: F,
    a1: A1,
) -> impl for<'a> AsyncBorrowFn1b1<'a, A2, Out = T>
where
    A1: Clone + Send + Sync + 'static,
    A2: ?Sized + 'static,
    F: for<'a> AsyncBorrowFn2b2<'a, A1, A2, Out = T> + 'static,
{
    fn nudge_inference<A1, A2, F, T, C>(closure: C) -> C
    where
        // this promotes the literal `|a2| …` closure to "infer"
        // (get imbued with) the right higher-order fn signature.
        // See https://docs.rs/higher-order-closure for more info
        // v
        C: Fn(&A2) -> <F as AsyncBorrowFn2b2<'_, A1, A2>>::Fut,

        A1: Clone + 'static,
        A2: ?Sized + 'static,
        F: for<'a> AsyncBorrowFn2b2<'a, A1, A2, Out = T> + 'static,
    {
        closure
    }

    nudge_inference::<A1, A2, F, T, _>(move |a2| f(a1.clone(), a2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    trait Trt {
        fn value(&self) -> u32;
    }

    impl Trt for u32 {
        fn value(&self) -> u32 {
            *self
        }
    }

    type DynTrt = dyn Trt + Send + Sync;

    async fn f(i: u32, j: &DynTrt) -> u32 {
        i + j.value()
    }

    async fn f0(i: u32, j: &u32) -> u32 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        i + j
    }

    /// Specialization of [partial_apply_async_borrow_fn_2b2_boxpin] with A2 = DynTrt.
    /// Needed because type inference fails when [partial_apply_async_borrow_fn_2b2_boxpin] is called with
    /// an `f` whose second argument's type is `&dyn`.
    fn partial_apply_async_borrow_fn_2b2_boxpin_dyntrt<A1, T>(
        f: impl for<'a> AsyncBorrowFn2b2<'a, A1, DynTrt, Out = T>,
        a1: A1,
    ) -> impl for<'a> Fn(&'a DynTrt) -> Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>> + Send + Sync
    where
        A1: Clone + Send + Sync,
    {
        partial_apply_async_borrow_fn_2b2_boxpin(f, a1)
    }

    /// Specialization of [partial_apply_async_borrow_fn_2b2] with A2 = DynTrt.
    /// Needed because type inference fails when [partial_apply_async_borrow_fn_2b2] is called with
    /// an `f` whose second argument's type is `&dyn`.
    fn partial_apply_async_borrow_fn_2b2_dyntrt<A1, F, T>(
        f: F,
        a1: A1,
    ) -> impl for<'a> AsyncBorrowFn1b1<'a, DynTrt, Out = T>
    where
        A1: Clone + Send + Sync + 'static,
        F: for<'a> AsyncBorrowFn2b2<'a, A1, DynTrt, Out = T> + 'static,
    {
        partial_apply_async_borrow_fn_2b2(f, a1)
    }

    #[tokio::test]
    async fn test_all() {
        let f_part = partial_apply_async_borrow_fn_2b2_boxpin(f0, 40);
        assert_eq!(42, f_part(&2).await);

        let f_part = partial_apply_async_borrow_fn_2b2(f0, 40);
        println!("{}", f_part(&2).await);

        // The commented-out lines below don't compile
        // let g = |x, y: &u32| f0(x, y);
        // let f_part = partial_apply_async_borrow_fn_2b2_boxpin(f, 40);
        // let f_part = partial_apply_async_borrow_fn_2b2(f, 40);

        let f_part = partial_apply_async_borrow_fn_2b2_boxpin_dyntrt(f, 40);
        assert_eq!(42, f_part(&2).await);

        let f_part = partial_apply_async_borrow_fn_2b2_dyntrt(f, 40);
        assert_eq!(42, f_part(&2).await);
    }
}
