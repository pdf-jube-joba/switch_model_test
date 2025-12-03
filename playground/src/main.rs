use axum::{routing::{get, post}, Json, Router, response::Html};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr};
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct Req { text: String }
#[derive(Serialize)]
struct Resp { result: String }

static INDEX_HTML: &str = include_str!("../index.html");

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { Html(INDEX_HTML) }))
        .route("/run", post(run));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("→ open http://{}", addr);
    axum::serve(TcpListener::bind(addr).await.unwrap(), app).await.unwrap();
}

async fn run(Json(req): Json<Req>) -> Json<Resp> {
    // --- テキスト処理本体 ---
    // 将来重くなる場合は:
    //   tokio::task::spawn_blocking(move || heavy_process(req.text))
    // のようにスレッドプールで処理を逃す。
    Json(Resp { result: req.text.to_uppercase() })
}