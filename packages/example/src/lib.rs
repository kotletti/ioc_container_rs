pub mod adapters;
pub mod entities;
pub mod ports;

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use ioc_container_rs::{
        adapters::{app_context::AppContext, di::DI},
        errors::error::Error,
        ports::{adapter_port::AdapterPort, container_port::ContainerPort},
    };
    use tokio::sync::RwLock;

    use crate::{
        adapters::user_repository_adapter::UserRepositoryAdapter,
        entities::user_entity::UserEntity,
        ports::{
            input::{
                add_user_port::AddUserPort, delete_user_port::DeleteUserPort,
                get_user_port::GetUserPort,
            },
            output::{
                user_repository_port::UserRepositoryPort, user_service_port::UserServicePort,
            },
        },
    };

    fn create_di() -> Result<Arc<dyn ContainerPort>, Error> {
        let store: Arc<RwLock<Vec<UserEntity>>> = Arc::new(RwLock::new(vec![]));

        let user_repository: Arc<dyn UserRepositoryPort> =
            Arc::new(UserRepositoryAdapter::new(store));

        let container = DI::new()
            .bind_instance::<dyn UserRepositoryPort>(user_repository)?
            .bind::<dyn UserServicePort>()?
            .build::<AppContext>();

        Ok(container)
    }

    #[tokio::test]
    async fn should_be_return_zero_count_from_repository() -> Result<(), Error> {
        let container = create_di()?;

        let user_repository = <dyn UserRepositoryPort>::get_adapter(&container)?;

        assert_eq!(user_repository.get_count().await?, 0, "Count should be 0");

        Ok(())
    }

    #[tokio::test]
    async fn should_be_return_zero_count_from_service() -> Result<(), Error> {
        let container = create_di()?;

        let user_service = <dyn UserServicePort>::get_adapter(&container)?;

        assert_eq!(user_service.get_count().await?, 0, "Count should be 0");

        Ok(())
    }

    #[tokio::test]
    async fn should_return_new_user() -> Result<(), Error> {
        let container = create_di()?;

        let user_service = <dyn UserServicePort>::get_adapter(&container)?;

        let user_entity = user_service
            .add_user(&AddUserPort {
                name: "Andrey".to_string(),
                email: "andrey@mail.ru".to_string(),
            })
            .await?;

        assert_eq!(user_service.get_count().await?, 1, "Count should be 1");
        assert_eq!(user_entity.name, "Andrey", "Name should be Andrey");

        Ok(())
    }

    #[tokio::test]
    async fn should_remove_new_user() -> Result<(), Error> {
        let container = create_di()?;

        let user_service = <dyn UserServicePort>::get_adapter(&container)?;

        let user_entity = user_service
            .add_user(&AddUserPort {
                name: "Andrey".to_string(),
                email: "andrey@mail.ru".to_string(),
            })
            .await?;

        user_service
            .delete_user(&DeleteUserPort {
                email: user_entity.email.to_string(),
            })
            .await?;

        let check_user_entity = user_service
            .get_user(&GetUserPort {
                email: user_entity.email.to_string(),
            })
            .await?;

        assert!(
            check_user_entity.is_none(),
            "User after deletion should be None"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_share_repository_between_service_and_container() -> Result<(), Error> {
        let container = create_di()?;

        let user_service = <dyn UserServicePort>::get_adapter(&container)?;
        let user_repository = <dyn UserRepositoryPort>::get_adapter(&container)?;

        user_service
            .add_user(&AddUserPort {
                name: "Andrey".to_string(),
                email: "andrey@mail.ru".to_string(),
            })
            .await?;

        assert_eq!(
            user_repository.get_count().await?,
            1,
            "Service and container should share the same repository"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_error_when_adapter_is_not_bound() -> Result<(), Error> {
        let container = DI::new()
            .bind::<dyn UserServicePort>()?
            .build::<AppContext>();

        let Err(error) = <dyn UserServicePort>::get_adapter(&container) else {
            return Err(Error::from("Resolve without repository must error"));
        };

        assert!(
            error.to_string().contains("UserRepositoryPort"),
            "Error should name the missing port"
        );

        Ok(())
    }
}
