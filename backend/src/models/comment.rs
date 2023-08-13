// use crate::make_db_id;
// use crate::models::answer::AnswerId;
// use crate::models::question::QuestionId;
use crate::models::apod::ApodId;
// use axum::response::{IntoResponse, Response};
// use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
    pub id: CommentId,
    pub content: String,
    pub apod_id: Option<ApodId>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommentId(pub i32);

impl From<i32> for CommentId {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateComment {
    pub content: String,
    pub apod_id: Option<i32>,
}
