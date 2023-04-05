use std::sync::Arc;
use crate::application::ports::inbound_ports::WineInboundPort;

pub struct AppState {
    pub wine_service: Arc<dyn WineInboundPort>,
}