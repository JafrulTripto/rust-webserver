use axum::{extract::Query, response::{Html, IntoResponse}, routing::{get, get_service}, Router};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .merge(routes_all())
        .fallback_service(routes_static());

    let listner = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Running webserver at {}", listner.local_addr().unwrap());

    axum::serve(listner, router).await.unwrap();
}

fn routes_all() ->Router {
    Router::new()
        .route("/hello", get(get_home))
}
fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>
}

async fn get_home(param: Query<HelloParams>) -> impl IntoResponse {
    let name = param.name.as_deref().unwrap_or("World");
    Html(format!("<div><h1>Hello</h1><strong>{}</strong></div>", name))
}
