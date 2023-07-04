#[macro_export]
macro_rules! pin_async_fn {
    ($f:ident) => {
        |s| Box::pin($f(s))
    };
}

#[macro_export]
macro_rules! pin_async_fn_2 {
    ($f:ident) => {
        |s1, s2| Box::pin($f(s1, s2))
    };
}
