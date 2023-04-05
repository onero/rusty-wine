use std::sync::{Arc};
use crate::application::ports::inbound_ports::WineInboundPort;
use crate::application::models::{NewWine, Wine};
use crate::application::ports::outbound_ports::WineOutboundPort;

pub struct WineService {
    pub wine_repository: Arc<dyn WineOutboundPort>,
}

impl WineInboundPort for WineService {
    fn get_by_id(&self, id: i32) -> Option<Wine> {
        let wine_option = &self.wine_repository.get_by_id(id);

        match wine_option {
            None => None,
            Some(wine) => Some(Wine {
                id: wine.id,
                name: wine.name.clone(),
                description: wine.description.clone(),
                year: wine.year,
                price: wine.price
            })
        }
    }

    fn get_all(&self) -> Option<Vec<Wine>> {
        let wines_option = &self.wine_repository.get_all();

        match wines_option {
            None => None,
            Some(wines) => Some(
                wines
                    .iter()
                    .map(|wine| Wine {
                        id: wine.id,
                        name: wine.name.clone(),
                        description: wine.description.clone(),
                        year: wine.year,
                        price: wine.price,
                    })
                    .collect::<Vec<Wine>>(),
            ),
        }
    }

    fn add_wine(&self, new_wine: NewWine) -> Option<Wine> {
        let added_wine_option = &self.wine_repository.add_wine(new_wine);

        match added_wine_option {
            None => None,
            Some(added_wine) => Some(Wine {
                id: added_wine.id,
                name: added_wine.name.clone(),
                description: added_wine.description.clone(),
                year: added_wine.year,
                price: added_wine.price
            })
        }
    }

    fn delete_wine(&self, wine_id: i32) -> bool {
        self.wine_repository.delete_wine(wine_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::application::ports::inbound_ports::WineInboundPort;
    use crate::application::models::{NewWine, Wine};
    use crate::application::ports::outbound_ports::WineOutboundPort;
    use crate::application::wine_service::WineService;
    use std::sync::{Arc};

    struct MockWineRepository {}

    impl WineOutboundPort for MockWineRepository {
        fn get_by_id(&self, wine_id: i32) -> Option<Wine> {
            Some(Wine {
                id: wine_id,
                name: "name".to_string(),
                description: "description".to_string(),
                year: 2020,
                price: 100,
            })
        }

        fn get_all(&self) -> Option<Vec<Wine>> {
            Some(vec![
                Wine {
                    id: 1,
                    name: "name".to_string(),
                    description: "description".to_string(),
                    year: 2020,
                    price: 100,
                },
                Wine {
                    id: 2,
                    name: "name".to_string(),
                    description: "description".to_string(),
                    year: 2020,
                    price: 100,
                },
            ])
        }

        fn add_wine(&self, new_wine: NewWine) -> Option<Wine> {
            Some(Wine {
                id: 1,
                name: new_wine.name,
                description: new_wine.description,
                year: new_wine.year,
                price: new_wine.price,
            })
        }

        fn delete_wine(&self, _id: i32) -> bool {
            true
        }
    }

    #[test]
    fn get_by_id() {
        let wine_service = WineService {
            wine_repository: Arc::new(MockWineRepository {}),
        };

        let wine = wine_service.get_by_id(1);

        assert_eq!(wine.unwrap().id, 1);
    }

    #[test]
    fn get_all() {
        let wine_service = WineService {
            wine_repository: Arc::new(MockWineRepository {}),
        };

        let wines = wine_service.get_all();

        assert_eq!(wines.unwrap().len(), 2);
    }

    #[test]
    fn add_wine() {
        let wine_service = WineService {
            wine_repository: Arc::new(MockWineRepository {}),
        };

        let new_wine = NewWine {
            name:
            "name".to_string(),
            description:
            "description".to_string(),
            year: 2020,
            price: 100,
        };

        let wine = wine_service.add_wine(new_wine);

        assert_eq!(wine.unwrap().id, 1);
    }

    #[test]
    fn delete_wine() {
        let wine_service = WineService {
            wine_repository: Arc::new(MockWineRepository {}),
        };

        let result = wine_service.delete_wine(1);

        assert_eq!(result, true);
    }
}