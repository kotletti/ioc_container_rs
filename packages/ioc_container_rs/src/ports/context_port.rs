use std::sync::Arc;

pub trait ContextPort {
    type Storage;

    fn create(storage: Self::Storage) -> Arc<Self>;
}
