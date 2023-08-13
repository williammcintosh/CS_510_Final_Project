// use crate::make_db_id;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};
use chrono::{
    DateTime,
    // TimeZone, NaiveDateTime,
    Utc
};

// This uses the `derive_more` crate to reduce the Display boilerplate (see below)
#[derive(Clone, Debug, Display, Serialize, Deserialize, sqlx::FromRow)]
#[display(
fmt = "id: {}, img_date: {}, explanation: {}, title: {}, url: {}",
id,
img_date,
explanation,
title,
url,
)]
pub struct Apod {
    pub id: ApodId,
    pub img_date: DateTime<Utc>,
    pub explanation: String,
    pub title: String,
    pub url: String,
}

impl Apod {
    #[allow(dead_code)]
    pub fn new(id: ApodId, img_date: DateTime<Utc>, explanation: String, title: String, url: String) -> Self {
        Apod {
            id,
            img_date,
            explanation,
            title,
            url,
        }
    }
}

#[derive(
Clone,
Copy,
Debug,
sqlx::Type,
Display,
derive_more::Deref,
PartialEq,
Eq,
Hash,
Serialize,
Deserialize,
)]
pub struct ApodId(pub i32);

impl From<i32> for ApodId {
    fn from(value: i32) -> Self {
        ApodId(value)
    }
}

impl From<ApodId> for i32 {
    fn from(value: ApodId) -> Self {
        value.0
    }
}

pub trait IntoApodId {
    fn into_apod_id(self) -> ApodId;
}

impl IntoApodId for i32 {
    fn into_apod_id(self) -> ApodId {
        ApodId::from(self)
    }
}

impl IntoApodId for ApodId {
    fn into_apod_id(self) -> ApodId {
        self
    }
}

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateApod {
    pub img_date: DateTime<Utc>,
    pub explanation: String,
    pub title: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct GetApodById {
    pub apod_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateApod {
    pub id: ApodId,
    pub img_date: DateTime<Utc>,
    pub explanation: String,
    pub title: String,
    pub url: String,
}
