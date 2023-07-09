use futures::Future;

/// Trait to enable high-order function accepting an async function that itself takes a reference.
/// See https://github.com/rust-lang/rust/issues/113495
/// and https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952.
pub trait AsyncBorrowFn<'a, A: ?Sized + 'a>: Fn(&'a A) -> Self::Fut {
    type Out;
    type Fut: Future<Output = Self::Out> + 'a;
}

impl<'a, A, F, Fut> AsyncBorrowFn<'a, A> for F
where
    A: ?Sized + 'a,
    F: Fn(&'a A) -> Fut,
    Fut: Future + 'a,
{
    type Out = Fut::Output;
    type Fut = Fut;
}
