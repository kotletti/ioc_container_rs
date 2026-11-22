pub mod adapters;

#[cfg(test)]
mod tests {
    use std::{any::type_name, sync::Arc};

    use example_domain::ports::{
        user_repository_port::UserRepositoryPort, user_service_port::UserServicePort,
    };
    use ioc_container_rs::{
        adapters::{app_context::AppContext, di::DI},
        errors::error::Error,
        ports::context_port::ContextResolverPort,
    };

    use crate::adapters::{
        user_repository_adapter::UserRepositoryAdapter, user_service_adapter::UserServiceAdapter,
    };

    async fn create_di() -> Result<Arc<AppContext>, Error> {
        DI::<AppContext>::default()
            .bind::<dyn UserRepositoryPort, UserRepositoryAdapter>()
            .bind::<dyn UserServicePort, UserServiceAdapter>()
            .build()
            .await
    }

    #[tokio::test]
    async fn should_be_return_zero_count_from_repository() -> Result<(), Error> {
        let context = create_di().await?;

        let user_repository = context.resolve::<dyn UserRepositoryPort>()?;

        assert_eq!(user_repository.get_count().await?, 0, "Count should be 0");

        Ok(())
    }

    #[tokio::test]
    async fn should_be_return_zero_count_from_service() -> Result<(), Error> {
        let context = create_di().await?;

        let user_service = context.resolve::<dyn UserServicePort>()?;

        assert_eq!(user_service.get_count().await?, 0, "Count should be 0");

        Ok(())
    }

    #[tokio::test]
    async fn should_return_new_user() -> Result<(), Error> {
        let context = create_di().await?;

        let user_service = context.resolve::<dyn UserServicePort>()?;

        let user_entity = user_service.add_user(1, "Andrey", "andrey@mail.ru").await?;

        assert_eq!(user_service.get_count().await?, 1, "Count should be 1");
        assert_eq!(user_entity.name, "Andrey", "Name should be Andrey");

        Ok(())
    }

    #[tokio::test]
    async fn should_remove_new_user() -> Result<(), Error> {
        let context = create_di().await?;

        let user_service = context.resolve::<dyn UserServicePort>()?;

        let user_entity = user_service.add_user(1, "Andrey", "andrey@mail.ru").await?;

        user_service.delete_user(user_entity.id).await?;

        let check_user_entity = user_service.get_user(user_entity.id).await;

        assert!(
            check_user_entity.is_err(),
            "User after deletion should be not exists"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_share_repository_between_service_and_container() -> Result<(), Error> {
        let context = create_di().await?;

        let user_service = context.resolve::<dyn UserServicePort>()?;
        let user_repository = context.resolve::<dyn UserRepositoryPort>()?;

        user_service.add_user(1, "Andrey", "andrey@mail.ru").await?;

        assert_eq!(
            user_repository.get_count().await?,
            1,
            "Service and container should share the same repository"
        );

        Ok(())
    }

    #[tokio::test]
    async fn should_error_when_adapter_is_not_bound() -> Result<(), Error> {
        let di = DI::<AppContext>::default()
            .bind::<dyn UserServicePort, UserServiceAdapter>()
            .build()
            .await;

        let Err(error) = di else {
            return Err(Error::from("Resolve without repository must error"));
        };

        let name = type_name::<dyn UserRepositoryPort>();

        assert!(
            error
                .to_string()
                .contains(&format!("Adapter {name} is not built")),
            "Error should name the missing port"
        );

        Ok(())
    }
}
