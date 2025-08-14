#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ioc_container_rs::{
        container::di::{InjectAdapter, DI},
        context::container_context::ContainerContext,
        ports::adapter_port::AdapterPort,
    };

    use crate::adapters::adapter_string_test::{AdapterStringTest, AdapterStringTestPort};

    fn create_di() -> DI {
        DI::new(Arc::new(ContainerContext::new()))
    }

    #[tokio::test]
    async fn should_be_build_and_resolve_service() {
        let di = create_di();

        let di = di
            .inject(InjectAdapter {
                token: AdapterStringTest::token(),
                factory: Arc::new(|_| AdapterStringTest::new()),
            })
            .await
            .expect("Failed to inject adapter");

        let context = di.get_context();

        let svc = AdapterStringTest::get_adapter(&context).await;

        assert!(svc.is_ok());
    }

    #[tokio::test]
    async fn should_be_mutate_string() {
        let di = create_di();

        let di = di
            .inject(InjectAdapter {
                token: AdapterStringTest::token(),
                factory: Arc::new(|_| AdapterStringTest::new()),
            })
            .await
            .expect("Failed to inject adapter");

        let context = di.get_context();

        const NEW_STRING: &str = "Hello, Rust!";

        let mut svc = AdapterStringTest::get_adapter(&context)
            .await
            .expect("Failed to get adapter");

        svc.set_message(NEW_STRING.to_string());

        let message = svc.get_message();

        assert_eq!(message, NEW_STRING, "Message should be equal");
    }
}
