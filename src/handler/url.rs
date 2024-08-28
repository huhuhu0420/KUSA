use axum::http::StatusCode;

#[utoipa::path(
    post,
    path = "/url",
    tag = "URL",
    request_body(content = String, description = "target url", example = json!("https://example.com")),
    responses(
        (status = 200, description = "Ok", body = String, example = json!("https://૮ • ﻌ - ა")),
        (status = 400, description = "Bad Request"),
        (status = 500, description = "Internal Server Error")
    )
)]
pub async fn get_shorten_url(body: String) -> (StatusCode, String) {
    (StatusCode::OK, "https://૮ • ﻌ - ა".to_string())
}