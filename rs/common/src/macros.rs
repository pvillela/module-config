#[macro_export]
macro_rules! pin_async_fn {
    ($f:ident) => {
        |s| Box::pin($f(s))
    };
}
