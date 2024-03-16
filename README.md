# IoC container for rust apps

### Container supply async registration & resolving any services or another solutions

Simple example (existed in repo)

```rust
pub async fn create_di() -> DI {
  let store: Arc<RwLock<Vec<UserEntity>>> = Arc::new(RwLock::new(vec![]));

  let user_repository_injector = InjectAdapter {
    token: user_repository_adapter::Adapter::token(),
    factory: Arc::new(move |_| user_repository_adapter::Adapter::new(store.clone())),
  };

  let di = DI::new()
    .inject(user_repository_injector)
    .await
    .inject(InjectAdapter {
      token: user_service_adapter::Adapter::token(),
      factory: Arc::new(user_service_adapter::Adapter::new),
    })
    .await;

  di
}

pub async fn get_user_service(context: &Arc<ContainerContext>) -> user_service_adapter::Adapter {
  context
    .resolve_provider::<user_service_adapter::Adapter>(user_service_adapter::Adapter::token())
    .await
}

pub async fn get_user_repository(
  context: &Arc<ContainerContext>,
) -> user_repository_adapter::Adapter {
  context
    .resolve_provider::<user_repository_adapter::Adapter>(user_repository_adapter::Adapter::token())
    .await
}

async fn should_be_return_zero_count_from_repository() {
    let di = create_di().await;

    let context = di.get_context();

    let user_repository = get_user_repository(&context).await;

    let count = user_repository.get_count().await.unwrap();

    assert_eq!(count, 0);
  }

  async fn should_be_return_zero_count_from_service() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = get_user_service(&context).await;

    let count = user_service.get_count().await.unwrap();

    assert_eq!(count, 0);
  }

  async fn should_return_new_user() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = get_user_service(&context).await;

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await
      .unwrap();

    let count = user_service.get_count().await.unwrap();

    assert_eq!(count, 1);
    assert_eq!(user_entity.name, "Andrey");
  }

  async fn should_remove_new_user() {
    let di = create_di().await;

    let context = di.get_context();

    let user_service = get_user_service(&context).await;

    let user_entity = user_service
      .add_user(&AddUserPort {
        name: "Andrey".to_string(),
        email: "andrey@mail.ru".to_string(),
      })
      .await
      .unwrap();

    let user_deletion = user_service
      .delete_user(&DeleteUserPort {
        email: user_entity.email.to_string(),
      })
      .await;

    assert_eq!(user_deletion.is_ok(), true);

    let check_user_entity = user_service
      .get_user(&GetUserPort {
        email: user_entity.email.to_string(),
      })
      .await
      .unwrap();

    assert_eq!(check_user_entity.is_none(), true);
  }
```

# Benchmarks:
<b>OS:</b>
> `Darwin 23.2.0 Darwin Kernel Version 23.2.0: Wed Nov 15 21:53:18 PST 2023; root:xnu-10002.61.3~2/RELEASE_ARM64_T6000 arm64 arm`

<b>CPU Info:</b>
```
sysctl -a | grep machdep.cpu
machdep.cpu.cores_per_package: 10
machdep.cpu.core_count: 10
machdep.cpu.logical_per_package: 10
machdep.cpu.thread_count: 10
machdep.cpu.brand_string: Apple M1 Max
```

```
Initialization of DI
Threads per iteration: 10
Iterations: 10000
Duration: 149.909333ms
```

```
Read the data from DI
Threads per iteration: 10
Iterations: 10000
Duration: 175.854958ms
```

```
Write the data from DI
Threads per iteration: 10
Iterations: 10000
Duration: 204.2085ms
```
