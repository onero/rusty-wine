use std::sync::Arc;
use async_graphql::{Context, EmptySubscription, Error, Object, Schema, SchemaBuilder};
use crate::application::inbound_ports::WineInboundPort;
use crate::application::models::Wine;
use crate::application::state::AppState;
use crate::application::wine_service::WineService;
use crate::inbound::rest::api_mapper::ApiMapper;
use crate::inbound::rest::dto_mapper::NewWinemapper;
use crate::inbound::rest::dto_models::NewWineDto;

pub struct WineQuery;

#[Object]
impl WineQuery {
    async fn hello(&self) -> &str {
        "Hello World"
    }

    async fn get_wine(&self, ctx: &Context<'_>, wine_id: i32) -> Result<Wine, Error> {
        let wine_service = ctx.data_unchecked::<Arc<WineService>>();
        let wine_option = wine_service.get_by_id(wine_id);
        match wine_option {
            None => Err(Error::new("Wine not found")),
            Some(wine) => {
                Ok(wine)
            }
        }
    }

    async fn get_wines(&self, ctx: &Context<'_>) -> Result<Vec<Wine>, Error> {
        let wine_service = ctx.data_unchecked::<Arc<WineService>>();
        let wines_option = wine_service.get_all();
        match wines_option {
            None => Err(Error::new("Failed to fetch wines")),
            Some(wines) => Ok(wines),
        }
    }
}

pub struct WineMutation;

#[Object]
impl WineMutation {
    async fn add_wine(&self, ctx: &Context<'_>, new_wine: NewWineDto) -> Result<Wine, Error> {
        let wine_service = ctx.data_unchecked::<Arc<WineService>>();
        let new_wine_entity = NewWinemapper::map_to_entity(new_wine);
        let added_wine = wine_service.add_wine(new_wine_entity);
        match added_wine {
            None => Err(Error::new("Failed to add wine")),
            Some(wine) => Ok(wine),
        }
    }

    async fn remove_wine(&self, ctx: &Context<'_>, wine_id: i32) -> Result<bool, Error> {
        let wine_service = ctx.data_unchecked::<Arc<WineService>>();
        let wine_to_be_deleted_option = wine_service.get_by_id(wine_id);

        if wine_to_be_deleted_option.is_none() {
            return Err(Error::new("Wine not found"));
        }

        let wine_deleted = wine_service.delete_wine(wine_id);
        Ok(wine_deleted)
    }
}

pub type WineSchema = Schema<WineQuery, WineMutation, EmptySubscription>;

pub fn create_schema() -> SchemaBuilder<WineQuery, WineMutation, EmptySubscription> {
    Schema::build(WineQuery, WineMutation, EmptySubscription)
}
