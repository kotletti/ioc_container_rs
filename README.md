# IoC container for rust apps

### Container supply registration & resolving any services or another solutions

Simple example (existed in repo)

```
pub fn get_user_by_name_test(user_name: &str) -> Result<User, Box<dyn Error>> {
  let container = Container::new();

  container.register("LoggerService", || {
    let logger: Arc<dyn LoggerService> = Arc::new(ConsoleLoggerService::new("User"));
    Box::new(logger)
  });

  container.register("UserService", || {
    let logger: Arc<dyn LoggerService> = Arc::new(ConsoleLoggerService::new("User"));
    Box::new(UserService::new(logger))
  });

  let user_service: Box<UserService> = container.resolve("UserService").unwrap();

  user_service.get_user_by_name(&user_name)
}
```