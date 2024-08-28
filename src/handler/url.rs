use axum::http::StatusCode;

#[utoipa::path(
    get,
    path = "/url",
    tag = "URL",
    responses(
        (status = 200, description = "Ok", body = String),
        (status = 400, description = "Bad Request", body = String),
        (status = 500, description = "Internal Server Error", body = String)
    )
)]
pub async fn get_shorten_url(url: String) -> (StatusCode, String) {
    (StatusCode::OK, "Shorten URL".to_string())
}