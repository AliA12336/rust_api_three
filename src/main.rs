use std::sync::{Arc, Mutex};

use axum::{
    Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::get
};
use serde::{Deserialize, Serialize};


#[derive(Clone, Serialize)]
struct Book {
    id: usize,
    title: String,
    status: Status,
}

type Books = Arc<Mutex<Vec<Book>>>;

#[derive(Clone, Serialize, Deserialize)]
enum Status {
    Reading,
    Completed, 
    Soon,
}

#[derive(Deserialize)]
struct PostBooks {
    title: String,
    status: Status,
}

enum AppError {
    Validation(String),
    ServerError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::ServerError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, Json( msg )).into_response(),
            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, Json( msg)).into_response(),
        }
    }
}


#[axum::debug_handler]
async fn get_books(State(books): State<Books>) -> Result<impl IntoResponse, AppError> {
    let book_lock = books.lock().unwrap();
    Ok(Json(book_lock.clone()))
}

#[axum::debug_handler]
async fn post_book_handler(State(books): State<Books>, Json(payload): Json<PostBooks>) -> Json<Book> {
    // let maybe_input = validate_input(&payload.title);
    

    
    let mut book_lock = books.lock().unwrap();
    let book_id = book_lock.len() + 1;
    let new_book = Book {
        id: book_id,
        // TODO add a match and replace this part after adding error handling
        title: payload.title,
        status: payload.status,
    };

    book_lock.push(new_book.clone());
    Json(new_book)
}

async fn validate_input(title: &str) -> Result<&str, ()>{
    let trimmed_title = title.trim();
    if trimmed_title.len() == 0 {
        return Err({})
    }
    
    Ok(trimmed_title)
}

async fn lock_books() {

}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(Vec::<Book>::new()));
    // build our application with a single route
    let app = Router::new().route("/books", get(get_books).post(post_book_handler)).with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}