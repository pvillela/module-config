# Observations on Different `pushdepstovar` Configuration Injection Approaches

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
