# Rust BBS API (게시판 CRUD Web API)

Rust + [axum](https://github.com/tokio-rs/axum) 기반 간단한 게시판 CRUD Web API 예제입니다.

## 기술 스택

| 항목 | 사용 |
|------|------|
| 언어 | Rust (edition 2021) |
| Web Framework | axum 0.7 |
| Async Runtime | tokio |
| 직렬화 | serde / serde_json |
| ID 생성 | uuid (v4) |
| 타임스탬프 | chrono |
| CORS | tower-http |

## 실행 방법

```bash
# Rust toolchain 설치 (없으면)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 프로젝트 빌드 & 실행
cargo run
```

서버가 `http://localhost:3000`에서 시작됩니다.

## API 엔드포인트

| Method | Path | 설명 |
|--------|------|------|
| `GET` | `/posts` | 전체 글 목록 |
| `GET` | `/posts/:id` | 특정 글 조회 |
| `POST` | `/posts` | 새 글 생성 |
| `PUT` | `/posts/:id` | 글 수정 (부분 수정 지원) |
| `DELETE` | `/posts/:id` | 글 삭제 |

## 사용 예시 (curl)

### 글 생성
```bash
curl -X POST http://localhost:3000/posts \
  -H "Content-Type: application/json" \
  -d '{"title":"첫 번째 글","content":"안녕하세요, 게시판입니다!","author":"admin"}'
```

### 전체 목록 조회
```bash
curl http://localhost:3000/posts
```

### 특정 글 조회
```bash
curl http://localhost:3000/posts/<UUID>
```

### 글 수정 (부분 수정)
```bash
curl -X PUT http://localhost:3000/posts/<UUID> \
  -H "Content-Type: application/json" \
  -d '{"title":"수정된 제목"}'
```

### 글 삭제
```bash
curl -X DELETE http://localhost:3000/posts/<UUID>
```

## 프로젝트 구조

```
rust-bbs-api/
├── Cargo.toml
├── .gitignore
├── README.md
└── src/
    ├── main.rs      # 진입점, Router 구성
    ├── models.rs    # Post 모델 & DTO
    ├── routes.rs    # CRUD 핸들러
    └── state.rs     # 전역 상태 (인메모리 저장소)
```

## 참고

- 현재 **인메모리 저장소**를 사용합니다. 서버 재시작 시 데이터가 초기화됩니다.
- 프로덕션 사용 시 PostgreSQL / SQLite / Redis 등으로 교체하세요.
- 인증/인가(JWT, OAuth 등)는 예제에서 생략했습니다.
