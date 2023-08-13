use crate::make_db_id;
// use crate::models::answer::AnswerId;
// use crate::models::question::QuestionId;
use crate::models::apod::ApodId;
use crate::models::user::UserId;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Favorite {
    pub id: Option<FavoriteId>,
    pub user_id: Option<UserId>,
    pub apod_id: Option<ApodId>,
}

make_db_id!(FavoriteId);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum FavoriteReference {
    User(UserId),
    Apod(ApodId),
}

impl IntoResponse for Favorite {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetFavorite {
    pub user_id: Option<i32>,
    pub apod_id: Option<i32>,
}