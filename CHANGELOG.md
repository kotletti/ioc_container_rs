# Changelog

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
