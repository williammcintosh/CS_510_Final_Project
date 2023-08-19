use http::{Request, StatusCode};
use hyper::Body;
use sqlx::PgPool;
use tower::ServiceExt;
use crate::models::user::{UserSignup, UserDetails};
use backend::main_routes::app;

#[sqlx::test(fixtures("0001_new_users"))]
async fn test_add_users(db_pool: PgPool) {
    let mut app = app(db_pool).await;

    let new_user = UserSignup {
        email: "fifth@apods.com".into(),
        password: "1qazxsw2".into(),
        confirm_password: "1qazxsw2".into(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/users")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&new_user).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("0002_new_comments"))]
async fn test_add_comments(db_pool: PgPool) {
    let mut app = app(db_pool).await;

    let new_comment = CreateComment {
        content: "Most premium!".into(),
        reference: "{'Apod': 3}".into(),
        user_id: "3".into(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/comment")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&new_comment).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// #[sqlx::test(fixtures("0003_make_user_admin"))]
// async fn test_update_user(db_pool: PgPool) {
//     let app = app(db_pool).await;
//
//     let updated_user = UserDetails {
//         id: 2.into(),
//         email: "second@apods.com".into(),
//         is_admin: true,
//         is_banned: false,
//     };
//
//     let response = app
//         .oneshot(
//             Request::builder()
//                 .method(http::Method::PUT)
//                 .uri("/question")
//                 .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
//                 .body(Body::from(
//                     serde_json::to_string(&updated_user).unwrap(),
//                 ))
//                 .unwrap(),
//         )
//         .await
//         .unwrap();
//
//     assert_eq!(response.status(), StatusCode::OK);
// }
