use std::sync::Arc;
use axum::{
    routing::get,
    routing::post,
    Router,
    http::StatusCode,
    response::{IntoResponse, Html},
};
use axum::routing::{delete, put};
use axum_swagger_ui::swagger_ui;
use tokio_postgres::{Client, NoTls};
use crate::actions::create::{delete_one, insert_one, select, update_one};

mod actions;

#[derive(Clone)]
pub struct DbState {
    pub client: Arc<Client>,
}

async fn hello_world() -> &'static str {
    "Hello, world test succeeded see user http://127.0.0.1:8000/user"
}

async fn swagger_ui_handler() -> impl IntoResponse {
    let spec_url = "/swagger/openapi.json";
    Html(swagger_ui(spec_url))
}

async fn openapi_json_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/json")],
        include_str!("openapi.json"),
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let (client, connection) = tokio_postgres::connect("", NoTls)
        .await
        .expect("Failed to connect to the database");

    // Spawn the connection handler
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let state = DbState {
        client: Arc::new(client),
    };

    let doc_url = "/swagger/openapi.json";
    let app = Router::new()
        .route("/swagger", get(swagger_ui_handler))
        .route(doc_url, get(openapi_json_handler))
        .route("/", get(hello_world))
        .route("/user", get(select))
        .route("/user",post(insert_one))
        .route("/user",put(update_one))
        .route("/user/:id",delete(delete_one))
        .with_state(state);

    Ok(app.into())
}