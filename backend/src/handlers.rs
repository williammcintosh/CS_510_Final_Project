use argon2::Config;
use axum::extract::{Path, Query, State};
use axum::response::{Html, Response};
use axum::{Form, Json};
use http::header::{LOCATION, SET_COOKIE};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use jsonwebtoken::Header;
use serde_json::{
    // json,
    Value
};
use tera::Context;
use tracing::error;

use crate::db::Store;
use crate::error::AppError;
use crate::get_timestamp_after_8_hours;
use crate::models::apod::{
    CreateApod, GetApodById, Apod, ApodId, UpdateApod,
};
use crate::models::user::{Claims, OptionalClaims, UserLogin, UserId, UserSignup, KEYS};
use crate::models::comment::{
    Comment,
    // CommentReference
};
use crate::models::favorite::{
    CreateFavorite,
    // GetFavoriteById,
    Favorite,
    // FavoriteId,
};
use crate::template::TEMPLATES;
use std::collections::HashMap;


#[allow(dead_code)]
pub async fn root(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Casey");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        // Get the favorite APODs for the logged-in user
        let favorites = am_database.get_favorites_by_user_id(UserId(claims_data.id)).await?;
        let all_apods = am_database.get_all_apod_pages().await?;
        context.insert("all_apods", &all_apods);

        let mut apod_map: HashMap<i32, bool> = HashMap::new();

        // Iterate through each apod and check if it is favorited
        for apod in all_apods.iter() {
            let is_favorited = favorites.iter().any(|favorite| favorite.id == apod.apod.apod.id);
            apod_map.insert(apod.apod.apod.id.0, is_favorited);
        }

        // Insert the hashmap into the context
        context.insert("apod_map", &apod_map);
        context.insert("all_apods", &all_apods);

        "pages.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

// CRUD create - read - update - delete
pub async fn get_apods(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Apod>>, AppError> {
    let all_apods = am_database.get_all_apods().await?;

    Ok(Json(all_apods))
}

pub async fn get_apod_by_id(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>, // localhost:3000/apod/5
) -> Result<Json<Apod>, AppError> {
    let apod = am_database.get_apod_by_id(ApodId(query)).await?;
    Ok(Json(apod))
}

pub async fn create_apod(
    State(mut am_database): State<Store>,
    Json(apod): Json<CreateApod>,
) -> Result<Json<Apod>, AppError> {
    let apod = am_database
        .add_apod(apod.title, apod.img_date, apod.content, apod.url)
        .await?;

    Ok(Json(apod))
}

// pub async fn pass_nasa_info_to_db(
//     State(mut am_database): axum::extract::State<crate::db::Store>,
//     json_body: Value,
// ) -> Result<Json<Vec<Apod>>, AppError> {
//     let apods = am_database
//         .seed_apod_table_with_nasa(json_body)
//         .await?;
//     Ok(Json(apods))
// }

pub async fn post_comment(
    State(am_database): State<Store>,
    Json(comment): Json<Comment>,
) -> Result<Json<Comment>, AppError> {
    // let apod_id = match &comment.reference {
    //     CommentReference::Apod(qid) => Some(qid.0),
    // }.unwrap_or_default();

    let new_comment = am_database.create_comment(comment).await?;
    Ok(Json(new_comment))
}

pub async fn post_favorite(
    State(mut am_database): State<Store>,
    Json(favorite): Json<CreateFavorite>,
) -> Result<Json<Favorite>, AppError> {
    let favorite = am_database
        .add_favorite(favorite.apod_id, favorite.user_id)
        .await?;

    Ok(Json(favorite))
}

pub async fn set_favorite_url(
    State(mut am_database): State<Store>,
    Path(apod_id): Path<i32>, // localhost:3000/favorite/2
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Casey");

    let template_name = if let Some(claims_data) = claims {
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        // Perform the favoriting, update db
        let a_id = Some(ApodId(apod_id));
        let u_id = Some(UserId(claims_data.id));
        let _ = am_database.add_favorite(a_id, u_id).await?;
        let fav_apod = am_database.get_apod_by_id(ApodId(apod_id)).await?;

        error!("Marked as favorite: {}", &apod_id);

        //Get apod title and set it to the context
        context.insert("new_fav_title", &fav_apod.title);
        context.insert("new_fav_url", &fav_apod.url);

        "favorite_set.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}


pub async fn update_apod(
    State(mut am_database): State<Store>,
    Json(apod): Json<UpdateApod>,
) -> Result<Json<Apod>, AppError> {
    let updated_apod = am_database.update_apod(apod).await?;
    Ok(Json(updated_apod))
}

pub async fn delete_apod(
    State(mut am_database): State<Store>,
    Query(query): Query<GetApodById>,
) -> Result<(), AppError> {
    am_database.delete_apod(query.apod_id).await?;

    Ok(())
}

pub async fn register(
    State(database): State<Store>,
    Json(mut credentials): Json<UserSignup>,
) -> Result<Json<Value>, AppError> {
    // We should also check to validate other things at some point like email address being in right format

    if credentials.email.is_empty() || credentials.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    if credentials.password != credentials.confirm_password {
        return Err(AppError::MissingCredentials);
    }

    // Check to see if there is already a user in the database with the given email address
    let existing_user = database.get_user_login(&credentials.email).await;

    if let Ok(_) = existing_user {
        return Err(AppError::UserAlreadyExists);
    }

    // Here we're assured that our credentials are valid and the user doesn't already exist
    // hash their password
    let hash_config = Config::default();
    let salt = std::env::var("SALT").expect("Missing SALT");
    let hashed_password = match argon2::hash_encoded(
        credentials.password.as_bytes(),
        // If you'd like unique salts per-user, simply pass &[] and argon will generate them for you
        salt.as_bytes(),
        &hash_config,
    ) {
        Ok(result) => result,
        Err(_) => {
            return Err(AppError::Any(anyhow::anyhow!("Password hashing failed")));
        }
    };

    credentials.password = hashed_password;

    let new_user = database.create_user(credentials).await?;
    Ok(new_user)
}

pub async fn login(
    State(database): State<Store>,
    Form(creds): Form<UserLogin>,
) -> Result<Response<Body>, AppError> {
    if creds.email.is_empty() || creds.password.is_empty() {
        return Err(AppError::MissingCredentials);
    }

    let existing_user = database.get_user_login(&creds.email).await?;

    let is_password_correct =
        match argon2::verify_encoded(&*existing_user.password, creds.password.as_bytes()) {
            Ok(result) => result,
            Err(_) => {
                return Err(AppError::InternalServerError);
            }
        };

    if !is_password_correct {
        return Err(AppError::InvalidPassword);
    }

    println!("User is authorized");
    // at this point we've authenticated the user's identity
    // create JWT to return
    let user_details = database.get_user_details(&creds.email).await?;

    let claims = Claims {
        id: user_details.id,
        email: creds.email.to_owned(),
        exp: get_timestamp_after_8_hours(),
        is_admin: user_details.is_admin,
    };

    let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AppError::MissingCredentials)?;

    let cookie = cookie::Cookie::build("jwt", token).http_only(true).finish();

    let mut response = Response::builder()
        .status(StatusCode::FOUND)
        .body(Body::empty())
        .unwrap();

    response
        .headers_mut()
        .insert(LOCATION, HeaderValue::from_static("/"));
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie.to_string()).unwrap(),
    );

    Ok(response)
}

pub async fn protected(claims: Claims) -> Result<String, AppError> {
    Ok(format!(
        "Welcome to the PROTECTED area :) \n Your claim data is: {}",
        claims
    ))
}

pub async fn get_all_apods(
    State(mut am_database): State<Store>,
) -> Result<Json<Vec<Apod>>, AppError> {
    let all_apods = am_database.get_all_apods().await?;

    Ok(Json(all_apods))
}

pub async fn profile(
    State(mut am_database): State<Store>,
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Casey");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        // Check if the logged-in user is an admin
        if claims_data.is_admin {
            context.insert("is_admin", &true);
            error!("Setting is_admin is TRUE now");

            // Get the favorite APODs for the logged-in user
            let all_users = am_database.get_all_users().await?;
            context.insert("all_users", &all_users);
        } else {
            context.insert("is_admin", &false);
            error!("is_admin is FALSE");
        }

        // Get the favorite APODs for the logged-in user
        let favorites = am_database.get_favorites_by_user_id(UserId(claims_data.id)).await?;
        context.insert("favorites", &favorites);

        "profile.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

//
pub async fn ban_user(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>, // localhost:3000/ban_user/2
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Casey");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        // Check if the logged-in user is an admin
        if claims_data.is_admin {
            context.insert("is_admin", &true);

            // Perform the banning, update db
            let banned_user = am_database.perform_user_ban(query).await?;
            context.insert("banned_user_id", &banned_user.id);
            context.insert("user_is_banned", &true);
            error!("User Banned is now: {}", &true)
        } else {
            context.insert("is_admin", &true);
        }

        "ban_user.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}

pub async fn un_ban_user(
    State(mut am_database): State<Store>,
    Path(query): Path<i32>, // localhost:3000/ban_user/2
    OptionalClaims(claims): OptionalClaims,
) -> Result<Html<String>, AppError> {
    let mut context = Context::new();
    context.insert("name", "Casey");

    let template_name = if let Some(claims_data) = claims {
        error!("Setting claims and is_logged_in is TRUE now");
        context.insert("claims", &claims_data);
        context.insert("is_logged_in", &true);

        // Check if the logged-in user is an admin
        if claims_data.is_admin {
            context.insert("is_admin", &true);

            // Perform the banning, update db
            let banned_user = am_database.perform_user_un_ban(query).await?;
            context.insert("banned_user_id", &banned_user.id);
            context.insert("user_is_banned", &false);
            error!("User Banned is now: {}", &false)
        } else {
            context.insert("is_admin", &true);
        }

        "ban_user.html" // Use the new template when logged in
    } else {
        // Handle the case where the user isn't logged in
        error!("is_logged_in is FALSE now");
        context.insert("is_logged_in", &false);
        "index.html" // Use the original template when not logged in
    };

    let rendered = TEMPLATES
        .render(template_name, &context)
        .unwrap_or_else(|err| {
            error!("Template rendering error: {}", err);
            panic!()
        });
    Ok(Html(rendered))
}
