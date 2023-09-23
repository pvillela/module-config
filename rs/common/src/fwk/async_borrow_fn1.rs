//! The trait defined here was recommended by https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952
//! in response to my issue https://github.com/rust-lang/rust/issues/113495;
//! Enhanced by https://github.com/rust-lang/rust/issues/113495#issuecomment-1728150795.
//! This file is essentially the same as in the above link.

use std::{future::Future, pin::Pin};

/// Represents an async function with a single argument that is a reference.
pub trait AsyncBorrowFn1b1<'a, A: ?Sized + 'a>: Fn(&'a A) -> Self::Fut {
    type Out;
    type Fut: Future<Output = Self::Out> + 'a;
}

impl<'a, A, F, Fut> AsyncBorrowFn1b1<'a, A> for F
where
    A: ?Sized + 'a,
    F: Fn(&'a A) -> Fut + 'a,
    Fut: Future + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

/// Represents an async function with 2 arguments; the first is not a reference, the last is a reference.
pub trait AsyncBorrowFn2b2<'a, A1, A2: ?Sized + 'a>: Fn(A1, &'a A2) -> Self::Fut {
    type Out;
    type Fut: Future<Output = Self::Out> + 'a;
}

impl<'a, A1, A2, F, Fut> AsyncBorrowFn2b2<'a, A1, A2> for F
where
    A2: ?Sized + 'a,
    F: Fn(A1, &'a A2) -> Fut + 'a,
    Fut: Future + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}

/// Partial application for async function, where the resulting closure returns a box-pinned future.
pub fn partial_apply_boxpin<A1, A2, T>(
    f: impl for<'a> AsyncBorrowFn2b2<'a, A1, A2, Out = T>,
    a1: A1,
) -> impl for<'a> Fn(&'a A2) -> Pin<Box<dyn Future<Output = T> + 'a>>
where
    A1: Clone,
    A2: ?Sized, // optional Sized relaxation
{
    move |a2| {
        let y = f(a1.clone(), a2);
        Box::pin(y)
    }
}

/// Partial application for async function, where the result is an AsyncBorrowFn1r1.
pub fn partial_apply<A1, A2, F, T>(f: F, a1: A1) -> impl for<'a> AsyncBorrowFn1b1<'a, A2, Out = T>
where
    A1: Clone + 'static,
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

#[tokio::test]
async fn test_all() {
    async fn f(i: u32, j: &u32) -> u32 {
        i + j
    }

    let f_part = partial_apply_boxpin(f, 40);
    assert_eq!(42, f_part(&2).await);

    let f_part = partial_apply(f, 40);
    assert_eq!(42, f_part(&2).await);
}
