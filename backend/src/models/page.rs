use crate::models::comment::Comment;
use crate::models::question::Question;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
    pub question: QuestionWithComments
}

impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QuestionWithComments {
    pub question: Question,
    pub comments: Vec<Comment>,
}
