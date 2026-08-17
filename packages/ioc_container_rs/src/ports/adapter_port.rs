use crate::{errors::error::Error, ports::container_port::ContainerPort};
use std::{
    any::{TypeId, type_name},
    sync::Arc,
};

pub trait AdapterPort: Send + Sync + 'static {
    fn construct(_: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        Err(Error::from(format!(
            "{} has no sync construct; provide it via bind_instance",
            type_name::<Self>()
        )))
    }

    fn get_adapter(container: &Arc<dyn ContainerPort>) -> Result<Arc<Self>, Error> {
        let key = TypeId::of::<Self>();
        let name = type_name::<Self>();

        let any = container.clone().resolve(key, name)?;

        let arc = any
            .downcast::<Arc<Self>>()
            .map_err(|_| Error::from(format!("Cant downcast adapter {name}")))?;

        Ok(Arc::clone(&arc))
    }
}
