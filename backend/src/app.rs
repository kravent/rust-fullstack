use crate::{error::AppResult, user::UserRepository};
use axum::{
    extract::Extension,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use axum_macros::debug_handler;

pub fn create_app() -> Router {
    Router::new()
        .merge(user_routes())
        .layer(Extension(UserRepository::default()))
}

pub fn user_routes() -> Router {
    Router::new().route("/user/list", get(list_users))
}

#[debug_handler]
async fn list_users(
    Extension(user_repository): Extension<UserRepository>,
) -> AppResult<impl IntoResponse> {
    let users = user_repository.get_users().await?;
    Ok(Json(users))
}
