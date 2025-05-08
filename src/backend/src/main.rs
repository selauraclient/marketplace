use axum::{
    routing::get,
    Json, Router,
};
use serde::Serialize;
use std::net::SocketAddr;
use tower_http::cors::{CorsLayer, Any};

#[derive(Serialize)]
struct ModInfo {
    name: String,
    description: String,
}

async fn list_mods() -> Json<Vec<ModInfo>> {
    Json(vec![
        ModInfo {
            name: "Better Grass".into(),
            description: "Makes grass greener and better.".into(),
        },
        ModInfo {
            name: "No Particles".into(),
            description: "Removes particles throughout the game.".into(),
        },
    ])
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/mods", get(list_mods))
        .layer(CorsLayer::new().allow_origin(Any));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Server running at http://{}", addr);

    hyper::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
