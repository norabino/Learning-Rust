use axum::{Json, Router};
use axum::routing::{get, post};
use axum::http::StatusCode;
use axum::extract::{Path, State};

use serde::{Serialize, Deserialize};
use tokio::net::TcpListener;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// L'attribut derive génère automatiquement le code pour convertir cette struct en JSON
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    username: String,
    email: String,
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
    email: String,
}

#[derive(Clone)]
struct AppState {
    db: Arc<Mutex<HashMap<u64, User>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(hello_world))
        .route("/user", get(get_user))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user_by_id))
        .with_state(state);

    // Préparation du port d'écoute
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("🚀 Serveur démarré sur http://127.0.0.1:3000");

    // Lancement du serveur
    axum::serve(listener, app).await.unwrap();
}

async fn hello_world() -> &'static str {
    "Hello, Rustacean!"
}

async fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        username: String::from("norabino"),
        email: String::from("norabino@example.com"),
    };
    Json(user)
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let mut db = state.db.lock().unwrap();
    let new_user = User {
        id: if !db.is_empty() { (db.len() + 1) as u64 } else { 1 }, // <-- Virgule ajoutée ici
        username: payload.username,
        email: payload.email,
    };

    db.insert(new_user.id, new_user.clone());
    (StatusCode::CREATED, Json(new_user))
}

async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<User>, StatusCode> {
    let db = state.db.lock().unwrap(); // <-- Retrait de 'mut' inutilisé

    match db.get(&id) {
        Some(user) => Ok(Json(user.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}