# IoC container for Rust apps

Dependency injection for ports & adapters (hexagonal) applications.

* Adapters are bound to ports (`dyn Trait`) and resolved by the port
* Adapters are built once, asynchronously, in `build().await` — a database connection can be opened right inside the adapter
* `resolve` is synchronous: after `build` the container only reads ready instances
* Works the same on `current_thread` and `multi_thread` runtimes, the context is `Send + Sync`
* The context is a port too: write your own context or storage and keep the same adapters
* No dependencies in the library itself

## Ports and adapters

A port is a trait of your domain, the adapter implements it and describes how to build itself

```rust
pub trait UserRepositoryPort: Send + Sync {
    fn get_count(&self) -> Result<usize, Error>;
}

pub struct UserRepositoryAdapter {
    store: RwLock<Vec<UserEntity>>,
}

impl<Context: ContextResolverPort> AdapterPort<dyn UserRepositoryPort, Context>
    for UserRepositoryAdapter
{
    async fn construct(_: &Context) -> Result<Arc<dyn UserRepositoryPort>, Error> {
        Ok(Arc::new(Self { store: RwLock::new(vec![]) }))
    }
}
```

An adapter takes its dependencies from the context once, inside `construct`

```rust
pub struct UserServiceAdapter {
    user_repository: Arc<dyn UserRepositoryPort>,
}

impl<Context: ContextResolverPort> AdapterPort<dyn UserServicePort, Context>
    for UserServiceAdapter
{
    async fn construct(context: &Context) -> Result<Arc<dyn UserServicePort>, Error> {
        let user_repository = context.resolve::<dyn UserRepositoryPort>()?;

        Ok(Arc::new(UserServiceAdapter::new(user_repository)))
    }
}
```

`construct` is async, so an adapter can connect to a database, a broker or any other service

```rust
impl<Context: ContextResolverPort> AdapterPort<dyn UserRepositoryPort, Context>
    for PgUserRepository
{
    async fn construct(_: &Context) -> Result<Arc<dyn UserRepositoryPort>, Error> {
        let pool = PgPool::connect("postgres://localhost/app").await?;

        Ok(Arc::new(Self { pool }))
    }
}
```

## Build the container

Bind the ports in dependency order and build the container. `build` is the only `await` of the API

```rust
async fn create_di() -> Result<Arc<AppContext>, Error> {
    AppDI::new()
        .bind::<dyn UserRepositoryPort, UserRepositoryAdapter>()
        .bind::<dyn UserServicePort, UserServiceAdapter>()
        .build()
        .await
}
```

Resolve an adapter by its port, synchronously

```rust
let context = create_di().await?;

let user_service = context.resolve::<dyn UserServicePort>()?;
```

Every resolve returns the same instance

```rust
let first = context.resolve::<dyn UserRepositoryPort>()?;
let second = context.resolve::<dyn UserRepositoryPort>()?;

assert!(Arc::ptr_eq(&first, &second));
```

`resolve` comes from `ContextResolverPort`, import it where you call it

```rust
use ioc_container_rs::ports::context_port::ContextResolverPort;
```

## Errors

All configuration errors are reported by `build`, `bind` itself never fails

* A port bound twice: `Ports are bound more than once: dyn UserServicePort, ...`. No adapter is built in this case
* A dependency that is not bound, bound later, or a dependency cycle: `Adapter dyn UserRepositoryPort is not built`. Bind the dependency before its dependents
* An error returned from `construct` is returned from `build` as is, the rest of the adapters are not built

```rust
let Err(error) = AppDI::new()
    .bind::<dyn UserServicePort, UserServiceAdapter>()
    .build()
    .await
else {
    unreachable!();
};

assert!(error.to_string().contains("is not built"));
```

## Custom context

The context is described by ports, any type implementing them can be used instead of `AppContext`

* `ContextReaderPort` — read a value by `TypeId`
* `ContextWriterPort` — write a value while building
* `ContextResolverPort` — typed `resolve`, implemented automatically for every `ContextReaderPort`

Describe what your context can do with your own port and wrap `AppContext`

```rust
pub trait ConfigPort {
    fn database_url(&self) -> &str;
}

#[derive(Default)]
pub struct AppConfigContext {
    inner: AppContext,
    database_url: String,
}

impl ContextReaderPort for AppConfigContext {
    fn read(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error> {
        self.inner.read(key, name)
    }
}

impl ContextWriterPort for AppConfigContext {
    fn write(&mut self, key: TypeId, value: ContainerValue) -> Result<(), Error> {
        self.inner.write(key, value)
    }
}

impl ConfigPort for AppConfigContext {
    fn database_url(&self) -> &str {
        &self.database_url
    }
}
```

An adapter asks for the capability, not for a concrete context

```rust
impl<Context: ContextResolverPort + ConfigPort> AdapterPort<dyn UserRepositoryPort, Context>
    for PgUserRepository
{
    async fn construct(context: &Context) -> Result<Arc<dyn UserRepositoryPort>, Error> {
        let pool = PgPool::connect(context.database_url()).await?;

        Ok(Arc::new(Self { pool }))
    }
}
```

Build the container with your context

```rust
let context = DI::<AppConfigContext>::new()
    .bind::<dyn UserRepositoryPort, PgUserRepository>()
    .bind::<dyn UserServicePort, UserServiceAdapter>()
    .build()
    .await?;

let database_url = context.database_url();
```

Binding such an adapter to a context without the capability is a compile error

```text
error[E0277]: the trait bound `AppContext: ConfigPort` is not satisfied
```

The storage is a port as well: implement `ContainerPort` to keep the values your own way

## Tests

```bash
make test
```

The library tests are split by topic

* `base_test` — binding, building, resolving and errors
* `context_test` — custom contexts, context capabilities and custom storage
* `runtime_test` — `current_thread` and `multi_thread` runtimes, `tokio::spawn`, shared context between tasks and threads, background tasks and connections opened in `construct`

# Benchmarks

```bash
make benchmarks
```

<b>OS:</b>
> `Linux 6.12.115+deb13.1-arm64 aarch64`

<b>CPU Info:</b>
```
lscpu
Architecture: aarch64
CPU(s): 8
```

Every benchmark runs the same operations twice, with the container and without it, so the difference is the price of the container itself

```
Read the data from DI
Threads: 10
Iterations per thread: 10000
Rounds: 5
Failures: 0
Duration without DI: 19.99806ms
Duration with DI: 27.958475ms
Overhead: 5.960415ms
```

```
Write the data to DI
Threads: 10
Iterations per thread: 10000
Rounds: 5
Failures: 0
Duration without DI: 14.991159ms
Duration with DI: 20.290478ms
Overhead: 5.299319ms
```
