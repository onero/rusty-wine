use std::sync::Arc;

use actix_web::{App, guard, HttpResponse, HttpServer, Responder, web};
use rusty_wine::application::wine_service::WineService;
use rusty_wine::inbound::rest::wine_controller::{add_wine, get_wine, get_wines, remove_wine};
use rusty_wine::inbound::graphql::graphql_schema::{create_schema, WineMutation, WineQuery, WineSchema};
use rusty_wine::infrastructure::graphql::{index, index_graphiql};
use rusty_wine::outbound::repositories::wine_postgres_repository;
use async_graphql::{EmptySubscription, Schema};
use rusty_wine::application::state::AppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // Create wine service and wrap it with Arc
    let wine_repository = Arc::new(wine_postgres_repository::WinePostgresRepository {});
    let wine_service = Arc::new(WineService { wine_repository });
    let rest_state = web::Data::new(AppState { wine_service: wine_service.clone() });

    let graphql_schema = Schema::build(WineQuery, WineMutation, EmptySubscription)
        .data(wine_service)
        .finish();
    let graphql_state = web::Data::new(graphql_schema.clone());

    // Start HTTP server
    println!("Running on http://127.0.0.1:7878");

    HttpServer::new(move || {
        App::new()
            .app_data(rest_state.clone())
            .app_data(graphql_state.clone())
            .service(web::resource("/graphql").guard(guard::Post()).to(index))
            .service(web::resource("/graphql").guard(guard::Get()).to(index_graphiql))

            // REST
            .service(get_wine)
            .service(get_wines)
            .service(add_wine)
            .service(remove_wine)
            .default_service(web::route().guard(guard::Not(guard::Get())).to(handle_unknown))
    })
        .bind(("127.0.0.1", 7878))?
        .run()
        .await
}

async fn handle_unknown() -> impl Responder {
    HttpResponse::NotFound().json("Did you forget to register the route?")
}
