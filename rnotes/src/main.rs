use axum::{extract::Path, response::IntoResponse, routing::get, Json, Router};
use std::fs;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/notes/:id", get(notes));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> impl IntoResponse {
    Json("<h1>Hello, World!</h1>")
}

async fn notes(
    Path(name): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let notes_dir = "C:/source/vaults/JDTalley/Journals/2024/W7 Journal.md";
    let name = name.to_string();

    println!("In file {}", notes_dir);

    let contents = fs::read_to_string(notes_dir).expect("Unable to open file");

    println!("{}", markdown::to_html(&contents));
    Json(markdown::to_html(&contents))
}
