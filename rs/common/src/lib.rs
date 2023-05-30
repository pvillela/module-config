pub mod config;
pub mod fs_data;
pub mod fs_util;
pub mod fwk;
pub mod test_support;
pub mod tokio_run;
pub mod web;

#[macro_export]
macro_rules! pin_async_fn {
    ($f:ident) => {
        |s| Box::pin($f(s))
    };
}
