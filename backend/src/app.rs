use crate::{error::AppResult, user::UserRepository};
use axum::{
    extract::Extension,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use common::UserInput;

pub fn create_app() -> Router {
    Router::new()
        .merge(user_routes())
        .layer(Extension(UserRepository::default()))
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/user/list", get(list_users))
        .route("/user", post(create_user))
}

#[debug_handler]
async fn list_users(
    Extension(user_repository): Extension<UserRepository>,
) -> AppResult<impl IntoResponse> {
    let users = user_repository.get_users().await?;
    Ok(Json(users))
}

#[debug_handler]
async fn create_user(
    Extension(user_repository): Extension<UserRepository>,
    Json(user): Json<UserInput>,
) -> AppResult<impl IntoResponse> {
    user_repository.create_user(user).await?;
    Ok(())
}
