use std::sync::Arc;
use actix_web::{App, http, test, web};
use actix_web::web::Data;
use rusty_wine::application::state::AppState;
use rusty_wine::application::wine_service::WineService;
use rusty_wine::inbound::dto_models::{NewWineDto, WineDto};
use rusty_wine::inbound::rest::wine_controller::{add_wine, remove_wine, get_wine, get_wines};
use rusty_wine::outbound::repositories::wine_in_memory_repository;

const BASE_URL: &str = "/wine";

#[test]
async fn should_list_wines() {
    // Arrange
    let app_state = setup_app_state();
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(get_wines)
    ).await;

    // Act
    let req = test::TestRequest::get()
        .uri("/").to_request();
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::OK);

    let result: Vec<WineDto> = test::read_body_json(response).await;

    assert!(!result.is_empty())
}

#[test]
async fn should_list_wine_by_id_given_valid_id() {
    // Arrange
    let app_state = setup_app_state();
    let mut app =
        test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(get_wine)
        ).await;

    let valid_id = 1337;

    // Act
    let req = test::TestRequest::get()
        .uri(format!("{}/{}",
                     BASE_URL,
                     valid_id)
            .as_str()
        )
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::OK);

    let wine_dto: WineDto = test::read_body_json(response).await;
    assert_eq!(wine_dto.id, valid_id);
}

#[test]
async fn should_return_not_found_given_invalid_id() {
    // Arrange
    let app_state = setup_app_state();
    let mut app =
        test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(get_wine)
        ).await;

    let invalid_id = 9999;

    // Act
    let req = test::TestRequest::get()
        .uri(format!("{}/{}",
                     BASE_URL,
                     invalid_id)
            .as_str()
        )
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}

#[test]
async fn should_add_new_wine_given_valid_input() {
    // Arrange
    let app_state = setup_app_state();
    let mut app = test::init_service(
        App::new()
            .app_data(app_state.clone())
            .service(add_wine)
    ).await;

    let new_wine = NewWineDto {
        name: "Adam's Wine".to_string(),
        year: 2023,
        description: "A wine for the future".to_string(),
        price: 1337,
    };

    // Act
    let req = test::TestRequest::post()
        .uri(format!("{}",
                     BASE_URL,)
            .as_str()
        )
        .set_json(&new_wine)
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::OK);

    let result: WineDto = test::read_body_json(response).await;
    assert_eq!(result.name, new_wine.name);
    assert_eq!(result.year, new_wine.year);
    assert_eq!(result.description, new_wine.description);
    assert_eq!(result.price, new_wine.price);
}

#[test]
async fn should_return_bad_request_given_invalid_input_when_adding_wine() {
    // Arrange
    let app_state = setup_app_state();
    let mut app =
        test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(add_wine)
        ).await;

    let new_wine = "foo bar";

    // Act
    let req = test::TestRequest::post()
        .uri(format!("{}",
                     BASE_URL,)
            .as_str()
        )
        .set_json(&new_wine)
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
}

#[test]
async fn should_remove_wine_given_valid_id() {
    // Arrange
    let app_state = setup_app_state();
    let mut app =
        test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(remove_wine)
        ).await;

    let valid_id = 1;

    // Act
    let req = test::TestRequest::delete()
        .uri(format!("{}/{}",
                     BASE_URL,
                     valid_id)
            .as_str()
        )
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::OK);
}

#[test]
async fn should_return_not_found_when_removing_wine_given_invalid_id() {
    // Arrange
    let app_state = setup_app_state();
    let mut app =
        test::init_service(
            App::new()
                .app_data(app_state.clone())
                .service(remove_wine)
        ).await;

    let invalid_id = 9999;

    // Act
    let req = test::TestRequest::delete()
        .uri(format!("{}/{}",
                     BASE_URL,
                     invalid_id)
            .as_str()
        )
        .to_request();
    let response = test::call_service(&mut app, req).await;

    // Assert
    assert_eq!(response.status(), http::StatusCode::NOT_FOUND);
}

fn setup_app_state() -> Data<AppState> {
    let wine_repository = Arc::new(wine_in_memory_repository::WineInMemoryRepository {});
    let wine_service = Arc::new(WineService { wine_repository });
    let app_state = web::Data::new(AppState { wine_service });
    app_state
}
