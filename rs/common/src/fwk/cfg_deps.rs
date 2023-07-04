/// Represents a combination of configuration and dependencies data structures for function
/// stereotypes, suitable for use as as a function input parameter.
pub struct CfgDeps<C, D> {
    pub cfg: C,
    pub deps: D,
}
