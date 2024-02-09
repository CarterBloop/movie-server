use std::collections::HashMap;
use axum::{
    extract::{Path, Extension},
    routing::{get, post},
    Router,
    Json,
    http::StatusCode,
};
use std::sync::{Arc, Mutex};
use axum_macros::debug_handler;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct Movie {
    id: String,
    name: String,
    year: u16,
    was_good: bool
}

#[tokio::main]
async fn main() {
    // Create Axum server with the following endpoints:
    // 1. GET /movie/{id} - This should return back a movie given the id
    // 2. POST /movie - this should save move in a DB (HashMap<String, Movie>). This movie will be sent
    // via a JSON payload. 
    
    // As a bonus: implement a caching layer so we don't need to make expensive "DB" lookups, etc.

    let mut hashmap: HashMap<String,Movie> = HashMap::new();
    // Insert dummy data
    hashmap.insert(String::from("1"),
        Movie {
            id: String::from("1"),
            name: String::from("Movie"),
            year: 2000,
            was_good: true
        }
    );

    // DB and Cache layers
    let db = Arc::new(Mutex::new(hashmap));
    let cache = Arc::new(Mutex::new(HashMap::<String,Movie>::new()));

    let app = Router::new()
        .route("/movie/:id", get(get_movie))
        .route("/movie", post(create_movie))
        .layer(Extension(db))
        .layer(Extension(cache));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[debug_handler]
async fn get_movie(
    Extension(db): Extension<Arc<Mutex<HashMap<String, Movie>>>>,
    Extension(cache): Extension<Arc<Mutex<HashMap<String, Movie>>>>,
    Path(id): Path<String>,
) -> Result<Json<Option<Movie>>, (StatusCode, String)> {
    { // Check cache
        let cache = cache.lock().unwrap();
        if let Some(movie) = cache.get(&id) {
            return Ok(Json(Some(movie.clone())));
        }
    }
    // Check DB
    let db = db.lock().unwrap();
    let movie = db.get(&id).cloned();

    if movie.is_some() {
        let mut cache = cache.lock().unwrap();
        cache.insert(id.clone(), movie.clone().unwrap());
        Ok(Json(movie))
    } else {
        // 404 Error
        Err((StatusCode::NOT_FOUND, format!("Movie with id {} not found", id)))
    }
    
}

#[debug_handler]
async fn create_movie(
    Extension(db): Extension<Arc<Mutex<HashMap<String, Movie>>>>,
    Json(new_movie): Json<Movie>,
) -> Result<StatusCode, (StatusCode, String)> {
    // Insert into db
    let mut db = db.lock().unwrap();
    db.insert(new_movie.id.clone(), new_movie);
    Ok(StatusCode::CREATED)
}