use axum::{
    routing::{get, post},
    Router,
};

use crate::handler::{get_shorten_url, ApiDoc};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub struct Application {
    port: u16,
    router: Router,
}

impl Application {
    pub async fn build() -> Self {
        let router = Router::new()
            .route("/url", get(get_shorten_url))
            .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));
        Self { port: 3000, router }
    }

    pub async fn run(self) {
        let addr = std::net::SocketAddr::from(([0, 0, 0, 0], self.port));
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        axum::serve(listener, self.router.clone()).await.unwrap();
    }
}
