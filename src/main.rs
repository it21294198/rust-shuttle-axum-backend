use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::{IntoResponse, Html},
};
use axum_swagger_ui::swagger_ui;

async fn hello_world() -> &'static str {
    "Hello, world test succeeded"
}

async fn swagger_ui_handler() -> impl IntoResponse {
    // Provide the URL for the OpenAPI specification
    let spec_url = "/swagger/openapi.json";
    // Call the swagger_ui function with the spec_url
    Html(swagger_ui(spec_url))  // Wrap the response in Html
}

async fn openapi_json_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        [("Content-Type", "application/json")],
        include_str!("openapi.json"), // Ensure this file is correctly located
    )
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let doc_url = "/swagger/openapi.json";
    let app = Router::new()
        .route("/swagger", get(swagger_ui_handler)) // Serve the Swagger UI at this route
        .route(doc_url, get(openapi_json_handler))  // Serve the OpenAPI JSON spec
        .route("/", get(hello_world));               // Your other route

    Ok(app.into())
}
