pub fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
