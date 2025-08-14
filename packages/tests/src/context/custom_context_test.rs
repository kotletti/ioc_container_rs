#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use ioc_container_rs::{
        container::di::{InjectAdapter, DI},
        ports::adapter_port::AdapterPort,
    };

    use crate::{
        adapters::adapter_number_test::AdapterNumberTest, context::custom_context::CustomContext,
    };

    async fn create_di() -> DI {
        let di = DI::new(Arc::new(CustomContext::new()));

        let di = di
            .inject(InjectAdapter {
                token: AdapterNumberTest::token(),
                factory: Arc::new(|_| AdapterNumberTest::new()),
            })
            .await
            .expect("Failed to inject adapter");

        di
    }

    #[tokio::test]
    async fn should_be_valid_di() {
        create_di().await;

        assert!(true, "Should be valid di");
    }

    #[tokio::test]
    async fn should_be_able_to_resolve_adapter() {
        let di = create_di().await;

        let context = di.get_context();

        let adapter = AdapterNumberTest::get_adapter(&context).await;

        assert!(adapter.is_ok());
    }

    #[tokio::test]
    async fn should_be_cast_to_context() {
        let di = create_di().await;

        let context = di.get_context();

        let context = context.as_any().downcast_ref::<CustomContext>();

        assert!(context.is_some());
    }

    #[tokio::test]
    async fn should_be_empty_metrics() {
        let di = create_di().await;

        let context = di.get_context();
        let context = context.as_any().downcast_ref::<CustomContext>();

        assert!(context.is_some());

        let context = context.unwrap();

        let metrics = context.get_metrics().await;

        let metrics = metrics.read().await;

        assert_eq!(metrics.len(), 0);
    }

    #[tokio::test]
    async fn should_be_able_to_increment_metrics() {
        let di = create_di().await;

        let context = di.get_context();

        let _ = AdapterNumberTest::get_adapter(&context).await;
        let _ = AdapterNumberTest::get_adapter(&context).await;

        let context = context.as_any().downcast_ref::<CustomContext>();

        assert!(context.is_some());

        let context = context.unwrap();

        let metrics = context.get_metrics().await;

        let metrics = metrics.read().await;

        let count = metrics.get(&AdapterNumberTest::token().to_string());

        assert!(count.is_some());

        let count = count.unwrap();

        assert_eq!(*count, 2);
    }
}
