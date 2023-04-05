use async_graphql::InputObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct NewWineDto {
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}