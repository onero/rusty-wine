use diesel::prelude::*;
use crate::application::models::{NewWine, Wine};
use crate::application::ports::outbound_ports::WineOutboundPort;
use crate::infrastructure::postgres::establish_connection;
use crate::infrastructure::schema::wines::dsl::wines;
use crate::infrastructure::schema::wines::{description, id, name, price, year};
use crate::outbound::repositories::db_entities::{NewWineDb, WineDb};

pub struct WinePostgresRepository {
}

impl WineOutboundPort for WinePostgresRepository {
    fn get_by_id(&self, wine_id: i32) -> Option<Wine> {

        let mut connection = establish_connection();

        wines
            .select((id, name, description, year, price))
            .filter(id.eq(wine_id))
            .first::<WineDb>(&mut connection)
            .ok()
            .map(|wine_db| Wine {
                id: wine_db.id,
                name: wine_db.name,
                description: wine_db.description,
                year: wine_db.year,
                price: wine_db.price,
            })
    }

    fn get_all(&self) -> Option<Vec<Wine>> {
        let mut connection = establish_connection();

        wines
            .load::<WineDb>(&mut connection)
            .ok()
            .map(|wine_db_list| {
                wine_db_list.into_iter().map(|wine_db| Wine {
                    id: wine_db.id,
                    name: wine_db.name,
                    description: wine_db.description,
                    year: wine_db.year,
                    price: wine_db.price,
                }).collect::<Vec<Wine>>()
            })
    }

    fn add_wine(&self, new_wine: NewWine) -> Option<Wine> {
        let mut connection = establish_connection();

        let new_wine_to_db = NewWineDb {
            name: new_wine.name,
            description: new_wine.description,
            year: new_wine.year,
            price: new_wine.price,
        };

        diesel::insert_into(wines)
            .values(&new_wine_to_db)
            .get_result::<WineDb>(&mut connection)
            .ok()
            .map(|wine_db| Wine {
                id: wine_db.id,
                name: wine_db.name,
                description: wine_db.description,
                year: wine_db.year,
                price: wine_db.price,
            })
    }

    fn delete_wine(&self, wine_id: i32) -> bool {
        let mut connection = establish_connection();

        diesel::delete(wines.filter(id.eq(wine_id)))
            .execute(&mut connection)
            .is_ok()
    }
}