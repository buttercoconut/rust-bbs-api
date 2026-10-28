mod models;
mod routes;
mod state;

use axum::{
    routing::{delete, get, post, put},
    Router,
};
use state::AppState;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    // 인메모리 저장소 초기화
    let state = AppState {
        posts: std::sync::Arc::new(tokio::sync::RwLock::new(Vec::new())),
    };

    // 라우트 구성
    let app = Router::new()
        .route("/posts", get(routes::list_posts).post(routes::create_post))
        .route(
            "/posts/:id",
            get(routes::get_post)
                .put(routes::update_post)
                .delete(routes::delete_post),
        )
        .with_state(state)
        .layer(CorsLayer::permissive());

    let addr = "0.0.0.0:3000";
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("🚀 BBS API server running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
