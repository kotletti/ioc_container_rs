use crate::{
    adapters::{app_context::AppContext, di::DI},
    errors::error::Error,
};
use std::sync::Arc;

pub trait AdapterPort<Port: Send + Sync + ?Sized + 'static> {
    fn construct(context: &Arc<AppContext>) -> Result<Arc<Port>, Error>;

    fn get_adapter(context: &Arc<AppContext>) -> Result<Arc<Port>, Error> {
        context.resolve::<Port>()
    }

    fn create_di(di: DI) -> impl Future<Output = Result<DI, Error>>;
}
