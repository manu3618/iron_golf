mod url_set;
use serde::Deserialize;
use url_set::*;

use axum::{
    Router,
    extract::{Form, Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
struct AppState {
    url_set: Mutex<UrlSet>,
    base_url: String,
}
#[derive(Deserialize, Debug)]
struct Input {
    /// url to shorten
    url: String,
}

/// return the shortened url
async fn shorten(
    State(url_set): State<Arc<AppState>>,
    Form(input): Form<Input>,
) -> impl IntoResponse {
    dbg!(&input);
    let url: Url = match input.url.parse() {
        Ok(v) => v,
        // Err(e) => return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", e)).into()
        Err(e) => {
            return Response::builder()
                .status(StatusCode::NOT_ACCEPTABLE)
                .body(format!("{:?}", e))
                .unwrap();
        }
    };
    let base_url = url_set.base_url.clone();
    let url_set = url_set.url_set.lock();
    let id = url_set.await.store_url(url);
    Response::new(format!("http://{}/{}", base_url, id))
}

/// return redirection to url
async fn get_url(
    State(url_set): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let url_set = url_set.url_set.lock();
    let url = url_set.await.retrieve_refresh(&id.parse().unwrap());
    match url {
        None => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(format!("no shortcut register for {id}"))
            .unwrap(),
        Some(v) => Response::builder()
            .status(StatusCode::PERMANENT_REDIRECT)
            .body(format!("{v}"))
            .unwrap(),
    }
}

#[tokio::main]
async fn main() {
    println!("Starting");
    let host = "0.0.0.0:3000";
    let url_set = Arc::new(AppState {
        url_set: Mutex::new(UrlSet::new()),
        base_url: String::from(host),
    });
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/{id}", get(get_url))
        .route("/shorten", post(shorten))
        .with_state(Arc::clone(&url_set));

    let listener = tokio::net::TcpListener::bind(host).await.unwrap();
    println!("listening");
    axum::serve(listener, app).await.unwrap();
}
