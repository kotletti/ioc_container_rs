use std::{
    any::{TypeId, type_name},
    sync::Arc,
};

use crate::{
    adapters::chain::ChainGuard,
    errors::error::Error,
    ports::{
        adapter_port::AdapterPort,
        container_port::{ContainerFactoryItem, ContainerPort, ContainerStorage},
    },
};

#[derive(Default)]
pub struct DI {
    storage: ContainerStorage,
}

impl DI {
    pub fn new() -> Self {
        Self {
            storage: ContainerStorage::new(),
        }
    }

    fn insert(
        mut self,
        key: TypeId,
        name: &'static str,
        factory: ContainerFactoryItem,
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

    pub fn bind<Port: AdapterPort + ?Sized>(self) -> Result<Self, Error> {
        self.insert(
            TypeId::of::<Port>(),
            type_name::<Port>(),
            Box::new(|container| Ok(Arc::new(Port::construct(&container)?))),
        )
    }

    pub fn bind_instance<Port: AdapterPort + ?Sized>(
        self,
        instance: Arc<Port>,
    ) -> Result<Self, Error> {
        self.insert(
            TypeId::of::<Port>(),
            type_name::<Port>(),
            Box::new(move |_| Ok(Arc::new(instance.clone()))),
        )
    }

    pub fn build<C: ContainerPort>(self) -> Arc<dyn ContainerPort> {
        C::create(self.storage)
    }
}

#[cfg(test)]
mod test {
    use std::{
        any::TypeId,
        sync::{Arc, RwLock},
    };

    use crate::{
        adapters::{app_context::AppContext, di::DI},
        errors::error::Error,
        ports::{adapter_port::AdapterPort, container_port::ContainerPort},
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

    impl AdapterPort for dyn DatabaseServicePort {
        fn construct(_: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
            Ok(Arc::new(DatabaseServiceAdapter::new(vec![])))
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

    impl AdapterPort for dyn UserServicePort {
        fn construct(container: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
            let database = <dyn DatabaseServicePort>::get_adapter(container)?;

            Ok(Arc::new(UserServiceAdapter::new(database)))
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
    impl AdapterPort for dyn NotRegisteredPort {}

    trait FailingPort: Send + Sync {}
    impl AdapterPort for dyn FailingPort {
        fn construct(_: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
            Err(Error::from("connection refused"))
        }
    }

    trait CyclicAPort: Send + Sync {}
    struct CyclicA;
    impl AdapterPort for dyn CyclicAPort {
        fn construct(container: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
            <dyn CyclicBPort>::get_adapter(container)?;
            Ok(Arc::new(CyclicA) as Arc<dyn CyclicAPort>)
        }
    }
    impl CyclicAPort for CyclicA {}

    trait CyclicBPort: Send + Sync {}
    struct CyclicB;
    impl AdapterPort for dyn CyclicBPort {
        fn construct(container: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
            <dyn CyclicAPort>::get_adapter(container)?;
            Ok(Arc::new(CyclicB) as Arc<dyn CyclicBPort>)
        }
    }
    impl CyclicBPort for CyclicB {}

    fn create_di() -> Result<Arc<dyn ContainerPort>, Error> {
        let container = DI::new()
            .bind::<dyn DatabaseServicePort>()?
            .bind::<dyn UserServicePort>()?
            .build::<AppContext>();

        Ok(container)
    }

    #[test]
    fn should_be_success_created_di() -> Result<(), Error> {
        let di = create_di();

        assert!(di.is_ok(), "DI creation has failed.");

        Ok(())
    }

    #[test]
    fn should_be_created_new_user_by_db() -> Result<(), Error> {
        let di = create_di()?;

        let user = User {
            id: 1,
            name: String::from("Andrey"),
        };

        let database = <dyn DatabaseServicePort>::get_adapter(&di)?;

        assert!(
            database.create(&user).is_ok(),
            "Database user creation has failed."
        );

        Ok(())
    }

    #[test]
    fn should_get_user_after_create() -> Result<(), Error> {
        let di = create_di()?;
        let database = <dyn DatabaseServicePort>::get_adapter(&di)?;

        database.create(&User {
            id: 7,
            name: "Andrey".to_string(),
        })?;

        assert_eq!(database.get_by_id(7)?.name, "Andrey");

        Ok(())
    }

    #[test]
    fn should_fail_on_duplicate_create() -> Result<(), Error> {
        let di = create_di()?;
        let database = <dyn DatabaseServicePort>::get_adapter(&di)?;
        let user = User {
            id: 1,
            name: "Andrey".to_string(),
        };

        database.create(&user)?;

        assert!(database.create(&user).is_err(), "duplicate must fail");

        Ok(())
    }

    #[test]
    fn should_report_is_exists() -> Result<(), Error> {
        let di = create_di()?;
        let database = <dyn DatabaseServicePort>::get_adapter(&di)?;
        let user = User {
            id: 2,
            name: "Ivan".to_string(),
        };

        assert!(!database.is_exists(&user)?);
        database.create(&user)?;
        assert!(database.is_exists(&user)?);

        Ok(())
    }

    #[test]
    fn should_resolve_user_service_with_zero_count() -> Result<(), Error> {
        let di = create_di()?;
        let service = <dyn UserServicePort>::get_adapter(&di)?;

        assert_eq!(service.count()?, 0);

        Ok(())
    }

    #[test]
    fn should_create_user_through_service() -> Result<(), Error> {
        let di = create_di()?;
        let service = <dyn UserServicePort>::get_adapter(&di)?;

        let user = service.create(1, "Andrey")?;

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "Andrey");

        Ok(())
    }

    #[test]
    fn should_error_when_user_not_found_through_service() -> Result<(), Error> {
        let di = create_di()?;
        let service = <dyn UserServicePort>::get_adapter(&di)?;

        assert!(service.get(999).is_err(), "unknown user must error");

        Ok(())
    }

    #[test]
    fn should_persist_user_created_through_service() -> Result<(), Error> {
        let di = create_di()?;
        let service = <dyn UserServicePort>::get_adapter(&di)?;

        service.create(1, "Andrey")?;

        assert_eq!(
            service.count()?,
            1,
            "singleton DB keeps state across resolves"
        );

        Ok(())
    }

    #[test]
    fn should_report_existed_tokens() -> Result<(), Error> {
        let di = create_di()?;

        assert!(di.existed(TypeId::of::<dyn DatabaseServicePort>()));
        assert!(di.existed(TypeId::of::<dyn UserServicePort>()));
        assert!(!di.existed(TypeId::of::<dyn NotRegisteredPort>()));

        Ok(())
    }

    #[test]
    fn should_error_on_unknown_token() -> Result<(), Error> {
        let di = create_di()?;

        let Err(error) = <dyn NotRegisteredPort>::get_adapter(&di) else {
            return Err(Error::from("not-found must error, got Ok"));
        };

        assert!(
            error.to_string().contains("NotRegisteredPort"),
            "not-found must name the adapter"
        );

        Ok(())
    }

    #[test]
    fn should_resolve_ready_instance_via_bind_instance() -> Result<(), Error> {
        let preloaded = User {
            id: 42,
            name: "Ready".to_string(),
        };

        let ready: Arc<dyn DatabaseServicePort> =
            Arc::new(DatabaseServiceAdapter::new(vec![preloaded.clone()]));

        let di = DI::new()
            .bind_instance::<dyn DatabaseServicePort>(ready.clone())?
            .build::<AppContext>();

        let resolved = <dyn DatabaseServicePort>::get_adapter(&di)?;
        assert_eq!(resolved.get_by_id(42)?.name, "Ready");

        assert!(
            Arc::ptr_eq(&ready, &resolved),
            "bind_instance must return the bound instance"
        );

        let a = <dyn DatabaseServicePort>::get_adapter(&di)?;
        let b = <dyn DatabaseServicePort>::get_adapter(&di)?;
        assert!(Arc::ptr_eq(&a, &b), "two resolves must be the same Arc");

        Ok(())
    }

    #[test]
    fn should_surface_factory_error_instead_of_masking() -> Result<(), Error> {
        let di = DI::new().bind::<dyn FailingPort>()?.build::<AppContext>();

        let Err(error) = <dyn FailingPort>::get_adapter(&di) else {
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
            .bind::<dyn CyclicAPort>()?
            .bind::<dyn CyclicBPort>()?
            .build::<AppContext>();

        let Err(error) = <dyn CyclicAPort>::get_adapter(&di) else {
            return Err(Error::from("expected a cycle error, got Ok"));
        };

        assert!(
            error.to_string().contains("Dependency cycle detected"),
            "must report a cycle"
        );

        Ok(())
    }

    #[test]
    fn should_keep_chain_clean_after_cycle() -> Result<(), Error> {
        let cyclic = DI::new()
            .bind::<dyn CyclicAPort>()?
            .bind::<dyn CyclicBPort>()?
            .build::<AppContext>();

        assert!(
            <dyn CyclicAPort>::get_adapter(&cyclic).is_err(),
            "cycle must be detected"
        );

        let healthy = create_di()?;

        assert!(
            <dyn UserServicePort>::get_adapter(&healthy).is_ok(),
            "healthy graph must build after a cycle error in the same thread"
        );

        Ok(())
    }

    #[test]
    fn should_reject_duplicate_bind() -> Result<(), Error> {
        let Err(error) = DI::new()
            .bind::<dyn DatabaseServicePort>()?
            .bind::<dyn DatabaseServicePort>()
        else {
            return Err(Error::from("duplicate bind must error, got Ok"));
        };

        assert!(
            error.to_string().contains("already bound"),
            "duplicate bind must be reported"
        );

        Ok(())
    }

    #[test]
    fn should_error_when_adapter_has_no_construct() -> Result<(), Error> {
        let di = DI::new()
            .bind::<dyn NotRegisteredPort>()?
            .build::<AppContext>();

        let Err(error) = <dyn NotRegisteredPort>::get_adapter(&di) else {
            return Err(Error::from("adapter without construct must error, got Ok"));
        };

        assert!(
            error.to_string().contains("has no sync construct"),
            "default construct must explain itself"
        );

        Ok(())
    }
}
