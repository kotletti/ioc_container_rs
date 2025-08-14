# IoC container for rust apps

### Container supply async registration & resolving any services or another solutions

Simple example (existed in repo)

```rust
async fn create_di() -> DI {
  let di = DI::new(Arc::new(ContainerContext::new()));

  let store: Arc<RwLock<Vec<UserEntity>>> = Arc::new(RwLock::new(vec![]));

  let user_repository_injector = InjectAdapter {
    token: UserRepositoryAdapter::token(),
    factory: Arc::new(move |_| UserRepositoryAdapter::new(store.clone())),
  };

  let di = di
    .inject(user_repository_injector)
    .await
    .expect("UserRepositoryAdapter should be injected");

  let di = di
    .inject(InjectAdapter {
      token: UserServiceAdapter::token(),
      factory: Arc::new(UserServiceAdapter::new),
    })
    .await
    .expect("UserServiceAdapter should be injected");

  di
}

async fn should_be_return_zero_count_from_repository() {
  let di = create_di().await;

  let context = di.get_context();

  let user_repository = UserRepositoryAdapter::get_adapter(&context).await;

  assert!(user_repository.is_ok());

  let user_repository = user_repository.expect("UserRepository should exist");

  let count = user_repository
    .get_count()
    .await
    .expect("Get count has error");

  assert_eq!(count, 0, "Count should be 0");
}

#[tokio::test]
async fn should_be_return_zero_count_from_service() {
  let di = create_di().await;

  let context = di.get_context();

  let user_service = UserServiceAdapter::get_adapter(&context)
    .await
    .expect("UserService should exist");

  let count = user_service.get_count().await.expect("Get count has error");

  assert_eq!(count, 0, "Count should be 0");
}

async fn should_return_new_user() {
  let di = create_di().await;

  let context = di.get_context();

  let user_service = UserServiceAdapter::get_adapter(&context)
    .await
    .expect("UserService should exist");

  let user_entity = user_service
    .add_user(&AddUserPort {
      name: "Andrey".to_string(),
      email: "andrey@mail.ru".to_string(),
    })
    .await
    .expect("Add user has error");

  let count = user_service.get_count().await.expect("Get count has error");

  assert_eq!(count, 1, "Count should be 1");
  assert_eq!(user_entity.name, "Andrey", "Name should be Andrey");
}

async fn should_remove_new_user() {
  let di = create_di().await;

  let context = di.get_context();

  let user_service = UserServiceAdapter::get_adapter(&context)
    .await
    .expect("UserService should exist");

  let user_entity = user_service
    .add_user(&AddUserPort {
      name: "Andrey".to_string(),
      email: "andrey@mail.ru".to_string(),
    })
    .await
    .expect("Add user has error");

  user_service
    .delete_user(&DeleteUserPort {
      email: user_entity.email.to_string(),
    })
    .await
    .expect("Delete user has error");

  let check_user_entity = user_service
    .get_user(&GetUserPort {
      email: user_entity.email.to_string(),
    })
    .await
    .expect("Get user has error");

  assert!(
    check_user_entity.is_none(),
    "User after deletion should be None"
  );
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
Duration: 170.909333ms
```

```
Read the data from DI
Threads per iteration: 10
Iterations: 10000
Duration: 221.854958ms
```

```
Write the data from DI
Threads per iteration: 10
Iterations: 10000
Duration: 253.2085ms
```
