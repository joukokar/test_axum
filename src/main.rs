use axum::{extract::State, routing::get, Router};
use std::sync::{
    atomic::{AtomicU32, Ordering},
    Arc,
};

#[derive(Debug)]
struct AppState {
    visitor_count: AtomicU32,
}

#[tokio::main]
async fn main() {
    let shared_state = Arc::new(AppState {
        visitor_count: 0.into(),
    });
    // build our application with a single route
    let app = Router::new().route("/", get(handler).with_state(shared_state));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(State(state): State<Arc<AppState>>) -> String {
    let new_state = state.as_ref();
    new_state.visitor_count.fetch_add(1, Ordering::SeqCst);
    println!("{:?}", new_state.visitor_count);
    let c = new_state.visitor_count.load(Ordering::SeqCst);
    let respo = format!("There have been {} visitors.", c);
    respo
}
