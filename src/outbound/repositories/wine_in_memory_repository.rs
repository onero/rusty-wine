use crate::application::models::{NewWine, Wine};
use crate::application::ports::outbound_ports::WineOutboundPort;

pub struct WineInMemoryRepository {}

impl WineOutboundPort for WineInMemoryRepository {
    fn get_by_id(&self, _id: i32) -> Option<Wine> {
        let not_found_id = 9999;
        if _id == not_found_id {
            return None;
        }

        return Some(Wine {
            id: 1337,
            name: "Barolo".to_string(),
            description: "InMemory wine of the year".to_string(),
            year: 2012,
            price: 250
        });
    }

    fn get_all(&self) -> Option<Vec<Wine>> {
        return Some(vec![
            Wine {
                id: 1337,
                name: "Barolo".to_string(),
                description: "InMemory wine of the year".to_string(),
                year: 2012,
                price: 250
            },
            Wine {
                id: 1338,
                name: "Barbaresco".to_string(),
                description: "InMemory runner-up wine of the year".to_string(),
                year: 2016,
                price: 150
            },
            Wine {
                id: 1339,
                name: "Barbera".to_string(),
                description: "InMemory wine".to_string(),
                year: 2018,
                price: 100
            },
        ]);
    }

    fn add_wine(&self, new_wine: NewWine) -> Option<Wine> {
        return Some(Wine {
            id: 1337,
            name: new_wine.name,
            description: new_wine.description,
            year: new_wine.year,
            price: new_wine.price
        });
    }

    fn delete_wine(&self, id: i32) -> bool {
        let not_found_id = 9999;
        if id == not_found_id {
            return false;
        }

        return true;
    }
}