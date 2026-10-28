use std::sync::Arc;
use tokio::sync::RwLock;

use crate::models::Post;

/// 전역 애플리케이션 상태 (인메모리 저장소)
///
/// 실제 프로덕션에서는 이 부분을 PostgreSQL, SQLite, Redis 등으로 교체하세요.
#[derive(Clone)]
pub struct AppState {
    pub posts: Arc<RwLock<Vec<Post>>>,
}
