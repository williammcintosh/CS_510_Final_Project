use crate::models::answer::Answer;
use crate::models::comment::Comment;
use crate::models::question::Question;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
    pub question: QuestionWithComments,
    pub answers: Vec<AnswerWithComments>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AnswerWithComments {
    pub answer: Answer,
    pub comments: Vec<Comment>,
}