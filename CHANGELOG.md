# Changelog

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