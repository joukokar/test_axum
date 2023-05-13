use axum::{
    extract::{Path, State},
    routing::get,
    Router,
};
use chrono::Utc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

#[derive(Debug)]
struct AppState {
    visitor_count: AtomicU32,
    pool: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost:25432/postgres")
        .await
        .unwrap();

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool)
        .await
        .unwrap();
    println!("{:?}", row.0);

    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS posts (
        id SERIAL PRIMARY KEY,  -- unique identifier
        title VARCHAR(255) NOT NULL,  -- post title
        content TEXT NOT NULL,  -- post content
        author VARCHAR(100),  -- author's name
        created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,  -- creation timestamp
        updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP  -- last update timestamp
    );"#,
    )
    .execute(&pool)
    .await
    .unwrap();

    // sqlx::migrate!("./migrations")
    //     .run(&pool)
    //     .await
    //     .unwrap();

    let shared_state = Arc::new(AppState {
        visitor_count: 0.into(),
        pool: pool,
    });
    // build our application with a single route
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/:user_id", get(handler))
        .with_state(shared_state);
    // .route("/posts", get(get_posts));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root_handler(State(state): State<Arc<AppState>>) -> String {
    let new_state = state.as_ref();
    new_state.visitor_count.fetch_add(1, Ordering::SeqCst);
    format!(
        "Hello, World! {}",
        new_state.visitor_count.load(Ordering::SeqCst)
    )
}

async fn handler(Path(user_id): Path<String>, State(state): State<Arc<AppState>>) -> String {
    println!("user_id: {}", user_id);
    let new_state = state.as_ref();
    new_state.visitor_count.fetch_add(1, Ordering::SeqCst);
    println!("{:?}", new_state.visitor_count);
    let c = new_state.visitor_count.load(Ordering::SeqCst);

    let rows = get_posts(&new_state.pool).await;

    for row in rows.iter() {
        println!("title: {}", row.title);
    }

    let respo = format!("There have been {} visitors.\n{:?}", c, rows);

    respo
}

async fn get_posts(pool: &Pool<Postgres>) -> Vec<Post> {
    let rows = sqlx::query_as!(
        Post,
        "SELECT id, title, content, author, created_at FROM posts"
    )
    .fetch_all(pool)
    .await
    .unwrap();
    rows
}

#[derive(Debug)]
struct Post {
    id: i32,
    title: String,
    content: String,
    author: Option<String>,
    created_at: Option<chrono::DateTime<Utc>>,
}
