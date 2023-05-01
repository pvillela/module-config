/// Gets the value from an Option<T> and returns a reference to T.
/// Panics if the source value is None.
pub fn get_initialized_option<T>(info: &Option<T>) -> &T {
    info.as_ref().expect("Option not initialized")
}

/// Initializes value if it is None, no-op otherwise.
pub fn init_option<T>(info: T, cfg: &mut Option<T>) {
    if cfg.is_none() {
        *cfg = Some(info);
    }
}
