mod user;

use std::sync::{Arc, Mutex};

use crate::user::{User, Users};

use axum::{extract::State, response::Json, routing::get, Router};

#[tokio::main]
async fn main() {
    // users
    let users = Users {
        users: vec![
            User {
                id: 1,
                name: "takashi".to_string(),
            },
            User {
                id: 2,
                name: "hitoshi".to_string(),
            },
            User {
                id: 3,
                name: "masashi".to_string(),
            },
        ],
    };

    let users_state = Arc::new(Mutex::new(users));

    let app = Router::new()
        .route("/", get(hello))
        .route("/users", get(get_users))
        .with_state(users_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn get_users(State(users_state): State<Arc<Mutex<Users>>>) -> Json<Users> {
    let users = users_state.lock().unwrap();
    Json(users.clone())
}
