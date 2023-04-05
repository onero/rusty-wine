use actix_web::web::{Json, Path};
use actix_web::{get, post, delete, HttpResponse, Responder, Result, web};
use crate::application::models::Wine;
use crate::application::state::AppState;
use crate::inbound::rest::api_mapper::ApiMapper;
use crate::inbound::rest::dto_mapper::NewWinemapper;
use crate::inbound::rest::dto_models::NewWineDto;

#[get("/wine/{wine_id}")]
pub async fn get_wine(wine_id: Path<i32>,
                      state: web::Data<AppState>
) -> Result<impl Responder> {
    let wine_id = wine_id.into_inner();

    let wine_option: Option<Wine> = state.wine_service.get_by_id(wine_id);

    match wine_option {
        None => Ok(HttpResponse::NotFound().finish().into()),
        Some(wine) => {
            Ok(HttpResponse::Ok().json(wine))
        }
    }
}

#[get("/")]
pub async fn get_wines(state: web::Data<AppState>) -> Result<impl Responder> {
    let wines_option: Option<Vec<Wine>> = state.wine_service.get_all();

    match wines_option {
        None => Ok(HttpResponse::InternalServerError().finish().into()),
        Some(wines) => {
            Ok(HttpResponse::Ok().json(wines))
        }
    }
}

#[post("/wine")]
pub async fn add_wine(new_wine: Json<NewWineDto>, state: web::Data<AppState>) -> Result<impl Responder> {
    let new_wine_dto = new_wine.into_inner();

    let new_wine_entity = NewWinemapper::map_to_entity(new_wine_dto);

    let added_wine: Option<Wine> = state.wine_service.add_wine(new_wine_entity);

    match added_wine {
        None => Ok(HttpResponse::InternalServerError().finish().into()),
        Some(wine) => {
            Ok(HttpResponse::Ok().json(wine))
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