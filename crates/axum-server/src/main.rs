// Please use latest axum version
mod config;
mod errors;

use crate::errors::CustomError;
use std::net::SocketAddr;
use axum::extract::{Extension, Path};
use axum::{response::Html, response::IntoResponse, routing::get, Router};
use deadpool_postgres::Pool;
use axum::body::{self, Body, Empty};
use axum::http::{header, HeaderValue, Response, StatusCode};
use assets::templates::statics::StaticFile;
// use axum::{response::Html, extract::Extension, routing::get, Router}; // added response::HTML here and removed response::Json,
// use db::User; , as we already have access to db/ folder using cargo new --path db....

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    let pool = db::create_pool(&config.database_url);

    // build our application with a route
    let app = Router::new()
        .route("/", get(users))
        .route("/static/*path", get(static_path))
        .layer(Extension(config))
        .layer(Extension(pool.clone()));

    // run it
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// here db::Pool, Pool inside deadpool_postgres in db/src/lib.rs
// changing from Json<Vec<User> to HTML<String>
async fn users(Extension(pool): Extension<db::Pool>) -> Result<Html<String>, CustomError> {
    let client = pool.get().await?;

    let users = db::queries::users::get_users()
        .bind(&client)
        .all()
        .await?;

// users = id, email from users
// Ok(Json(users))
    Ok(Html(ui_components::users::users(
        users,
    )))
}

async fn static_path(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');

    if let Some(data) = StaticFile::get(path) {
        Response::builder()
            .status(StatusCode::OK)
            .header(
                header::CONTENT_TYPE,
                HeaderValue::from_str(data.mime.as_ref()).unwrap(),
            )
            .body(body::boxed(Body::from(data.content)))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(body::boxed(Empty::new()))
            .unwrap()
    }
}
