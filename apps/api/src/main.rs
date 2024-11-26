use std::net::SocketAddr;

use axum::{
    http::{HeaderValue, Method},
    routing::get,
    Router,
};
use dotenv::dotenv;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

mod routes;

const PORT: u16 = 3000;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let address = SocketAddr::from(([127, 0, 0, 1], PORT));

    let listener = TcpListener::bind(address).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new().route("/", get(routes::index)).layer(
        CorsLayer::new()
            .allow_origin(format!("http://localhost:{PORT}").parse::<HeaderValue>().unwrap())
            .allow_methods(Method::GET),
    )
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    #[tokio::test]
    async fn hello_world() {
        let app = super::app();

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();

        assert_eq!(&body[..], b"Hello, world!");
    }
}
