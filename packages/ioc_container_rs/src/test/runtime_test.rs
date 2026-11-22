
use std::{
    sync::{
        Arc,
        atomic::{AtomicU32, Ordering},
    },
    thread,
};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    runtime::Builder,
    sync::{Mutex, mpsc, oneshot},
    task::JoinSet,
};

use crate::{
    adapters::{app_context::AppContext, di::AppDI},
    errors::error::Error,
    ports::{adapter_port::AdapterPort, context_port::ContextResolverPort},
};

trait CounterPort: Send + Sync {
    fn increment(&self) -> u32;
}

struct CounterAdapter(AtomicU32);
impl CounterPort for CounterAdapter {
    fn increment(&self) -> u32 {
        self.0.fetch_add(1, Ordering::SeqCst) + 1
    }
}
impl<Context: ContextResolverPort> AdapterPort<dyn CounterPort, Context> for CounterAdapter {
    async fn construct(_: &Context) -> Result<Arc<dyn CounterPort>, Error> {
        tokio::task::yield_now().await;

        Ok(Arc::new(Self(AtomicU32::new(0))))
    }
}

type Request = (u32, oneshot::Sender<u32>);

trait WorkerPort: Send + Sync {
    fn ask(&self, value: u32) -> Result<oneshot::Receiver<u32>, Error>;
}

struct WorkerAdapter(mpsc::Sender<Request>);
impl WorkerPort for WorkerAdapter {
    fn ask(&self, value: u32) -> Result<oneshot::Receiver<u32>, Error> {
        let (reply, answer) = oneshot::channel();

        self.0
            .try_send((value, reply))
            .map_err(|e| Error::from(e.to_string()))?;

        Ok(answer)
    }
}
impl<Context: ContextResolverPort> AdapterPort<dyn WorkerPort, Context> for WorkerAdapter {
    async fn construct(_: &Context) -> Result<Arc<dyn WorkerPort>, Error> {
        let (sender, mut receiver) = mpsc::channel::<Request>(8);

        tokio::spawn(async move {
            while let Some((value, reply)) = receiver.recv().await {
                let _ = reply.send(value * 2);
            }
        });

        Ok(Arc::new(Self(sender)))
    }
}

trait EchoPort: Send + Sync {
    fn stream(&self) -> &Mutex<TcpStream>;
}

struct EchoAdapter(Mutex<TcpStream>);
impl EchoPort for EchoAdapter {
    fn stream(&self) -> &Mutex<TcpStream> {
        &self.0
    }
}
impl<Context: ContextResolverPort> AdapterPort<dyn EchoPort, Context> for EchoAdapter {
    async fn construct(_: &Context) -> Result<Arc<dyn EchoPort>, Error> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let address = listener.local_addr()?;

        tokio::spawn(async move {
            while let Ok((mut socket, _)) = listener.accept().await {
                tokio::spawn(async move {
                    let mut buffer = [0; 64];

                    while let Ok(read) = socket.read(&mut buffer).await {
                        if read == 0 || socket.write_all(&buffer[..read]).await.is_err() {
                            break;
                        }
                    }
                });
            }
        });

        let stream = TcpStream::connect(address).await?;

        Ok(Arc::new(Self(Mutex::new(stream))))
    }
}

async fn echo(port: &dyn EchoPort, message: &[u8]) -> Result<Vec<u8>, Error> {
    let mut stream = port.stream().lock().await;
    let mut buffer = vec![0; message.len()];

    stream.write_all(message).await?;
    stream.read_exact(&mut buffer).await?;

    Ok(buffer)
}

async fn create_di() -> Result<Arc<AppContext>, Error> {
    AppDI::default()
        .bind::<dyn CounterPort, CounterAdapter>()
        .bind::<dyn WorkerPort, WorkerAdapter>()
        .bind::<dyn EchoPort, EchoAdapter>()
        .build()
        .await
}

async fn check_resources(context: &AppContext) -> Result<(), Error> {
    let worker = context.resolve::<dyn WorkerPort>()?;
    let echo_port = context.resolve::<dyn EchoPort>()?;

    assert_eq!(
        worker.ask(21)?.await?,
        42,
        "background task must outlive construct"
    );
    assert_eq!(
        echo(echo_port.as_ref(), b"ping").await?,
        b"ping",
        "connection must outlive construct"
    );

    Ok(())
}

#[tokio::test]
async fn should_keep_resources_alive_on_current_thread() -> Result<(), Error> {
    let context = create_di().await?;

    check_resources(&context).await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn should_keep_resources_alive_on_multi_thread() -> Result<(), Error> {
    let context = create_di().await?;

    check_resources(&context).await
}

#[test]
fn should_build_on_manual_current_thread_runtime() -> Result<(), Error> {
    let runtime = Builder::new_current_thread().enable_all().build()?;

    runtime.block_on(async {
        let context = create_di().await?;

        check_resources(&context).await
    })
}

#[test]
fn should_build_on_manual_multi_thread_runtime() -> Result<(), Error> {
    let runtime = Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()?;

    runtime.block_on(async {
        let context = create_di().await?;

        check_resources(&context).await
    })
}

#[tokio::test]
async fn should_build_inside_spawned_task() -> Result<(), Error> {
    let context = tokio::spawn(create_di()).await??;

    check_resources(&context).await
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn should_share_context_between_tasks() -> Result<(), Error> {
    let context = create_di().await?;
    let mut tasks = JoinSet::new();

    for _ in 0..100 {
        let context = context.clone();

        tasks.spawn(async move {
            context
                .resolve::<dyn CounterPort>()
                .map(|counter| counter.increment())
        });
    }

    while let Some(result) = tasks.join_next().await {
        result??;
    }

    assert_eq!(
        context.resolve::<dyn CounterPort>()?.increment(),
        101,
        "all tasks must hit the same instance"
    );

    Ok(())
}

#[tokio::test]
async fn should_resolve_from_plain_threads() -> Result<(), Error> {
    let context = create_di().await?;

    let handles: Vec<_> = (0..8)
        .map(|_| {
            let context = context.clone();

            thread::spawn(move || {
                context
                    .resolve::<dyn CounterPort>()
                    .map(|counter| counter.increment())
            })
        })
        .collect();

    for handle in handles {
        handle
            .join()
            .map_err(|_| Error::from("thread panicked"))??;
    }

    assert_eq!(context.resolve::<dyn CounterPort>()?.increment(), 9);

    Ok(())
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn should_build_independent_containers_concurrently() -> Result<(), Error> {
    let mut builds = JoinSet::new();

    for _ in 0..10 {
        builds.spawn(create_di());
    }

    let mut contexts = Vec::new();

    while let Some(result) = builds.join_next().await {
        contexts.push(result??);
    }

    for context in &contexts {
        assert_eq!(
            context.resolve::<dyn CounterPort>()?.increment(),
            1,
            "containers must not share instances"
        );
    }

    Ok(())
}
