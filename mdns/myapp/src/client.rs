use mdns_sd::{ServiceDaemon, ServiceEvent};
use tracing::info;

async fn request_hello(hostname: &str, port: u16) {
    let url = format!("http://{hostname}:{port}/");
    info!("Sending request to {url}...");
    let body = reqwest::get(&url)
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    info!("Received response from {hostname}:{port}: {body}");
}

async fn discover_services() {
    let mdns = ServiceDaemon::new().unwrap();
    let receiver = mdns.browse("_myapp._tcp.local.").unwrap();
    info!("Browsing for services of type _myapp._tcp.local...");
    while let Ok(event) = receiver.recv() {
        match event {
            ServiceEvent::ServiceFound(stype, name) => {
                info!("Service found: {stype}, {name}");
            }
            ServiceEvent::ServiceResolved(info) => {
                let name = info.get_fullname();
                let hostname = info.get_hostname();
                let addr = info.get_addresses().iter().find(|a| a.is_ipv4()).map(|a| a.to_string()).unwrap_or_else(|| "127.0.0.1".to_string());
                let port = info.get_port();
                info!("Service resolved: {name} on port {port} (ip: {addr}, hostname: {hostname})");
                request_hello(&addr, port).await;
            }
            ServiceEvent::ServiceRemoved(stype, fullname) => {
                info!("Service removed: {stype}, {fullname}");
            }
            _ => {}
        }
    }
    info!("Stopping service discovery...");
    mdns.shutdown().unwrap();
}

fn main() {
    tracing_subscriber::fmt::init();
    info!("Discovering services...");
    tokio::runtime::Runtime::new().unwrap().block_on(discover_services());
    info!("Service discovery stopped.");
}