use std::{
    any::{TypeId, type_name},
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};

use crate::{
    adapters::{app_context::AppContext, container::Container, di::DI},
    errors::error::Error,
    ports::{
        adapter_port::AdapterPort,
        container_port::{ContainerPort, ContainerValue},
        context_port::{ContextReaderPort, ContextResolverPort, ContextWriterPort},
    },
};

trait ReadCounterPort {
    fn reads(&self) -> usize;
}

trait ConfigPort {
    fn database_url(&self) -> &str;
}

struct AppConfigContext {
    inner: AppContext,
    reads: AtomicUsize,
    database_url: String,
}

impl Default for AppConfigContext {
    fn default() -> Self {
        Self {
            inner: AppContext::default(),
            reads: AtomicUsize::new(0),
            database_url: "postgres://test".to_string(),
        }
    }
}

impl ContextReaderPort for AppConfigContext {
    fn read(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error> {
        self.reads.fetch_add(1, Ordering::SeqCst);

        self.inner.read(key, name)
    }
}

impl ContextWriterPort for AppConfigContext {
    fn write(&mut self, key: TypeId, value: ContainerValue) -> Result<(), Error> {
        self.inner.write(key, value)
    }
}

impl ReadCounterPort for AppConfigContext {
    fn reads(&self) -> usize {
        self.reads.load(Ordering::SeqCst)
    }
}

impl ConfigPort for AppConfigContext {
    fn database_url(&self) -> &str {
        &self.database_url
    }
}

struct RecordingContainer {
    inner: Container,
    written: Arc<Mutex<Vec<TypeId>>>,
}

impl ContainerPort for RecordingContainer {
    fn create(storage: HashMap<TypeId, ContainerValue>) -> Self {
        Self {
            inner: Container::create(storage),
            written: Arc::default(),
        }
    }

    fn insert(&mut self, key: TypeId, value: ContainerValue) {
        if let Ok(mut written) = self.written.lock() {
            written.push(key);
        }

        self.inner.insert(key, value);
    }

    fn get(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error> {
        self.inner.get(key, name)
    }

    fn existed(&self, key: TypeId) -> bool {
        self.inner.existed(key)
    }
}

struct RecordingContext(RecordingContainer);

impl Default for RecordingContext {
    fn default() -> Self {
        Self(RecordingContainer::create(HashMap::new()))
    }
}

impl RecordingContext {
    fn written(&self) -> Vec<TypeId> {
        self.0.written.lock().map(|w| w.clone()).unwrap_or_default()
    }
}

impl ContextReaderPort for RecordingContext {
    fn read(&self, key: TypeId, name: &'static str) -> Result<ContainerValue, Error> {
        self.0.get(key, name)
    }
}

impl ContextWriterPort for RecordingContext {
    fn write(&mut self, key: TypeId, value: ContainerValue) -> Result<(), Error> {
        self.0.insert(key, value);

        Ok(())
    }
}

trait RepositoryPort: Send + Sync {
    fn url(&self) -> String;
}

trait ServicePort: Send + Sync {
    fn url(&self) -> String;
}

trait ProbePort: Send + Sync {
    fn reads_at_construct(&self) -> usize;
}

struct Repository(String);
impl RepositoryPort for Repository {
    fn url(&self) -> String {
        self.0.clone()
    }
}
impl<Context: ContextResolverPort + ConfigPort> AdapterPort<dyn RepositoryPort, Context>
    for Repository
{
    async fn construct(context: &Context) -> Result<Arc<dyn RepositoryPort>, Error> {
        Ok(Arc::new(Self(context.database_url().to_string())))
    }
}

struct Service(Arc<dyn RepositoryPort>);
impl ServicePort for Service {
    fn url(&self) -> String {
        self.0.url()
    }
}
impl<Context: ContextResolverPort> AdapterPort<dyn ServicePort, Context> for Service {
    async fn construct(context: &Context) -> Result<Arc<dyn ServicePort>, Error> {
        Ok(Arc::new(Self(context.resolve::<dyn RepositoryPort>()?)))
    }
}

struct Probe(usize);
impl ProbePort for Probe {
    fn reads_at_construct(&self) -> usize {
        self.0
    }
}
impl<Context: ContextResolverPort + ReadCounterPort> AdapterPort<dyn ProbePort, Context> for Probe {
    async fn construct(context: &Context) -> Result<Arc<dyn ProbePort>, Error> {
        Ok(Arc::new(Self(context.reads())))
    }
}

trait PlainPort: Send + Sync {}
struct Plain;
impl PlainPort for Plain {}
impl<Context: ContextResolverPort> AdapterPort<dyn PlainPort, Context> for Plain {
    async fn construct(_: &Context) -> Result<Arc<dyn PlainPort>, Error> {
        Ok(Arc::new(Self))
    }
}

async fn create_di() -> Result<Arc<AppConfigContext>, Error> {
    DI::<AppConfigContext>::default()
        .bind::<dyn RepositoryPort, Repository>()
        .bind::<dyn ServicePort, Service>()
        .bind::<dyn ProbePort, Probe>()
        .build()
        .await
}

#[tokio::test]
async fn should_pass_context_capability_into_construct() -> Result<(), Error> {
    let context = create_di().await?;

    let service = context.resolve::<dyn ServicePort>()?;

    assert_eq!(
        service.url(),
        "postgres://test",
        "config must come from the context"
    );

    Ok(())
}

#[tokio::test]
async fn should_apply_custom_read_during_build() -> Result<(), Error> {
    let context = create_di().await?;

    let probe = context.resolve::<dyn ProbePort>()?;

    assert_eq!(
        probe.reads_at_construct(),
        1,
        "Service read the repository once before Probe was built"
    );

    Ok(())
}

#[tokio::test]
async fn should_apply_custom_read_after_build() -> Result<(), Error> {
    let context = create_di().await?;
    let before = context.reads();

    context.resolve::<dyn ServicePort>()?;
    context.resolve::<dyn RepositoryPort>()?;

    assert_eq!(context.reads(), before + 2);

    Ok(())
}

#[tokio::test]
async fn should_expose_custom_methods_after_build() -> Result<(), Error> {
    let context = create_di().await?;

    assert_eq!(context.database_url(), "postgres://test");

    Ok(())
}

#[tokio::test]
async fn should_share_instance_in_custom_context() -> Result<(), Error> {
    let context = create_di().await?;

    let first = context.resolve::<dyn RepositoryPort>()?;
    let second = context.resolve::<dyn RepositoryPort>()?;

    assert!(Arc::ptr_eq(&first, &second), "adapter must be built once");

    Ok(())
}

#[tokio::test]
async fn should_write_through_custom_container() -> Result<(), Error> {
    let context = DI::<RecordingContext>::default()
        .bind::<dyn PlainPort, Plain>()
        .build()
        .await?;

    assert_eq!(context.written(), vec![TypeId::of::<dyn PlainPort>()]);
    assert!(context.resolve::<dyn PlainPort>().is_ok());

    Ok(())
}

#[tokio::test]
async fn should_report_wrong_order_in_custom_context() -> Result<(), Error> {
    let Err(error) = DI::<AppConfigContext>::default()
        .bind::<dyn ServicePort, Service>()
        .bind::<dyn RepositoryPort, Repository>()
        .build()
        .await
    else {
        return Err(Error::from("wrong order must fail the build"));
    };

    let name = type_name::<dyn RepositoryPort>();

    assert!(
        error
            .to_string()
            .contains(&format!("Adapter {name} is not built"))
    );

    Ok(())
}

#[tokio::test]
async fn should_reject_duplicates_in_custom_context() -> Result<(), Error> {
    let result = DI::<AppConfigContext>::default()
        .bind::<dyn RepositoryPort, Repository>()
        .bind::<dyn RepositoryPort, Repository>()
        .build()
        .await;

    assert!(result.is_err(), "duplicate bind must fail the build");

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn should_build_custom_context_on_multi_thread_runtime() -> Result<(), Error> {
    let context = create_di().await?;

    assert_eq!(
        context.resolve::<dyn ServicePort>()?.url(),
        "postgres://test"
    );

    Ok(())
}

#[tokio::test]
async fn should_build_custom_context_inside_spawned_task() -> Result<(), Error> {
    let context = tokio::spawn(create_di()).await??;

    assert!(context.resolve::<dyn ProbePort>().is_ok());

    Ok(())
}
