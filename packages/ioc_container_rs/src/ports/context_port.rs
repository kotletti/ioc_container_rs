use std::{
    any::{TypeId, type_name},
    sync::Arc,
};

use crate::{errors::error::Error, ports::container_port::ContainerValue};

pub trait ContextReaderPort: Send + Sync + 'static {
    fn read(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error>;
}

pub trait ContextWriterPort: Send + Sync + 'static {
    fn write(&mut self, key: TypeId, value: ContainerValue) -> Result<(), Error>;
}

pub trait ContextResolverPort: Send + Sync + 'static {
    fn resolve<Port: Send + Sync + ?Sized + 'static>(&self) -> Result<Arc<Port>, Error>;
}

impl<C: ContextReaderPort + ?Sized> ContextResolverPort for C {
    fn resolve<Port: ?Sized + Send + Sync + 'static>(&self) -> Result<Arc<Port>, Error> {
        let name = type_name::<Port>();

        let value = self.read(TypeId::of::<Port>(), name)?;

        let port = value
            .downcast_ref::<Arc<Port>>()
            .ok_or_else(|| format!("Cant downcast adapter {name}"))?;

        Ok(port.clone())
    }
}
