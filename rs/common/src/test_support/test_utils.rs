use once_cell::sync::Lazy;

/// Overrides the value of a lazy static. This function is thread-unsafe and
/// should only be used for test execution.
pub unsafe fn override_lazy<T>(r: &mut Lazy<T>, ovd_fn: fn() -> T) {
    let mr = r as *const Lazy<T> as *mut Lazy<T>;
    *mr = Lazy::new(ovd_fn);
}
