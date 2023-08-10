use axum::Json;
use serde_json::Value;
use std::sync::{Arc, Mutex, RwLock};

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Row};
use tracing::info;

use crate::error::AppError;
use crate::models::answer::{Answer, AnswerId};
use crate::models::comment::{Comment, CommentId, CommentReference};
use crate::models::page::{AnswerWithComments, PagePackage, QuestionWithComments};
use crate::models::question::{
    GetQuestionById, IntoQuestionId, Question, QuestionId, UpdateQuestion,
};
use crate::models::apod::{
    GetApodById, IntoApodId, Apod, ApodId, UpdateApod,
};
use crate::models::user::{User, UserSignup};

#[derive(Clone)]
pub struct Store {
    pub conn_pool: PgPool,
    pub questions: Arc<Mutex<Vec<Question>>>,
    pub answers: Arc<RwLock<Vec<Answer>>>,
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
            questions: Default::default(),
            answers: Default::default(),
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

    pub async fn add_answer(
        &mut self,
        content: String,
        question_id: i32,
    ) -> Result<Answer, AppError> {
        let res = sqlx::query!(
            r#"
    INSERT INTO answers (content, question_id)
    VALUES ($1, $2)
    RETURNING *
    "#,
            content,
            question_id,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let answer = Answer {
            id: AnswerId(res.id),
            content: res.content,
            question_id: QuestionId(res.question_id.unwrap()),
        };

        Ok(answer)
    }

    pub async fn get_all_questions(&mut self) -> Result<Vec<Question>, AppError> {
        let rows = sqlx::query!(
            r#"
SELECT * FROM questions
"#
        )
            .fetch_all(&self.conn_pool)
            .await?;

        let questions: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Question {
                    id: row.id.into(), // Assuming you have a From<u32> for QuestionId
                    title: row.title,
                    content: row.content,
                    tags: row.tags,
                }
            })
            .collect();

        Ok(questions)
    }

    pub async fn get_question_by_id<T: IntoQuestionId>(
        &mut self,
        id: T,
    ) -> Result<Question, AppError> {
        let id = id.into_question_id();

        let row = sqlx::query!(
            r#"
    SELECT * FROM questions WHERE id = $1
    "#,
            id.0,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let question = Question {
            id: row.id.into(), // Assuming you have a From<u32> for QuestionId
            title: row.title,
            content: row.content,
            tags: row.tags,
        };

        Ok(question)
    }

    pub async fn add_question(
        &mut self,
        title: String,
        content: String,
        tags: Option<Vec<String>>,
    ) -> Result<Question, AppError> {
        let res = sqlx::query!(
            r#"INSERT INTO "questions"(title, content, tags)
           VALUES ($1, $2, $3)
           RETURNING *
        "#,
            title,
            content,
            tags.as_deref()
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let new_question = Question {
            id: QuestionId(res.id),
            title: res.title,
            content: res.content,
            tags: res.tags,
        };

        Ok(new_question)
    }

    pub async fn update_question(
        &mut self,
        new_question: UpdateQuestion,
    ) -> Result<Question, AppError> {
        sqlx::query!(
            r#"
    UPDATE questions
    SET title = $1, content = $2, tags = $3
    WHERE id = $4
    "#,
            new_question.title,
            new_question.content,
            new_question.tags.as_deref(),
            new_question.id.0,
        )
            .execute(&self.conn_pool)
            .await?;

        let row = sqlx::query!(
            r#"
SELECT title, content, id, tags FROM questions WHERE id = $1
"#,
            new_question.id.0,
        )
            .fetch_one(&self.conn_pool)
            .await?;

        let question = Question {
            title: row.title,
            content: row.content,
            id: QuestionId(row.id),
            tags: row.tags,
        };

        Ok(question)
    }

    pub async fn delete_question(&mut self, question_id: i32) -> Result<(), AppError> {
        let question_id = question_id.into_question_id();
        println!("DELETE - Question id is {}", &question_id);
        sqlx::query!(
            r#"
    DELETE FROM questions WHERE id = $1
    "#,
            question_id.0,
        )
            .execute(&self.conn_pool)
            .await
            .unwrap();

        Ok(())
    }

    pub async fn get_user(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
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
        let (question_id, answer_id) = match &comment.reference {
            CommentReference::Question(qid) => (Some(qid.0), None),
            CommentReference::Answer(aid) => (None, Some(aid.0)),
        };

        let res = sqlx::query(
            r#"
            INSERT INTO comments (content, question_id, answer_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
        )
            .bind(comment.content)
            .bind(question_id)
            .bind(answer_id)
            .fetch_one(&self.conn_pool)
            .await?;

        let comment = Comment {
            id: Some(CommentId(res.get("id"))),
            content: res.get("content"),
            reference: comment.reference,
        };

        Ok(comment)
    }

    pub async fn get_all_question_pages(&self) -> Result<Vec<PagePackage>, AppError> {
        let questions = sqlx::query("SELECT id from questions")
            .fetch_all(&self.conn_pool)
            .await?;

        let mut res = Vec::new();

        for row in questions {
            let id = GetQuestionById {
                question_id: row.get("id"),
            };

            let page = self.get_page_for_question(id).await?;
            res.push(page)
        }

        Ok(res)
    }

    pub async fn get_page_for_question(
        &self,
        question: GetQuestionById,
    ) -> Result<PagePackage, AppError> {
        let question_row = sqlx::query("SELECT * FROM questions WHERE id = $1")
            .bind(question.question_id)
            .fetch_one(&self.conn_pool)
            .await?;

        let answer_rows = sqlx::query("SELECT * FROM answers WHERE question_id = $1")
            .bind(question.question_id)
            .fetch_all(&self.conn_pool)
            .await?;

        let comments_rows = sqlx::query("SELECT * FROM comments WHERE question_id = $1 OR answer_id IN (SELECT id FROM answers WHERE question_id = $1)")
            .bind(question.question_id)
            .fetch_all(&self.conn_pool)
            .await?;

        let question = Question {
            id: QuestionId(question_row.get("id")),
            title: question_row.get("title"),
            content: question_row.get("content"),
            tags: question_row.get("tags"),
        };

        // TODO: Remove the below code duplication by abstracting into fn
        let mut answers_with_comments = Vec::new();

        for row in answer_rows {
            let answer = Answer {
                id: AnswerId(row.get("id")),
                content: row.get("content"),
                question_id: QuestionId(row.get("question_id")),
            };

            let comments_for_answer: Vec<Comment> = comments_rows
                .iter()
                .filter_map(|row| {
                    if let Ok(answer_id) = row.try_get::<i32, _>("answer_id") {
                        if answer_id == answer.id.0 {
                            Some(Comment {
                                id: Some(CommentId(row.get("id"))),
                                content: row.get("content"),
                                reference: CommentReference::Answer(AnswerId(answer_id)),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            answers_with_comments.push(AnswerWithComments {
                answer,
                comments: comments_for_answer,
            });
        }

        let comments_for_question: Vec<Comment> = comments_rows
            .iter()
            .filter_map(|row| {
                if let Ok(question_id) = row.try_get::<i32, _>("question_id") {
                    if question_id == question.id.0 {
                        Some(Comment {
                            id: Some(CommentId(row.get("id"))),
                            content: row.get("content"),
                            reference: CommentReference::Question(QuestionId(question_id)),
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        let question_with_comments = QuestionWithComments {
            question,
            comments: comments_for_question,
        };

        let package = PagePackage {
            question: question_with_comments,
            answers: answers_with_comments,
        };

        Ok(package)
    }

    pub async fn get_all_apods(&mut self) -> Result<Vec<Apod>, AppError> {
        let rows = sqlx::query!(r#"SELECT * FROM apods"#)
            .fetch_all(&self.conn_pool)
            .await?;

        let apods: Vec<_> = rows
            .into_iter()
            .map(|row| {
                Apod {
                    id: row.id.into(), // Assuming you have a From<u32> for ApodId
                    img_date: row.img_date,
                    explanation: row.explanation,
                    title: row.title,
                    url: row.url,
                }
            })
            .collect();

        Ok(apods)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
