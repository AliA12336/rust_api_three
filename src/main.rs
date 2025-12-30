use std::sync::{Arc, Mutex};

use axum::{
    Error, Json, Router, extract::State, response::IntoResponse, routing::get
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


#[axum::debug_handler]
async fn get_books(State(books): State<Books>) -> impl IntoResponse {
    let book_lock = books.lock().unwrap();
    Json(book_lock.clone())
}

#[axum::debug_handler]
async fn post_book_handler(State(books): State<Books>, Json(payload): Json<PostBooks>) -> Json<Book> {
    let mut book_lock = books.lock().unwrap();
    let book_id = book_lock.len() + 1;
    let new_book = Book {
        id: book_id,
        title: payload.title,
        status: payload.status,
    };

    book_lock.push(new_book.clone());
    Json(new_book)
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