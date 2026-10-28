use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;

use crate::models::{CreatePostRequest, Post, UpdatePostRequest};
use crate::state::AppState;

/// GET /posts — 전체 글 목록 조회
pub async fn list_posts(State(state): State<AppState>) -> Json<Vec<Post>> {
    let posts = state.posts.read().await;
    Json(posts.clone())
}

/// GET /posts/:id — 특정 글 조회
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let posts = state.posts.read().await;
    posts
        .iter()
        .find(|p| p.id == id)
        .map(|p| Json(p.clone()))
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Post not found".to_string()))
}

/// POST /posts — 새 글 생성
pub async fn create_post(
    State(state): State<AppState>,
    Json(req): Json<CreatePostRequest>,
) -> Result<(StatusCode, Json<Post>), (StatusCode, String)> {
    // 입력값 검증
    if req.title.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Title is required".to_string()));
    }
    if req.content.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Content is required".to_string()));
    }
    if req.author.trim().is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Author is required".to_string()));
    }

    let now = chrono::Utc::now();
    let post = Post {
        id: Uuid::new_v4(),
        title: req.title,
        content: req.content,
        author: req.author,
        created_at: now,
        updated_at: now,
    };

    let mut posts = state.posts.write().await;
    posts.push(post.clone());
    Ok((StatusCode::CREATED, Json(post)))
}

/// PUT /posts/:id — 글 수정 (부분 수정 지원)
pub async fn update_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePostRequest>,
) -> Result<Json<Post>, (StatusCode, String)> {
    let mut posts = state.posts.write().await;
    let post = posts
        .iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Post not found".to_string()))?;

    if let Some(title) = req.title {
        if title.trim().is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Title cannot be empty".to_string()));
        }
        post.title = title;
    }
    if let Some(content) = req.content {
        if content.trim().is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Content cannot be empty".to_string()));
        }
        post.content = content;
    }
    if let Some(author) = req.author {
        if author.trim().is_empty() {
            return Err((StatusCode::BAD_REQUEST, "Author cannot be empty".to_string()));
        }
        post.author = author;
    }
    post.updated_at = chrono::Utc::now();

    Ok(Json(post.clone()))
}

/// DELETE /posts/:id — 글 삭제
pub async fn delete_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let mut posts = state.posts.write().await;
    let len_before = posts.len();
    posts.retain(|p| p.id != id);
    if posts.len() == len_before {
        return Err((StatusCode::NOT_FOUND, "Post not found".to_string()));
    }
    Ok(StatusCode::NO_CONTENT)
}
