#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ioc_container_rs::container::di::{InjectAdapter, DI};
    use ioc_container_rs::context::container_context::ContainerContext;
    use ioc_container_rs::ports::adapter_port::AdapterPort;

    use crate::adapters::adapter_nested_test::{AdapterNested, AdapterNestedPort};
    use crate::adapters::adapter_number_test::AdapterNumberTest;
    use crate::adapters::adapter_string_test::AdapterStringTest;

    const DEFAULT_STRING: &str = "Hello, World!";
    const DEFAULT_NUMBER: i32 = 0;

    async fn create_di() -> DI {
        let di = DI::new(Arc::new(ContainerContext::new()))
            .inject(InjectAdapter {
                token: AdapterStringTest::token(),
                factory: Arc::new(|_| AdapterStringTest::new()),
            })
            .await
            .expect("Failed to inject AdapterStringTest");

        let di = di
            .inject(InjectAdapter {
                token: AdapterNumberTest::token(),
                factory: Arc::new(|_| AdapterNumberTest::new()),
            })
            .await
            .expect("Failed to inject AdapterNumberTest");

        let di = di
            .inject(InjectAdapter {
                token: AdapterNested::token(),
                factory: Arc::new(|context| AdapterNested::new(context)),
            })
            .await
            .expect("Failed to inject AdapterNested");

        di
    }

    #[tokio::test]
    async fn should_be_build_and_resolve_service_nested() {
        let di = create_di().await;

        let context = di.get_context();

        let svc = AdapterNested::get_adapter(&context).await;

        assert!(svc.is_ok());
    }

    #[tokio::test]
    async fn should_be_access_to_nested_services() {
        let di = create_di().await;

        let context = di.get_context();

        let svc = AdapterNested::get_adapter(&context)
            .await
            .expect("Failed to get AdapterNested");

        let number = svc.get_number().await.expect("Failed to get number");
        let string = svc.get_string().await.expect("Failed to get string");

        assert_eq!(
            number,
            DEFAULT_NUMBER,
            "{}",
            format!("Number should be {}", DEFAULT_NUMBER)
        );
        assert_eq!(
            string,
            DEFAULT_STRING,
            "{}",
            format!("String should be {}", DEFAULT_STRING)
        );
    }
}
