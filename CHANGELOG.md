# Changelog

## Version 0.4.0 BREAKING CHANGES
* Build adapters eagerly and asynchronously in `DI::build().await`, in the order of `bind`
* Make `AdapterPort::construct` async: an adapter can open a connection right inside it
* Make `AdapterPort` generic over the context: `AdapterPort<Port, Context>`, `construct(&Context)`
* Make `resolve` synchronous, it only reads instances that are already built
* Make `bind` infallible, report every duplicated port from `build` before any adapter is built
* Return the error of a failed `construct` from `build` and stop building the rest
* Report a missing dependency, a wrong bind order or a dependency cycle as `Adapter ... is not built`
* Remove lazy factories, the `OnceLock` cache, `ContainerSlot` and `ChainGuard` cycle detection
* Remove `bind_instance`, every value comes from `construct`
* Split the context into ports `ContextReaderPort`, `ContextWriterPort` & `ContextResolverPort`, remove `ContextPort`
* Implement `ContextResolverPort::resolve` once for every `ContextReaderPort`
* Make `DI<Context>` generic over the context, add `AppDI` alias for `DI<AppContext>`
* Support custom contexts and context capabilities through ports
* Store ready instances in `ContainerPort`
* Make the container work the same on `current_thread` & `multi_thread` runtimes
* Move domain ports of the example to `example_domain` crate
* Split library tests into `base_test`, `context_test` & `runtime_test`
* Share the measuring code of benchmarks in `bench.rs`, rename `benchmarks.bash` to `benchmarks.sh`

## Version 0.3.0 BREAKING CHANGES
* Make the container synchronous, remove `async`/`await` from registration and resolving
* Drop `tokio` & `async-trait` from the library deps
* Remove `ContextPort` & `ContainerContext`, the context layer is `AppContext` over `ContainerPort`
* Replace `AdapterPort<T>` with `AdapterPort` implemented for `dyn Port`, not for the adapter itself
* Remove `fn token()`, adapters are keyed by `TypeId` of their port
* Replace `DI::inject(InjectAdapter { .. })` with `DI::bind::<dyn Port>()` & `DI::bind_instance::<dyn Port>(instance)`
* Replace factory closures with `AdapterPort::construct(&Arc<dyn ContainerPort>)`
* Return `Arc<Self>` from `get_adapter(...)` instead of `Box<Self>`
* Cache the resolved instance in `OnceLock`, adapters became singletons
* Detect dependency cycles while resolving, return an error instead of a stack overflow
* Report an error on binding the same port twice
* Name the port in resolve errors through `type_name`
* Move `Container`, `AppContext` & `DI` to `adapters`, keep abstractions in `ports`
* Rewrite the example on ports & constructor injection, the service holds `Arc<dyn UserRepositoryPort>` instead of the container
* Measure the same graph with and without the container in benchmarks, add warmup & median of rounds

## Version 0.2.1
* Move `async fn get_adapter(...)` to `AdapterPort<T>`
* Change tab size `2` -> `4`
* Update `tokio` & `async-trait` deps

## Version 0.2.0 BREAKING CHANGES
* Add strict trait `AdapterPort<T>` to factory of `InjectAdapter<T>`
* `Context` trait to `ContextPort`
* Wrap to `Result` most methods of context for safety without panics
* Remove `ContainerStore` layer
* Add implementation of custom `Context` with side effects
* Add tests for `CustomContext`

## Version 0.1.0 BREAKING CHANGES
* Replace the `std::sync::Mutex` to `tokio::sync::RwLock`
* Modify examples
* Separating the container to three layers (Store, Container, DI)
* Add tests
* Add async/await for thread safety
* Add benchmarks

## Version 0.0.1
* Implement base logic to register & resolve providers
* Add examples
