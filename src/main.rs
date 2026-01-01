use axum::{routing::get, Router};

// Handler for the home page
async fn home() -> &'static str {
    "Welcome to my Rust blog!"
}

// Handler for the about page
async fn about() -> &'static str {
    "About me: I'm learning Rust!"
}

#[tokio::main]
async fn main() {
    // Build our router with routes
    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(about));

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Server running at http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}