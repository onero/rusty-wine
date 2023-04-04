use std::sync::Arc;
use crate::application::inbound_ports::WineInboundPort;

pub struct AppState {
    pub wine_service: Arc<dyn WineInboundPort>,
}