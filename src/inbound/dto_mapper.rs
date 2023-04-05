use crate::application::models::{NewWine, Wine};
use crate::application::ports::api_mapper_port::ApiMapperPort;
use crate::inbound::dto_models::{NewWineDto, WineDto};

pub struct WineMapper {}

impl ApiMapperPort<Wine, WineDto> for WineMapper {
    fn map_to_entity(dto: WineDto) -> Wine {
        Wine {
            id: dto.id,
            name: dto.name.clone(),
            year: dto.year,
            description: dto.description.clone(),
            price: dto.price,
        }
    }

    fn map_to_dto(entity: &Wine) -> WineDto {
        WineDto {
            id: entity.id,
            name: entity.name.clone(),
            year: entity.year,
            description: entity.description.clone(),
            price: entity.price,
        }
    }
}

pub struct NewWineMapper {}

impl ApiMapperPort<NewWine, NewWineDto> for NewWineMapper {
    fn map_to_entity(dto: NewWineDto) -> NewWine {
        NewWine {
            name: dto.name.clone(),
            year: dto.year,
            description: dto.description.clone(),
            price: dto.price,
        }
    }

    fn map_to_dto(entity: &NewWine) -> NewWineDto {
        NewWineDto {
            name: entity.name.clone(),
            year: entity.year,
            description: entity.description.clone(),
            price: entity.price,
        }
    }
}

#[cfg(test)]
mod wine_mapper_tests {
    use crate::application::models::NewWine;
    use crate::application::ports::api_mapper_port::ApiMapperPort;
    use crate::inbound::dto_mapper::NewWineMapper;
    use crate::inbound::dto_models::NewWineDto;

    #[test]
    fn wine_mapper_should_map_to_entity() {
        // Arrange
        let wine_name = "Barolo".to_string();
        let wine_year = 2012;
        let wine_description = "Wine of the year".to_string();
        let wine_price = 500;

        let dto = NewWineDto {
            name: wine_name.clone(),
            year: wine_year,
            description: wine_description.clone(),
            price: wine_price,
        };

        // Act
        let entity = NewWineMapper::map_to_entity(dto);

        // Assert
        assert_eq!(entity.name, wine_name.clone());
        assert_eq!(entity.year, wine_year);
        assert_eq!(entity.description, wine_description.clone());
        assert_eq!(entity.price, wine_price);
    }

    #[test]
    fn wine_mapper_should_map_to_dto() {
        // Arrange
        let wine_name = "Barolo".to_string();
        let wine_year = 2012;
        let wine_description = "Wine of the year".to_string();
        let wine_price = 500;

        let entity = NewWine {
            name: wine_name.clone(),
            year: wine_year,
            description: wine_description.clone(),
            price: wine_price,
        };

        // Act
        let dto = NewWineMapper::map_to_dto(&entity);

        // Assert
        assert_eq!(dto.name, wine_name.clone());
        assert_eq!(dto.year, wine_year);
        assert_eq!(dto.description, wine_description.clone());
        assert_eq!(dto.price, wine_price);
    }
}