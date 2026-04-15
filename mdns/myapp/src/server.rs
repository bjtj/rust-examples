use axum::{routing::get, Router};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use tracing::info;

async fn start_server() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let _ = axum::serve(listener, app).await;
}

fn reg_service(service_type: &str, hostname: &str, ip: [u8; 4], port: u16) -> Result<ServiceDaemon, Box<dyn std::error::Error>> {
    info!("Registering mDNS service with type: {service_type}, hostname: {hostname}, IP: {}.{}.{}.{}:{port}", ip[0], ip[1], ip[2], ip[3]);
    let mdns = ServiceDaemon::new()?;
    let props = [("path", "/")];
    let ip_str = format!("{}.{}.{}.{}", ip[0], ip[1], ip[2], ip[3]);
    let service_info = ServiceInfo::new(
        service_type,
        "My Rust App Service",
        hostname,
        &ip_str,
        port,
        &props[..],
    )
    .unwrap();
    mdns.register(service_info)?;
    info!("mDNS service registered successfully.");

    Ok(mdns)
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    info!("Starting server...");
    tokio::spawn(async {
        info!("Server is starting...");
        start_server().await;
        info!("Server has stopped.");
    });

    info!("Registering mDNS service...");
    let local_ip = local_ip_address::local_ip().unwrap_or_else(|_| "127.0.0.1".parse().unwrap());
    let ip_arr = if let std::net::IpAddr::V4(v4) = local_ip {
        v4.octets()
    } else {
        [127, 0, 0, 1]
    };
    let hostname = hostname::get().map(|h| h.to_string_lossy().to_string()).unwrap_or_else(|_| "localhost".to_string());
    let hostname_local = format!("{}.local.", hostname);
    let _mdns = reg_service(
        "_myapp._tcp.local.",
        &hostname_local,
        ip_arr,
        3000);

    info!("Server is running. Press Ctrl+C to stop.");
    tokio::signal::ctrl_c().await.unwrap();
}