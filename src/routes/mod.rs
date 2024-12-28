use axum::Router;

mod challenge_routes;

pub fn app_routes() -> Router {
    Router::new()
        .merge(challenge_routes::routes())
}
