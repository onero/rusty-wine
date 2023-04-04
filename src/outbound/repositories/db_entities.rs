use crate::infrastructure::schema::wines;
use diesel::prelude::*;
use std::cmp::{Ord, Eq, PartialOrd, PartialEq};

#[derive(Queryable, Ord, Eq, PartialEq, PartialOrd)]
pub struct WineDb {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32
}

#[derive(Insertable)]
#[diesel(table_name = wines)]
pub struct NewWineDb {
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32
}
