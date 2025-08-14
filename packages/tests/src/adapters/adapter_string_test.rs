use async_trait::async_trait;
use ioc_container_rs::ports::adapter_port::AdapterPort;

pub trait AdapterStringTestPort {
    fn get_message(&self) -> &str;
    fn set_message(&mut self, message: String);
}

pub struct AdapterStringTest {
    message: String,
}

impl AdapterStringTest {
    pub fn new() -> Self {
        Self {
            message: String::from("Hello, World!"),
        }
    }
}

#[async_trait]
impl AdapterPort<AdapterStringTest> for AdapterStringTest {
    fn token() -> &'static str {
        "ADAPTER_STRING_TEST"
    }
}

impl AdapterStringTestPort for AdapterStringTest {
    fn get_message(&self) -> &str {
        &self.message
    }

    fn set_message(&mut self, message: String) {
        self.message = message;
    }
}
