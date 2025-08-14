use async_trait::async_trait;
use ioc_container_rs::ports::adapter_port::AdapterPort;

pub trait AdapterNumberTestPort {
    fn get_number(&self) -> i32;
    fn set_number(&mut self, number: i32);
}

pub struct AdapterNumberTest {
    number: i32,
}

impl AdapterNumberTest {
    pub fn new() -> Self {
        Self { number: 0 }
    }
}

#[async_trait]
impl AdapterPort<AdapterNumberTest> for AdapterNumberTest {
    fn token() -> &'static str {
        "ADAPTER_NUMBER_TEST"
    }
}

impl AdapterNumberTestPort for AdapterNumberTest {
    fn get_number(&self) -> i32 {
        self.number
    }

    fn set_number(&mut self, number: i32) {
        self.number = number;
    }
}
