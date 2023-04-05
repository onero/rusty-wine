use async_graphql::InputObject;
use async_graphql::SimpleObject;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct WineDto {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}

#[derive(Debug, Serialize, Deserialize, InputObject)]
pub struct NewWineDto {
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}