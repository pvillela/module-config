/// The trait defined here was recommended by https://github.com/rust-lang/rust/issues/113495#issuecomment-1627640952
/// in response to my issue https://github.com/rust-lang/rust/issues/113495.
/// However, I found a simpler solution: https://github.com/rust-lang/rust/issues/113495#issuecomment-1627893701.
/// So, this trait isn't really necessary.
use futures::Future;

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
