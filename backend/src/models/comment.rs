use crate::make_db_id;
use crate::models::apod::ApodId;
use crate::models::user::{UserId};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Comment {
    pub id: Option<CommentId>,
    pub content: String,
    pub reference: CommentReference,
    pub user_id: Option<UserId>,
}

make_db_id!(CommentId);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CommentReference {
    Apod(ApodId),
}

impl IntoResponse for Comment {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}