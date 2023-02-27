# Module Configuration

Every nontrivial application requires some form of external configuration capability that allows configuration properties to be defined separately from application code.

The key considerations when designing/selecting a configuration framework are:

- Type safety -- have the compiler detect many or most configuration problems rather than waiting for a runtime error.
- Fail-fast -- if there are runtime configuration errors, they should happen at application startup.
- Minimization of module dependencies -- the configuration approach and framework should support low coupling/dependence among modules. Any dependence on a configuration framework should be minimal and unobtrusive.
- Unit testing ease -- the configuration approach and framework should facilitate and not get in the way of unit testing.
- Ease of use -- the configuration approach should be easy to adopt and use in the application code.

There are three main kinds of configuration approach:

- *pull*
- *push-to-variable*
- *push-to-function*

### _Pull_ approaches

#### *Naive pull*

- Each module or function requiring configuration makes a direct call to a function or framework that returns application configuration information. The module or function selects and uses whatever configuration properties it needs from the returned application configuration data structure.
- It is often used because it is simple to implement and use.
- This approach inevitably causes dependencies to be hard-wired and also introduces a dependency of all configurable modules and functions on the global application configuration source.
- Is often implemented in a simplistic way that exacerbates the above-mentioned dependencies, makes it harder to structure and control configuration information, and makes it hard to unit test modules.

#### *Pull config, with push override*

- The naive pull approach can be refined to yield a configuration approach that is easy to use and facilitates unit testing of modules with respect to configuration. Ease of unit testing is comparable to that of the *push-to-variable* approach.
- This involves constructing one or more module-level configuration variables, used by functions defined in the module, that pull in global application configuration but also allow configuration information to be injected for unit testing.
- When unit testing with this approach, configuration information must be pushed for each dependency. 

#### *Pull config and dependencies, with push override*

- The above approach can be extended to include the pulling of dependencies by having the module-level variables contain both the configuration source and the references to dependencies.
- This is a hybrid of the above approach with the *Push config and dependencies to variable* approach below.
- This works around the hard-wiring of dependencies and provides flexibility for unit testing of modules separately from their dependencies.
- When unit testing with this approach, both configuration information and dependencies are pushed to the module-level objects.
- On balance, this is the most practical of the approaches, as it combines ease of use with full unit testing control.

##### *Workflow considerations*

To promote a natural design workflow that starts with service flow modules and later defines modules for the other stereotypes, the following steps should be followed:

- Start by defining each service flow.
- For each service flow, define its dependencies as functional interfaces. 
- The service flow's module-level configuration object is initially implemented as the result of ***todo*** function(s).
- Once the service flow's dependencies are defined in their respective modules, import the dependencies and replace the corresponding *todo* functions with the imports.
- At any time, the service flow can be unit tested by using the ability to push overriding configuration source and dependencies. 
- Subflows as well as Business functions that compose other business functions can follow a similar process to the above.

### *Push to variable* approaches

#### *Push config to variable* 

- Each configurable module has top-level variables that can be set with configuration information and which are accessed by functions in the file. For example a module can have an exported function `setConfig(configData)` that is called by application initialization logic to set the aforementioned top-level variable.
- This approach inevitably causes dependencies to be hard-wired.
- The *Push config to variable* approach requires a few more files, and a bit more work, but it avoids a direct dependence on a global application configuration source. It naturally enables unit testing of modules with respect to configuration by allowing configuration information to be pushed to the modules. 
- When unit testing with this approach, configuration information must be pushed to each dependency. 


*Note: In some languages, including TypeScript, using this approach for modules with other top-level variables that depend on configuration information can lead to circular dependencies and the use of uninitialized variables. In the case of TypeScript, this is because, during initialization, the module/package to be configured is loaded so that its exported configuration function can be called by the initialization logic but, as the module/package to be configured is loaded, its top-level variables are initialized and that (by assumption) require the configuration information that has not yet been injected into the module/package. This problem can be solved by moving other top-level variables that depend on configuration information to a separate module/package and having those variables reference the top-level configuration variable in the original module/package.*

#### *Push config and dependencies to variable*

- The above approach can be extended to include the pushing of dependencies by having the module-level variables contain both the configuration source and the references to dependencies.
- This works around the hard-wiring of dependencies and provides flexibility for unit testing of modules separately from their dependencies.
- When unit testing a module with this approach, both configuration information and dependencies are pushed to the module-level variables.

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
