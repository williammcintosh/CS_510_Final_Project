use axum::Json;
use serde_json::{Value};
use std::sync::{
    Arc,
    Mutex,
    // RwLock
};
use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::info;
use crate::error::AppError;
use crate::models::comment::{
    Comment,
    CommentId,
    // IntoCommentId,
    CommentReference
};
use crate::models::page::{PagePackage, ApodWithComments};
use crate::models::apod::{
    GetApodById, IntoApodId, Apod, ApodId, UpdateApod,
};
use crate::models::favorite::{
    // GetFavoriteById,
    // IntoFavoriteId,
    Favorite,
    FavoriteId,
};
use crate::models::user::{UserLogin, UserDetails, UserId, UserSignup};

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
    pub apods: Arc<Mutex<Vec<Apod>>>,
}

pub async fn new_pool() -> PgPool {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .unwrap()
}

impl Store {
    pub fn with_pool(pool: PgPool) -> Self {
        Self {
            conn_pool: pool,
            apods: Default::default(),
        }
    }

    pub async fn test_database(&self) -> Result<(), sqlx::Error> {
        let row: (i64,) = sqlx::query_as("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.conn_pool)
            .await?;

        info!("{}", &row.0);

        assert_eq!(row.0, 150);
        Ok(())
    }

    pub async fn get_all_apods(&mut self) -> Result<Vec<Apod>, AppError> {
        let rows = sqlx::query!(
            r#"
        SELECT * FROM apods
        ORDER BY id ASC
    "#
        )
            .fetch_all(&self.conn_pool)
            .await?;

        let mut apods: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Apod {
                    id: row.id.into(), // Assuming you have a From<u32> for ApodId
                    title: row.title,
                    img_date: row.img_date,
                    content: row.content,
                    url: row.url,
                }
            })
            .collect();

        apods.reverse();

        Ok(apods)
    }

    pub async fn get_apod_by_id<T: IntoApodId>(
        &mut self,
        id: T,
    ) -> Result<Apod, AppError> {
        let id = id.into_apod_id();

        let row = sqlx::query!(
            r#"
    SELECT * FROM apods WHERE id = $1
    "#,
            id.0,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let apod = Apod {
            id: row.id.into(), // Assuming you have a From<u32> for ApodId
            title: row.title,
            img_date: row.img_date,
            content: row.content,
            url: row.url,
        };

        Ok(apod)
    }

    pub async fn add_apod(
        &mut self,
        title: String,
        img_date: String,
        content: String,
        url: String,
    ) -> Result<Apod, AppError> {
        let res = sqlx::query!(
            r#"INSERT INTO "apods"(title, img_date, content, url)
           VALUES ($1, $2, $3, $4)
           RETURNING *
        "#,
            title,
            img_date,
            content,
            url,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let new_apod = Apod {
            id: ApodId(res.id),
            title: res.title,
            img_date: res.img_date,
            content: res.content,
            url: res.url,
        };

        Ok(new_apod)
    }

    pub async fn add_favorite(
        &mut self,
        apod_id: Option<ApodId>,
        user_id: Option<UserId>,
    ) -> Result<Favorite, AppError> {

        let a_id = i32::from(apod_id.unwrap_or(ApodId(0)));
        let u_id = i32::from(user_id.unwrap_or(UserId(0)));

        //On constraint violation, updates existing entry instead of creating a new one.
        let res = sqlx::query(
                r#"
                INSERT INTO "favorites"(apod_id, user_id)
                VALUES ($1, $2)
                ON CONFLICT (apod_id, user_id) DO UPDATE SET apod_id = $1, user_id = $2
                RETURNING *
            "#
        )
        .bind(a_id)
        .bind(u_id)
        .fetch_one(&self.conn_pool)
        .await?;

        let new_favorite = Favorite {
            id: Some(FavoriteId(res.get("id"))),
            apod_id: Some(ApodId(res.get("apod_id"))),
            user_id: Some(UserId(res.get("user_id"))),
        };

        Ok(new_favorite)
    }

    pub async fn remove_favorite(
        &mut self,
        apod_id: Option<ApodId>,
        user_id: Option<UserId>,
    ) -> Result<Favorite, AppError> {

        let a_id = i32::from(apod_id.unwrap_or(ApodId(0)));
        let u_id = i32::from(user_id.unwrap_or(UserId(0)));

        let res = sqlx::query(
            r#"
            DELETE FROM favorites
            WHERE apod_id = $1 AND user_id = $2
            RETURNING *
        "#
        )
            .bind(a_id)
            .bind(u_id)
            .fetch_one(&self.conn_pool)
            .await?;

        let removed_favorite = Favorite {
            id: Some(FavoriteId(res.get("id"))),
            apod_id: Some(ApodId(res.get("apod_id"))),
            user_id: Some(UserId(res.get("user_id"))),
        };

        Ok(removed_favorite)
    }

    pub async fn update_apod(
        &mut self,
        new_apod: UpdateApod,
    ) -> Result<Apod, AppError> {
        sqlx::query!(
            r#"
    UPDATE apods
    SET title = $1, img_date = $2, content = $3, url = $4
    WHERE id = $5
    "#,
            new_apod.title,
            new_apod.img_date,
            new_apod.content,
            new_apod.url,
            new_apod.id.0,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
            r#"
SELECT title, img_date, content, url, id FROM apods WHERE id = $1
"#,
            new_apod.id.0,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let apod = Apod {
            title: row.title,
            img_date: row.img_date,
            content: row.content,
            url: row.url,
            id: ApodId(row.id),
        };

        Ok(apod)
    }

    pub async fn delete_apod(&mut self, apod_id: i32) -> Result<(), AppError> {
        let apod_id = apod_id.into_apod_id();
        println!("DELETE - Apod id is {}", &apod_id);
        sqlx::query!(
            r#"
    DELETE FROM apods WHERE id = $1
    "#,
            apod_id.0,
        )
            .execute(&self.conn_pool)
            .await
            .unwrap();

        Ok(())
    }

    pub async fn get_user_details(&self, email: &str) -> Result<UserDetails, AppError> {
        let user = sqlx::query_as::<_, UserDetails>(
            r#"
                SELECT id, email, is_admin, is_banned FROM users WHERE email = $1
            "#,
        )
            .bind(email)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(user)
    }

    pub async fn get_user_login(&self, email: &str) -> Result<UserLogin, AppError> {
        let user = sqlx::query_as::<_, UserLogin>(
            r#"
                SELECT email, password FROM users WHERE email = $1
            "#,
        )
            .bind(email)
            .fetch_one(&self.conn_pool)
            .await?;

        Ok(user)
    }

    pub async fn create_user(&self, user: UserSignup) -> Result<Json<Value>, AppError> {
        // TODO: Encrypt/bcrypt user passwords
        let result = sqlx::query("INSERT INTO users(email, password) values ($1, $2)")
            .bind(&user.email)
            .bind(&user.password)
            .execute(&self.conn_pool)
            .await
            .map_err(|_| AppError::InternalServerError)?;

        if result.rows_affected() < 1 {
            Err(AppError::InternalServerError)
        } else {
            Ok(Json(
                serde_json::json!({"message": "User created successfully!"}),
            ))
        }
    }

    pub async fn create_comment(&self, comment: Comment) -> Result<Comment, AppError> {
        let apod_id = match &comment.reference {
            CommentReference::Apod(qid) => Some(qid.0)
        }.unwrap_or_default();

        let user_id = i32::from(comment.user_id.unwrap_or(UserId(0)));

        let res = sqlx::query(
            r#"
            INSERT INTO comments (content, apod_id, user_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
        .bind(comment.content)
        .bind(apod_id)
        .bind(user_id)
        .fetch_one(&self.conn_pool)
        .await?;

        let comment = Comment {
            id: Some(CommentId(res.get("id"))),
            content: res.get("content"),
            reference: comment.reference,
            user_id: Some(UserId(res.get("user_id"))),
        };

        Ok(comment)
    }

    pub async fn get_all_apod_pages(&self) -> Result<Vec<PagePackage>, AppError> {
        let apods = sqlx::query(r#"
        SELECT * FROM apods
        ORDER BY img_date DESC
        "#)
            .fetch_all(&self.conn_pool)
            .await?;

        let mut res = Vec::new();

        for row in apods {
            let id = GetApodById {
                apod_id: row.get("id"),
            };

            let page = self.get_page_for_apod(id).await?;
            res.push(page)
        }

        Ok(res)
    }

    pub async fn get_all_users(&self) -> Result<Vec<UserDetails>, AppError> {
        let rows = sqlx::query("SELECT * FROM users")
            .fetch_all(&self.conn_pool)
            .await?;

        let all_users: Vec<_> = rows
            .into_iter()
            .map(|row| {
                UserDetails {
                    id: row.get("id"),
                    email: row.get("email"),
                    is_admin: row.get("is_admin"),
                    is_banned: row.get("is_banned"),
                }
            })
            .collect();

        Ok(all_users)
    }

    pub async fn get_page_for_apod(
        &self,
        apod: GetApodById,
    ) -> Result<PagePackage, AppError> {
        let apod_row = sqlx::query("SELECT * FROM apods WHERE id = $1")
            .bind(apod.apod_id)
            .fetch_one(&self.conn_pool)
            .await?;

        let comments_rows = sqlx::query("SELECT * FROM comments WHERE apod_id = $1")
            .bind(apod.apod_id)
            .fetch_all(&self.conn_pool)
            .await?;

        let apod = Apod {
            id: ApodId(apod_row.get("id")),
            title: apod_row.get("title"),
            img_date: apod_row.get("img_date"),
            content: apod_row.get("content"),
            url: apod_row.get("url"),
        };

        let comments_for_apod: Vec<Comment> = comments_rows
            .iter()
            .filter_map(|row| {
                if let Ok(apod_id) = row.try_get::<i32, _>("apod_id") {
                    if apod_id == apod.id.0 {
                        Some(Comment {
                            id: Some(CommentId(row.get("id"))),
                            content: row.get("content"),
                            reference: CommentReference::Apod(ApodId(apod_id)),
                            user_id: Some(UserId(row.get("user_id"))),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let apod_with_comments = ApodWithComments {
            apod,
            comments: comments_for_apod,
        };

        let package = PagePackage {
            apod: apod_with_comments,
        };

        Ok(package)
    }

    pub async fn get_favorites_by_user_id(
        &mut self,
        user_id: UserId,
    ) -> Result<Vec<Apod>, AppError> {
        let rows = sqlx::query!(
        r#"
        SELECT apods.* FROM apods
        INNER JOIN favorites ON apods.id = favorites.apod_id
        WHERE favorites.user_id = $1
        "#,
        user_id.0,
    )
            .fetch_all(&self.conn_pool)
            .await?;

        let apods: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Apod {
                    id: row.id.into(),
                    title: row.title,
                    img_date: row.img_date,
                    content: row.content,
                    url: row.url,
                }
            })
            .collect();

        Ok(apods)
    }

    pub async fn perform_user_ban(
        &mut self,
        user_id: i32,
    ) -> Result<UserDetails, AppError> {
        sqlx::query!(
            r#"
                UPDATE users
                SET is_banned = true
                WHERE id = $1
            "#,
            user_id,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            user_id,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let banned_user = UserDetails {
            id: row.id,
            email: row.email,
            is_admin: row.is_admin.is_some(),
            is_banned: row.is_banned.is_some(),
        };

        Ok(banned_user)
    }

    pub async fn perform_user_un_ban(
        &mut self,
        user_id: i32,
    ) -> Result<UserDetails, AppError> {
        sqlx::query!(
            r#"
                UPDATE users
                SET is_banned = false
                WHERE id = $1
            "#,
            user_id,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
            r#"
                SELECT * FROM users WHERE id = $1
            "#,
            user_id,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let banned_user = UserDetails {
            id: row.id,
            email: row.email,
            is_admin: row.is_admin.is_some(),
            is_banned: row.is_banned.is_some(),
        };

        Ok(banned_user)
    }



    // pub async fn seed_apod_table_with_nasa(
    //     &mut self,
    //     body_json: String
    // ) -> Result<Vec<Apod>, AppError> {
    //
    //     let rows = sqlx::query(
    //         r#"
    //             INSERT INTO "apods"(img_date, content, title, url)
    //             SELECT to_timestamp(apod->>'date', 'YYYY-MM-DD'), apod->>'explanation', apod->>'title', apod->>'url'
    //             FROM json_array_elements(' $1 ') AS apod
    //         "#,
    //     )
    //         .bind(body_json)
    //         .fetch_all(&self.conn_pool)
    //         .await?;
    //
    //     let apods: Vec<_> = rows
    //         .into_iter()
    //         .map(|row| {
    //             Apod {
    //                 id: row.id.into(), // Assuming you have a From<u32> for ApodId
    //                 title: row.title,
    //                 img_date: row.img_date,
    //                 content: row.content,
    //                 url: row.url,
    //             }
    //         })
    //         .collect();
    //
    //     Ok(apods)
    // }


    // pub async fn get_all_apods(&mut self) -> Result<Vec<Apod>, AppError> {
    //     let rows = sqlx::query!(r#"SELECT * FROM apods"#)
    //         .fetch_all(&self.conn_pool)
    //         .await?;
    //
    //     let apods: Vec<_> = rows
    //         .into_iter()
    //         .map(|row| {
    //             Apod {
    //                 id: row.id.into(), // Assuming you have a From<u32> for ApodId
    //                 img_date: row.img_date,
    //                 explanation: row.explanation,
    //                 title: row.title,
    //                 url: row.url,
    //             }
    //         })
    //         .collect();
    //
    //     Ok(apods)
    // }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
