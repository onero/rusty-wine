use crate::application::models::{NewWine, Wine};

pub trait WineOutboundPort: Send + Sync {
    fn get_by_id(&self, id: i32) -> Option<Wine>;
    fn get_all(&self) -> Option<Vec<Wine>>;
    fn add_wine(&self, new_wine: NewWine) -> Option<Wine>;
}