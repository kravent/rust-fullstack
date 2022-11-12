use std::sync::Arc;

use crate::di::AppModule;
use crate::{error::AppResult, user::UserRepository};
use axum::{
    extract::Extension,
    response::{IntoResponse, Json},
    routing::{get, post},
    Router,
};
use axum_macros::debug_handler;
use common::UserInput;
use shaku_axum::Inject;

pub fn create_app() -> Router {
    let module = Arc::new(AppModule::builder().build());

    Router::new().merge(user_routes()).layer(Extension(module))
}

pub fn user_routes() -> Router {
    Router::new()
        .route("/user/list", get(list_users))
        .route("/user", post(create_user))
}

#[debug_handler]
async fn list_users(
    user_repository: Inject<AppModule, dyn UserRepository>,
) -> AppResult<impl IntoResponse> {
    let users = user_repository.get_users().await?;
    Ok(Json(users))
}

#[debug_handler]
async fn create_user(
    user_repository: Inject<AppModule, dyn UserRepository>,
    Json(user): Json<UserInput>,
) -> AppResult<impl IntoResponse> {
    user_repository.create_user(user).await?;
    Ok(())
}
