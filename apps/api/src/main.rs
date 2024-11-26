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

#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("{}:{}", ip, port)
        .parse::<SocketAddr>()
        .expect("Invalid IP or Port");

    let listener = TcpListener::bind(address).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app(&ip, &port)).await.unwrap();
}

fn app(ip: &str, port: &str) -> Router {
    Router::new().route("/", get(routes::index)).layer(
        CorsLayer::new()
            .allow_origin(format!("http://{ip}:{port}")
                .parse::<HeaderValue>()
                .expect("Invalid HeaderValue"))
            .allow_methods(Method::GET),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use dotenv::dotenv;
    use http_body_util::BodyExt;
    use tower::ServiceExt;
    use std::env;

    #[tokio::test]
    async fn hello_world() {
        dotenv().ok(); 
        
        let ip = env::var("IP").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
        let app = app(&ip, &port);
        let request = Request::builder().uri("/").body(Body::empty()).unwrap();
        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, world!");
    }
}
