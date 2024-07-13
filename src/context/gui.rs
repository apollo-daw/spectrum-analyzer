use std::sync::Arc;
use crate::plugin::Plugin;

pub struct AsyncExecutor<P: Plugin> {
    pub(crate) execute_background: Arc<dyn Fn(P::BackgroundTask) + Send + Sync>,
    pub(crate) execute_gui: Arc<dyn Fn(P::BackgroundTask) + Send + Sync>,
}