use axum::{
    routing::{get},
    Router,
};
use axum_server::tls_rustls::RustlsConfig;

async fn handler() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let config = RustlsConfig::from_pem_file("cert.pem", "key.pem")
        .await.unwrap();

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8443").await.unwrap();
    println!("port {:?}", listener.local_addr().unwrap().port());
    let std_listener = listener.into_std().unwrap();

    axum_server::from_tcp_rustls(std_listener, config)
        .unwrap()
        .serve(app.into_make_service())
        .await
        .unwrap();
}
