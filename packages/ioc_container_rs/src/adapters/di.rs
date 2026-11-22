use std::{
    any::{TypeId, type_name},
    collections::HashSet,
    sync::Arc,
};

use crate::{
    adapters::app_context::AppContext,
    errors::error::Error,
    ports::{
        adapter_port::AdapterPort,
        container_port::{ContainerFactory, ContainerValue},
        context_port::{ContextReaderPort, ContextWriterPort},
    },
};

pub type AppDI = DI<AppContext>;

pub struct DI<Context> {
    bound: HashSet<TypeId>,
    duplicates: Vec<&'static str>,
    factories: Vec<(TypeId, ContainerFactory<Context>)>,
}

impl<Context: ContextReaderPort + ContextWriterPort + Default> Default for DI<Context> {
    fn default() -> Self {
        Self {
            bound: HashSet::new(),
            duplicates: vec![],
            factories: vec![],
        }
    }
}

impl<Context: ContextReaderPort + ContextWriterPort + Default> DI<Context> {
    pub fn bind<Port, Adapter>(mut self) -> Self
    where
        Port: ?Sized + Send + Sync + 'static,
        Adapter: AdapterPort<Port, Context>,
    {
        let key = TypeId::of::<Port>();

        if !self.bound.insert(key) {
            self.duplicates.push(type_name::<Port>());

            return self;
        }

        let factory: ContainerFactory<Context> = Box::new(|ctx| {
            Box::pin(async move {
                let port: Arc<Port> = Adapter::construct(ctx).await?;

                Ok(Arc::new(port) as ContainerValue)
            })
        });

        self.factories.push((key, factory));

        self
    }

    pub async fn build(self) -> Result<Arc<Context>, Error> {
        if !self.duplicates.is_empty() {
            return Err(Error::from(format!(
                "Ports are bound more than once: {}",
                self.duplicates.join(", ")
            )));
        }

        let mut context = Context::default();

        for (key, factory) in self.factories {
            let value = factory(&context).await?;

            context.write(key, value)?;
        }

        Ok(Arc::new(context))
    }
}
