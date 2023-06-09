#[derive(Debug)]
pub struct Wine {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}

#[derive(Debug)]
pub struct NewWine {
    pub name: String,
    pub description: String,
    pub year: i32,
    pub price: i32,
}