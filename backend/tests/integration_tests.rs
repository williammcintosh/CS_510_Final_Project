use http::{Request, StatusCode};
use hyper::Body;
use sqlx::PgPool;
use tower::ServiceExt;


use backend::main_routes::app;
use backend::apod::{CreateApod, Apod};

#[sqlx::test(fixtures("0001_apods"))]
async fn test_add_apod(db_pool: PgPool) {
    let mut app = app(db_pool).await;

    let apod = CreateApod {
        title: "New Title".into(),
        content: "Test content2".into(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/apod")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&apod).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("0001_apods"))]
async fn test_get_apods(db_pool: PgPool) {
    let app = app(db_pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/apods")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let apods: Vec<Apod> = serde_json::from_slice(&body).unwrap();
    assert!(!apods.is_empty());
}

#[sqlx::test(fixtures("0001_apods"))]
async fn test_get_apod_by_id(db_pool: PgPool) {
    let app = app(db_pool).await;

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/apod/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let apod: Apod = serde_json::from_slice(&body).unwrap();
    assert_eq!(apod.id.0, 1);
}

#[sqlx::test(fixtures("0001_apods"))]
async fn test_update_apod(db_pool: PgPool) {
    let mut app = app(db_pool).await;

    let updated_apod = Apod {
        id: 1.into(),
        title: "Updated Title".into(),
        content: "Updated content".into(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PUT)
                .uri("/apod")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(
                    serde_json::to_string(&updated_apod).unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test(fixtures("0001_apods"))]
async fn test_delete_apod(db_pool: PgPool) {
    println!("In test delete");
    let app = app(db_pool).await;

    let query_uri = format!("/apod?apod_id=1");

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::DELETE)
                .uri(query_uri)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    dbg!("DELETED APOD RESPONSE");
    dbg!(&response);
    assert_eq!(response.status(), StatusCode::OK);
}
