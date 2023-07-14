/// Represents a higher-order function that executes another function within
/// a context.
/// An example is a function that executes a target function while delimiting the target function
/// within a transaction.
///
/// - ctx_cfg: CC -  represents configuration that is passed to the higher-orfer function.
/// For example, a database object.
///
/// - runtime_ctx: RC - is a runtime argument produced by the Contextualizer and passed to
/// the target function.
/// For example, a transaction object.
pub type Contextualizer<'a, CC, RC, T> =
    fn(ctx_cfg: &'a CC, block: Box<dyn FnOnce(&'a RC) -> T>) -> T;

/// Returns a closure that is the partial application of f
/// within the execution context provided by contextualizer.
pub fn contextualize_2<CC, RC, S1, S2, T>(
    contextualizer: Contextualizer<'static, CC, RC, T>,
    ctx_cfg: &'static CC,
    f: fn(runtime_ctx: &'static RC, S1, S2) -> T,
) -> impl Fn(S1, S2) -> T + 'static
where
    RC: 'static,
    S1: 'static,
    S2: 'static,
    T: 'static,
{
    move |s1, s2| {
        let g = move |rc| f(rc, s1, s2);
        contextualizer(ctx_cfg, Box::new(g))
    }
}

/// Returns a closure that is the partial application of f
/// within the execution context provided by contextualizer.
pub fn contextualize_2a<'a, CC, RC, S1, S2, T>(
    contextualizer: Contextualizer<'a, CC, RC, T>,
    ctx_cfg: &'a CC,
    f: impl Fn(&'a RC, S1, S2) -> T + 'static + Clone,
) -> impl Fn(S1, S2) -> T + 'a
where
    RC: 'static,
    S1: 'static,
    S2: 'static,
    T: 'static,
{
    move |s1, s2| {
        let fc = f.clone();
        let g = move |rc: &'a RC| fc(rc, s1, s2);
        contextualizer(ctx_cfg, Box::new(g))
    }
}

/// Returns a closure that is the partial application of f
/// within the execution context provided by contextualizer.
/// This version is bad because it requires `f` to accept an &RC argument without a lifetime, which is
/// practically infeasible when `f` is async.
pub fn contextualize_2bad<'a, CC, RC, S1, S2, T>(
    contextualizer: Contextualizer<'a, CC, RC, T>,
    ctx_cfg: &'a CC,
    f: fn(runtime_ctx: &RC, S1, S2) -> T,
) -> impl Fn(S1, S2) -> T + 'a
where
    RC: 'static,
    S1: 'static,
    S2: 'static,
    T: 'static,
{
    move |s1, s2| {
        let g = move |rc| f(rc, s1, s2);
        contextualizer(ctx_cfg, Box::new(g))
    }
}
