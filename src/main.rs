use std::sync::{Arc};

use actix_web::{App, guard, HttpResponse, HttpServer, Responder, web};

use rusty_wine::application::state::AppState;
use rusty_wine::application::wine_service::WineService;
use rusty_wine::inbound::api::wine_controller::{add_wine, get_wine, get_wines};
use rusty_wine::outbound::repositories::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //let wine_repository = Arc::new(wine_in_memory_repository::WineInMemoryRepository {});
    let wine_repository = Arc::new(wine_postgres_repository::WinePostgresRepository {});
    let wine_service = Arc::new(WineService { wine_repository });
    let app_state = web::Data::new(AppState { wine_service });

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(get_wine)
            .service(get_wines)
            .service(add_wine)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(handle_unknown),
            )
    })
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}

async fn handle_unknown() -> impl Responder {
    HttpResponse::NotFound().json("Did you forget to register the route?")
}