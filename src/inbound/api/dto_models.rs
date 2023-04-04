use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct NewWineDto {
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}