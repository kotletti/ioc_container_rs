use crate::{errors::error::Error, ports::context_port::ContextResolverPort};
use std::sync::Arc;

pub trait AdapterPort<Port: Send + Sync + ?Sized + 'static, Context: ContextResolverPort>:
    Send + Sync
{
    fn construct(context: &Context) -> impl Future<Output = Result<Arc<Port>, Error>> + Send;
}
