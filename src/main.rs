use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::{IntoResponse, Html},
};
use axum_swagger_ui::swagger_ui;

async fn hello_world() -> &'static str {
    "Hello, world test succeed"
}

fn custom_swagger_ui(openapi_url: &str) -> Html<String> {
    Html(format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>SwaggerUI</title>
    <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui.css" />
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@4.5.0/swagger-ui-bundle.js" crossorigin></script>
    <script>
        window.onload = () => {{
            window.ui = SwaggerUIBundle({{
                url: '{}',
                dom_id: '#swagger-ui',
                presets: [
                    SwaggerUIBundle.presets.apis,
                    SwaggerUIBundle.SwaggerUIStandalonePreset
                ],
                layout: "BaseLayout",
            }});
        }};
    </script>
</body>
</html>
        "#,
        openapi_url
    ))
}

async fn swagger_ui_handler() -> impl IntoResponse {
    custom_swagger_ui("/swagger/openapi.json")
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
    let doc_url = "/swagger/openapi.json";
    let app = Router::new()
        .route("/swagger", get(swagger_ui_handler))
        .route(doc_url, get(openapi_json_handler))
        .route("/", get(hello_world));

    Ok(app.into())
}