use axum::{http::Response, Router};
use hyper::Body;
use std::net::SocketAddr;
use tokio::fs::read;

async fn serve_static_file(path: &str) -> Result<Response<Body>, axum::BoxError> {
    match read(path).await {
        Ok(bytes) => Ok(Response::new(Body::from(bytes))),
        Err(_) => Ok(Response::builder()
            .status(404)
            .body(Body::from("404 - Not Found"))
            .unwrap()),
    }
}

async fn index() -> Response<Body> {
    Response::new(Body::from("<h1>Hello, Axum!</h1>"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", index)
        .route("/styles.css", serve_static_file("static/styles.css"))
        .route("/app.js", serve_static_file("static/app.js"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());
    println!("Listening on http://{}", addr);
    server.await.expect("Server failed");
}
