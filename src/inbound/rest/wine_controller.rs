use actix_web::web::{Json, Path};
use actix_web::{get, post, delete, HttpResponse, Responder, Result, web};
use crate::application::models::Wine;
use crate::application::state::AppState;
use crate::application::ports::api_mapper_port::ApiMapperPort;
use crate::inbound::dto_mapper::{NewWineMapper, WineMapper};
use crate::inbound::dto_models::{NewWineDto, WineDto};

#[get("/wine/{wine_id}")]
pub async fn get_wine(wine_id: Path<i32>,
                      state: web::Data<AppState>
) -> Result<impl Responder> {
    let wine_id = wine_id.into_inner();

    let wine_option: Option<Wine> = state.wine_service.get_by_id(wine_id);

    match wine_option {
        None => Ok(HttpResponse::NotFound().finish().into()),
        Some(wine) => {
            let response = WineMapper::map_to_dto(&wine);
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

#[get("/")]
pub async fn get_wines(state: web::Data<AppState>) -> Result<impl Responder> {
    let wines_option: Option<Vec<Wine>> = state.wine_service.get_all();

    match wines_option {
        None => Ok(HttpResponse::InternalServerError().finish().into()),
        Some(wines) => {
            let response: Vec<WineDto> = wines
                .iter()
                .map(|wine| WineMapper::map_to_dto(wine))
                .collect();
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

#[post("/wine")]
pub async fn add_wine(new_wine: Json<NewWineDto>, state: web::Data<AppState>) -> Result<impl Responder> {
    let new_wine_dto = new_wine.into_inner();

    let new_wine_entity = NewWineMapper::map_to_entity(new_wine_dto);

    let added_wine: Option<Wine> = state.wine_service.add_wine(new_wine_entity);

    match added_wine {
        None => Ok(HttpResponse::InternalServerError().finish().into()),
        Some(wine) => {
            let response = WineMapper::map_to_dto(&wine);
            Ok(HttpResponse::Ok().json(response))
        }
    }
}

#[delete("/wine/{wine_id}")]
pub async fn remove_wine(wine_id: Path<i32>, state: web::Data<AppState>) -> Result<impl Responder> {
    let wine_id = wine_id.into_inner();

    let wine_to_be_deleted_option: Option<Wine> = state.wine_service.get_by_id(wine_id);

    if wine_to_be_deleted_option.is_none() {
        return Ok(HttpResponse::NotFound().json("Wine not found!"));
    }

    let wine_deleted= state.wine_service.delete_wine(wine_id);

    match wine_deleted {
        false => Ok(HttpResponse::InternalServerError().finish().into()),
        true => {
            Ok(HttpResponse::Ok().json("It is gone sir!"))
        }
    }
}