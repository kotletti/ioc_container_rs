# IoC container for rust apps

### Container supply registration & resolving any services or another solutions

Simple example (existed in repo)

```rust
pub trait UserRepositoryPort: Send + Sync {
    fn add_user(&self, entity: &UserEntity) -> Result<(), Error>;
    fn get_count(&self) -> Result<usize, Error>;
}

pub struct UserRepositoryAdapter {
    store: RwLock<Vec<UserEntity>>,
}

impl AdapterPort for dyn UserRepositoryPort {
    fn construct(_: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        Ok(Arc::new(UserRepositoryAdapter::new()))
    }
}
```

An adapter keeps its dependencies in fields and resolves them once, inside `construct`

```rust
pub trait UserServicePort: Send + Sync {
    fn add_user(&self, payload: &AddUserPort) -> Result<UserEntity, Error>;
    fn get_count(&self) -> Result<usize, Error>;
}

pub struct UserServiceAdapter {
    user_repository: Arc<dyn UserRepositoryPort>,
}

impl AdapterPort for dyn UserServicePort {
    fn construct(container: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        let user_repository = <dyn UserRepositoryPort>::get_adapter(container)?;

        Ok(Arc::new(UserServiceAdapter::new(user_repository)))
    }
}
```

Bind the ports and build the container

```rust
fn create_di() -> Result<Arc<dyn ContainerPort>, Error> {
    let container = DI::new()
        .bind::<dyn UserRepositoryPort>()?
        .bind::<dyn UserServicePort>()?
        .build::<AppContext>();

    Ok(container)
}
```

Use `bind_instance` when the adapter is built outside of the container

```rust
fn create_di() -> Result<Arc<dyn ContainerPort>, Error> {
    let store: Arc<RwLock<Vec<UserEntity>>> = Arc::new(RwLock::new(vec![]));

    let user_repository: Arc<dyn UserRepositoryPort> = Arc::new(UserRepositoryAdapter::new(store));

    let container = DI::new()
        .bind_instance::<dyn UserRepositoryPort>(user_repository)?
        .bind::<dyn UserServicePort>()?
        .build::<AppContext>();

    Ok(container)
}
```

Resolve an adapter by its port

```rust
fn should_be_return_zero_count_from_service() -> Result<(), Error> {
    let container = create_di()?;

    let user_service = <dyn UserServicePort>::get_adapter(&container)?;

    assert_eq!(user_service.get_count()?, 0, "Count should be 0");

    Ok(())
}
```

An adapter is built once, every resolve returns the same instance

```rust
fn should_share_repository_between_service_and_container() -> Result<(), Error> {
    let container = create_di()?;

    let user_service = <dyn UserServicePort>::get_adapter(&container)?;
    let user_repository = <dyn UserRepositoryPort>::get_adapter(&container)?;

    user_service.add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
    })?;

    assert_eq!(user_repository.get_count()?, 1);

    Ok(())
}
```

A dependency cycle is reported while resolving, not by a stack overflow

```rust
let Err(error) = <dyn CyclicAPort>::get_adapter(&container) else {
    return Err(Error::from("Cycle must be detected"));
};

assert!(error.to_string().contains("Dependency cycle detected"));
```

# Benchmarks:
<b>OS:</b>
> `Linux 6.12.74+deb13+1-arm64 aarch64`

<b>CPU Info:</b>
```
lscpu
Architecture: aarch64
CPU(s): 8
```

Every benchmark builds the same graph twice, with the container and without it, so the difference is the price of the container itself

```
Initialization of DI
Threads: 10
Iterations per thread: 10000
Rounds: 5
Failures: 0
Duration without DI: 1.487773ms
Duration with DI: 6.77844ms
Overhead: 5.290667ms
```

```
Read the data from DI
Threads: 10
Iterations per thread: 10000
Rounds: 5
Failures: 0
Duration without DI: 21.058915ms
Duration with DI: 26.03966ms
Overhead: 4.980745ms
```

```
Write the data to DI
Threads: 10
Iterations per thread: 10000
Rounds: 5
Failures: 0
Duration without DI: 16.719222ms
Duration with DI: 22.553438ms
Overhead: 5.834216ms
```
