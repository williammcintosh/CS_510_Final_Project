use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
use crate::{handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    Router::new()
        // The router matches these FROM TOP TO BOTTOM explicitly!
        .route("/", get(root))
        .route("/questions", get(handlers::get_questions))
        .route("/question/:question_id", get(handlers::get_question_by_id))
        .route("/question", post(handlers::create_question))
        .route("/question", put(handlers::update_question))
        .route("/question", delete(handlers::delete_question))
        .route("/answer", post(handlers::create_answer))
        .route("/users", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/protected", get(handlers::protected))
        .route("/*_", get(handle_404))
        .layer(cors_layer)
        .layer(trace_layer)
        .with_state(db)
}

async fn handle_404() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("The requested page could not be found"))
        .unwrap()
}
