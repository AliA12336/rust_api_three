use std::sync::{Arc, Mutex};

use axum::{
    Json, Router, extract::State, response::IntoResponse, routing::get
};
use serde::Serialize;


#[derive(Clone, Serialize)]
struct Book {
    id: usize,
    title: String,
    status: Status,
}

type Books = Arc<Mutex<Vec<Book>>>;

#[derive(Clone, Serialize)]
enum Status {
    Reading,
    Completed, 
    Soon,
}



#[axum::debug_handler]
async fn get_books(State(books): State<Books>) -> impl IntoResponse {
    let book_lock = books.lock().unwrap();
    Json(book_lock.clone())
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(Vec::<Book>::new()));
    // build our application with a single route
    let app = Router::new().route("/books", get(get_books)).with_state(state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}