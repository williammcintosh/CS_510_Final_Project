// use crate::make_db_id;
use derive_more::Display;
use serde_derive::{Deserialize, Serialize};
use crate::models::apod::ApodId;
use crate::models::user::{UserId};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Favorite {
    pub id: Option<FavoriteId>,
    pub apod_id: Option<ApodId>,
    pub user_id: Option<UserId>,
}

impl Favorite {
    #[allow(dead_code)]
    pub fn new(id: Option<FavoriteId>, apod_id: Option<ApodId>, user_id: Option<UserId>) -> Self {
        Favorite {
            id,
            apod_id,
            user_id,
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
pub struct FavoriteId(pub i32);

impl From<i32> for FavoriteId {
    fn from(value: i32) -> Self {
        FavoriteId(value)
    }
}

impl From<FavoriteId> for i32 {
    fn from(value: FavoriteId) -> Self {
        value.0
    }
}

pub trait IntoFavoriteId {
    fn into_favorite_id(self) -> FavoriteId;
}

impl IntoFavoriteId for i32 {
    fn into_favorite_id(self) -> FavoriteId {
        FavoriteId::from(self)
    }
}

impl IntoFavoriteId for FavoriteId {
    fn into_favorite_id(self) -> FavoriteId {
        self
    }
}

// Clients use this to create new requests
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateFavorite {
    pub apod_id: Option<ApodId>,
    pub user_id: Option<UserId>,
}

#[derive(Deserialize)]
pub struct GetFavoriteById {
    pub favorite_id: i32,
}
