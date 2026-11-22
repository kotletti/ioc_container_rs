
use std::{
    any::type_name,
    sync::{Arc, RwLock},
    time::Duration,
};

use crate::{
    adapters::{app_context::AppContext, di::AppDI},
    errors::error::Error,
    ports::{adapter_port::AdapterPort, context_port::ContextResolverPort},
};

#[derive(Clone, PartialEq)]
struct User {
    id: u64,
    name: String,
}

trait DatabaseServicePort: Send + Sync {
    fn create(&self, user: &User) -> Result<(), Error>;
    fn get_by_id(&self, id: u64) -> Result<User, Error>;
    fn get_all(&self) -> Result<Vec<User>, Error>;
    fn is_exists(&self, user: &User) -> Result<bool, Error>;
}

struct DatabaseServiceAdapter {
    storage: RwLock<Vec<User>>,
}

impl DatabaseServiceAdapter {
    pub fn new(storage: Vec<User>) -> Self {
        Self {
            storage: RwLock::new(storage),
        }
    }
}

impl<Context: ContextResolverPort> AdapterPort<dyn DatabaseServicePort, Context>
    for DatabaseServiceAdapter
{
    async fn construct(_: &Context) -> Result<Arc<dyn DatabaseServicePort>, Error> {
        Ok(Arc::new(Self::new(vec![])))
    }
}

impl DatabaseServicePort for DatabaseServiceAdapter {
    fn create(&self, user: &User) -> Result<(), Error> {
        if self.is_exists(user)? {
            return Err(Error::from("User already exists."));
        }

        let mut storage = self
            .storage
            .write()
            .map_err(|e| Error::from(e.to_string()))?;

        storage.push(user.clone());

        Ok(())
    }

    fn get_by_id(&self, id: u64) -> Result<User, Error> {
        let storage = self
            .storage
            .read()
            .map_err(|e| Error::from(e.to_string()))?;

        match storage.iter().find(|u| u.id == id) {
            Some(r) => Ok(r.clone()),
            None => Err(Error::from("User not found.")),
        }
    }

    fn get_all(&self) -> Result<Vec<User>, Error> {
        let storage = self
            .storage
            .read()
            .map_err(|e| Error::from(e.to_string()))?;

        Ok(storage.clone())
    }

    fn is_exists(&self, user: &User) -> Result<bool, Error> {
        let storage = self
            .storage
            .read()
            .map_err(|e| Error::from(e.to_string()))?;

        Ok(storage.iter().any(|u| u.id == user.id))
    }
}

trait UserServicePort: Send + Sync {
    fn create(&self, id: u64, name: &str) -> Result<User, Error>;
    fn get(&self, id: u64) -> Result<User, Error>;
    fn count(&self) -> Result<u64, Error>;
}

#[derive(Clone)]
struct UserServiceAdapter {
    database: Arc<dyn DatabaseServicePort>,
}

impl UserServiceAdapter {
    pub fn new(database: Arc<dyn DatabaseServicePort>) -> Self {
        Self { database }
    }
}

impl<Context: ContextResolverPort> AdapterPort<dyn UserServicePort, Context>
    for UserServiceAdapter
{
    async fn construct(context: &Context) -> Result<Arc<dyn UserServicePort>, Error> {
        let database = context.resolve::<dyn DatabaseServicePort>()?;

        Ok(Arc::new(Self::new(database)))
    }
}

impl UserServicePort for UserServiceAdapter {
    fn create(&self, id: u64, name: &str) -> Result<User, Error> {
        let user = User {
            id,
            name: name.to_string(),
        };

        self.database.create(&user)?;

        Ok(user)
    }

    fn get(&self, id: u64) -> Result<User, Error> {
        let user = self.database.get_by_id(id)?;

        Ok(user)
    }

    fn count(&self) -> Result<u64, Error> {
        let count = self.database.get_all()?.len() as u64;

        Ok(count)
    }
}

trait FailingPort: Send + Sync {}
struct FailingAdapter;
impl FailingPort for FailingAdapter {}
impl<Context: ContextResolverPort> AdapterPort<dyn FailingPort, Context> for FailingAdapter {
    async fn construct(_: &Context) -> Result<Arc<dyn FailingPort>, Error> {
        Err(Error::from("connection refused"))
    }
}

trait CyclicAPort: Send + Sync {}
struct CyclicA {
    #[allow(unused)]
    b: Arc<dyn CyclicBPort>,
}
impl CyclicAPort for CyclicA {}
impl<Context: ContextResolverPort> AdapterPort<dyn CyclicAPort, Context> for CyclicA {
    async fn construct(context: &Context) -> Result<Arc<dyn CyclicAPort>, Error> {
        let b = context.resolve::<dyn CyclicBPort>()?;

        Ok(Arc::new(Self { b }))
    }
}

trait CyclicBPort: Send + Sync {}
struct CyclicB {
    #[allow(unused)]
    a: Arc<dyn CyclicAPort>,
}
impl CyclicBPort for CyclicB {}
impl<Context: ContextResolverPort> AdapterPort<dyn CyclicBPort, Context> for CyclicB {
    async fn construct(context: &Context) -> Result<Arc<dyn CyclicBPort>, Error> {
        let a = context.resolve::<dyn CyclicAPort>()?;

        Ok(Arc::new(Self { a }))
    }
}

async fn create_di() -> Result<Arc<AppContext>, Error> {
    AppDI::default()
        .bind::<dyn DatabaseServicePort, DatabaseServiceAdapter>()
        .bind::<dyn UserServicePort, UserServiceAdapter>()
        .build()
        .await
}

#[tokio::test]
async fn should_be_success_created_di() -> Result<(), Error> {
    let di = create_di().await;

    assert!(di.is_ok(), "DI creation has failed.");

    Ok(())
}

#[tokio::test]
async fn should_be_created_new_user_by_db() -> Result<(), Error> {
    let context = create_di().await?;

    let user = User {
        id: 1,
        name: String::from("Andrey"),
    };

    let database = context.resolve::<dyn DatabaseServicePort>()?;

    assert!(
        database.create(&user).is_ok(),
        "Database user creation has failed."
    );

    Ok(())
}

#[tokio::test]
async fn should_get_user_after_create() -> Result<(), Error> {
    let context = create_di().await?;
    let database = context.resolve::<dyn DatabaseServicePort>()?;

    database.create(&User {
        id: 7,
        name: "Andrey".to_string(),
    })?;

    assert_eq!(database.get_by_id(7)?.name, "Andrey");

    Ok(())
}

#[tokio::test]
async fn should_fail_on_duplicate_create() -> Result<(), Error> {
    let di = create_di().await?;
    let database = di.resolve::<dyn DatabaseServicePort>()?;
    let user = User {
        id: 1,
        name: "Andrey".to_string(),
    };

    database.create(&user)?;

    assert!(database.create(&user).is_err(), "duplicate must fail");

    Ok(())
}

#[tokio::test]
async fn should_report_is_exists() -> Result<(), Error> {
    let di = create_di().await?;
    let database = di.resolve::<dyn DatabaseServicePort>()?;
    let user = User {
        id: 2,
        name: "Ivan".to_string(),
    };

    assert!(!database.is_exists(&user)?);
    database.create(&user)?;
    assert!(database.is_exists(&user)?);

    Ok(())
}

#[tokio::test]
async fn should_resolve_user_service_with_zero_count() -> Result<(), Error> {
    let di = create_di().await?;
    let service = di.resolve::<dyn UserServicePort>()?;

    assert_eq!(service.count()?, 0);

    Ok(())
}

#[tokio::test]
async fn should_create_user_through_service() -> Result<(), Error> {
    let di = create_di().await?;
    let service = di.resolve::<dyn UserServicePort>()?;

    let user = service.create(1, "Andrey")?;

    assert_eq!(user.id, 1);
    assert_eq!(user.name, "Andrey");

    Ok(())
}

#[tokio::test]
async fn should_error_when_user_not_found_through_service() -> Result<(), Error> {
    let di = create_di().await?;
    let service = di.resolve::<dyn UserServicePort>()?;

    assert!(service.get(999).is_err(), "unknown user must error");

    Ok(())
}

#[tokio::test]
async fn should_persist_user_created_through_service() -> Result<(), Error> {
    let di = create_di().await?;
    let service = di.resolve::<dyn UserServicePort>()?;

    service.create(1, "Andrey")?;

    assert_eq!(
        service.count()?,
        1,
        "singleton DB keeps state across resolves"
    );

    Ok(())
}

#[tokio::test]
async fn should_surface_factory_error_instead_of_masking() -> Result<(), Error> {
    let di = AppDI::default()
        .bind::<dyn FailingPort, FailingAdapter>()
        .build()
        .await;

    let Err(error) = di else {
        return Err(Error::from("factory error must surface, got Ok"));
    };

    let error = error.to_string();

    assert!(
        error.contains("connection refused"),
        "factory error must surface"
    );
    assert!(
        !error.contains("has not exists"),
        "must not be masked as missing"
    );

    Ok(())
}

#[tokio::test]
async fn should_fail_build_on_cycle_or_wrong_order() -> Result<(), Error> {
    let di = AppDI::default()
        .bind::<dyn CyclicAPort, CyclicA>()
        .bind::<dyn CyclicBPort, CyclicB>()
        .build()
        .await;

    let Err(error) = di else {
        return Err(Error::from("expected a cycle error, got Ok"));
    };

    let name = type_name::<dyn CyclicBPort>();

    assert!(
        error
            .to_string()
            .contains(&format!("Adapter {name} is not built")),
        "must report a cycle"
    );

    Ok(())
}

#[tokio::test]
async fn should_reject_duplicate_bind() -> Result<(), Error> {
    let context = AppDI::default()
        .bind::<dyn DatabaseServicePort, DatabaseServiceAdapter>()
        .bind::<dyn DatabaseServicePort, DatabaseServiceAdapter>()
        .build()
        .await;

    let Err(error) = context else {
        return Err(Error::from("duplicate bind must error, got Ok"));
    };

    let name = type_name::<dyn DatabaseServicePort>();

    assert!(
        error
            .to_string()
            .contains(&format!("Ports are bound more than once: {name}")),
        "duplicate bind must be reported"
    );

    Ok(())
}

trait AsyncConstructPort: Send + Sync {
    fn number(&self) -> u32;
}
struct AsyncConstructAdapter(u32);
impl AsyncConstructPort for AsyncConstructAdapter {
    fn number(&self) -> u32 {
        self.0
    }
}
impl<Context: ContextResolverPort> AdapterPort<dyn AsyncConstructPort, Context>
    for AsyncConstructAdapter
{
    async fn construct(_: &Context) -> Result<Arc<dyn AsyncConstructPort>, Error> {
        tokio::time::sleep(Duration::from_millis(100)).await;

        Ok(Arc::new(Self(1)))
    }
}

#[tokio::test]
async fn should_be_return_valid_number_from_async_construct_adapter() -> Result<(), Error> {
    let context = AppDI::default()
        .bind::<dyn AsyncConstructPort, AsyncConstructAdapter>()
        .build()
        .await?;

    let adapter = context.resolve::<dyn AsyncConstructPort>()?;

    let number = adapter.number();

    assert!(number == 1, "Number from async construct should be 1");

    Ok(())
}
