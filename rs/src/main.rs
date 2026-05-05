pub mod db;
pub mod live;
pub mod livecrud;
pub mod upload;

use axum::{
    extract::{DefaultBodyLimit, Path},
    response::Response,
    routing::{delete, get, post},
    Extension, Json, Router,
};
use db::{db_result2json, list, one, remove, save};
use serde::{Deserialize, Serialize};
use tower_http::services::{ServeDir, ServeFile};

use crate::db::Database;

// the output to our `create_user` handler
#[derive(Serialize, Deserialize)]
pub struct User {
    id: Option<i64>,
    username: String,
}

// run with `RUST_BACKTRACE=1 RUST_LOG=debug cargo watch -x run`

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let api = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", get(list_users))
        .route("/users", post(create_user))
        .route("/users/:id", delete(delete_user))
        .route("/upload", post(upload::root))
        .layer(DefaultBodyLimit::disable())
        // provides db to all handlers
        .layer(Database::new("test.db"));
    // live view server (experimental)
    let live = Router::new()
        .route("/", get(live::root))
        .route("/crud", get(livecrud::root))
        .layer(Database::new("test.db"))
        .route("/live-view.js", axum_live_view::precompiled_js());
    // directory from where to serve static files = front-end compiled output
    let file_dir = "fe/build";
    // what to serve when nothing mathes? "index.html" of course!
    let spa_service = ServeFile::new(format!("{}/index.html", file_dir));
    // static file server
    let file_service = ServeDir::new(file_dir).fallback(spa_service);
    let app = Router::new()
        .nest("/api", api)
        .nest("/live", live)
        .fallback_service(file_service);

    let addr = "0.0.0.0:3000".parse().unwrap();
    tracing::debug!("listening on ... {}", addr);
    // Start the server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn list_users(Extension(database): Extension<Database<'_>>) -> Response {
    db_result2json(list(database.connection()))
}

async fn get_user(
    Extension(database): Extension<Database<'_>>,
    Path(user_id): Path<i64>,
) -> Response {
    // UserResult(one(database.connection(), user_id))
    db_result2json(one(database.connection(), user_id))
}

async fn create_user(
    Extension(database): Extension<Database<'_>>,
    Json(user): Json<User>,
) -> Response {
    db_result2json(save(database.connection(), &user))
}

async fn delete_user(
    Extension(database): Extension<Database<'_>>,
    Path(user_id): Path<i64>,
) -> Response {
    // UserResult(one(database.connection(), user_id))
    db_result2json(remove(database.connection(), user_id))
}
