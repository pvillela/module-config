# Module Configuration

Every nontrivial application requires some form of external configuration capability that allows configuration properties to be defined separately from application code.

The key considerations when designing/selecting a configuration framework are:

- Type safety -- have the compiler detect many or most configuration problems rather than waiting for a runtime error.
- Fail-fast -- if there are runtime configuration errors, they should happen at application startup.
- Minimization of module dependencies -- the configuration approach and framework should support low coupling/dependence among modules. Any dependence on a configuration framework should be minimal and unobtrusive.
- Unit testing ease -- the configuration approach and framework should facilitate and not get in the way of unit testing.
- Ease of use -- the configuration approach should be easy to adopt and use in the application code.

There are three main configuration approaches:

- *pull*
- *push-to-variable*
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

[This repo](https://github.com/pvillela/module-config) demonstrates simple frameworks and patterns for the above configuration approaches, written in Kotlin, TypeScript, Go, and Rust (under the `kt`, `ts`, `go`, and `rs` directories, respectively).

_Note: Unlike the `ts` , `go` , and `rs` directories, the project definition files for Kotlin code are not under the `kt` directory. That is because this code was created with IntelliJ IDEA, which at the time this code was created had a bug that required a project containing a Kotlin module to be a top-level Kotlin project._

Notice that the _push_ framework examples have a _pull_ aspect to them as what is pushed is a thunk function that returns the configuration data, not the configuration data itself. The reason for that is to provide the flexibility to support configuration properties that change dynamically at runtime. There is no significant performance penalty associated with the use a thunk instead of the data structure itself since the thunk can simply return a cached data structure by reference.

It is worth comparing the Rust examples with those in the other languages, especially Kotlin, to see how much more complex these patterns are to implement in Rust. That is due to Rust's complex type system, which keeps track of memory allocation and ownership. In particular, Rust differentiates between functions and closures, and requires multiple levels of wrapping and detailed type annotations to accomplish tasks that are straightforward in the other three languages.
