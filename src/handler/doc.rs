use crate::handler::url;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(paths(url::get_shorten_url,))]
pub struct ApiDoc;
