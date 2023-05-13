use std::sync::Arc;

use sqlx::{postgres::PgPoolOptions, Postgres, Pool};

use axum::{
    routing::get,
    Router,
    Json, http::StatusCode, extract::State,
};


async fn hello_handler(
    State(pool): State<Arc<Pool<Postgres>>>
) -> (StatusCode, Json<()>) {
    println!("helole");

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&*pool).await.unwrap();

    println!("Row: {}", row.0);

    (StatusCode::OK, Json(()))
}



#[tokio::main]
async fn main() {
    let pool = Arc::new(PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:password@127.0.0.1:5432/database").await.unwrap());

    println!("pool done");

    // build our application with a single route
    let app = Router::new().route("/", get(hello_handler)).with_state(pool);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
