# cfgdepsobj_r

This package demonstrates a configuration and dependency injection approach that is analogous to push-to-function but uses a struct to represent the combination of config and dependencies. A dependency consists of a pair containing cfg-deps object and a function that takes the object as its first argument.

This variant uses dependencies by reference.

This approach is undesirable because it leaks details of the implementation of dependencies.
