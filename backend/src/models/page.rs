use crate::models::comment::Comment;
use crate::models::apod::Apod;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PagePackage {
    pub apod: ApodWithComments
}

impl IntoResponse for PagePackage {
    fn into_response(self) -> Response {
        Json(self).into_response()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApodWithComments {
    pub apod: Apod,
    pub comments: Vec<Comment>,
}
