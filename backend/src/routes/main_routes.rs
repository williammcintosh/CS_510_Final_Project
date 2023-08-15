use axum::response::Response;
use axum::routing::*;
use axum::Router;
use http::StatusCode;
use hyper::Body;
use sqlx::PgPool;

use crate::db::Store;
use crate::handlers::root;
// use crate::handlers::profile;
use crate::{file_handler, handlers, layers};

pub async fn app(pool: PgPool) -> Router {
    let db = Store::with_pool(pool);

    let (cors_layer, trace_layer) = layers::get_layers();

    let static_router = Router::new()
        .route("/:filename", get(file_handler))
        .with_state(db.clone());

    Router::new()
        // The router matches these FROM TOP TO BOTTOM explicitly!
        .nest("/static", static_router)
        .route("/", get(root))
        .route("/profile", get(handlers::profile))


        .route("/apods", get(handlers::get_apods))
        .route("/apod/:apod_id", get(handlers::get_apod_by_id))
        .route("/apod", post(handlers::create_apod))
        .route("/apod", put(handlers::update_apod))
        .route("/apod", delete(handlers::delete_apod))

        .route("/comment", post(handlers::post_comment))
        .route("/favorite", post(handlers::post_favorite))
        // .route("/apods", get(handlers::get_all_apods))
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
