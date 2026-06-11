use network_interface::NetworkInterface;
use network_interface::NetworkInterfaceConfig;

fn main() {
    print_hostname();
    print_network_interfaces();
}

fn print_hostname() {
    if let Ok(h) = hostname::get() {
        println!("hostname: {:?}", h);
    }
}

fn print_network_interfaces() {
    let network_interfaces = NetworkInterface::show().unwrap();
    for iface in network_interfaces.iter() {
        println!("{:?}", iface);
    }
}
