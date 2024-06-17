# Notes on Rust Code

## Rust Workspaces

These are the Rust workspaces:

- cfgdepsarg -- A version of "Push-to-function" that demonstrates:
  - How to write the stereotype constructor in a simple style that takes configuration and dependencies as an additional parameter and uses a transformation to lift the constructor into a higher-order function.
  - How to include a transaction context argument in stereotype functions, leveraging appropriate transformation functions.
- common -- Common code used by other workspaces.
- pullcfgwithoverride -- Implementation of "Pull config, with push override".
- pulldepswithoverride -- Implementation of "Pull config and dependencies, with push override".
- pushcfgtovar -- Implementation of "Push config to variable".
- pushdepstovar -- Implementation of "Push config and dependencies to variable".
- pushdepstovar_c -- A variant implementation of "Push config and dependencies to variable".
- pushdepstovar_o -- Another variant implementation of "Push config and dependencies to variable".
- pushtofunction -- Implementation of "Push-to-function".
- pushtofunction_old -- Older, deprecated implementation of "Push-to-function".

## File Naming Convention

Meaning of infix letters:

- `_a_` -- async version.
- `_i_` -- immediate, i.e., the stereotype's `CfgInfo` struct is injected directly instead of a `Cfg` object.
- `_r_` -- reference-based stereotype `CfgInfo`. The stereotype `CfgInfo` is generated from a reference to `AppCfgInfo`, `CfgInfo` may contain references to fields in `AppCfgInfo`, and the lifetime of the `CfgInfo` instance is the same as that of the source reference. This provides a simpler configuration model with good efficiency. The game plan is as follows:
  - A raw config process updates a global `AppCfgInfo` object that may contain all sorts of stuff in it, like codes tables, database handles, etc.
  - A stereotype has its own view of configuration through a stereotype-specific data structure, say `MyCfgInfo`, that picks and chooses relevant elements from the global config. It provides an implementation of its configuration view from a reference to the global config object by implementing the `RefInto` trait. `MyCfgInfo` can be efficiently instantiated if it just contains references to fields in `AppCfgInfo` or copies of small by-value fields.
  - Whenever the global config is updated, the stereotype local view will reflect the new values.

- `_s_` -- simple configuration, i.e., the stereotype uses the application-level configuration info directly and the latter is injected into the stereotype constructor function.
- `_t_` -- transactional, i.e., additional transaction handle argument passed by reference.
- `_ac_` -- async with `const` configuration.
- `_ai_` -- async + immediate.
- `_ar_` -- async + reference-based.
- `_at_` -- async + transactional.
- `_aw_` -- async using `CfgRefCellId` instead of `CfgArcSwapArc`. Can be used with Axum but not Actix.
- `_ast_` -- async + simple + transactional.

## Observations on Different `pushdepstovar` Configuration Injection Approaches

`pushdepstovar` and `pushdepstovar_o` demonstrate configuration and dependency injection approaches that are intermediate between `pushtofunction` and `pushtovar`. The advantage of this kind of approach is that it provides a level of decoupling comparable to that of `pushtofunction` with simpler usage as higher-order functions and their resulting closures are avoided and coding can be done directly with normal functions.

`pushdepstovar` includes the demonstration of dynamic refresh and transformation of application configuration using closures that use an atomic cache. `pushdepstovar_o` does something similar, but using clonable objects instead. `pushdepstovar` is a bit easier to use than `pushdepstovar_o` because it leaves dependence on the global application configuration source to the very end.

Notice that the timing results of the following executables in `pushdepstovar`:

- `pdv_run_foo_a_bar_a_tokio_no_cache`
- `pdv_run_foo_a_bar_a_tokio_with_cache`
- `pdv_run_foo_ac_bar_ac_tokio`

and the following executables in `pushdepstovar_o`:

- `pdvo_run_foo_a_bar_a_tokio_no_cache`
- `pdvo_run_foo_a_bar_a_tokio_with_cache`

are all very close.

This demonstrates that:

- Any synchronization overheads from the different configuration transformation and caching approaches in `pushdepstovar` and `pushdepstovar_o`are essentially identical.
- Furthermore, those results are indistinguishable from the results from the `pushdepstovar` `pdv_run_foo_ac_bar_ac_tokio` which is the fastest possible approach and uses static configuration and static dependency binding. (This latter example is included only for comparison purposes as it does not directly demonstrate a practical way to do application configuration.)
