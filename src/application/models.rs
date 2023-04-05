use async_graphql::SimpleObject;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject)]
pub struct Wine {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}

#[derive(Debug, SimpleObject)]
pub struct NewWine {
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}