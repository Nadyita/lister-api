use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{auth, handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        // Lists routes
        .route("/lists", get(handlers::get_all_lists))
        .route("/lists", post(handlers::create_list))
        .route("/lists/:id", get(handlers::get_list))
        .route("/lists/:id", put(handlers::update_list))
        .route("/lists/:id", delete(handlers::delete_list))
        // Items routes
        .route("/lists/:list_id/items", get(handlers::get_list_items))
        .route("/lists/:list_id/items", post(handlers::create_item))
        .route("/items/:id", get(handlers::get_item))
        .route("/items/:id", put(handlers::update_item))
        .route("/items/:id", delete(handlers::delete_item))
        .route("/items/:id/toggle", patch(handlers::toggle_item))
        // Categories routes
        .route("/categories", get(handlers::get_all_categories))
        .route("/categories", post(handlers::create_category))
        .route("/categories/:id", get(handlers::get_category))
        .route("/categories/:id", put(handlers::update_category))
        .route("/categories/:id", delete(handlers::delete_category))
        // Search routes
        .route("/search", get(handlers::search_names))
        .route(
            "/search/category-mappings",
            get(handlers::get_category_mappings),
        )
        .layer(middleware::from_fn_with_state(
            state.config.clone(),
            auth::auth_middleware,
        ))
        .with_state(state);

    Router::new()
        .nest("/api", api_routes)
        .route("/health", get(health_check))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

async fn health_check() -> &'static str {
    "OK"
}

