# Module Configuration

This directory contains examples of simple frameworks and patterns for module configuration.

The key considerations when designing/selecting a configuration framework are:
- Type safety
- Fail-fast
- Minimization of module dependencies
- Unit testing ease
- Ease of use

There are three main configuration approaches:
- *pull*
- *push-to-file*
- *push-to-function*

### _Pull_ approach

#### *Naive pull*

- Each module or function requiring configuration makes a direct call to a function or framework that returns application configuration information. The module or function selects and uses whatever configuration properties it needs from the returned application configuration data structure.
- It is often used because it is simple to implement and use.
- This approach inevitably causes dependencies to be hard-wired and also introduces a dependency of all configurable modules and functions on the global application configuration source.
- Is often implemented in a simplistic way that exacerbates the above-mentioned dependencies, makes it harder to structure and control configuration information, and makes it hard to unit test modules.

#### *Pull-with-push-override*

- The naive pull approach can be refined to yield a configuration approach that is easy to use and facilitates module unit testing. Ease of unit testing is comparable to that of the *push-to-file* approach.
- This involves constructing a file-level configuration object, used by functions defined in the file, that pulls in global application configuration but also allows configuration information to be injected for unit testing.
- When unit testing with this approach, configuration information must be pushed to each dependency. 

### *Push-to-variable* approach

- Each configurable module has a top-level variable that can be set with configuration information and which is accessed by functions in the file. For example a module can have an exported function `setConfig(configData)` that is called by application initialization logic to set the aforementioned top-level variable.
- This approach inevitably causes dependencies to be hard-wired.
- The *push-to-file* approach requires a few more files, and a bit more work, but it avoids a direct dependence on a global application configuration source. It naturally enables unit testing by allowing configuration information to be pushed to files. 
- When unit testing with this approach, configuration information must be pushed to each dependency. 

- Using this approach for modules with top-level variables that depend on configuration information leads to circular dependencies and the use of uninitialized variables. This is because, during initialization, the module/package to be configured is loaded so that its exported configuration function can be called by the initialization logic but, as the module/package to be configured is loaded, its top-level variables are initialized and that (by assumption) requires the configuration information that has not yet been injected into the module/package. This problem can be solved by moving top-level variables that depend on configuration information to a separate module/package and having those variables reference the top-level configuration variable in the original module/package.

### **_Push-to-function_** approach

- Each configurable function needs to be created from a factory/constructor function or class that takes the required configuration properties as input. Application initialization logic is responsible for calling the factories/constructors and pass the required configuration information to create configured function instances.
- Goes hand-in-hand with dependency injection. If function f depends on a configurable function g then f needs to be instantiated via a factory function or class fC that takes a configured instance of g as an input.
- Minimizes coupling among modules, provides the greatest unit testing flexibility, and easily enables the creation of multiple instances of the same function with different configurations.
- Requires the most planning, the highest number of files, and the most work of all the configuration approaches.

## Examples

This directory demonstrates simple frameworks and patterns for the above configuration approaches.

Notice that the _push_ frameworks demonstrated here have a _pull_ aspect to them as what is pushed is a thunk function that returns the configuration data, not the configuration data itself. The reason for that is to provide the flexibility to support configuration properties that change dynamically at runtime. There is no real performance penalty associated with the use a thunk instead of the data structure itself since the thunk can simply return a cached data structure by reference.
