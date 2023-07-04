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

/// Creates a leaked reference to an async closure.
#[macro_export]
macro_rules! ref_pin_async_fn {
    ($f:ident) => {
        Box::leak(Box::new(move |s| Box::pin($f(s)) as _))
    };
}
