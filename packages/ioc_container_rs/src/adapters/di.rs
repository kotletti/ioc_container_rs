use std::{
    any::{TypeId, type_name},
    sync::Arc,
};

use crate::{
    adapters::chain::ChainGuard,
    errors::error::Error,
    ports::{
        adapter_port::AdapterPort,
        container_port::{ContainerFactory, ContainerStorage},
        context_port::ContextPort,
    },
};

pub struct DI {
    storage: ContainerStorage,
}

impl DI {
    pub fn new() -> Self {
        let storage = ContainerStorage::new();

        Self { storage }
    }

    fn insert(
        mut self,
        key: TypeId,
        name: &'static str,
        factory: ContainerFactory,
    ) -> Result<Self, Error> {
        if self.storage.contains_key(&key) {
            return Err(Error::from(format!("Port {name} is already bound",)));
        }

        self.storage.insert(
            key,
            Box::new(move |container| match ChainGuard::enter(key) {
                Some(_) => factory(container),
                None => Err(Error::from(format!("Dependency cycle detected at {name}"))),
            }),
        );

        Ok(self)
    }

    pub fn bind<Port, Adapter>(self) -> Result<Self, Error>
    where
        Port: Send + Sync + ?Sized + 'static,
        Adapter: AdapterPort<Port>,
    {
        let key = TypeId::of::<Port>();
        let name = type_name::<Port>();

        self.insert(
            key,
            name,
            Box::new(|context| {
                let port = Adapter::construct(&context)?;

                Ok(Arc::new(port))
            }),
        )
    }

    pub fn build<C: ContextPort<Storage = ContainerStorage>>(self) -> Arc<C> {
        C::create(self.storage)
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, RwLock};

    use crate::{
        adapters::{app_context::AppContext, di::DI},
        errors::error::Error,
        ports::adapter_port::AdapterPort,
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

    impl AdapterPort<dyn DatabaseServicePort> for DatabaseServiceAdapter {
        fn construct(_: &Arc<AppContext>) -> Result<Arc<dyn DatabaseServicePort>, Error> {
            Ok(Arc::new(Self::new(vec![])))
        }

        async fn create_di(di: DI) -> Result<DI, Error> {
            di.bind::<dyn DatabaseServicePort, DatabaseServiceAdapter>()
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

    impl AdapterPort<dyn UserServicePort> for UserServiceAdapter {
        fn construct(context: &Arc<AppContext>) -> Result<Arc<dyn UserServicePort>, Error> {
            let database = context.resolve::<dyn DatabaseServicePort>()?;

            Ok(Arc::new(Self::new(database)))
        }

        async fn create_di(di: DI) -> Result<DI, Error> {
            di.bind::<dyn UserServicePort, UserServiceAdapter>()
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

    trait NotRegisteredPort: Send + Sync {}

    trait FailingPort: Send + Sync {}
    struct FailingAdapter;
    impl FailingPort for FailingAdapter {}
    impl AdapterPort<dyn FailingPort> for FailingAdapter {
        fn construct(_: &Arc<AppContext>) -> Result<Arc<dyn FailingPort>, Error> {
            Ok(Arc::new(Self))
        }

        async fn create_di(di: DI) -> Result<DI, Error> {
            di.bind::<dyn FailingPort, FailingAdapter>()
        }
    }

    trait CyclicAPort: Send + Sync {}
    struct CyclicA;
    impl CyclicAPort for CyclicA {}
    impl AdapterPort<dyn CyclicAPort> for CyclicA {
        fn construct(_: &Arc<AppContext>) -> Result<Arc<dyn CyclicAPort>, Error> {
            Ok(Arc::new(Self))
        }

        async fn create_di(di: DI) -> Result<DI, Error> {
            di.bind::<dyn CyclicAPort, CyclicA>()
        }
    }

    trait CyclicBPort: Send + Sync {}
    struct CyclicB;
    impl CyclicBPort for CyclicB {}
    impl AdapterPort<dyn CyclicBPort> for CyclicB {
        fn construct(_: &Arc<AppContext>) -> Result<Arc<dyn CyclicBPort>, Error> {
            Ok(Arc::new(Self))
        }

        async fn create_di(di: DI) -> Result<DI, Error> {
            di.bind::<dyn CyclicBPort, CyclicB>()
        }
    }

    async fn create_di() -> Result<Arc<AppContext>, Error> {
        let di = DI::new();
        let di = DatabaseServiceAdapter::create_di(di).await?;
        let di = UserServiceAdapter::create_di(di).await?;

        let context = di.build::<AppContext>();

        Ok(context)
    }

    #[tokio::test]
    async fn should_be_success_created_di() -> Result<(), Error> {
        let di = create_di().await;

        assert!(di.is_ok(), "DI creation has failed.");

        Ok(())
    }

    #[tokio::test]
    async fn should_be_downcast_success() -> Result<(), Error> {
        let context = create_di().await?;

        let database_service = context.resolve::<dyn DatabaseServicePort>();

        assert!(database_service.is_ok(), "Database service resolve failed.");

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
    async fn should_report_existed_tokens() -> Result<(), Error> {
        let di = create_di().await?;

        assert!(
            di.resolve::<dyn DatabaseServicePort>().is_ok(),
            "database is bound"
        );
        assert!(
            di.resolve::<dyn UserServicePort>().is_ok(),
            "user service is bound"
        );
        assert!(
            di.resolve::<dyn NotRegisteredPort>().is_err(),
            "not-registered must fail"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_error_on_unknown_token() -> Result<(), Error> {
        let di = create_di().await?;

        let Err(error) = di.resolve::<dyn NotRegisteredPort>() else {
            return Err(Error::from("not-found must error, got Ok"));
        };

        assert!(
            error.to_string().contains("NotRegisteredPort"),
            "not-found must name the adapter"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_resolve_ready_instance_via_bind_instance() -> Result<(), Error> {
        let preloaded = User {
            id: 42,
            name: "Ready".to_string(),
        };

        let ready: Arc<dyn DatabaseServicePort> =
            Arc::new(DatabaseServiceAdapter::new(vec![preloaded.clone()]));

        let di = DI::new()
            .bind::<dyn DatabaseServicePort, DatabaseServiceAdapter>()?
            .build::<AppContext>();

        let resolved = di.resolve::<dyn DatabaseServicePort>()?;
        assert_eq!(resolved.get_by_id(42)?.name, "Ready");

        assert!(
            Arc::ptr_eq(&ready, &resolved),
            "bind must return the bound instance"
        );

        let a = di.resolve::<dyn DatabaseServicePort>()?;
        let b = di.resolve::<dyn DatabaseServicePort>()?;
        assert!(Arc::ptr_eq(&a, &b), "two resolves must be the same Arc");

        Ok(())
    }

    #[test]
    fn should_surface_factory_error_instead_of_masking() -> Result<(), Error> {
        let di = DI::new()
            .bind::<dyn FailingPort, FailingAdapter>()?
            .build::<AppContext>();

        let Err(error) = di.resolve::<dyn FailingPort>() else {
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

    #[test]
    fn should_detect_dependency_cycle() -> Result<(), Error> {
        let di = DI::new()
            .bind::<dyn CyclicAPort, CyclicA>()?
            .bind::<dyn CyclicBPort, CyclicB>()?
            .build::<AppContext>();

        let Err(error) = di.resolve::<dyn CyclicAPort>() else {
            return Err(Error::from("expected a cycle error, got Ok"));
        };

        assert!(
            error.to_string().contains("Dependency cycle detected"),
            "must report a cycle"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_keep_chain_clean_after_cycle() -> Result<(), Error> {
        let cyclic = DI::new()
            .bind::<dyn CyclicAPort, CyclicA>()?
            .bind::<dyn CyclicBPort, CyclicB>()?
            .build::<AppContext>();

        assert!(
            cyclic.resolve::<dyn CyclicAPort>().is_err(),
            "cycle must be detected"
        );

        let healthy = create_di().await?;

        assert!(
            healthy.resolve::<dyn UserServicePort>().is_ok(),
            "healthy graph must build after a cycle error in the same thread"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_reject_duplicate_bind() -> Result<(), Error> {
        let di = DI::new();
        let di = DatabaseServiceAdapter::create_di(di).await?;

        let di = DatabaseServiceAdapter::create_di(di).await;

        let Err(error) = di else {
            return Err(Error::from("duplicate bind must error, got Ok"));
        };

        assert!(
            error.to_string().contains("already bound"),
            "duplicate bind must be reported"
        );

        Ok(())
    }
}
