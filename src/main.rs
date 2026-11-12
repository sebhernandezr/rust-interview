use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use bcrypt::hash;

#[derive(Clone, Serialize)]
struct User {
    email: String,
    password_hash: String,
}

#[derive(Deserialize)]
struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Clone)]
struct AppState {
    users: Arc<Mutex<HashMap<String, User>>>,
}

// HTTP client for the email service (works like reqwest::Client)
struct EmailClient;

impl EmailClient {
    fn new() -> Self {
        EmailClient
    }

    async fn send_welcome(&self, _email: &str) -> Result<(), String> {
        tokio::time::sleep(Duration::from_millis(100)).await;
        Ok(())
    }
}

#[axum::debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Json<User> {
    let mut users = state.users.lock().unwrap();
    if users.contains_key(&req.email) {
        panic!("user already exists");
    }

    let password_hash = hash(&req.password, 12).unwrap(); // ~250 ms of CPU

    let client = EmailClient::new();
    client.send_welcome(&req.email).await.unwrap();

    let user = User { email: req.email.clone(), password_hash };
    users.insert(req.email, user.clone());
    Json(user)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
    };
    let app = Router::new()
        .route("/register", post(register))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}